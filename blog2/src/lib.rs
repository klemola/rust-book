pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals_amount: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    pub approvals_amount: u32,
}

impl PendingReviewPost {
    pub fn approve(mut self) -> PendingReviewPost {
        self.approvals_amount = self.approvals_amount + 1;
        self
    }

    pub fn publish(self) -> Option<Post> {
        if self.approvals_amount == 2 {
            Some(Post {
                content: self.content,
            })
        } else {
            None
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
