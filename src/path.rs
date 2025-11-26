use crate::error::Error;
use serde::{ser, Serialize, Serializer};
use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub(crate) const TOKEN: &str = "$ser_nix::private::Path";

/// Internal serializer that writes strings without quotes for Nix paths.
pub(crate) struct PathStrEmitter<'a> {
    pub output: &'a mut String,
}

impl ser::Serializer for PathStrEmitter<'_> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(v);
        Ok(())
    }

    serde::serde_if_integer128! {
        fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
            Err(ser::Error::custom("expected Path"))
        }

        fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
            Err(ser::Error::custom("expected Path"))
        }
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("expected Path"))
    }
}

/// A Nix path that serializes without quotes.
///
/// Use this wrapper type when you want a path to be serialized as an unquoted
/// Nix path (e.g., `./foo.nix` or `/etc/nixos/configuration.nix`).
///
/// `NixPath` can hold either a borrowed `&Path` or an owned `PathBuf`.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::{to_string, NixPath};
/// use std::path::PathBuf;
///
/// #[derive(Serialize)]
/// struct Config {
///     source: NixPath<'static>,
/// }
///
/// let config = Config {
///     source: NixPath::from(PathBuf::from("./hardware-configuration.nix")),
/// };
///
/// let result = to_string(&config).unwrap();
/// assert!(result.contains("source = ./hardware-configuration.nix;"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NixPath<'a>(Cow<'a, Path>);

impl<'a> NixPath<'a> {
    /// Creates a new `NixPath` from a borrowed `Path`.
    pub fn new(path: &'a Path) -> Self {
        NixPath(Cow::Borrowed(path))
    }

    /// Returns a reference to the underlying `Path`.
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    /// Converts into an owned `PathBuf`.
    pub fn into_path_buf(self) -> PathBuf {
        self.0.into_owned()
    }
}

impl From<PathBuf> for NixPath<'static> {
    fn from(path: PathBuf) -> Self {
        NixPath(Cow::Owned(path))
    }
}

impl<'a> From<&'a Path> for NixPath<'a> {
    fn from(path: &'a Path) -> Self {
        NixPath(Cow::Borrowed(path))
    }
}

impl<'a> From<&'a PathBuf> for NixPath<'a> {
    fn from(path: &'a PathBuf) -> Self {
        NixPath(Cow::Borrowed(path.as_path()))
    }
}

impl AsRef<Path> for NixPath<'_> {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

fn serialize_path<S>(path: &Path, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match path.to_str() {
        Some(s) => serializer.serialize_newtype_struct(TOKEN, s),
        None => Err(serde::ser::Error::custom("path contains invalid UTF-8 characters")),
    }
}

impl Serialize for NixPath<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_path(&self.0, serializer)
    }
}

/// Serialize a `Path` or `PathBuf` as an unquoted Nix path.
///
/// Use this function with `#[serde(serialize_with = "...")]` to serialize
/// `Path` or `PathBuf` fields without quotes.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::to_string;
/// use std::path::PathBuf;
///
/// #[derive(Serialize)]
/// struct Config {
///     #[serde(serialize_with = "ser_nix::as_nix_path")]
///     source: PathBuf,
/// }
///
/// let config = Config {
///     source: PathBuf::from("./hardware-configuration.nix"),
/// };
///
/// let result = to_string(&config).unwrap();
/// assert!(result.contains("source = ./hardware-configuration.nix;"));
/// ```
pub fn as_nix_path<S>(value: &Path, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serialize_path(value, serializer)
}

/// Serialize an `Option<PathBuf>` or `Option<&Path>` as an unquoted Nix path, or null if None.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::to_string;
/// use std::path::PathBuf;
///
/// #[derive(Serialize)]
/// struct Config {
///     #[serde(serialize_with = "ser_nix::as_optional_nix_path")]
///     source: Option<PathBuf>,
/// }
///
/// let config = Config {
///     source: Some(PathBuf::from("./path.nix")),
/// };
///
/// let result = to_string(&config).unwrap();
/// assert!(result.contains("source = ./path.nix;"));
/// ```
pub fn as_optional_nix_path<S>(value: &Option<PathBuf>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serialize_path(v, serializer),
        None => serializer.serialize_none(),
    }
}
