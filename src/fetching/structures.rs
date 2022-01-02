use std::sync::Mutex;

use rss::{Channel, Item};

pub enum SortType {
    Date,
    Author,
}
/// A collection of channel borrows.
pub struct ChannelCollection<'a> {
    /// A mutex is necessary in this case as later the vector will be edited by multiple threads running asynchronously.
    channels: Mutex<Vec<&'a Channel>>,
    /// Keeping a direct reference to the items will hopefully speed up some retrievals.
    items: Mutex<ItemCollection<'a>>,
}

impl<'a> ChannelCollection<'a> {
    /// Create a new ChannelCollection.
    pub fn new() -> ChannelCollection<'a> {
        ChannelCollection {
            channels: Mutex::new(vec![]),
            items: Mutex::new(ItemCollection::new()),
        }
    }

    /// Push a new channel to the collection.
    /// This function will block until it receives lock on the channels mutex.
    /// # Panics
    /// This function panics if another thread panicked while holding the lock.
    pub fn push(&self, channel: &'a Channel) {
        // Lock once and perform all operations
        // This helps avoid deadlocks
        let mut channels = self.channels.lock().unwrap();
        let items = self.items.lock().unwrap();
        for item in channel.items() {
            items.push(item);
        }
        channels.push(channel);
    }
}

/// A collection of item borrows.
struct ItemCollection<'a> {
    items: Vec<&'a Item>,
}

impl<'a> ItemCollection<'a> {
    /// Create a new ItemCollection.
    pub fn new() -> ItemCollection<'a> {
        ItemCollection {
            items: Vec::new(),
        }
    }

    /// Push a new item to the collection.
    fn push(&mut self, item: &'a Item) {
        self.items.push(item);
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
        collection.push(&item);
        assert_eq!(collection.items.len(), 1);
    }

    // Testing for ChannelCollection.
    #[test]
    /// Test pushing a channel to a ChannelCollection.
    fn test_push_channel() {
        let collection = ChannelCollection::new();
        assert_eq!(collection.channels.lock().unwrap().len(), 0);

        // Check empty channel.
        let channel = Channel::default();
        collection.push(&channel);
        assert_eq!(collection.channels.lock().unwrap().len(), 1);
        assert_eq!(collection.items.lock().unwrap().items.len(), 0);

        // Check nonempty channel.
        let mut channel = Channel::default();
        let item = Item::default();
        channel.set_items(vec![item]);
        collection.push(&channel);
        assert_eq!(collection.channels.lock().unwrap().len(), 2);
        assert_eq!(collection.items.lock().unwrap().items.len(), 1);
    }
}