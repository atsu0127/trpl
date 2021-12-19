extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    // 草稿
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // 査読
    post.request_review();
    assert_eq!("", post.content());

    // 公開
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content);
}
