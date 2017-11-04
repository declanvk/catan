use piston_window::{Context, Graphics, Event};
use piston_window::character::CharacterCache;
use std::fmt;

pub trait Controller {
    type Model;
    type View;

    fn handle_events(&mut self, e: &Event, model: &mut Self::Model, view: &mut Self::View);
}

pub trait Renderer {
    type Model;
    type View;

    fn render<C, G>(&self, model: &Self::Model, view: &mut Self::View, context: &Context, glyphs: &mut C, graphics: &mut G)
    where
        C: CharacterCache,
        C::Error: fmt::Debug,
        G: Graphics<Texture = C::Texture>;
}

pub trait Builder {
    type Output;

    fn build(&self) -> Self::Output;
}