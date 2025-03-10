//! The name table

include!("../../generated/generated_name.rs");

impl Name {
    fn compute_storage_offset(&self) -> u16 {
        let v0 = 6 // version, count, storage_offset
            + self.name_record.len() * 12;
        if let Some(lang_tag_records) = self.lang_tag_record.as_ref() {
            v0 + 4 * lang_tag_records.len()
        } else {
            v0
        }
        .try_into()
        .unwrap()
    }
}

impl NameRecord {
    fn string(&self) -> &str {
        self.string_offset
            .get()
            .map(|s| s.as_str())
            .unwrap_or_default()
    }

    fn string_writer(&self) -> NameStringWriter {
        NameStringWriter {
            encoding: Encoding::new(self.platform_id, self.encoding_id),
            string: self.string(),
        }
    }

    fn validate_string_data(&self, ctx: &mut ValidationCtx) {
        let encoding = Encoding::new(self.platform_id, self.encoding_id);
        match encoding {
            Encoding::Unknown => ctx.report(format!(
                "Unhandled platform/encoding id pair: ({}, {})",
                self.platform_id, self.encoding_id
            )),
            Encoding::Utf16Be => (), // lgtm
            Encoding::MacRoman => {
                for c in self.string().chars() {
                    let raw_c = c as u32;
                    match raw_c {
                        0..=127 => (),
                        128.. => {
                            let raw_c = raw_c as u16;
                            if MAC_ROMAN.binary_search(&raw_c).is_err() {
                                ctx.report(format!(
                                    "char {c} {} not representable in MacRoman encoding",
                                    c.escape_unicode()
                                ))
                            }
                        }
                    }
                }
            }
        }
    }
}

impl FontWrite for NameRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.platform_id.write_into(writer);
        self.encoding_id.write_into(writer);
        self.language_id.write_into(writer);
        self.name_id.write_into(writer);
        let string_writer = self.string_writer();
        string_writer.compute_length().write_into(writer);
        writer.write_offset(&string_writer, 2);
    }
}

impl LangTagRecord {
    fn lang_tag(&self) -> &str {
        self.lang_tag_offset
            .get()
            .map(|s| s.as_str())
            .unwrap_or_default()
    }

    fn string_writer(&self) -> NameStringWriter {
        NameStringWriter {
            encoding: Encoding::Utf16Be,
            string: self.lang_tag(),
        }
    }
}

impl FontWrite for LangTagRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        let string_writer = self.string_writer();
        string_writer.compute_length().write_into(writer);
        writer.write_offset(&string_writer, 2);
    }
}

struct NameStringWriter<'a> {
    encoding: Encoding,
    string: &'a str,
}

impl NameStringWriter<'_> {
    fn compute_length(&self) -> u16 {
        match self.encoding {
            Encoding::Utf16Be => self.string.chars().map(|c| c.len_utf16() as u16 * 2).sum(),
            // this will be correct assuming we pass validation
            Encoding::MacRoman => self.string.len().try_into().unwrap(),
            Encoding::Unknown => 0,
        }
    }
}

impl FontWrite for NameStringWriter<'_> {
    fn write_into(&self, writer: &mut TableWriter) {
        for c in self.string.chars() {
            match self.encoding {
                Encoding::Utf16Be => {
                    let mut buf = [0, 0];
                    let enc = c.encode_utf16(&mut buf);
                    enc.iter()
                        .for_each(|unit| writer.write_slice(&unit.to_be_bytes()))
                }
                Encoding::MacRoman => {
                    let raw_c = c as u32;
                    let raw_c: u16 = raw_c.try_into().expect("validate plz");
                    match raw_c {
                        0..=127 => writer.write_slice(&[raw_c as u8]),
                        other => match MAC_ROMAN.binary_search(&other) {
                            Ok(idx) => writer.write_slice(&[128 + idx as u8]),
                            Err(_) => panic!("invalid character for MacRoman"),
                        },
                    }
                }
                Encoding::Unknown => panic!("unknown encoding"),
            }
        }
    }
}

#[cfg(feature = "parsing")]
impl FromObjRef<read_fonts::tables::name::NameString<'_>> for String {
    fn from_obj_ref(obj: &read_fonts::tables::name::NameString<'_>, _: FontData) -> Self {
        obj.chars().collect()
    }
}

#[cfg(feature = "parsing")]
impl FromTableRef<read_fonts::tables::name::NameString<'_>> for String {}

impl PartialEq for NameRecord {
    fn eq(&self, other: &Self) -> bool {
        self.platform_id == other.platform_id
            && self.encoding_id == other.encoding_id
            && self.language_id == other.language_id
            && self.name_id == other.name_id
            && self.string_offset.get() == other.string_offset.get()
    }
}

impl Eq for NameRecord {}

impl Ord for NameRecord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (
            self.platform_id,
            self.encoding_id,
            self.language_id,
            self.name_id,
        )
            .cmp(&(
                other.platform_id,
                other.encoding_id,
                other.language_id,
                other.name_id,
            ))
    }
}

impl PartialOrd for NameRecord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
//FIXME: things like this should live in font-types? or like.. font-utils or something?
#[derive(Copy, Clone)]
enum Encoding {
    Utf16Be,
    MacRoman,
    Unknown,
}

impl Encoding {
    pub fn new(platform_id: u16, encoding_id: u16) -> Encoding {
        match (platform_id, encoding_id) {
            (0, _) => Encoding::Utf16Be,
            (1, 0) => Encoding::MacRoman,
            (3, 0) => Encoding::Utf16Be,
            (3, 1) => Encoding::Utf16Be,
            (3, 10) => Encoding::Utf16Be,
            _ => Encoding::Unknown,
        }
    }
}

#[rustfmt::skip]
const MAC_ROMAN: [u16; 128] = [
    196, 197, 199, 201, 209, 214, 220, 225, 224, 226, 228, 227, 229, 231, 233,
    232, 234, 235, 237, 236, 238, 239, 241, 243, 242, 244, 246, 245, 250, 249,
    251, 252, 8224, 176, 162, 163, 167, 8226, 182, 223, 174, 169, 8482, 180,
    168, 8800, 198, 216, 8734, 177, 8804, 8805, 165, 181, 8706, 8721, 8719,
    960, 8747, 170, 186, 937, 230, 248, 191, 161, 172, 8730, 402, 8776, 8710,
    171, 187, 8230, 160, 192, 195, 213, 338, 339, 8211, 8212, 8220, 8221, 8216,
    8217, 247, 9674, 255, 376, 8260, 8364, 8249, 8250, 64257, 64258, 8225, 183,
    8218, 8222, 8240, 194, 202, 193, 203, 200, 205, 206, 207, 204, 211, 212,
    63743, 210, 218, 219, 217, 305, 710, 732, 175, 728, 729, 730, 184, 733,
    731, 711,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding() {
        let stringthing = NameStringWriter {
            encoding: Encoding::Utf16Be,
            string: "hello",
        };
        assert_eq!(stringthing.compute_length(), 10);
    }

    #[test]
    fn sorting() {
        let mut table = Name {
            version: 0,
            name_record: Default::default(),
            lang_tag_record: None,
        };
        table.name_record.insert(NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0,
            name_id: 1030,
            string_offset: OffsetMarker::new("Ordinær".into()),
        });
        table.name_record.insert(NameRecord {
            platform_id: 0,
            encoding_id: 4,
            language_id: 0,
            name_id: 4,
            string_offset: OffsetMarker::new("oh".into()),
        });
        table.name_record.insert(NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0,
            name_id: 1029,
            string_offset: OffsetMarker::new("Regular".into()),
        });

        let _dumped = crate::dump_table(&table).unwrap();
        #[cfg(feature = "parsing")]
        {
            let loaded = read_fonts::tables::name::Name::read(FontData::new(&_dumped)).unwrap();
            assert_eq!(loaded.name_record()[0].encoding_id, 4);
            assert_eq!(loaded.name_record()[1].name_id, 1029);
            assert_eq!(loaded.name_record()[2].name_id, 1030);
        }
    }

    #[test]
    #[cfg(feature = "parsing")]
    fn roundtrip() {
        #[rustfmt::skip]
        static COLINS_BESPOKE_DATA: &[u8] = &[
            0x0, 0x0, // version
            0x0, 0x03, // count
            0x0, 42, // storage offset
            //record 1:
            0x00, 0x03, // platformID
            0x00, 0x01, // encodingID
            0x04, 0x09, // languageID
            0x00, 0x01, // nameID
            0x00, 0x0a, // length
            0x00, 0x00, // offset
            //record 2:
            0x00, 0x03, // platformID
            0x00, 0x01, // encodingID
            0x04, 0x09, // languageID
            0x00, 0x02, // nameID
            0x00, 0x10, // length
            0x00, 0x0a, // offset
            //record 2:
            0x00, 0x03, // platformID
            0x00, 0x01, // encodingID
            0x04, 0x09, // languageID
            0x00, 0x03, // nameID
            0x00, 0x18, // length
            0x00, 0x1a, // offset
            // storage area:
            // string 1 'colin'
            0x0, 0x63, 0x0, 0x6F, 0x0, 0x6C, 0x0, 0x69,
            0x0, 0x6E,
            // string 2, 'nicelife'
            0x0, 0x6E, 0x0, 0x69, 0x0, 0x63, 0x0, 0x65,
            0x0, 0x6C, 0x0, 0x69, 0x0, 0x66, 0x0, 0x65,
            // string3 'i hate fonts'
            0x0, 0x69, 0x0, 0x20, 0x0, 0x68, 0x0, 0x61,
            0x0, 0x74, 0x0, 0x65, 0x0, 0x20, 0x0, 0x66,
            0x0, 0x6F, 0x0, 0x6E, 0x0, 0x74, 0x0, 0x73,
        ];

        let raw_table =
            read_fonts::tables::name::Name::read(FontData::new(COLINS_BESPOKE_DATA)).unwrap();
        let owned: Name = raw_table.to_owned_table();
        let dumped = crate::dump_table(&owned).unwrap();
        let reloaded = read_fonts::tables::name::Name::read(FontData::new(&dumped)).unwrap();

        for rec in raw_table.name_record() {
            let raw_str = rec.string(raw_table.string_data()).unwrap();
            eprintln!("{raw_str}");
        }

        assert_eq!(raw_table.version(), reloaded.version());
        assert_eq!(raw_table.count(), reloaded.count());
        assert_eq!(raw_table.storage_offset(), reloaded.storage_offset());

        let mut fail = false;
        for (old, new) in raw_table
            .name_record()
            .iter()
            .zip(reloaded.name_record().iter())
        {
            assert_eq!(old.platform_id(), new.platform_id());
            assert_eq!(old.encoding_id(), new.encoding_id());
            assert_eq!(old.language_id(), new.language_id());
            assert_eq!(old.name_id(), new.name_id());
            assert_eq!(old.length(), new.length());
            eprintln!("{:?} {:?}", old.string_offset(), new.string_offset());
            let old_str = old.string(raw_table.string_data()).unwrap();
            let new_str = new.string(reloaded.string_data()).unwrap();
            if old_str != new_str {
                eprintln!("'{old_str}' != '{new_str}'");
                fail = true;
            }
        }
        if fail {
            panic!("some comparisons failed");
        }
    }
}
