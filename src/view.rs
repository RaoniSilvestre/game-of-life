use macroquad::prelude::*;

use crate::automaton::{Automaton, Point};

const HEADER: f32 = 40.0;

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    origin_x: f32,
    origin_y: f32,
    cell_w: f32,
    cell_h: f32,
    width: usize,
    height: usize,
}

impl Layout {
    pub fn compute<A: Automaton>(automaton: &A) -> Self {
        let board = automaton.board();
        let width = board.width();
        let height = board.height();

        let sw = screen_width();
        let sh = screen_height() - HEADER;
        let cell_w = sw / width as f32;
        let cell_h = sh / height as f32;

        Self {
            origin_x: 0.0,
            origin_y: HEADER,
            cell_w,
            cell_h,
            width,
            height,
        }
    }

    pub fn screen_to_cell(&self, (mx, my): (f32, f32)) -> Option<Point> {
        if mx < self.origin_x || my < self.origin_y {
            return None;
        }

        let col = ((mx - self.origin_x) / self.cell_w) as usize;
        let row = ((my - self.origin_y) / self.cell_h) as usize;

        if col >= self.width || row >= self.height {
            return None;
        }

        Some((row as i32, col as i32))
    }
}

pub fn draw<A: Automaton>(automaton: &A, layout: &Layout) {
    clear_background(BLACK);

    let board = automaton.board();

    let grid_color = Color::from_rgba(30, 30, 30, 255);
    let min_dim = layout.cell_w.min(layout.cell_h);
    let gap = (min_dim * 0.08).max(1.0).min(min_dim / 3.0);

    for (idx, cell) in board.cells().iter().enumerate() {
        let row = idx / layout.width;
        let col = idx % layout.width;
        let x = layout.origin_x + col as f32 * layout.cell_w;
        let y = layout.origin_y + row as f32 * layout.cell_h;

        draw_rectangle(x, y, layout.cell_w, layout.cell_h, grid_color);
        draw_rectangle(
            x + gap,
            y + gap,
            layout.cell_w - 2.0 * gap,
            layout.cell_h - 2.0 * gap,
            (*cell).into(),
        );
    }

    let title = format!("{} — Iterations: {}", A::name(), automaton.iterations());
    draw_text(&title, 10.0, 24.0, 24.0, WHITE);
}
