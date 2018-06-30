use super::Rect;
use super::super::Pos;

/// A grid that consists of a certain number of a cells.
pub struct Grid {
    /// The x co-ordinate of the top-left corner of the grid.
    pub x: f64,
    /// The y co-ordinate of the top-left corner of the grid.
    pub y: f64,
    /// The width of the grid in cells.
    pub w: u32, // Width in cells
    /// The height of the grid in cells.
    pub h: u32, // Height in cells
    /// The width of a cell in pixels.
    pub cell_w: f64, // Absolute Width of one cell
    /// The height of a cell in pixels.
    pub cell_h: f64, // Absolute Height of one cell
}
impl Grid {
    /// Create a new ``Grid`` struct.
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
    /// Turn the rectangle with values of the grid to a rectangle with values in pixels.
    pub fn get_abs_rect(&self, rect: Rect) -> Rect {
        Rect {
            x: rect.x * self.cell_w,
            y: rect.y * self.cell_h,
            w: rect.w * self.cell_w,
            h: rect.h * self.cell_h,
        }
    }
    /// Turn a position with values of the grid to a position with values in pixels.
    pub fn get_pos(&self, pos: Pos) -> Pos {
        Pos {x: pos.x * self.cell_w, y: pos.y * self.cell_h}
    }
}