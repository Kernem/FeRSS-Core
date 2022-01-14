//! Definition and implementation of the channel collection.

// External Imports
use rss::{Channel, Item};

// Local Imports
use super::item_collection::ItemCollection;
use crate::enums::{ItemSortType, ItemFilterType};

/// A collection of channels.
pub struct ChannelCollection {
    channels: Vec<Channel>,
}

impl<'a> Default for ChannelCollection {
    fn default() -> Self {
        Self::new()
    }
}

/// Function implementations for ChannelCollection.
impl ChannelCollection {
    /// Create a new empty ChannelCollection.
    pub fn new() -> ChannelCollection {
        ChannelCollection {
            channels: vec![],
        }
    }

    /// Push a new channel to the collection.
    pub fn push(&mut self, channel: Channel) {
        self.channels.push(channel);
    }

    /// Return a reference to the channels.
    pub fn channels(&self) -> Vec<&Channel> {
        let mut channels = vec![];
        for channel in self.channels.iter() {
            channels.push(channel);
        }
        channels
    }

    fn item_collection(&self) -> ItemCollection {
        let mut collection = ItemCollection::new();
        for channel in &self.channels {
            for item in channel.items() {
                collection.push(item);
            }
        }
        collection

    }

    /// Return a reference to the items.
    pub fn items(&self) -> Vec<&Item> {
        let item_collection = self.item_collection();
        item_collection.items()
    }

    /// Sort the items in the collection and return a reference to them.
    /// This will either sort by channel properties, returning the items within in an arbitrary order
    /// or by item properties, returning the channels in an arbitrary order.
    /// This alters the actual order of the channels and items stored in the collection.
    pub fn sort(&mut self, sort_type: ItemSortType) -> ItemCollection {
        let mut items = self.item_collection();
        items.sort(sort_type);
        items
    }

    /// Filter the items in the collection and return a reference to them.
    /// This does *not* remove any items from the actual collection, rather it returns a new vector containing references to the collection's items. 
    pub fn filter(&mut self, filter_type: ItemFilterType) -> ItemCollection {
        let mut items = self.item_collection();
        items.filter(filter_type);
        items
    }
}

#[cfg(test)]
mod tests {

    use rss::Source;

    use crate::processing::enums::{ItemFilterType, ItemSortType};

    use super::*;

    #[test]
    fn test_channel_collection_push() {
        let mut channel_collection = ChannelCollection::new();
        assert_eq!(channel_collection.channels().len(), 0);
        assert_eq!(channel_collection.items().len(), 0);

        // Empty channel is added, but doesn't affect number of items
        let channel = Channel::default();
        channel_collection.push(channel);
        assert_eq!(channel_collection.channels().len(), 1);
        assert_eq!(channel_collection.items().len(), 0);

        // Channel with items is added, and items are added to the item collection
        let mut channel2 = Channel::default();
        channel2.set_items(vec![Item::default()]);
        channel_collection.push(channel2);

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
        item1.set_pub_date(String::from("Sun, 01 Jan 2017 12:00:00 GMT"));
        item1.set_description(String::from("Description 1 a"));
        let mut source = Source::default();
        source.set_title(String::from("A"));
        item1.set_source(source);

        let mut item2 = Item::default();
        item2.set_title("c Item 2".to_string());
        item2.set_pub_date(String::from("Mon, 02 Jan 2017 12:00:00 GMT"));
        item2.set_description(String::from("Description 2 aaaa"));
        let mut source = Source::default();
        source.set_title(String::from("A"));
        item2.set_source(source);

        channel.set_items(vec![item1, item2]);
        channel_collection.push(channel);

        let mut channel = Channel::default();
        channel.set_title("b Channel 2".to_string());

        let mut item1 = Item::default();
        item1.set_title("b Item 3".to_string());
        item1.set_pub_date(String::from("Tue, 03 Jan 2017 10:00:00 GMT"));
        item1.set_description(String::from("Description 3 aa"));
        let mut source = Source::default();
        source.set_title(String::from("C"));
        item1.set_source(source);

        channel.set_items(vec![item1]);
        channel_collection.push(channel);

        let mut channel = Channel::default();
        channel.set_title("a Channel 3".to_string());

        let mut item1 = Item::default();
        item1.set_title("d Item 4".to_string());
        item1.set_pub_date(String::from("Mon, 02 Jan 2017 14:00:00 GMT"));
        item1.set_description(String::from("Description 4 aaa"));
        let mut source = Source::default();
        source.set_title(String::from("B"));
        item1.set_source(source);

        channel.set_items(vec![item1]);
        channel_collection.push(channel);

        assert_eq!(channel_collection.channels().len(), 3);
        assert_eq!(channel_collection.items().len(), 4);

        let item_collection = channel_collection.sort(ItemSortType::Date);
        let items = item_collection.items();
        //let items = channel_collection.items();
        assert_eq!(items[0].title(), Some("a Item 1"));
        assert_eq!(items[1].title(), Some("c Item 2"));
        assert_eq!(items[2].title(), Some("d Item 4"));
        assert_eq!(items[3].title(), Some("b Item 3"));

        let item_collection = channel_collection.sort(ItemSortType::Title);
        let items = item_collection.items();
        //let items = channel_collection.items();
        assert_eq!(items[0].title(), Some("a Item 1"));
        assert_eq!(items[1].title(), Some("b Item 3"));
        assert_eq!(items[2].title(), Some("c Item 2"));
        assert_eq!(items[3].title(), Some("d Item 4"));

        let item_collection = channel_collection.sort(ItemSortType::Length);
        let items = item_collection.items();
        //let items = channel_collection.items();
        assert_eq!(items[0].title(), Some("a Item 1"));
        assert_eq!(items[1].title(), Some("b Item 3"));
        assert_eq!(items[2].title(), Some("d Item 4"));
        assert_eq!(items[3].title(), Some("c Item 2"));

        let item_collection = channel_collection.sort(ItemSortType::Source);
        let items = item_collection.items();
        //let items = channel_collection.items();
        assert_eq!(items[0].source().unwrap().title(), Some("A"));
        assert_eq!(items[1].source().unwrap().title(), Some("A"));
        assert_eq!(items[2].source().unwrap().title(), Some("B"));
        assert_eq!(items[3].source().unwrap().title(), Some("C"));
    }

    #[test]
    fn test_channel_collection_filter() {
        let mut channel_collection = ChannelCollection::new();

        // Add a couple of channels with items in them as well as a title
        let mut channel = Channel::default();
        channel.set_title("c Channel 1".to_string());

        let mut item1 = Item::default();
        item1.set_title("a Item 1".to_string());
        item1.set_pub_date(String::from("Sun, 01 Jan 2017 12:00:00 GMT"));
        item1.set_description(String::from("Description 1 a"));
        let mut source = Source::default();
        source.set_title(String::from("A"));
        item1.set_source(source);

        let mut item2 = Item::default();
        item2.set_title("c Item 2".to_string());
        item2.set_pub_date(String::from("Mon, 02 Jan 2017 12:00:00 GMT"));
        item2.set_description(String::from("Description 2 aaaa"));
        let mut source = Source::default();
        source.set_title(String::from("A"));
        item2.set_source(source);

        channel.set_items(vec![item1, item2]);
        channel_collection.push(channel);

        let mut channel = Channel::default();
        channel.set_title("b Channel 2".to_string());

        let mut item1 = Item::default();
        item1.set_title("b Item 3".to_string());
        item1.set_pub_date(String::from("Mon, 02 Jan 2017 12:00:00 GMT"));
        item1.set_description(String::from("Description 3 aa"));
        let mut source = Source::default();
        source.set_title(String::from("B"));
        item1.set_source(source);

        channel.set_items(vec![item1]);
        channel_collection.push(channel);

        let mut channel = Channel::default();
        channel.set_title("a Channel 3".to_string());

        let mut item1 = Item::default();
        item1.set_title("d Item 4".to_string());
        item1.set_pub_date(String::from("Tue, 03 Jan 2017 12:00:00 GMT"));
        item1.set_description(String::from("Description 4 aaa"));
        let mut source = Source::default();
        source.set_title(String::from("C"));
        item1.set_source(source);

        channel.set_items(vec![item1]);
        channel_collection.push(channel);

        assert_eq!(channel_collection.channels().len(), 3);
        assert_eq!(channel_collection.items().len(), 4);

        let filtered_collection =
            channel_collection.filter(ItemFilterType::Source(String::from("A")));
        assert_eq!(filtered_collection.items().len(), 2);

        let filtered_collection = channel_collection.filter(ItemFilterType::Title(String::from("b")));
        assert_eq!(filtered_collection.items().len(), 1);

        let filtered_collection = channel_collection.filter(ItemFilterType::Length(17));
        assert_eq!(filtered_collection.items().len(), 3);

        let filtered_collection = channel_collection.filter(ItemFilterType::Date(String::from("Mon, 02 Jan 2017 12:00:00 GMT")));
        assert_eq!(filtered_collection.items().len(), 3);

        // Check that the original collection is unchanged
        assert_eq!(channel_collection.channels().len(), 3);
        assert_eq!(channel_collection.items().len(), 4);
    }
}
