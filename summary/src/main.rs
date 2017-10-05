extern crate mylib;

use mylib::*;

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    let article = NewsArticle {
        headline: "Penguins win the Stanley Cup Championship!".to_string(),
        location: "Pittsburgh, PA, USA".to_string(),
        author: "Iceburgh".to_string(),
        content: "The Pittsburgh Penguins once again are the best hockey team in the NHL."
            .to_string(),
    };

    println!("New article available! {}", article.summary());
}
