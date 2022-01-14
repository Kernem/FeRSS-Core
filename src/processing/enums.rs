//! Sorting and Filtering enums.

/// Defines how an ItemCollection should be sorted,
pub enum ItemSortType {
    /// Sort by the item's title.
    Title,
    /// Sort by the item's date.
    Date,
    /// Sort by the item's length.
    Length,
    /// Sort by the item's source
    Source,
}

/// Defines how an ItemCollection should be filtered.
pub enum ItemFilterType {
    /// Filter by the item's title. Ensuring that the title matches the string.
    Title(String),
    /// Filter by the item's date. Ensuring that the date matches the string
    Date(String),
    /// Filter by the item's length. Ensuring that the description is smaller than usize
    Length(usize),
    /// Filter by the item's source. Ensuring that the source matches the string
    Source(String),
}
