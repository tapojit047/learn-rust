use aggregator::{NewsArticle, Summary, Tweet};
use conditional_impl;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        )
    };
    println!("New article available! {}", article.summarize());

    notify(&tweet);
    notify(&article);

    let tweet_tapojit = returns_summarizable();
    notify(&tweet_tapojit);

    // Using Trait Bounds to Conditionally Implement Methods
    let mut pair = conditional_impl::generate_pair(1, 2);
    pair = pair.new(10, 20);
    pair.cmp_display();
}

// Traits as Parameters

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("tapojit047"),
        content: String::from("I have been awarded a fully funded PhD at the University of North Texas"),
        reply: false,
        retweet: false,
    }
}