use rss::Item;

pub struct SafeItem<'a> {
    title: &'a str,
    link: &'a str,
    description: &'a str,
    pub_date: &'a str,
    author: &'a str,

}

impl<'a> SafeItem<'a> {
    pub fn new(item: &Item) -> SafeItem {
        let mut title = "No title";
        if let Some(title_some) = item.title() {
            title = title_some;
        }

        let mut link = "No link";
        if let Some(link_some) = item.link() {
            link = link_some;
        }

        let mut description = "No description";
        if let Some(description_some) = item.description() {
            description = description_some;
        }

        let mut pub_date = "No pub_date";
        if let Some(pub_date_some) = item.pub_date() {
            pub_date = pub_date_some;
        }

        let mut author = "No author";
        if let Some(author_some) = item.author() {
            author = author_some;
        }

        SafeItem {
            title: title,
            link: link,
            description: description,
            pub_date: pub_date,
            author: author,
        }
    }
}

///Getters
impl<'a> SafeItem<'a> {
    pub fn title(&self) -> &'a str {
        self.title
    }

    pub fn link(&self) -> &'a str {
        self.link
    }

    pub fn description(&self) -> &'a str {
        self.description
    }

    pub fn pub_date(&self) -> &'a str {
        self.pub_date
    }

    pub fn author(&self) -> &'a str {
        self.author
    }
}