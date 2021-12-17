use core::ops::Deref;
use pokedex::{item::Item, moves::Move, pokemon::Pokemon};

use pokedex::engine::{
    graphics::{draw_cursor, draw_text_left, DrawParams},
    gui::Panel,
    input::controls::{pressed, Control},
    text::MessagePage,
    utils::Reset,
    Context,
};

use crate::view::PlayerView;

pub struct TargetPanel {
    pub names: Vec<Option<String>>,
    pub cursor: usize,
}

impl TargetPanel {
    pub fn new() -> Self {
        Self {
            names: Vec::with_capacity(4),
            cursor: 0,
        }
    }

    pub fn update_names<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        targets: &dyn PlayerView<ID, P, M, I>,
    ) {
        self.names = targets.names();
    }

    pub fn input(&mut self, ctx: &Context) {
        if pressed(ctx, Control::Up) && self.cursor >= 2 {
            self.cursor -= 2;
        } else if pressed(ctx, Control::Down) && self.cursor <= 2 {
            self.cursor += 2;
        } else if pressed(ctx, Control::Left) && self.cursor > 0 {
            self.cursor -= 1;
        } else if pressed(ctx, Control::Right) && self.cursor < 3 {
            self.cursor += 1;
        }
        if self.cursor >= self.names.len() {
            self.cursor = self.names.len() - 1;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        Panel::draw(ctx, 0.0, 113.0, 160.0, 47.0);
        for (index, name) in self.names.iter().enumerate() {
            let x_offset = if index % 2 == 1 { 72.0 } else { 0.0 };
            let y_offset = if index >> 1 == 1 { 17.0 } else { 0.0 };
            draw_text_left(
                ctx,
                &0,
                name.as_ref().map(|name| name.as_str()).unwrap_or("None"),
                16.0 + x_offset,
                121.0 + y_offset,
                DrawParams::color(MessagePage::BLACK),
            );
            if index == self.cursor {
                draw_cursor(ctx, 10.0 + x_offset, 123.0 + y_offset, Default::default());
            }
        }
    }
}

impl Reset for TargetPanel {
    fn reset(&mut self) {
        let len = self.names.len();
        if self.cursor >= len {
            self.cursor = 0;
        }
    }
}
