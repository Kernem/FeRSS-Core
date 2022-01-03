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
