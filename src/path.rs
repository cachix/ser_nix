use crate::error::Error;
use serde::{ser, Serialize, Serializer};
use std::fmt::Display;

/// Internal serializer that writes strings without quotes for Nix paths
pub(crate) struct RawStringSerializer<'a> {
    pub output: &'a mut String,
}

impl<'a> ser::Serializer for RawStringSerializer<'a> {
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
        *self.output += v;
        Ok(())
    }

    serde::serde_if_integer128! {
        fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
            Err(ser::Error::custom("path must be a string"))
        }

        fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
            Err(ser::Error::custom("path must be a string"))
        }
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("path must be a string"))
    }
}

/// Serialize a value as an unquoted Nix path.
///
/// Use this function with `#[serde(serialize_with = "...")]` to serialize
/// strings or paths without quotes, making them Nix path literals.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::to_string;
///
/// #[derive(Serialize)]
/// struct Config {
///     #[serde(serialize_with = "ser_nix::as_path")]
///     source: String,
///     name: String,
/// }
///
/// let config = Config {
///     source: "./hardware-configuration.nix".to_string(),
///     name: "my-config".to_string(),
/// };
///
/// let result = to_string(&config).unwrap();
/// // Output: { source = ./hardware-configuration.nix; name = "my-config"; }
/// ```
pub fn as_path<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    serializer.serialize_newtype_struct("__ser_nix_path", &format!("{}", value))
}

/// Serialize an Option value as an unquoted Nix path, or null if None.
///
/// Use this function with `#[serde(serialize_with = "...")]` to serialize
/// optional paths.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::to_string;
///
/// #[derive(Serialize)]
/// struct Config {
///     #[serde(serialize_with = "ser_nix::as_optional_path")]
///     source: Option<String>,
/// }
///
/// let config = Config {
///     source: Some("./path.nix".to_string()),
/// };
/// ```
pub fn as_optional_path<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_newtype_struct("__ser_nix_path", &format!("{}", v)),
        None => serializer.serialize_none(),
    }
}
