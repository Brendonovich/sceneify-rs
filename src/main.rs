use std::sync::Arc;

use sceneify::Input;
use sceneify_derive::{Input, Scene};

struct Source {
    pub name: String,
    pub kind: String,
    pub obs: Arc<obws::Client>,
}

#[derive(Input)]
#[sceneify(kind = "image_source")]
struct ImageSource {
    file: String,
}

#[derive(Input)]
#[sceneify(kind = "color_source")]
struct ColorSource {
    color: i64,
    width: i64,
    height: i64,
}

#[derive(Scene)]
struct MainScene {
    image: ImageSource,
    color: ColorSource,
}

fn main() {
    let image_input = ImageSource::declare("ImageInput")
        .set(ImageSourceSettings::File("/path/to/image.jpg".into()));

    let color_input = ColorSource::declare("ColorInput");

    let scene = MainScene::new("MainScene", image_input.item(), color_input.item());
}
