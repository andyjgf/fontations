// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [glyf (Glyph Data)](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct GlyfMarker {}

impl GlyfMarker {}

impl<'a> FontRead<'a> for Glyf<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let cursor = data.cursor();
        cursor.finish(GlyfMarker {})
    }
}

/// The [glyf (Glyph Data)](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf) table
pub type Glyf<'a> = TableRef<'a, GlyfMarker>;

impl<'a> Glyf<'a> {}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Glyf<'a> {
    fn type_name(&self) -> &str {
        "Glyf"
    }

    #[allow(unused_variables)]
    #[allow(clippy::match_single_binding)]
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Glyf<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// The [Glyph Header](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf#glyph-headers)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct SimpleGlyphMarker {
    end_pts_of_contours_byte_len: usize,
    instructions_byte_len: usize,
    glyph_data_byte_len: usize,
}

impl SimpleGlyphMarker {
    fn number_of_contours_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + i16::RAW_BYTE_LEN
    }
    fn x_min_byte_range(&self) -> Range<usize> {
        let start = self.number_of_contours_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn y_min_byte_range(&self) -> Range<usize> {
        let start = self.x_min_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn x_max_byte_range(&self) -> Range<usize> {
        let start = self.y_min_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn y_max_byte_range(&self) -> Range<usize> {
        let start = self.x_max_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn end_pts_of_contours_byte_range(&self) -> Range<usize> {
        let start = self.y_max_byte_range().end;
        start..start + self.end_pts_of_contours_byte_len
    }
    fn instruction_length_byte_range(&self) -> Range<usize> {
        let start = self.end_pts_of_contours_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn instructions_byte_range(&self) -> Range<usize> {
        let start = self.instruction_length_byte_range().end;
        start..start + self.instructions_byte_len
    }
    fn glyph_data_byte_range(&self) -> Range<usize> {
        let start = self.instructions_byte_range().end;
        start..start + self.glyph_data_byte_len
    }
}

impl<'a> FontRead<'a> for SimpleGlyph<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let number_of_contours: i16 = cursor.read()?;
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        let end_pts_of_contours_byte_len = number_of_contours.max(0) as usize * u16::RAW_BYTE_LEN;
        cursor.advance_by(end_pts_of_contours_byte_len);
        let instruction_length: u16 = cursor.read()?;
        let instructions_byte_len = instruction_length as usize * u8::RAW_BYTE_LEN;
        cursor.advance_by(instructions_byte_len);
        let glyph_data_byte_len = cursor.remaining_bytes();
        cursor.advance_by(glyph_data_byte_len);
        cursor.finish(SimpleGlyphMarker {
            end_pts_of_contours_byte_len,
            instructions_byte_len,
            glyph_data_byte_len,
        })
    }
}

/// The [Glyph Header](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf#glyph-headers)
pub type SimpleGlyph<'a> = TableRef<'a, SimpleGlyphMarker>;

impl<'a> SimpleGlyph<'a> {
    /// If the number of contours is greater than or equal to zero,
    /// this is a simple glyph. If negative, this is a composite glyph
    /// — the value -1 should be used for composite glyphs.
    pub fn number_of_contours(&self) -> i16 {
        let range = self.shape.number_of_contours_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Minimum x for coordinate data.
    pub fn x_min(&self) -> i16 {
        let range = self.shape.x_min_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Minimum y for coordinate data.
    pub fn y_min(&self) -> i16 {
        let range = self.shape.y_min_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Maximum x for coordinate data.
    pub fn x_max(&self) -> i16 {
        let range = self.shape.x_max_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Maximum y for coordinate data.
    pub fn y_max(&self) -> i16 {
        let range = self.shape.y_max_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Array of point indices for the last point of each contour,
    /// in increasing numeric order
    pub fn end_pts_of_contours(&self) -> &'a [BigEndian<u16>] {
        let range = self.shape.end_pts_of_contours_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Total number of bytes for instructions. If instructionLength is
    /// zero, no instructions are present for this glyph, and this
    /// field is followed directly by the flags field.
    pub fn instruction_length(&self) -> u16 {
        let range = self.shape.instruction_length_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Array of instruction byte code for the glyph.
    pub fn instructions(&self) -> &'a [BigEndian<u8>] {
        let range = self.shape.instructions_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// the raw data for flags & x/y coordinates
    pub fn glyph_data(&self) -> &'a [u8] {
        let range = self.shape.glyph_data_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for SimpleGlyph<'a> {
    fn type_name(&self) -> &str {
        "SimpleGlyph"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("number_of_contours", self.number_of_contours())),
            1usize => Some(Field::new("x_min", self.x_min())),
            2usize => Some(Field::new("y_min", self.y_min())),
            3usize => Some(Field::new("x_max", self.x_max())),
            4usize => Some(Field::new("y_max", self.y_max())),
            5usize => Some(Field::new(
                "end_pts_of_contours",
                self.end_pts_of_contours(),
            )),
            6usize => Some(Field::new("instruction_length", self.instruction_length())),
            7usize => Some(Field::new("instructions", self.instructions())),
            8usize => Some(Field::new("glyph_data", self.glyph_data())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for SimpleGlyph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

bitflags::bitflags! { # [doc = " Flags used in [SimpleGlyph]"] pub struct SimpleGlyphFlags : u8 { # [doc = " Bit 0: If set, the point is on the curve; otherwise, it is off"] # [doc = " the curve."] const ON_CURVE_POINT = 0x01 ; # [doc = " Bit 1: If set, the corresponding x-coordinate is 1 byte long,"] # [doc = " and the sign is determined by the"] # [doc = " X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR flag. If not set, its"] # [doc = " interpretation depends on the"] # [doc = " X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR flag: If that other flag"] # [doc = " is set, the x-coordinate is the same as the previous"] # [doc = " x-coordinate, and no element is added to the xCoordinates"] # [doc = " array. If both flags are not set, the corresponding element in"] # [doc = " the xCoordinates array is two bytes and interpreted as a signed"] # [doc = " integer. See the description of the"] # [doc = " X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR flag for additional"] # [doc = " information."] const X_SHORT_VECTOR = 0x02 ; # [doc = " Bit 2: If set, the corresponding y-coordinate is 1 byte long,"] # [doc = " and the sign is determined by the"] # [doc = " Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR flag. If not set, its"] # [doc = " interpretation depends on the"] # [doc = " Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR flag: If that other flag"] # [doc = " is set, the y-coordinate is the same as the previous"] # [doc = " y-coordinate, and no element is added to the yCoordinates"] # [doc = " array. If both flags are not set, the corresponding element in"] # [doc = " the yCoordinates array is two bytes and interpreted as a signed"] # [doc = " integer. See the description of the"] # [doc = " Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR flag for additional"] # [doc = " information."] const Y_SHORT_VECTOR = 0x04 ; # [doc = " Bit 3: If set, the next byte (read as unsigned) specifies the"] # [doc = " number of additional times this flag byte is to be repeated in"] # [doc = " the logical flags array — that is, the number of additional"] # [doc = " logical flag entries inserted after this entry. (In the"] # [doc = " expanded logical array, this bit is ignored.) In this way, the"] # [doc = " number of flags listed can be smaller than the number of points"] # [doc = " in the glyph description."] const REPEAT_FLAG = 0x08 ; # [doc = " Bit 4: This flag has two meanings, depending on how the"] # [doc = " X_SHORT_VECTOR flag is set. If X_SHORT_VECTOR is set, this bit"] # [doc = " describes the sign of the value, with 1 equalling positive and"] # [doc = " 0 negative. If X_SHORT_VECTOR is not set and this bit is set,"] # [doc = " then the current x-coordinate is the same as the previous"] # [doc = " x-coordinate. If X_SHORT_VECTOR is not set and this bit is also"] # [doc = " not set, the current x-coordinate is a signed 16-bit delta"] # [doc = " vector."] const X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR = 0x10 ; # [doc = " Bit 5: This flag has two meanings, depending on how the"] # [doc = " Y_SHORT_VECTOR flag is set. If Y_SHORT_VECTOR is set, this bit"] # [doc = " describes the sign of the value, with 1 equalling positive and"] # [doc = " 0 negative. If Y_SHORT_VECTOR is not set and this bit is set,"] # [doc = " then the current y-coordinate is the same as the previous"] # [doc = " y-coordinate. If Y_SHORT_VECTOR is not set and this bit is also"] # [doc = " not set, the current y-coordinate is a signed 16-bit delta"] # [doc = " vector."] const Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR = 0x20 ; # [doc = " Bit 6: If set, contours in the glyph description may overlap."] # [doc = " Use of this flag is not required in OpenType — that is, it is"] # [doc = " valid to have contours overlap without having this flag set. It"] # [doc = " may affect behaviors in some platforms, however. (See the"] # [doc = " discussion of “Overlapping contours” in Apple’s"] # [doc = " specification for details regarding behavior in Apple"] # [doc = " platforms.) When used, it must be set on the first flag byte"] # [doc = " for the glyph. See additional details below."] const OVERLAP_SIMPLE = 0x40 ; } }

impl font_types::Scalar for SimpleGlyphFlags {
    type Raw = <u8 as font_types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        self.bits().to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u8>::from_raw(raw);
        Self::from_bits_truncate(t)
    }
}

#[cfg(feature = "traversal")]
impl<'a> From<SimpleGlyphFlags> for FieldType<'a> {
    fn from(src: SimpleGlyphFlags) -> FieldType<'a> {
        src.bits().into()
    }
}

/// [CompositeGlyph](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf#glyph-headers)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct CompositeGlyphMarker {
    component_data_byte_len: usize,
}

impl CompositeGlyphMarker {
    fn number_of_contours_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + i16::RAW_BYTE_LEN
    }
    fn x_min_byte_range(&self) -> Range<usize> {
        let start = self.number_of_contours_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn y_min_byte_range(&self) -> Range<usize> {
        let start = self.x_min_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn x_max_byte_range(&self) -> Range<usize> {
        let start = self.y_min_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn y_max_byte_range(&self) -> Range<usize> {
        let start = self.x_max_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn component_data_byte_range(&self) -> Range<usize> {
        let start = self.y_max_byte_range().end;
        start..start + self.component_data_byte_len
    }
}

impl<'a> FontRead<'a> for CompositeGlyph<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        let component_data_byte_len = cursor.remaining_bytes();
        cursor.advance_by(component_data_byte_len);
        cursor.finish(CompositeGlyphMarker {
            component_data_byte_len,
        })
    }
}

/// [CompositeGlyph](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf#glyph-headers)
pub type CompositeGlyph<'a> = TableRef<'a, CompositeGlyphMarker>;

impl<'a> CompositeGlyph<'a> {
    /// If the number of contours is greater than or equal to zero,
    /// this is a simple glyph. If negative, this is a composite glyph
    /// — the value -1 should be used for composite glyphs.
    pub fn number_of_contours(&self) -> i16 {
        let range = self.shape.number_of_contours_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Minimum x for coordinate data.
    pub fn x_min(&self) -> i16 {
        let range = self.shape.x_min_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Minimum y for coordinate data.
    pub fn y_min(&self) -> i16 {
        let range = self.shape.y_min_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Maximum x for coordinate data.
    pub fn x_max(&self) -> i16 {
        let range = self.shape.x_max_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Maximum y for coordinate data.
    pub fn y_max(&self) -> i16 {
        let range = self.shape.y_max_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// component flag
    /// glyph index of component
    pub fn component_data(&self) -> &'a [u8] {
        let range = self.shape.component_data_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for CompositeGlyph<'a> {
    fn type_name(&self) -> &str {
        "CompositeGlyph"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("number_of_contours", self.number_of_contours())),
            1usize => Some(Field::new("x_min", self.x_min())),
            2usize => Some(Field::new("y_min", self.y_min())),
            3usize => Some(Field::new("x_max", self.x_max())),
            4usize => Some(Field::new("y_max", self.y_max())),
            5usize => Some(Field::new("component_data", self.component_data())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for CompositeGlyph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

bitflags::bitflags! { # [doc = " Flags used in [CompositeGlyph]"] pub struct CompositeGlyphFlags : u16 { # [doc = " Bit 0: If this is set, the arguments are 16-bit (uint16 or"] # [doc = " int16); otherwise, they are bytes (uint8 or int8)."] const ARG_1_AND_2_ARE_WORDS = 0x0001 ; # [doc = " Bit 1: If this is set, the arguments are signed xy values;"] # [doc = " otherwise, they are unsigned point numbers."] const ARGS_ARE_XY_VALUES = 0x0002 ; # [doc = " Bit 2: If set and ARGS_ARE_XY_VALUES is also set, the xy values"] # [doc = " are rounded to the nearest grid line. Ignored if"] # [doc = " ARGS_ARE_XY_VALUES is not set."] const ROUND_XY_TO_GRID = 0x0004 ; # [doc = " Bit 3: This indicates that there is a simple scale for the"] # [doc = " component. Otherwise, scale = 1.0."] const WE_HAVE_A_SCALE = 0x0008 ; # [doc = " Bit 5: Indicates at least one more glyph after this one."] const MORE_COMPONENTS = 0x0020 ; # [doc = " Bit 6: The x direction will use a different scale from the y"] # [doc = " direction."] const WE_HAVE_AN_X_AND_Y_SCALE = 0x0040 ; # [doc = " Bit 7: There is a 2 by 2 transformation that will be used to"] # [doc = " scale the component."] const WE_HAVE_A_TWO_BY_TWO = 0x0080 ; # [doc = " Bit 8: Following the last component are instructions for the"] # [doc = " composite character."] const WE_HAVE_INSTRUCTIONS = 0x0100 ; # [doc = " Bit 9: If set, this forces the aw and lsb (and rsb) for the"] # [doc = " composite to be equal to those from this component glyph. This"] # [doc = " works for hinted and unhinted glyphs."] const USE_MY_METRICS = 0x0200 ; # [doc = " Bit 10: If set, the components of the compound glyph overlap."] # [doc = " Use of this flag is not required in OpenType — that is, it is"] # [doc = " valid to have components overlap without having this flag set."] # [doc = " It may affect behaviors in some platforms, however. (See"] # [doc = " Apple’s specification for details regarding behavior in Apple"] # [doc = " platforms.) When used, it must be set on the flag word for the"] # [doc = " first component. See additional remarks, above, for the similar"] # [doc = " OVERLAP_SIMPLE flag used in simple-glyph descriptions."] const OVERLAP_COMPOUND = 0x0400 ; # [doc = " Bit 11: The composite is designed to have the component offset"] # [doc = " scaled. Ignored if ARGS_ARE_XY_VALUES is not set."] const SCALED_COMPONENT_OFFSET = 0x0800 ; # [doc = " Bit 12: The composite is designed not to have the component"] # [doc = " offset scaled. Ignored if ARGS_ARE_XY_VALUES is not set."] const UNSCALED_COMPONENT_OFFSET = 0x1000 ; } }

impl font_types::Scalar for CompositeGlyphFlags {
    type Raw = <u16 as font_types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        self.bits().to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u16>::from_raw(raw);
        Self::from_bits_truncate(t)
    }
}

#[cfg(feature = "traversal")]
impl<'a> From<CompositeGlyphFlags> for FieldType<'a> {
    fn from(src: CompositeGlyphFlags) -> FieldType<'a> {
        src.bits().into()
    }
}

pub enum Glyph<'a> {
    Simple(SimpleGlyph<'a>),
    Composite(CompositeGlyph<'a>),
}

impl<'a> FontRead<'a> for Glyph<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let format: i16 = data.read_at(0)?;
        match format {
            format if format >= 0 => Ok(Self::Simple(FontRead::read(data)?)),
            format if format < 0 => Ok(Self::Composite(FontRead::read(data)?)),
            other => Err(ReadError::InvalidFormat(other.into())),
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> Glyph<'a> {
    fn dyn_inner<'b>(&'b self) -> &'b dyn SomeTable<'a> {
        match self {
            Self::Simple(table) => table,
            Self::Composite(table) => table,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Glyph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.dyn_inner().fmt(f)
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Glyph<'a> {
    fn type_name(&self) -> &str {
        self.dyn_inner().type_name()
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        self.dyn_inner().get_field(idx)
    }
}
