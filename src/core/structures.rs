use std::sync::Mutex;

use rss::{Channel, Item};





impl<'a> InnerSortable for ItemCollection<'a> {
    fn sort(&mut self, sort_type: ItemSortType) -> &Vec<&'a Item> {
        match sort_type {
            ItemSortType::Title => self.items().sort_by(|a, b| a.title.cmp(&b.title)),
            ItemSortType::Date => self.items().sort_by(|a, b| a.pub_date.cmp(&b.pub_date)),
            ItemSortType::Length => self.items().sort_by(|a, b|  {
                if let Some(ad) = &a.description {
                    if let Some(bd) = &b.description {
                        return ad.len().cmp(&bd.len())
                    } else {
                        return std::cmp::Ordering::Greater
                    }
                } else {
                    return std::cmp::Ordering::Less
                }
            }),
        };
        self.items()
    }
}

// TODO: Trickle down the sorting down to the underlying ItemCollection. Some sorting happens on the channels, while some happens on the items.
// Possibly move definition of sort function implementation to the structures file to have access to private fields instead of making all fields public
impl<'a> Sortable for ChannelCollection<'a> {
    fn sort(&mut self, sort_type: ChannelSortType) -> &Vec<&'a Item> {
        match sort_type {
            ChannelSortType::ItemSortType(sort_type) => {
                self.items().lock().unwrap().sort(sort_type);
            },
            ChannelSortType::Publisher => {
                self.channels().sort_by(|a, b| a.title.cmp(&b.title));
                &self.items()
            },
        }
    }
}
    // /// Sort the channels by the given sort type.
    // /// This function will block until it receives lock on the channels mutex.
    // /// # Panics
    // /// This function panics if another thread panicked while holding the lock.
    // pub fn sort(&self, sort_type: SortType) {
    //     let mut channels = self.channels.lock().unwrap();
    //     channels.sort(sort_type);
    // }


    // /// Sort the items in the collection based on the sort type.
    // fn sort(&mut self, sort_type: SortType) {
    //     match sort_type {
    //         SortType::Date => self.items.sort_by(|a, b| a.pub_date().cmp(&b.pub_date())),
    //         SortType::Author => self.items.sort_by(|a, b| a.author().cmp(&b.author())),
    //     }
    // }

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    /// Test sorting a ChannelCollection by date.
    fn test_sort_channel_date() {
        let collection = ChannelCollection::new();

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