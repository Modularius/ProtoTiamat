use crate::Post;

#[derive(Default, Clone, Debug)]
pub struct Feed {
    pub posts: Vec<Post>,
}
