use super::Rect;
use super::super::Pos;

pub struct Grid {
    pub x: f64,
    pub y: f64,
    pub w: u32, // Width in cells
    pub h: u32, // Height in cells
    pub cell_w: f64, // Absolute Width of one cell
    pub cell_h: f64, // Absolute Height of one cell
}
impl Grid {
    pub fn new(w: u32, h: u32, rect: Rect) -> Grid {
        let cell_h = rect.h / h as f64;
        let cell_w = rect.w / w as f64;
        Grid {
            x: rect.x,
            y: rect.y,
            h,
            w,
            cell_h,
            cell_w,
        }
    }
    pub fn get_abs_rect(&self, rect: Rect) -> Rect {
        Rect {
            x: rect.x * self.cell_w,
            y: rect.y * self.cell_h,
            w: rect.w * self.cell_w,
            h: rect.h * self.cell_h,
        }
    }
    pub fn get_pos(&self, pos: Pos) -> Pos {
        Pos {x: pos.x * self.cell_w, y: pos.y * self.cell_h}
    }
}