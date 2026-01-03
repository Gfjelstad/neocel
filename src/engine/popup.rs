use serde::Deserialize;

use crate::{engine::layout::LayoutNode, render::Rect};
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RelativeTo {
    Editor,
    Win(String),
    Cursor,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PopupPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottonLeft,
    Center,
    Absolute,
}
#[derive(Clone)]
pub struct PopupWindow {
    pub layout: LayoutNode,
    pub position: PopupPosition,
    pub width: usize,
    pub height: usize,
    pub relative_to: RelativeTo,
    pub row: Option<usize>,
    pub col: Option<usize>,
}
impl PopupWindow {
    pub fn get_rect(&mut self, container: &Rect) -> Result<Rect, String> {
        match self.position {
            PopupPosition::TopRight => Ok(Rect {
                x: container.width - self.width,
                y: 0,
                width: self.width,
                height: self.height,
            }),
            PopupPosition::TopLeft => Ok(Rect {
                x: 0,
                y: 0,
                width: self.width,
                height: self.height,
            }),
            PopupPosition::BottomRight => Ok(Rect {
                x: container.width - self.width,
                y: 0,
                width: self.width,
                height: self.height,
            }),
            PopupPosition::BottonLeft => Ok(Rect {
                x: 0,
                y: container.height - self.height,
                width: self.width,
                height: self.height,
            }),
            PopupPosition::Center => Ok(Rect {
                x: container.width.div_euclid(2) - self.width.div_euclid(2),
                y: container.height.div_euclid(2) - self.height.div_euclid(2),
                width: self.width,
                height: self.height,
            }),
            PopupPosition::Absolute => {
                if self.row.is_none() || self.col.is_none() {
                    return Err(
                        "Absolute position with no location (missing row and col)".to_string()
                    );
                }
                return Ok(Rect {
                    x: self.col.unwrap(),
                    y: self.row.unwrap(),
                    width: self.width,
                    height: self.height,
                });
            }
        }
    }
}
