extern crate blog;
use blog::Post;

fn main() {
    let mut post1 = Post::new();

    post1.add_text("I ate a salad for lunch today");
    assert_eq!("", post1.content());

    post1.request_review();
    assert_eq!("", post1.content());

    post1.approve();
    assert_eq!("", post1.content());

    post1.approve();
    assert_eq!("I ate a salad for lunch today", post1.content());

    // Should be a no-op
    post1.reject();
    assert_eq!("I ate a salad for lunch today", post1.content());

    let mut post2 = Post::new();

    post2.add_text("Controversial opinion follows");
    assert_eq!("", post2.content());

    post2.request_review();
    assert_eq!("", post2.content());

    post2.reject();
    assert_eq!("", post2.content());

    // Should not be able to add text after post is not a draft anymore
    let mut post3 = Post::new();

    post3.add_text("First block of text");
    assert_eq!("", post3.content());

    post3.request_review();
    assert_eq!("", post3.content());

    post3.add_text("Second block of text");
    assert_eq!("", post3.content());
}
