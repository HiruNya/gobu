use super::Rect;
use super::super::Pos;
use coord::vec2::Vec2;

/// A grid that consists of a certain number of a cells.
pub struct Grid {
    /// The number of cells that make the width and height of the grid.
    pub cells: Vec2<u32>,
    /// The position of the Top-Left corner of the grid in the game.
    pub pos: Pos,
//    /// The x co-ordinate of the top-left corner of the grid.
//    pub x: f64,
//    /// The y co-ordinate of the top-left corner of the grid.
//    pub y: f64,
//    /// The width of the grid in cells.
//    pub w: u32, // Width in cells
//    /// The height of the grid in cells.
//    pub h: u32, // Height in cells
//    /// The width of a cell in pixels.
//    pub cell_w: f64, // Absolute Width of one cell
//    /// The height of a cell in pixels.
//    pub cell_h: f64, // Absolute Height of one cell
    /// The size of one cell
    cell_size: Vec2<f64>,
}
impl Grid {
    /// Create a new [`Grid`] struct.
    pub fn new(w: u32, h: u32, rect: Rect) -> Grid {
        let cells = vec2![w, h];
//        let cell_h = rect.h / h as f64;
//        let cell_w = rect.w / w as f64;
        let cell_size = rect.size / vec2![cells.x as f64, cells.y as f64];
        Grid {
            cells,
            pos: rect.pos,
//            h,
//            w,
            cell_size,
//            cell_h,
//            cell_w,
        }
    }
    /// Turn the rectangle with values of the grid to a rectangle with values in pixels.
    pub fn get_abs_rect(&self, rect: Rect) -> Rect {
        Rect {
            pos: rect.pos * self.cell_size,
            size: rect.size * self.cell_size,
//            x: rect.x * self.cell_w,
//            y: rect.y * self.cell_h,
//            w: rect.w * self.cell_w,
//            h: rect.h * self.cell_h,
        }
    }
    /// Turn a position with values of the grid to a position with values in pixels.
    pub fn get_pos(&self, pos: Pos) -> Pos {
//        Pos {x: pos.x * self.cell_w, y: pos.y * self.cell_h}
        pos * self.cell_size
    }
}