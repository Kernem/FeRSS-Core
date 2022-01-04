/// Defines how a struct should sort itself.
pub enum ChannelSortType {
    /// Sort by items.
    ItemSortType(ItemSortType),
    /// Sort by the channel's publisher.
    Publisher,
}

pub enum ItemSortType {
    /// Sort by the item's title.
    Title,
    /// Sort by the item's date.
    Date,
    /// Sort by the item's length.
    Length,
}

pub enum ChannelFilterType {
    /// Filter by items.
    ItemFilterType(ItemFilterType),
    /// Filter by the channel's publisher.
    Publisher,
}

pub enum ItemFilterType {
    /// Filter by the item's title. Ensuring that the title matches the string.
    Title(String),
    /// Filter by the item's date. Ensuring that the date matches the string
    Date(String),
    /// Filter by the item's length. Ensuring that the description is smaller than usize
    Length(usize),
}