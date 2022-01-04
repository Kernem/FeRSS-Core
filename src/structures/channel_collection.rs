// External Imports
use rss::{Channel, Item};

// Local Imports
use super::item_collection::ItemCollection;
use crate::processing::enums::{ChannelSortType, ChannelFilterType};

/// A collection of channel borrows.
pub struct ChannelCollection<'a> {
    channels: Vec<&'a Channel>,
    /// Keeping a direct reference to the items will hopefully speed up some retrievals.
    items: ItemCollection<'a>,
}

impl<'a> Default for ChannelCollection<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ChannelCollection<'a> {
    /// Create a new ChannelCollection.
    pub fn new() -> ChannelCollection<'a> {
        ChannelCollection {
            channels: vec![],
            items: ItemCollection::new(),
        }
    }

    /// Push a new channel to the collection.
    pub fn push(&mut self, channel: &'a Channel) {
        for item in channel.items() {
            self.items.push(item);
        }
        self.channels.push(channel);
    }

    /// Return a reference to the channels.
    /// This function will block until it receives lock on the channels mutex.
    /// # Panics
    /// This function panics if another thread panicked while holding the lock.
    pub fn channels(&self) -> Vec<&'a Channel> {
        let mut channels = vec![];
        for channel in self.channels.iter() {
            channels.push(<&Channel>::clone(channel));
        }
        channels
    }

    /// Return a reference to the items.
    /// This function will block until it receives lock on the channels mutex.
    /// # Panics
    /// This function panics if another thread panicked while holding the lock.
    pub fn items(&self) -> Vec<&Item> {
        let mut items: Vec<&Item> = vec![];
        for item in self.items.items() {
            items.push(<&Item>::clone(item));
        }
        items
    }

    /// Sort the channels in the collection.
    /// This will either sort by channel properties, returning the items within in an arbitrary order
    /// or by item properties, returning the channels in an arbitrary order.
    pub fn sort(&mut self, sort_type: ChannelSortType) -> Vec<&Item> {
        match sort_type {
            ChannelSortType::ItemSortType(item_sort_type) => {
                self.items.sort(item_sort_type);
            }
            ChannelSortType::Publisher => {
                self.channels.sort_by(|a, b| a.title().cmp(b.title()));
            }
        }
        self.items()
    }

    pub fn filter(&mut self, filter_type: ChannelFilterType) -> Vec<&Item> {
        match filter_type {
            ChannelFilterType::ItemFilterType(filter_type) => {
                self.items.filter(filter_type)
            }
            ChannelFilterType::Name(name) => {
                let filtered_channels: Vec<&Channel> = self.channels.iter().filter(|channel| {
                    channel.title.contains(&name)
                })
                .map(|channel| {
                    *channel
                })
                .collect();
                let mut items = Vec::new();
                for channel in filtered_channels {
                    for item in channel.items() {
                        items.push(item);
                    }
                }
                items
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::processing::enums::{ItemSortType, ChannelFilterType, ItemFilterType};

    use super::*;

    #[test]
    fn test_channel_collection_push() {
        let mut channel_collection = ChannelCollection::new();
        assert_eq!(channel_collection.channels().len(), 0);
        assert_eq!(channel_collection.items().len(), 0);

        // Empty channel is added, but doesn't affect number of items
        let channel = Channel::default();
        channel_collection.push(&channel);
        assert_eq!(channel_collection.channels().len(), 1);
        assert_eq!(channel_collection.items().len(), 0);

        // Channel with items is added, and items are added to the item collection
        let mut channel2 = Channel::default();
        channel2.set_items(vec![Item::default()]);
        channel_collection.push(&channel2);

        assert_eq!(channel_collection.channels().len(), 2);
        assert_eq!(channel_collection.items().len(), 1);
    }

    #[test]
    fn test_channel_collection_sort() {
        let mut channel_collection = ChannelCollection::new();

        // Add a couple of channels with items in them as well as a title
        let mut channel = Channel::default();
        channel.set_title("c Channel 1".to_string());

        let mut item1 = Item::default();
        item1.set_title("a Item 1".to_string());
        item1.set_pub_date(String::from("2020-01-01"));
        item1.set_description(String::from("Description 1 a"));

        let mut item2 = Item::default();
        item2.set_title("c Item 2".to_string());
        item2.set_pub_date(String::from("2020-01-02"));
        item2.set_description(String::from("Description 2 aaaa"));

        channel.set_items(vec![item1, item2]);
        channel_collection.push(&channel);

        let mut channel = Channel::default();
        channel.set_title("b Channel 2".to_string());

        let mut item1 = Item::default();
        item1.set_title("b Item 3".to_string());
        item1.set_pub_date(String::from("2020-01-04"));
        item1.set_description(String::from("Description 3 aa"));

        channel.set_items(vec![item1]);
        channel_collection.push(&channel);

        let mut channel = Channel::default();
        channel.set_title("a Channel 3".to_string());

        let mut item1 = Item::default();
        item1.set_title("d Item 4".to_string());
        item1.set_pub_date(String::from("2020-01-03"));
        item1.set_description(String::from("Description 4 aaa"));

        channel.set_items(vec![item1]);
        channel_collection.push(&channel);

        assert_eq!(channel_collection.channels().len(), 3);
        assert_eq!(channel_collection.items().len(), 4);

        channel_collection.sort(ChannelSortType::ItemSortType(ItemSortType::Date));
        assert_eq!(channel_collection.items()[0].title(), Some("a Item 1"));
        assert_eq!(channel_collection.items()[1].title(), Some("c Item 2"));
        assert_eq!(channel_collection.items()[2].title(), Some("d Item 4"));
        assert_eq!(channel_collection.items()[3].title(), Some("b Item 3"));

        channel_collection.sort(ChannelSortType::ItemSortType(ItemSortType::Title));
        assert_eq!(channel_collection.items()[0].title(), Some("a Item 1"));
        assert_eq!(channel_collection.items()[1].title(), Some("b Item 3"));
        assert_eq!(channel_collection.items()[2].title(), Some("c Item 2"));
        assert_eq!(channel_collection.items()[3].title(), Some("d Item 4"));

        channel_collection.sort(ChannelSortType::ItemSortType(ItemSortType::Length));
        assert_eq!(channel_collection.items()[0].title(), Some("a Item 1"));
        assert_eq!(channel_collection.items()[1].title(), Some("b Item 3"));
        assert_eq!(channel_collection.items()[2].title(), Some("d Item 4"));
        assert_eq!(channel_collection.items()[3].title(), Some("c Item 2"));

        channel_collection.sort(ChannelSortType::Publisher);
        assert_eq!(channel_collection.channels()[0].title(), "a Channel 3");
        assert_eq!(channel_collection.channels()[1].title(), "b Channel 2");
        assert_eq!(channel_collection.channels()[2].title(), "c Channel 1");
    }

    #[test]
    fn test_channel_collection_filter() {
        let mut channel_collection = ChannelCollection::new();

        // Add a couple of channels with items in them as well as a title
        let mut channel = Channel::default();
        channel.set_title("c Channel 1".to_string());

        let mut item1 = Item::default();
        item1.set_title("a Item 1".to_string());
        item1.set_pub_date(String::from("2020-01-01"));
        item1.set_description(String::from("Description 1 a"));

        let mut item2 = Item::default();
        item2.set_title("c Item 2".to_string());
        item2.set_pub_date(String::from("2020-01-02"));
        item2.set_description(String::from("Description 2 aaaa"));

        channel.set_items(vec![item1, item2]);
        channel_collection.push(&channel);

        let mut channel = Channel::default();
        channel.set_title("b Channel 2".to_string());

        let mut item1 = Item::default();
        item1.set_title("b Item 3".to_string());
        item1.set_pub_date(String::from("2020-01-04"));
        item1.set_description(String::from("Description 3 aa"));

        channel.set_items(vec![item1]);
        channel_collection.push(&channel);

        let mut channel = Channel::default();
        channel.set_title("a Channel 3".to_string());

        let mut item1 = Item::default();
        item1.set_title("d Item 4".to_string());
        item1.set_pub_date(String::from("2020-01-03"));
        item1.set_description(String::from("Description 4 aaa"));

        channel.set_items(vec![item1]);
        channel_collection.push(&channel);

        assert_eq!(channel_collection.channels().len(), 3);
        assert_eq!(channel_collection.items().len(), 4);

        let filtered_collection = channel_collection.filter(ChannelFilterType::Name(String::from("b")));
        assert_eq!(filtered_collection.len(), 1);

        let filtered_collection = channel_collection.filter(ChannelFilterType::ItemFilterType(ItemFilterType::Title(String::from("b"))));
        assert_eq!(filtered_collection.len(), 1);

        let filtered_collection = channel_collection.filter(ChannelFilterType::ItemFilterType(ItemFilterType::Length(17)));
        assert_eq!(filtered_collection.len(), 2);

        let filtered_collection = channel_collection.filter(ChannelFilterType::ItemFilterType(ItemFilterType::Date(String::from("2020-01-01"))));
        assert_eq!(filtered_collection.len(), 1);
    
        // Check that the original collection is unchanged
        assert_eq!(channel_collection.channels().len(), 3);
        assert_eq!(channel_collection.items().len(), 4);
    }
}
