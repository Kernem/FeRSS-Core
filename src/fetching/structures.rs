use std::sync::Mutex;

use rss::Channel;

impl ChannelCollection {
    /// Create a new ChannelCollection.
    pub fn new() -> ChannelCollection {
        ChannelCollection {
            channels: Mutex::new(Vec::new()),
        }
    }

    /// Push a new item to the collection.
    /// This function will block until it receives lock on the channels mutex.
    /// # Panics
    /// This function panics if another thread panicked while holding the lock.
    pub fn push(&self, channel: Channel) {
        let mut channels = self.channels.lock().unwrap();
        channels.push(channel);
    }

    /// Sort the channels in the collection according to the given sort type.
    pub fn sort(&self, sort_type: SortType) {
        let mut channels = self.channels.lock().unwrap();
        match sort_type {
            SortType::Date => channels.sort_by(|a, b| a.pub_date().cmp(&b.pub_date())),
            SortType::Title => channels.sort_by(|a, b| a.title().cmp(&b.title())),
        }
    }

}