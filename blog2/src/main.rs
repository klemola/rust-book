extern crate blog2;
use blog2::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    // Second approval required
    let post = post.approve();

    let post = post.publish();

    match post {
        Some(p) => assert_eq!("I ate a salad for lunch today", p.content()),
        None => panic!("Post could not be pubished"),
    }
}
