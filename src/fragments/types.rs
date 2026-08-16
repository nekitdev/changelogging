use std::{borrow::Cow, collections::HashMap, convert::Infallible, fmt, str::FromStr};

use non_empty_str::{
    EmptyStr, FromNonEmptyStr, NonEmptyCowStr, NonEmptyStr, NonEmptyString, const_non_empty_str,
};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The default type count.
pub const COUNT: usize = 7;

/// The `security` type.
pub const SECURITY: &NonEmptyStr = const_non_empty_str!("security");

/// The `feature` type.
pub const FEATURE: &NonEmptyStr = const_non_empty_str!("feature");

/// The `change` type.
pub const CHANGE: &NonEmptyStr = const_non_empty_str!("change");

/// The `fix` type.
pub const FIX: &NonEmptyStr = const_non_empty_str!("fix");

/// The `deprecation` type.
pub const DEPRECATION: &NonEmptyStr = const_non_empty_str!("deprecation");

/// The `removal` type.
pub const REMOVAL: &NonEmptyStr = const_non_empty_str!("removal");

/// The `internal` type.
pub const INTERNAL: &NonEmptyStr = const_non_empty_str!("internal");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, IntoOwned)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Name<'n> {
    string: NonEmptyCowStr<'n>,
}

impl fmt::Display for Name<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.string().fmt(formatter)
    }
}

#[derive(Debug, Error)]
#[error("type names must be non-empty: {error}")]
pub struct NameError {
    #[from]
    error: EmptyStr,
}

impl NameError {
    pub const fn new(error: EmptyStr) -> Self {
        Self { error }
    }
}

impl FromStr for Name<'_> {
    type Err = NameError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let non_empty = NonEmptyStr::try_from_str(string)?;

        Ok(Self::from_non_empty(non_empty))
    }
}

impl FromNonEmptyStr for Name<'_> {
    type Error = Infallible;

    fn from_non_empty_str(string: &NonEmptyStr) -> Result<Self, Self::Error> {
        Ok(Self::from_non_empty(string))
    }
}

impl<'n> Name<'n> {
    pub const fn new(string: NonEmptyCowStr<'n>) -> Self {
        Self { string }
    }

    pub const fn borrowed(string: &'n NonEmptyStr) -> Self {
        Self::new(Cow::Borrowed(string))
    }

    pub const fn owned(string: NonEmptyString) -> Self {
        Self::new(Cow::Owned(string))
    }

    pub fn get(self) -> NonEmptyCowStr<'n> {
        self.string
    }
}

impl Name<'_> {
    pub fn from_non_empty<S: AsRef<NonEmptyStr>>(string: S) -> Self {
        let non_empty = string.as_ref().to_owned();

        Self::owned(non_empty)
    }

    pub fn string(&self) -> &NonEmptyStr {
        self.string.as_ref()
    }

    pub const SECURITY: Self = Self::borrowed(SECURITY);
    pub const FEATURE: Self = Self::borrowed(FEATURE);
    pub const CHANGE: Self = Self::borrowed(CHANGE);
    pub const FIX: Self = Self::borrowed(FIX);
    pub const DEPRECATION: Self = Self::borrowed(DEPRECATION);
    pub const REMOVAL: Self = Self::borrowed(REMOVAL);
    pub const INTERNAL: Self = Self::borrowed(INTERNAL);

    pub const ARRAY: [Self; COUNT] = [
        Self::SECURITY,
        Self::FEATURE,
        Self::CHANGE,
        Self::FIX,
        Self::DEPRECATION,
        Self::REMOVAL,
        Self::INTERNAL,
    ];
}

pub const NAMES: [Name<'_>; COUNT] = Name::ARRAY;

pub type Names<'n> = Vec<Name<'n>>;

pub type OwnedNames = Names<'static>;

pub fn names() -> OwnedNames {
    NAMES.into()
}

/// The `security` title.
pub const SECURITY_TITLE: &NonEmptyStr = const_non_empty_str!("Security");

/// The `feature` title.
pub const FEATURE_TITLE: &NonEmptyStr = const_non_empty_str!("Features");

/// The `change` title.
pub const CHANGE_TITLE: &NonEmptyStr = const_non_empty_str!("Changes");

/// The `fix` title.
pub const FIX_TITLE: &NonEmptyStr = const_non_empty_str!("Fixes");

/// The `deprecation` title.
pub const DEPRECATION_TITLE: &NonEmptyStr = const_non_empty_str!("Deprecations");

/// The `removal` title.
pub const REMOVAL_TITLE: &NonEmptyStr = const_non_empty_str!("Removals");

/// The `internal` title.
pub const INTERNAL_TITLE: &NonEmptyStr = const_non_empty_str!("Internal");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, IntoOwned)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Title<'t> {
    string: NonEmptyCowStr<'t>,
}

impl fmt::Display for Title<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.string.fmt(formatter)
    }
}

#[derive(Debug, Error)]
#[error("type titles must be non-empty: {error}")]
pub struct TitleError {
    #[from]
    error: EmptyStr,
}

impl FromStr for Title<'_> {
    type Err = TitleError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let non_empty: &NonEmptyStr = string.try_into()?;

        Ok(Self::owned(non_empty.to_owned()))
    }
}

impl FromNonEmptyStr for Title<'_> {
    type Error = Infallible;

    fn from_non_empty_str(string: &NonEmptyStr) -> Result<Self, Self::Error> {
        Ok(Self::owned(string.to_owned()))
    }
}

impl<'t> Title<'t> {
    pub const fn new(string: NonEmptyCowStr<'t>) -> Self {
        Self { string }
    }

    pub const fn borrowed(string: &'t NonEmptyStr) -> Self {
        Self::new(Cow::Borrowed(string))
    }

    pub const fn owned(string: NonEmptyString) -> Self {
        Self::new(Cow::Owned(string))
    }

    pub fn string(&self) -> &NonEmptyStr {
        self.string.as_ref()
    }

    pub const SECURITY: Self = Self::borrowed(SECURITY_TITLE);
    pub const FEATURE: Self = Self::borrowed(FEATURE_TITLE);
    pub const CHANGE: Self = Self::borrowed(CHANGE_TITLE);
    pub const FIX: Self = Self::borrowed(FIX_TITLE);
    pub const DEPRECATION: Self = Self::borrowed(DEPRECATION_TITLE);
    pub const REMOVAL: Self = Self::borrowed(REMOVAL_TITLE);
    pub const INTERNAL: Self = Self::borrowed(INTERNAL_TITLE);

    pub const ARRAY: [Self; COUNT] = [
        Self::SECURITY,
        Self::FEATURE,
        Self::CHANGE,
        Self::FIX,
        Self::DEPRECATION,
        Self::REMOVAL,
        Self::INTERNAL,
    ];
}

pub const TITLES: [Title<'_>; COUNT] = Title::ARRAY;

pub type Titles<'t> = Vec<Title<'t>>;

pub type OwnedTitles = Titles<'static>;

pub fn titles() -> OwnedTitles {
    TITLES.into()
}

pub const MAPPING: [(Name<'_>, Title<'_>); COUNT] = [
    (Name::SECURITY, Title::SECURITY),
    (Name::FEATURE, Title::FEATURE),
    (Name::CHANGE, Title::CHANGE),
    (Name::FIX, Title::FIX),
    (Name::DEPRECATION, Title::DEPRECATION),
    (Name::REMOVAL, Title::REMOVAL),
    (Name::INTERNAL, Title::INTERNAL),
];

pub type Mapping<'m> = HashMap<Name<'m>, Title<'m>>;

pub type OwnedMapping = Mapping<'static>;

pub fn mapping() -> OwnedMapping {
    MAPPING.into()
}
