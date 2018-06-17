extern crate traits;

use traits::{NewsArticle,Tweet,BlogPost,Summary,TweetType};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        tweet_type: TweetType::Tweet,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    traits::notify(&article);
    println!("1 new blog post: {}", article.summarize());

    let post = BlogPost {
        title: String::from("How to deal with the crazies?"),
        author: String::from("defakator"),
        content: String::from("use your better judgement"),
    };

    traits::notify_author(&post);
    println!("1 new blog post: {}", post.summarize());

    let int = 0xdeadc0deu32;
    println!("1 new int: {}", int.summarize());
}
