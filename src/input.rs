use crate::source::Source;

pub struct InputInstance<T: Input> {
    settings: T,
}

pub trait Input: Source + Sized {
    fn declare(name: &str) -> InputBuilder<Self, Self::Settings>
    where
        Self: Sized,
    {
        InputBuilder::new(name)
    }
}

pub struct InputBuilder<T: Input<Settings = S>, S> {
    name: String,
    initial_settings: Vec<S>,
    _t: std::marker::PhantomData<T>,
}

impl<T: Input<Settings = S>, S> InputBuilder<T, S> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            initial_settings: vec![],
            _t: std::marker::PhantomData,
        }
    }
}

impl<T: Input<Settings = S>, S> InputBuilder<T, S> {
    pub fn set(mut self, setting: S) -> Self {
        self.initial_settings.push(setting);
        self
    }

    pub fn item() -> SceneItemBuilder {
        
    }

    pub fn build(self) {}
}
