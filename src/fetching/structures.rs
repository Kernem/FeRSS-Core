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
    // TODO: Change channels to actually hold channels and add a second field
    //       being an item collection holding references to the items in the channels.
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
}