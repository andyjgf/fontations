//! The [loca (Index to Location)][loca] table
//!
//! [loca]: https://docs.microsoft.com/en-us/typography/opentype/spec/loca

use crate::read::{FontRead, FontReadWithArgs, ReadArgs, ReadError};
use font_types::{BigEndian, GlyphId, Tag};

#[cfg(feature = "traversal")]
use crate::traversal;

/// 'loca'
pub const TAG: Tag = Tag::new(b"loca");

/// The [loca] table.
///
/// [loca]: https://docs.microsoft.com/en-us/typography/opentype/spec/loca
#[derive(Clone)]
pub enum Loca<'a> {
    Short(&'a [BigEndian<u16>]),
    Long(&'a [BigEndian<u32>]),
}

impl<'a> Loca<'a> {
    pub fn len(&self) -> usize {
        match self {
            Loca::Short(data) => data.len().saturating_sub(1),
            Loca::Long(data) => data.len().saturating_sub(1),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Attempt to return the offset for a given glyph id.
    pub fn get_raw(&self, idx: usize) -> Option<u32> {
        match self {
            Loca::Short(data) => data.get(idx).map(|x| x.get() as u32 * 2),
            Loca::Long(data) => data.get(idx).map(|x| x.get()),
        }
    }

    pub fn get_glyf(
        &self,
        gid: GlyphId,
        glyf: &super::glyf::Glyf<'a>,
    ) -> Result<super::glyf::Glyph<'a>, ReadError> {
        let idx = gid.to_u16() as usize;
        let start = self.get_raw(idx).ok_or(ReadError::OutOfBounds)?;
        let end = self.get_raw(idx + 1).ok_or(ReadError::OutOfBounds)?;
        let data = glyf
            .offset_data()
            .slice(start as usize..end as usize)
            .ok_or(ReadError::OutOfBounds)?;
        super::glyf::Glyph::read(data)
    }
}

impl ReadArgs for Loca<'_> {
    type Args = bool;
}

impl<'a> FontReadWithArgs<'a> for Loca<'a> {
    fn read_with_args(
        data: crate::FontData<'a>,
        args: &Self::Args,
    ) -> Result<Self, crate::ReadError> {
        let is_long = *args;
        if is_long {
            data.read_array(0..data.len()).map(Loca::Long)
        } else {
            data.read_array(0..data.len()).map(Loca::Short)
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> traversal::SomeTable<'a> for Loca<'a> {
    fn type_name(&self) -> &str {
        "loca"
    }

    fn get_field(&self, idx: usize) -> Option<traversal::Field<'a>> {
        match idx {
            0usize => Some(traversal::Field::new("offsets", self.clone())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> traversal::SomeArray<'a> for Loca<'a> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, idx: usize) -> Option<traversal::FieldType<'a>> {
        self.get_raw(idx).map(|off| off.into())
    }

    fn type_name(&self) -> &str {
        "Offset32"
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Loca<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn traversal::SomeTable<'a>).fmt(f)
    }
}
