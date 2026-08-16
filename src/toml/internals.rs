pub mod import {
    pub use std::{convert::AsRef, path::Path};

    pub use from_path::FromPath;
}

macro_rules! impl_from_path_with_toml {
    ($type: ty) => {
        impl $crate::toml::internals::import::FromPath for $type {
            type Error = $crate::toml::load::Error;

            fn from_path<
                P: $crate::toml::internals::import::AsRef<$crate::toml::internals::import::Path>,
            >(
                path: P,
            ) -> Result<Self, Self::Error> {
                $crate::toml::load::load(path)
            }
        }
    };
}

pub(crate) use impl_from_path_with_toml;
