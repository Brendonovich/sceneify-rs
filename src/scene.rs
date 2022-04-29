use crate::{SceneItem, Source};

pub struct SceneInternals {
    pub name: String,
}

pub struct SceneSettings;

pub trait Scene: Source {
    type SceneItemType: SceneItem<Self, Self::Settings>;
}

impl<TScene: Scene> Source for TScene {
    type Settings = SceneSettings;
    type SceneItemType = <TScene as Scene>::SceneItemType;

    fn get_kind() -> &'static str {
        "scene"
    }
}
