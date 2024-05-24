//! Various macros.

/// Implements [`FromStr`] for types via calling [`toml::from_str`].
///
/// [`FromStr`]: core::str::FromStr
macro_rules! impl_from_str_with_toml {
    ($type: ty) => {
        impl core::str::FromStr for $type {
            type Err = toml::de::Error;

            fn from_str(string: &str) -> core::result::Result<Self, Self::Err> {
                toml::from_str(string)
            }
        }
    };
}

/// Implements [`FromPath`] for types via reading from the path provided and calling [`parse`]
/// on the contents.
///
/// [`FromPath`]: crate::paths::FromPath
/// [`parse`]: str::parse
macro_rules! impl_from_path_with_parse {
    ($type: ty, $error: ty) => {
        impl crate::paths::FromPath for $type {
            type Error = $error;

            fn from_path<P: core::convert::AsRef<std::path::Path>>(
                path: P,
            ) -> core::result::Result<Self, Self::Error> {
                let string = std::fs::read_to_string(path)?;

                let value: Self = string.parse()?;

                Ok(value)
            }
        }
    };
}

pub(crate) use impl_from_str_with_toml;
pub(crate) use impl_from_path_with_parse;
