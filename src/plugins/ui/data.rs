use bevy::{platform::collections::HashMap, prelude::*};
use bevy_hui::prelude::*;
use strum::EnumIter;

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct TemplateHandles(HashMap<&'static str, Handle<HtmlTemplate>>);

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct ScreenRoot;

#[derive(States, Copy, Clone, Default, Hash, PartialEq, Eq, Debug, EnumIter)]
pub enum CurrentScreen {
    #[default]
    Init,
    MainMenu,
    GameSettings,
    MainLoop,
    Results,
    Sandbox,
}
