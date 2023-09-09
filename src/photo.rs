use crate::{user::User, Id};

#[derive(Debug, Clone)]
pub struct Photo {
    pub id: Id,
    pub width: u32,
    pub height: u32,
    pub url: String,
    pub user: User,
    pub src: Src,
    pub average_color: String,
    pub alt_text: String,
}

#[derive(Debug, Clone)]
pub struct Src {
    pub original: String,
    pub large: String,
    // ...
}

impl crate::Credit for Photo {
    fn credit(&self) -> String {
        todo!()
    }
}
