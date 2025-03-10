//! Glyph Identifiers
//!
//! Although these are treated as u16s in the spec, we choose to represent them
//! as a distinct type.

/// A 16-bit glyph identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphId(u16);

impl GlyphId {
    /// The identifier reserved for unknown glyphs
    pub const NOTDEF: GlyphId = GlyphId(0);

    /// Construct a new `GlyphId`.
    pub fn new(raw: u16) -> Self {
        GlyphId(raw)
    }

    /// The identifier as a u16.
    pub fn to_u16(self) -> u16 {
        self.0
    }

    pub fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl std::fmt::Display for GlyphId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GID_{}", self.0)
    }
}

crate::newtype_scalar!(GlyphId, [u8; 2]);
