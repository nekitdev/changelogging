//! Trimming [`String`] values in place.

mod sealed {
    pub trait Sealed {}
}

/// Trimming [`String`] values in place (sealed extension trait).
pub trait TrimInPlace: sealed::Sealed {
    /// Trims the start of the string in place.
    fn trim_start_in_place(&mut self);

    /// Trims the end of the string in place.
    fn trim_end_in_place(&mut self);

    /// Trims the start and end of the string in place.
    fn trim_in_place(&mut self) {
        self.trim_end_in_place();
        self.trim_start_in_place();
    }
}

impl sealed::Sealed for String {}

impl TrimInPlace for String {
    fn trim_end_in_place(&mut self) {
        string_trim_end_in_place(self);
    }

    fn trim_start_in_place(&mut self) {
        string_trim_start_in_place(self);
    }
}

pub(crate) fn string_trim_end_in_place(string: &mut String) {
    string.truncate(string.trim_end().len());
}

pub(crate) fn string_trim_start_in_place(string: &mut String) {
    let end = string.len() - string.trim_start().len();

    string.drain(..end);
}
