extern crate blog2;
use blog2::*;

fn main() {
    let mut post = Post::new();

    post.add_text("document");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("document", post.content());
}
