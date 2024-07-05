// Listing 17-12: Definition of a Post struct and a new function
// that creates a new Post instance, a State trait, and a Draft struct
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // Listing 17-13: Implementing the add_text method to add text
    // to a post’s content
    pub fn add_text(&mut self, text: &str) {
        self.content = self.state.as_ref().unwrap().add_text(&self.content, text);
    }
    // Listing 17-14: Adding a placeholder implementation for the
    // content method on Post that always returns an empty string slice
    pub fn content(&self) -> &str {
        // Listing 17-17: Updating the content method on Post to delegate
        // to a content method on State
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

// Listing 17-15: Implementing request_review methods on Post and the
// State trait
trait State {
    fn add_text(&self, original_text: &str, _text_to_add: &str) -> String {
        original_text.to_string()
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn add_text(&self, original_text: &str, text_to_add: &str) -> String {
        format!("{}{}", original_text, text_to_add)
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {
            approvals: 0,
        })
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box <dyn State> {
        self
    }
}

struct PendingReview {
    approvals: i32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        if self.approvals == 1 {
            Box::new(Published {})
        } else {
            self.approvals += 1;
            self
        }
    }
}

// // Listing 17-16: Implementing the approve method on Post and the
// State trait
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Listing 17-18: Adding the content method to the State trait
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}


