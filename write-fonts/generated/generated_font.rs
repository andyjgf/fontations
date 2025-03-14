// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The OpenType [Table Directory](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#table-directory)
#[derive(Clone, Debug)]
pub struct TableDirectory {
    /// 0x00010000 or 0x4F54544F
    pub sfnt_version: u32,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    /// Table records array—one for each top-level table in the font
    pub table_records: Vec<TableRecord>,
}

impl FontWrite for TableDirectory {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.sfnt_version.write_into(writer);
        (array_len(&self.table_records).unwrap() as u16).write_into(writer);
        self.search_range.write_into(writer);
        self.entry_selector.write_into(writer);
        self.range_shift.write_into(writer);
        self.table_records.write_into(writer);
    }
}

impl Validate for TableDirectory {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("TableDirectory", |ctx| {
            ctx.in_field("table_records", |ctx| {
                if self.table_records.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.table_records.validate_impl(ctx);
            });
        })
    }
}

/// Record for a table in a font.
#[derive(Clone, Debug)]
pub struct TableRecord {
    /// Table identifier.
    pub tag: Tag,
    /// Checksum for the table.
    pub checksum: u32,
    /// Offset from the beginning of the font data.
    pub offset: u32,
    /// Length of the table.
    pub length: u32,
}

impl FontWrite for TableRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.tag.write_into(writer);
        self.checksum.write_into(writer);
        self.offset.write_into(writer);
        self.length.write_into(writer);
    }
}

impl Validate for TableRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}
