//! Sorting and Filtering enums.

/// Defines how a ChannelCollection should be sorted.
pub enum ChannelSortType {
    /// Sort by items.
    ItemSortType(ItemSortType),
    /// Sort by the channel's publisher.
    Publisher,
}

/// Defines how an ItemCollection should be sorted,
pub enum ItemSortType {
    /// Sort by the item's title.
    Title,
    /// Sort by the item's date.
    Date,
    /// Sort by the item's length.
    Length,
}

/// Defines how a ChannelCollection should be filtered.
pub enum ChannelFilterType {
    /// Filter by items.
    ItemFilterType(ItemFilterType),
    /// Filter by the channel's name. Ensuring that the name matches the string.
    Name(String),
}

/// Defines how an ItemCollection should be filtered.
pub enum ItemFilterType {
    /// Filter by the item's title. Ensuring that the title matches the string.
    Title(String),
    /// Filter by the item's date. Ensuring that the date matches the string
    Date(String),
    /// Filter by the item's length. Ensuring that the description is smaller than usize
    Length(usize),
}
