pub mod photo;
pub mod user;
pub mod video;

use reqwest::blocking::Client;

use crate::photo::Photo;
use crate::video::Video;

type Id = u32;

#[derive(Debug)]
pub struct Pexels {
    key: String,
    client: Client,
}

impl Pexels {
    pub fn new(api_key: String) -> Self {
        todo!()
    }

    pub fn photo_search(&self, query: impl Into<String>, per_page: u32, page: u32) -> Vec<Photo> {
        todo!()
    }
    pub fn photo_popular(&self, per_page: u32, page: u32) -> Vec<Photo> {
        todo!()
    }

    pub fn video_search(&self, query: impl Into<String>, per_page: u32, page: u32) -> Vec<Video> {
        todo!()
    }
    pub fn video_popular(&self, per_page: u32, page: u32) -> Vec<Photo> {
        todo!()
    }
}

impl Credit for Pexels {
    fn credit(&self) -> String {
        todo!()
    }
}

trait Credit {
    fn credit(&self) -> String;
}
