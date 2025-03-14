// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct KindsOfOffsetsMarker {
    versioned_nonnullable_offset_byte_start: Option<usize>,
    versioned_nullable_offset_byte_start: Option<usize>,
}

impl KindsOfOffsetsMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn nonnullable_offset_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn nullable_offset_byte_range(&self) -> Range<usize> {
        let start = self.nonnullable_offset_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn versioned_nonnullable_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_nonnullable_offset_byte_start?;
        Some(start..start + Offset16::RAW_BYTE_LEN)
    }
    fn versioned_nullable_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_nullable_offset_byte_start?;
        Some(start..start + Offset16::RAW_BYTE_LEN)
    }
}

impl<'a> FontRead<'a> for KindsOfOffsets<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let version: MajorMinor = cursor.read()?;
        cursor.advance::<Offset16>();
        cursor.advance::<Offset16>();
        let versioned_nonnullable_offset_byte_start = version
            .compatible(MajorMinor::VERSION_1_1)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(MajorMinor::VERSION_1_1)
            .then(|| cursor.advance::<Offset16>());
        let versioned_nullable_offset_byte_start = version
            .compatible(MajorMinor::VERSION_1_1)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(MajorMinor::VERSION_1_1)
            .then(|| cursor.advance::<Offset16>());
        cursor.finish(KindsOfOffsetsMarker {
            versioned_nonnullable_offset_byte_start,
            versioned_nullable_offset_byte_start,
        })
    }
}

pub type KindsOfOffsets<'a> = TableRef<'a, KindsOfOffsetsMarker>;

impl<'a> KindsOfOffsets<'a> {
    /// The major/minor version of the GDEF table
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// A normal offset
    pub fn nonnullable_offset(&self) -> Offset16 {
        let range = self.shape.nonnullable_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`nonnullable_offset`][Self::nonnullable_offset].
    pub fn nonnullable(&self) -> Result<Dummy<'a>, ReadError> {
        let data = self.data;
        self.nonnullable_offset().resolve(data)
    }

    /// An offset that is nullable, but always present
    pub fn nullable_offset(&self) -> Nullable<Offset16> {
        let range = self.shape.nullable_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`nullable_offset`][Self::nullable_offset].
    pub fn nullable(&self) -> Option<Result<Dummy<'a>, ReadError>> {
        let data = self.data;
        self.nullable_offset().resolve(data)
    }

    /// A normal offset that is versioned
    pub fn versioned_nonnullable_offset(&self) -> Option<Offset16> {
        let range = self.shape.versioned_nonnullable_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Attempt to resolve [`versioned_nonnullable_offset`][Self::versioned_nonnullable_offset].
    pub fn versioned_nonnullable(&self) -> Option<Result<Dummy<'a>, ReadError>> {
        let data = self.data;
        self.versioned_nonnullable_offset().map(|x| x.resolve(data))
    }

    /// An offset that is nullable and versioned
    pub fn versioned_nullable_offset(&self) -> Option<Nullable<Offset16>> {
        let range = self.shape.versioned_nullable_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Attempt to resolve [`versioned_nullable_offset`][Self::versioned_nullable_offset].
    pub fn versioned_nullable(&self) -> Option<Result<Dummy<'a>, ReadError>> {
        let data = self.data;
        self.versioned_nullable_offset().map(|x| x.resolve(data))?
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for KindsOfOffsets<'a> {
    fn type_name(&self) -> &str {
        "KindsOfOffsets"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        let version = self.version();
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new(
                "nonnullable_offset",
                FieldType::offset(self.nonnullable_offset(), self.nonnullable()),
            )),
            2usize => Some(Field::new(
                "nullable_offset",
                FieldType::offset(self.nullable_offset(), self.nullable()),
            )),
            3usize if version.compatible(MajorMinor::VERSION_1_1) => Some(Field::new(
                "versioned_nonnullable_offset",
                FieldType::offset(
                    self.versioned_nonnullable_offset().unwrap(),
                    self.versioned_nonnullable().unwrap(),
                ),
            )),
            4usize if version.compatible(MajorMinor::VERSION_1_1) => Some(Field::new(
                "versioned_nullable_offset",
                FieldType::offset(
                    self.versioned_nullable_offset().unwrap(),
                    self.versioned_nullable().unwrap(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for KindsOfOffsets<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct KindsOfArraysOfOffsetsMarker {
    nonnullable_offsets_byte_len: usize,
    nullable_offsets_byte_len: usize,
    versioned_nonnullable_offsets_byte_start: Option<usize>,
    versioned_nonnullable_offsets_byte_len: Option<usize>,
    versioned_nullable_offsets_byte_start: Option<usize>,
    versioned_nullable_offsets_byte_len: Option<usize>,
}

impl KindsOfArraysOfOffsetsMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn count_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn nonnullable_offsets_byte_range(&self) -> Range<usize> {
        let start = self.count_byte_range().end;
        start..start + self.nonnullable_offsets_byte_len
    }
    fn nullable_offsets_byte_range(&self) -> Range<usize> {
        let start = self.nonnullable_offsets_byte_range().end;
        start..start + self.nullable_offsets_byte_len
    }
    fn versioned_nonnullable_offsets_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_nonnullable_offsets_byte_start?;
        Some(start..start + self.versioned_nonnullable_offsets_byte_len?)
    }
    fn versioned_nullable_offsets_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_nullable_offsets_byte_start?;
        Some(start..start + self.versioned_nullable_offsets_byte_len?)
    }
}

impl<'a> FontRead<'a> for KindsOfArraysOfOffsets<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let version: MajorMinor = cursor.read()?;
        let count: u16 = cursor.read()?;
        let nonnullable_offsets_byte_len = count as usize * Offset16::RAW_BYTE_LEN;
        cursor.advance_by(nonnullable_offsets_byte_len);
        let nullable_offsets_byte_len = count as usize * Offset16::RAW_BYTE_LEN;
        cursor.advance_by(nullable_offsets_byte_len);
        let versioned_nonnullable_offsets_byte_start = version
            .compatible(MajorMinor::VERSION_1_1)
            .then(|| cursor.position())
            .transpose()?;
        let versioned_nonnullable_offsets_byte_len = version
            .compatible(MajorMinor::VERSION_1_1)
            .then_some(count as usize * Offset16::RAW_BYTE_LEN);
        if let Some(value) = versioned_nonnullable_offsets_byte_len {
            cursor.advance_by(value);
        }
        let versioned_nullable_offsets_byte_start = version
            .compatible(MajorMinor::VERSION_1_1)
            .then(|| cursor.position())
            .transpose()?;
        let versioned_nullable_offsets_byte_len = version
            .compatible(MajorMinor::VERSION_1_1)
            .then_some(count as usize * Offset16::RAW_BYTE_LEN);
        if let Some(value) = versioned_nullable_offsets_byte_len {
            cursor.advance_by(value);
        }
        cursor.finish(KindsOfArraysOfOffsetsMarker {
            nonnullable_offsets_byte_len,
            nullable_offsets_byte_len,
            versioned_nonnullable_offsets_byte_start,
            versioned_nonnullable_offsets_byte_len,
            versioned_nullable_offsets_byte_start,
            versioned_nullable_offsets_byte_len,
        })
    }
}

pub type KindsOfArraysOfOffsets<'a> = TableRef<'a, KindsOfArraysOfOffsetsMarker>;

impl<'a> KindsOfArraysOfOffsets<'a> {
    /// The major/minor version of the GDEF table
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of items in each array
    pub fn count(&self) -> u16 {
        let range = self.shape.count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// A normal array offset
    pub fn nonnullable_offsets(&self) -> &'a [BigEndian<Offset16>] {
        let range = self.shape.nonnullable_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Attempt to resolve [`nonnullable_offsets`][Self::nonnullable_offsets].
    pub fn nonnullables(&self) -> impl Iterator<Item = Result<Dummy<'a>, ReadError>> + 'a {
        let data = self.data;
        self.nonnullable_offsets()
            .iter()
            .map(move |off| off.get().resolve(data))
    }

    /// An offset that is nullable, but always present
    pub fn nullable_offsets(&self) -> &'a [BigEndian<Nullable<Offset16>>] {
        let range = self.shape.nullable_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Attempt to resolve [`nullable_offsets`][Self::nullable_offsets].
    pub fn nullables(&self) -> impl Iterator<Item = Option<Result<Dummy<'a>, ReadError>>> + 'a {
        let data = self.data;
        self.nullable_offsets()
            .iter()
            .map(move |off| off.get().resolve(data))
    }

    /// A normal offset that is versioned
    pub fn versioned_nonnullable_offsets(&self) -> Option<&'a [BigEndian<Offset16>]> {
        let range = self.shape.versioned_nonnullable_offsets_byte_range()?;
        Some(self.data.read_array(range).unwrap())
    }

    /// Attempt to resolve [`versioned_nonnullable_offsets`][Self::versioned_nonnullable_offsets].
    pub fn versioned_nonnullables(
        &self,
    ) -> Option<impl Iterator<Item = Result<Dummy<'a>, ReadError>> + 'a> {
        let data = self.data;
        self.versioned_nonnullable_offsets()
            .map(|x| x.iter().map(move |off| off.get().resolve(data)))
    }

    /// An offset that is nullable and versioned
    pub fn versioned_nullable_offsets(&self) -> Option<&'a [BigEndian<Nullable<Offset16>>]> {
        let range = self.shape.versioned_nullable_offsets_byte_range()?;
        Some(self.data.read_array(range).unwrap())
    }

    /// Attempt to resolve [`versioned_nullable_offsets`][Self::versioned_nullable_offsets].
    pub fn versioned_nullables(
        &self,
    ) -> Option<impl Iterator<Item = Option<Result<Dummy<'a>, ReadError>>> + 'a> {
        let data = self.data;
        self.versioned_nullable_offsets()
            .map(|x| x.iter().map(move |off| off.get().resolve(data)))
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for KindsOfArraysOfOffsets<'a> {
    fn type_name(&self) -> &str {
        "KindsOfArraysOfOffsets"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        let version = self.version();
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("count", self.count())),
            2usize => Some({
                let data = self.data;
                Field::new(
                    "nonnullable_offsets",
                    FieldType::offset_array(
                        better_type_name::<Dummy>(),
                        self.nonnullable_offsets(),
                        move |off| {
                            let target = off.get().resolve::<Dummy>(data);
                            FieldType::offset(off.get(), target)
                        },
                    ),
                )
            }),
            3usize => Some({
                let data = self.data;
                Field::new(
                    "nullable_offsets",
                    FieldType::offset_array(
                        better_type_name::<Dummy>(),
                        self.nullable_offsets(),
                        move |off| {
                            let target = off.get().resolve::<Dummy>(data);
                            FieldType::offset(off.get(), target)
                        },
                    ),
                )
            }),
            4usize if version.compatible(MajorMinor::VERSION_1_1) => Some({
                let data = self.data;
                Field::new(
                    "versioned_nonnullable_offsets",
                    FieldType::offset_array(
                        better_type_name::<Dummy>(),
                        self.versioned_nonnullable_offsets().unwrap(),
                        move |off| {
                            let target = off.get().resolve::<Dummy>(data);
                            FieldType::offset(off.get(), target)
                        },
                    ),
                )
            }),
            5usize if version.compatible(MajorMinor::VERSION_1_1) => Some({
                let data = self.data;
                Field::new(
                    "versioned_nullable_offsets",
                    FieldType::offset_array(
                        better_type_name::<Dummy>(),
                        self.versioned_nullable_offsets().unwrap(),
                        move |off| {
                            let target = off.get().resolve::<Dummy>(data);
                            FieldType::offset(off.get(), target)
                        },
                    ),
                )
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for KindsOfArraysOfOffsets<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct DummyMarker {}

impl DummyMarker {
    fn value_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
}

impl<'a> FontRead<'a> for Dummy<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.finish(DummyMarker {})
    }
}

pub type Dummy<'a> = TableRef<'a, DummyMarker>;

impl<'a> Dummy<'a> {
    pub fn value(&self) -> u16 {
        let range = self.shape.value_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Dummy<'a> {
    fn type_name(&self) -> &str {
        "Dummy"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("value", self.value())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Dummy<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}
