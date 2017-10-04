use piston_window::{Context, Graphics};
use piston_window::character::CharacterCache;
use std::fmt;

pub trait RenderView {
    type Settings;

    fn render<C, G>(&self, settings: &Self::Settings, context: &Context, glyphs: &mut C, graphics: &mut G)
    where
        C: CharacterCache,
        C::Error: fmt::Debug,
        G: Graphics<Texture = C::Texture>;
}
