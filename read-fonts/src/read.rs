//! Traits for interpreting font data

use font_types::{FixedSize, ReadScalar, Tag};

use crate::font_data::FontData;

/// A type that can be read from raw table data.
///
/// This trait is implemented for all font tables that are self-describing: that
/// is, tables that do not require any external state in order to interpret their
/// underlying bytes. (Tables that require external state implement
/// [`FontReadWithArgs`] instead)
pub trait FontRead<'a>: Sized {
    /// Read an instace of `Self` from the provided data, performing validation.
    ///
    /// In the case of a table, this method is responsible for ensuring the input
    /// data is consistent: this means ensuring that any versioned fields are
    /// present as required by the version, and that any array lengths are not
    /// out-of-bounds.
    fn read(data: FontData<'a>) -> Result<Self, ReadError>;
}

//NOTE: this is separate so that it can be a super trait of FontReadWithArgs and
//ComputeSize, without them needing to know about each other? I'm not sure this
//is necessary, but I don't know the full heirarchy of traits I'm going to need
//yet, so this seems... okay?

/// A trait for a type that needs additional arguments to be read.
pub trait ReadArgs {
    type Args: Copy;
}

/// A trait for types that require external data in order to be constructed.
pub trait FontReadWithArgs<'a>: Sized + ReadArgs {
    /// read an item, using the provided args.
    ///
    /// If successful, returns a new item of this type, and the number of bytes
    /// used to construct it.
    ///
    /// If a type requires multiple arguments, they will be passed as a tuple.
    //TODO: split up the 'takes args' and 'reports size' parts of this into
    // separate traits
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError>;
}

/// A trait for tables that have multiple possible formats.
pub trait Format<T> {
    /// The format value for this table.
    const FORMAT: T;
}

/// A type that can compute its size at runtime, based on some input.
///
/// For types with a constant size, see [`FixedSize`](font_types::FixedSize) and
/// for types which store their size inline, see [`VarSize`].
pub trait ComputeSize: ReadArgs {
    /// Compute the number of bytes required to represent this type.
    fn compute_size(args: &Self::Args) -> usize;
}

/// A trait for types that have variable length.
///
/// As a rule, these types have an initial length field.
///
/// For types with a constant size, see [`FixedSize`](font_types::FixedSize) and
/// for types which can pre-compute their size, see [`ComputeSize`].
pub trait VarSize {
    /// The type of the first (length) field of the item.
    ///
    /// When reading this type, we will read this value first, and use it to
    /// determine the total length.
    type Size: ReadScalar + FixedSize + Into<u32>;

    #[doc(hidden)]
    fn read_len_at(data: FontData, pos: usize) -> Option<usize> {
        let asu32 = data.read_at::<Self::Size>(pos).ok()?.into();
        Some(asu32 as usize + Self::Size::RAW_BYTE_LEN)
    }
}

/// An error that occurs when reading font data
#[derive(Debug, Clone)]
pub enum ReadError {
    OutOfBounds,
    // i64 is flexible enough to store any value we might encounter
    InvalidFormat(i64),
    InvalidSfnt(u32),
    InvalidArrayLen,
    ValidationError,
    NullOffset,
    TableIsMissing(Tag),
    MalformedData(&'static str),
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ReadError::OutOfBounds => write!(f, "An offset was out of bounds"),
            ReadError::InvalidFormat(x) => write!(f, "Invalid format '{x}'"),
            ReadError::InvalidSfnt(ver) => write!(f, "Invalid sfnt version 0x{ver:08X}"),
            ReadError::InvalidArrayLen => {
                write!(f, "Specified array length not a multiple of item size")
            }
            ReadError::ValidationError => write!(f, "A validation error occured"),
            ReadError::NullOffset => write!(f, "An offset was unexpectedly null"),
            ReadError::TableIsMissing(tag) => write!(f, "the {tag} table is missing"),
            ReadError::MalformedData(msg) => write!(f, "Malformed data: '{msg}'"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ReadError {}
