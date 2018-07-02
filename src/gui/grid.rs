use super::Rect;
use super::super::Pos;
use coord::vec2::Vec2;

/// A grid that consists of a certain number of a cells.
pub struct Grid {
    /// The number of cells that make the width and height of the grid.
    pub cells: Vec2<u32>,
    /// The position of the Top-Left corner of the grid in the game.
    pub pos: Pos,
    /// The size of one cell
    cell_size: Vec2<f64>,
}
impl Grid {
    /// Create a new [`Grid`] struct.
    pub fn new(w: u32, h: u32, rect: Rect) -> Grid {
        let cells = vec2![w, h];
        let cell_size = rect.size / vec2![cells.x as f64, cells.y as f64];
        Grid {
            cells,
            pos: rect.pos,
            cell_size,
        }
    }
    /// Turn the rectangle with values of the grid to a rectangle with values in pixels.
    pub fn get_abs_rect(&self, rect: Rect) -> Rect {
        Rect {
            pos: rect.pos * self.cell_size,
            size: rect.size * self.cell_size,
        }
    }
    /// Turn a position with values of the grid to a position with values in pixels.
    pub fn get_pos(&self, pos: Pos) -> Pos {
        pos * self.cell_size
    }
}