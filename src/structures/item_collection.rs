//! Definition and implementation of the item collection.

// Standard Library Imports
use std::{cmp::Ordering};

// External Imports
use rss::Item;
use chrono::prelude::*;

// Local Imports
use crate::processing::enums::{ItemFilterType, ItemSortType};

/// A collection of items.
pub struct ItemCollection<'a> {
    items: Vec<&'a Item>,
}

impl<'a> Default for ItemCollection<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Function implementations for ItemCollection.
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
    pub fn items(self) -> Vec<&'a Item> {
        self.items
    }

    /// Sort the items in the collection.
    /// This alters the actual order of the items stored in the collection.
    pub fn sort(&mut self, sort_type: ItemSortType) {
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
    }

    /// Filter the items in the collection.
    /// This *does* remove any items from the actual collection. 
    pub fn filter(&mut self, filter_type: ItemFilterType){


        match filter_type {
            ItemFilterType::Title(filter_title) => {
                self.items.retain(|item| {
                    if let Some(title) = item.title() {
                        title.contains(filter_title.as_str())
                    } else {
                        false
                    }
                });
            }
            ItemFilterType::Length(filter_length) => {
                self.items.retain(|item| {
                    if let Some(description) = item.description() {
                        description.len() < filter_length
                    } else {
                        false
                    }
                });
            }
            ItemFilterType::Date(filter_date) => {
                self.items.retain(|item| {
                    if let Some(date) = item.pub_date() {
                        DateTime::parse_from_rfc2822(date).unwrap() <= DateTime::parse_from_rfc2822(&filter_date).unwrap()
                    } else {
                        false
                    }
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_collection_push() {
        let item_collection = ItemCollection::new();
        assert_eq!(item_collection.items().len(), 0);
        let mut item_collection = ItemCollection::new();
        let item = Item::default();
        item_collection.push(&item);
        assert_eq!(item_collection.items().len(), 1);
    }

    #[test]
    fn test_item_collection_sort_title() {
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

        item_collection.sort(ItemSortType::Title);
        let items = item_collection.items();
        assert_eq!(items[0].title(), Some("a"));
        assert_eq!(items[1].title(), Some("b"));
        assert_eq!(items[2].title(), Some("c"));
    }

    #[test]
    fn test_item_collection_sort_length() {
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

        item_collection.sort(ItemSortType::Length);
        let items = item_collection.items();
        assert_eq!(items[0].description(), Some("a"));
        assert_eq!(items[1].description(), Some("aa"));
        assert_eq!(items[2].description(), Some("aaa"));
    }
    #[test]
    fn test_item_collection_sort_date() {
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
        let items = item_collection.items();
        assert_eq!(items[0].pub_date(), Some("2018-01-01"));
        assert_eq!(items[1].pub_date(), Some("2018-01-02"));
        assert_eq!(items[2].pub_date(), Some("2018-01-03"));
    }
    #[test]
    fn test_item_collection_filter_title() {
        let mut item_collection = ItemCollection::new();

        // Items
        let mut item = Item::default();
        item.set_pub_date(String::from("2018-01-01"));
        item.set_title(String::from("a"));
        item.set_description(Some(String::from("a")));

        let mut item2 = Item::default();
        item2.set_pub_date(String::from("2018-01-02"));
        item2.set_title(String::from("ab"));
        item2.set_description(Some(String::from("aa")));

        let mut item3 = Item::default();
        item3.set_pub_date(String::from("2018-01-03"));
        item3.set_title(String::from("c"));
        item3.set_description(Some(String::from("aaa")));

        item_collection.push(&item);
        item_collection.push(&item3);
        item_collection.push(&item2);

        item_collection.filter(ItemFilterType::Title(String::from("a")));
        assert_eq!(item_collection.items().len(), 2);

    }

    #[test]
    fn test_item_collection_filter_length() {
        let mut item_collection = ItemCollection::new();

        // Items
        let mut item = Item::default();
        item.set_pub_date(String::from("2018-01-01"));
        item.set_title(String::from("a"));
        item.set_description(Some(String::from("a")));

        let mut item2 = Item::default();
        item2.set_pub_date(String::from("2018-01-02"));
        item2.set_title(String::from("ab"));
        item2.set_description(Some(String::from("aa")));

        let mut item3 = Item::default();
        item3.set_pub_date(String::from("2018-01-03"));
        item3.set_title(String::from("c"));
        item3.set_description(Some(String::from("aaa")));

        item_collection.push(&item);
        item_collection.push(&item3);
        item_collection.push(&item2);

        item_collection.filter(ItemFilterType::Length(2));
        assert_eq!(item_collection.items().len(), 2);

    }

    #[test]
    fn test_item_collection_filter_date() {
        let mut item_collection = ItemCollection::new();

        // Items
        let mut item = Item::default();
        item.set_pub_date(String::from("2018-01-01"));
        item.set_title(String::from("a"));
        item.set_description(Some(String::from("a")));

        let mut item2 = Item::default();
        item2.set_pub_date(String::from("2018-01-02"));
        item2.set_title(String::from("ab"));
        item2.set_description(Some(String::from("aa")));

        let mut item3 = Item::default();
        item3.set_pub_date(String::from("2018-01-03"));
        item3.set_title(String::from("c"));
        item3.set_description(Some(String::from("aaa")));

        item_collection.push(&item);
        item_collection.push(&item3);
        item_collection.push(&item2);

        item_collection.filter(ItemFilterType::Date(String::from("2018-01-02")));
        assert_eq!(item_collection.items().len(), 2);

    }
}
