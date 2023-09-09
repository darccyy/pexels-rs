use crate::{user::User, Id};

#[derive(Debug, Clone)]
pub struct Video {
    pub id: Id,
    pub width: u32,
    pub heigth: u32,
    pub duration: u32,
    pub url: String,
    pub user: User,
    pub image_url: String,
    pub tags: Vec<String>,
    pub files: Vec<File>,
    // pub full_res: null,
}

#[derive(Debug, Clone)]
pub struct File {
    pub id: Id,
    pub quality: Quality,
    pub file_type: FileType,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub url: String,
}

#[derive(Debug, Clone)]
pub enum Quality {
    SD,
    HD,
}

#[derive(Debug, Clone)]
pub enum FileType {
    Mp4,
}

impl Video {
    pub fn choose_best_file(&self) -> &File {
        todo!()
    }
}

impl crate::Credit for Video {
    fn credit(&self) -> String {
        todo!()
    }
}
