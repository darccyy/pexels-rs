use crate::Id;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub url: String,
}

impl crate::Credit for User {
    fn credit(&self) -> String {
        todo!()
    }
}
