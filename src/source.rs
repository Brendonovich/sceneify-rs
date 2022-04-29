use std::sync::Arc;

use crate::SceneItem;

pub trait Source: Sized {
    type Settings;
    type SceneItemType: SceneItem<Self, Self::Settings>;

    fn get_kind() -> &'static str;

    fn item(item_ref: &str, source: &Arc<Self>) -> Self::SceneItemType {
        Self::SceneItemType::create(item_ref, source)
    }
}
