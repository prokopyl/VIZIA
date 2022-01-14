

use crate::{Context, View, Handle};


pub struct Image {

}

impl Image {
    pub fn new<'a>(cx: &'a mut Context, file_path: &'static str) -> Handle<'a,Self> {
        Self {

        }.build2(cx, |cx|{
            cx.add_image_file(file_path, file_path);
        }).background_image(file_path)
    }
}

impl View for Image {

}