use macroquad::prelude::*;

use crate::{
    automaton::Automaton,
    view::{self, Layout},
    Configuration, Res,
};

#[derive(Debug)]
pub struct Runner<A: Automaton> {
    automaton: A,
    config: Configuration,
    stop: bool,
}

impl<A: Automaton> Runner<A> {
    pub fn new(config: Configuration, scene: A::Scene) -> Self {
        let automaton = A::from(scene);

        Runner {
            automaton,
            config,
            stop: true,
        }
    }

    pub async fn run(&mut self) -> Res<()> {
        let tick_interval = 1.0 / self.config.fps as f64;
        let mut last_step = get_time();

        loop {
            let layout = Layout::compute(&self.automaton);
            self.handle_input(&layout);

            if !self.stop && get_time() - last_step >= tick_interval {
                self.automaton.step();
                last_step = get_time();
            }

            view::draw::<A>(&self.automaton, &layout);

            next_frame().await;
        }
    }

    fn handle_input(&mut self, layout: &Layout) {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            std::process::exit(0);
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            self.stop = !self.stop;
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(p) = layout.screen_to_cell(mouse_position()) {
                self.automaton.paint_left(p);
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            if let Some(p) = layout.screen_to_cell(mouse_position()) {
                self.automaton.paint_right(p);
            }
        }
    }
}
