// Standard Library Imports
use std::cmp::Ordering;

// External Imports
use rss::Item;

// Local Imports
use crate::processing::enums::ItemSortType;

/// A collection of item borrows.
pub struct ItemCollection<'a> {
    items: Vec<&'a Item>,
}

impl<'a> Default for ItemCollection<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ItemCollection<'a> {
    /// Create a new ItemCollection.
    pub fn new() -> ItemCollection<'a> {
        ItemCollection { items: Vec::new() }
    }

    /// Push a new item to the collection.
    pub fn push(&mut self, item: &'a Item) {
        self.items.push(item);
    }

    /// Return a reference to the items in the collection.
    pub fn items(&self) -> &Vec<&'a Item> {
        &self.items
    }

    /// Sort the items in the collection.
    pub fn sort(&mut self, sort_type: ItemSortType) -> &Vec<&'a Item> {
        match sort_type {
            ItemSortType::Title => self.items.sort_by(|a, b| a.title().cmp(&b.title())),
            ItemSortType::Date => self.items.sort_by(|a, b| a.pub_date().cmp(&b.pub_date())),
            ItemSortType::Length => self.items.sort_by(|a, b| {
                if let Some(a_description) = a.description() {
                    if let Some(b_description) = b.description() {
                        a_description.len().cmp(&b_description.len())
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Less
                }
            }),
        };
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_collection_push() {
        let mut item_collection = ItemCollection::new();
        assert_eq!(item_collection.items().len(), 0);
        let item = Item::default();
        item_collection.push(&item);
        assert_eq!(item_collection.items().len(), 1);
    }

    #[test]
    fn test_item_collection_sort() {
        let mut item_collection = ItemCollection::new();

        // Items
        let mut item = Item::default();
        item.set_pub_date(String::from("2018-01-01"));
        item.set_title(String::from("a"));
        item.set_description(Some(String::from("a")));

        let mut item2 = Item::default();
        item2.set_pub_date(String::from("2018-01-02"));
        item2.set_title(String::from("b"));
        item2.set_description(Some(String::from("aa")));

        let mut item3 = Item::default();
        item3.set_pub_date(String::from("2018-01-03"));
        item3.set_title(String::from("c"));
        item3.set_description(Some(String::from("aaa")));

        item_collection.push(&item);
        item_collection.push(&item3);
        item_collection.push(&item2);

        item_collection.sort(ItemSortType::Date);
        assert_eq!(item_collection.items()[0].pub_date(), Some("2018-01-01"));
        assert_eq!(item_collection.items()[1].pub_date(), Some("2018-01-02"));
        assert_eq!(item_collection.items()[2].pub_date(), Some("2018-01-03"));

        item_collection.sort(ItemSortType::Title);
        assert_eq!(item_collection.items()[0].title(), Some("a"));
        assert_eq!(item_collection.items()[1].title(), Some("b"));
        assert_eq!(item_collection.items()[2].title(), Some("c"));

        item_collection.sort(ItemSortType::Length);
        assert_eq!(item_collection.items()[0].description(), Some("a"));
        assert_eq!(item_collection.items()[1].description(), Some("aa"));
        assert_eq!(item_collection.items()[2].description(), Some("aaa"));
    }
}