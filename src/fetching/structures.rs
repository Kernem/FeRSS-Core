use std::sync::Mutex;

use rss::{Channel, Item};

pub enum SortType {
    Date,
    Author,
}

enum ItemWrapper {
    Item(Item),
    Channel(Channel),
}

pub struct ChannelCollection {
    /// A mutex is necessary in this case as later the vector will be edited by multiple threads running asynchronously.
    channels: Mutex<ItemCollection>,
}

impl ChannelCollection {
    /// Create a new ChannelCollection.
    pub fn new() -> ChannelCollection {
        ChannelCollection {
            channels: Mutex::new(ItemCollection::new()),
        }
    }

    /// Push a new channel to the collection.
    /// This function will block until it receives lock on the channels mutex.
    /// # Panics
    /// This function panics if another thread panicked while holding the lock.
    pub fn push(&self, channel: Channel) {
        let mut channels = self.channels.lock().unwrap();
        channels.push(ItemWrapper::Channel(channel));
    }

    /// Push a new channel to the collection.
    /// This function will block until it receives lock on the channels mutex.
    /// # Panics
    /// This function panics if another thread panicked while holding the lock.
    pub fn push_item(&self, item: Item) {
        let mut channels = self.channels.lock().unwrap();
        channels.push(ItemWrapper::Item(item));
    }

    /// Sort the channels by the given sort type.
    /// This function will block until it receives lock on the channels mutex.
    /// # Panics
    /// This function panics if another thread panicked while holding the lock.
    pub fn sort(&self, sort_type: SortType) {
        let mut channels = self.channels.lock().unwrap();
        channels.sort(sort_type);
    }

}

struct ItemCollection {
    items: Vec<Item>,
}

impl ItemCollection {
    /// Create a new ItemCollection.
    pub fn new() -> ItemCollection {
        ItemCollection {
            items: Vec::new(),
        }
    }

    /// Push a new item or items from a channel to the collection.
    fn push(&mut self, wrapper: ItemWrapper) {
        match wrapper {
            ItemWrapper::Item(item) => self.items.push(item.to_owned()),
            ItemWrapper::Channel(channel) => {
                for item in channel.items() {
                    self.items.push(item.to_owned());
                }
            }
        }
    }

    /// Sort the items in the collection based on the sort type.
    fn sort(&mut self, sort_type: SortType) {
        match sort_type {
            SortType::Date => self.items.sort_by(|a, b| a.pub_date().cmp(&b.pub_date())),
            SortType::Author => self.items.sort_by(|a, b| a.author().cmp(&b.author())),
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    // Testing for ItemCollection.
    #[test]
    /// Test pushing an item to a ItemCollection.
    fn test_push_item() {
        let mut collection = ItemCollection::new();
        assert_eq!(collection.items.len(), 0);
        let item = Item::default();
        collection.push(ItemWrapper::Item(item));
        assert_eq!(collection.items.len(), 1);
    }

    #[test]
    /// Test sorting an ItemCollection by date.
    fn test_sort_date() {
        let mut collection = ItemCollection::new();

        let mut item1 = Item::default();
        item1.set_pub_date(String::from("2018-01-01"));
        let mut item2 = Item::default();
        item2.set_pub_date(String::from("2018-01-02"));
        let mut item3 = Item::default();
        item3.set_pub_date(String::from("2018-01-03"));

        // Push items in 'wrong' order.
        collection.push(ItemWrapper::Item(item1));
        collection.push(ItemWrapper::Item(item3));
        collection.push(ItemWrapper::Item(item2));

        collection.sort(SortType::Date);
        // We expect the items to be in the 'right' order.
        assert_eq!(collection.items[0].pub_date().unwrap(), String::from("2018-01-01"));
        assert_eq!(collection.items[1].pub_date().unwrap(), String::from("2018-01-02"));
        assert_eq!(collection.items[2].pub_date().unwrap(), String::from("2018-01-03"));
    }

    #[test]
    /// Test sorting an ItemCollection by author.
    fn test_sort_author() {
        let mut collection = ItemCollection::new();

        let mut item1 = Item::default();
        item1.set_author(String::from("author1"));
        let mut item2 = Item::default();
        item2.set_author(String::from("author2"));
        let mut item3 = Item::default();
        item3.set_author(String::from("author3"));

        // Push items in 'wrong' order.
        collection.push(ItemWrapper::Item(item1));
        collection.push(ItemWrapper::Item(item3));
        collection.push(ItemWrapper::Item(item2));

        collection.sort(SortType::Author);
        // We expect the items to be in the 'right' order.
        assert_eq!(collection.items[0].author().unwrap(), String::from("author1"));
        assert_eq!(collection.items[1].author().unwrap(), String::from("author2"));
        assert_eq!(collection.items[2].author().unwrap(), String::from("author3"));
    }

    // Testing for ChannelCollection.
    #[test]
    /// Test pushing a channel to a ChannelCollection.
    fn test_push_channel() {
        let collection = ChannelCollection::new();
        assert_eq!(collection.channels.lock().unwrap().items.len(), 0);

        // Check empty channel.
        let channel = Channel::default();
        collection.push(channel);
        assert_eq!(collection.channels.lock().unwrap().items.len(), 0);

        // Check nonempty channel.
        let mut channel = Channel::default();
        let item = Item::default();
        channel.set_items(vec![item]);
        collection.push(channel);
    }

    #[test]
    /// Test sorting a ChannelCollection by date.
    fn test_sort_channel_date() {
        let collection = ChannelCollection::new();

        // TODO: Instead of adding channels, add items to channels.
        let mut channel1 = Channel::default();

        let mut item1 = Item::default();
        item1.set_pub_date(String::from("2018-01-01"));
        let mut item2 = Item::default();
        item2.set_pub_date(String::from("2018-01-02"));
        let mut item3 = Item::default();
        item3.set_pub_date(String::from("2018-01-03"));

        channel1.set_items(vec![item1, item2, item3]);


        // Push items in 'wrong' order.
        collection.push(channel1);

        collection.sort(SortType::Date);
        // We expect the items to be in the 'right' order.
        assert_eq!(collection.channels.lock().unwrap().items[0].pub_date().unwrap(), String::from("2018-01-01"));
        assert_eq!(collection.channels.lock().unwrap().items[1].pub_date().unwrap(), String::from("2018-01-02"));
        assert_eq!(collection.channels.lock().unwrap().items[2].pub_date().unwrap(), String::from("2018-01-03"));
    }

    #[test]
    /// Test sorting a ChannelCollection by author.
    fn test_sort_channel_author() {
        let collection = ChannelCollection::new();

        let mut channel1 = Channel::default();

        let mut item1 = Item::default();
        item1.set_author(String::from("author1"));
        let mut item2 = Item::default();
        item2.set_author(String::from("author2"));
        let mut item3 = Item::default();
        item3.set_author(String::from("author3"));

        channel1.set_items(vec![item1, item2, item3]);

        // Push items in 'wrong' order.
        collection.push(channel1);

        collection.sort(SortType::Author);
        // We expect the items to be in the 'right' order.
        assert_eq!(collection.channels.lock().unwrap().items[0].author().unwrap(), String::from("author1"));
        assert_eq!(collection.channels.lock().unwrap().items[1].author().unwrap(), String::from("author2"));
        assert_eq!(collection.channels.lock().unwrap().items[2].author().unwrap(), String::from("author3"));
    }
}