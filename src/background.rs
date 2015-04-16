
use color::{Color, Colorable};
use draw::Drawable;
use graphics::{self, Graphics};
use graphics::character::CharacterCache;
use ui::Ui;

/// A type for drawing a colored window background.
#[derive(Copy, Clone)]
pub struct Background {
    maybe_color: Option<Color>,
}

impl Background {

    /// Construct a background.
    pub fn new() -> Background {
        Background {
            maybe_color: None,
        }
    }

}

impl Colorable for Background {
    fn color(mut self, color: Color) -> Self {
        self.maybe_color = Some(color);
        self
    }
}

impl Drawable for Background {
    fn draw<B, C>(&mut self, ui: &mut Ui<C>, graphics: &mut B)
        where
            B: Graphics<Texture = <C as CharacterCache>::Texture>,
            C: CharacterCache
    {
        let color = self.maybe_color.unwrap_or(ui.theme.background_color);
        graphics::clear(color.to_fsa(), graphics);
    }
}

