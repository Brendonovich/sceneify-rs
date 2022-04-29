use std::sync::Arc;

use crate::source::Source;

pub trait SceneItem<TSource: Source<Settings = TSettings>, TSettings> {
    fn get_source(&self) -> Arc<TSource>;

    fn create(item_ref: &str, source: &Arc<TSource>) -> Self
    where
        Self: Sized;
}

pub struct SceneItemInstance<TSource: Source<Settings = TSettings>, TSettings> {
    pub source: Arc<TSource>,
    pub item_ref: String,
}

impl<TSource: Source<Settings = TSettings>, TSettings> SceneItem<TSource, TSettings>
    for SceneItemInstance<TSource, TSettings>
{
    fn get_source(&self) -> Arc<TSource> {
        self.source.clone()
    }

    fn create(item_ref: &str, source: &Arc<TSource>) -> Self
    where
        Self: Sized,
    {
        Self {
            source: source.clone(),
            item_ref: item_ref.into(),
        }
    }
}
