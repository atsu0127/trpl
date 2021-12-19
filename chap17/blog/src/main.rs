extern crate blog;
use blog::Post;

fn main() {
    // 正常系
    let mut post = Post::new();

    // 草稿
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // 査読(1回目)
    post.request_review();
    assert_eq!("", post.content());

    // 査読(2回目)
    post.request_review();
    assert_eq!("", post.content());

    // 公開
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // reject確認
    let mut post = Post::new();

    // 草稿
    post.add_text("It will reject");
    assert_eq!("", post.content());

    // 査読
    post.request_review();
    post.request_review();
    assert_eq!("", post.content());

    // リジェクト
    post.reject();
    assert_eq!("", post.content());

    // 一回じゃ公開できない
    let mut post = Post::new();

    // 草稿
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // 査読(1回目)
    post.request_review();
    assert_eq!("", post.content());

    // 公開
    post.approve();
    assert_eq!("", post.content());
}
