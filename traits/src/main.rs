// https://doc.rust-lang.org/book/ch10-02-traits.html
// Is 'trait' rust's version of Inteface?
pub trait Summary {
    fn summarise_trait(&self) -> String;
}

pub struct NewsPaper {
    name: String,
    headline: String,
    content: String,
}

impl Summary for NewsPaper {
    fn summarise_trait(&self) -> String {
        format!(
            "{}, by yours truly:{}\n {}",
            self.headline, self.name, self.content
        )
    }
}

pub struct Comment {
    username: String,
    content: String,
    like: bool,
    repost: bool,
}

impl Summary for Comment {
    fn summarise_trait(&self) -> String {
        format!(
            "{} posted {} Repost ? {}",
            &self.username, &self.content, &self.repost
        )
    }
}
fn main() {
    println!("carvan");

    let whiplash = Comment {
        like: true,
        username: "persona-mp3".to_string(),
        content: "Are you sure?".to_string(),
        repost: true,
    };

    println!("{}", whiplash.summarise_trait());
}
