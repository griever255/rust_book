// Listing 17-21: Modifications to main to use the new implementation
// of the blog post workflow
use blog2::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    
    let post = post.request_review();

    let post = post.reject();

    let post = post.request_review();

    let post = post.approve();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}