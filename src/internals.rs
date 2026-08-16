macro_rules! impl_default_via_builder_build {
    ($type: ty) => {
        impl Default for $type {
            fn default() -> Self {
                Self::builder().build()
            }
        }
    };
}

pub(crate) use impl_default_via_builder_build;
