pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}…)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        //NOTE: can't call the default implementation from there
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub enum TweetType {
    Tweet,
    Retweet,
    Reply,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub tweet_type: TweetType,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct BlogPost {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for BlogPost {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl<T: ToString> Summary for T {
    fn summarize_author(&self) -> String {
        String::new()
    }

    fn summarize(&self) -> String {
        self.to_string().chars().take(42).collect()
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_author<T>(item: &T)
    where T: Summary
{
    println!("Hot from {}! {}", item.summarize_author(), item.summarize());
}
