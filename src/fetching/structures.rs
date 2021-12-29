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