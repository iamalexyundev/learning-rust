use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    post.request_review();
    post.approve();
}
