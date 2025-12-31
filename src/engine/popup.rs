use crate::{engine::layout::LayoutNode, render::Rect};

#[derive(Clone)]
pub enum PopupPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottonLeft,
    Center,
}
#[derive(Clone)]
pub struct PopupWindow {
    pub layout: LayoutNode,
    pub position: PopupPosition,
    pub width: usize,
    pub height: usize,
}
impl PopupWindow {
    pub fn get_rect(&mut self, container: &Rect) -> Rect {
        match self.position {
            PopupPosition::TopRight => Rect {
                x: container.width - self.width,
                y: 0,
                width: self.width,
                height: self.height,
            },
            PopupPosition::TopLeft => Rect {
                x: 0,
                y: 0,
                width: self.width,
                height: self.height,
            },
            PopupPosition::BottomRight => Rect {
                x: container.width - self.width,
                y: 0,
                width: self.width,
                height: self.height,
            },
            PopupPosition::BottonLeft => Rect {
                x: 0,
                y: container.height - self.height,
                width: self.width,
                height: self.height,
            },
            PopupPosition::Center => Rect {
                x: container.width.div_euclid(2) - self.width.div_euclid(2),
                y: container.height.div_euclid(2) - self.height.div_euclid(2),
                width: self.width,
                height: self.height,
            },
        }
    }
}
