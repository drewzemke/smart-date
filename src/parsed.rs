use std::ops::Range;

/// Represents some data that has been parsed out of a string.
/// Contains the data that was extracted as well as the location in
/// the input string of the substring that was related to the data.
pub struct Parsed<T> {
    pub data: T,

    // TODO: consider storing a substring instead, then provide a method to
    // compute the offset.
    // see https://stackoverflow.com/questions/67148359/check-if-a-str-is-a-sub-slice-of-another-str
    pub range: Range<usize>,
}

impl<T> Parsed<T> {
    pub fn map<U, F>(self, f: F) -> Parsed<U>
    where
        F: FnOnce(T) -> U,
    {
        Parsed {
            data: f(self.data),
            range: self.range,
        }
    }
}
