use crate::{engine::WindowId, render::Rect};

#[derive(Clone)]
pub enum SplitDir {
    Vert,
    Horz,
}
#[derive(Clone)]
pub enum LayoutNode {
    Leaf(WindowId),
    Split {
        direction: SplitDir,
        ratio: f32,
        first: Box<LayoutNode>,
        second: Box<LayoutNode>,
    },
}
impl LayoutNode {
    pub fn walk_nodes<F>(&self, rect: &Rect, f: &mut F)
    where
        F: FnMut(&WindowId, &Rect),
    {
        match self {
            LayoutNode::Leaf(typed) => f(typed, rect),
            LayoutNode::Split {
                direction,
                ratio,
                first,
                second,
            } => {
                let (rect_1, rect_2) = match direction {
                    SplitDir::Vert => {
                        let split_value = (rect.height as f32 * ratio) as usize;
                        (
                            Rect {
                                x: rect.x,
                                y: rect.y,
                                width: rect.width,
                                height: split_value,
                            },
                            Rect {
                                x: rect.x,
                                y: rect.y + split_value,
                                width: rect.width,
                                height: rect.height - split_value,
                            },
                        )
                    }
                    SplitDir::Horz => {
                        let split_value = (rect.width as f32 * ratio) as usize;
                        (
                            Rect {
                                x: rect.x,
                                y: rect.y,
                                width: split_value,
                                height: rect.height,
                            },
                            Rect {
                                x: rect.x + split_value,
                                y: rect.y,
                                width: rect.width - split_value,
                                height: rect.height,
                            },
                        )
                    }
                };
                first.walk_nodes(&rect_1, f);
                second.walk_nodes(&rect_2, f);
            }
        }
    }
    pub fn get_rects(&self, rect: &Rect) -> Vec<(WindowId, Rect)> {
        let mut result = vec![];
        self.walk_nodes(rect, &mut |node, r| result.push((node.clone(), r.clone())));
        result
    }
    pub fn find_child(&mut self, window_id: String) -> Option<&mut LayoutNode> {
        let mut search = vec![self];

        while let Some(current) = search.pop() {
            match current {
                LayoutNode::Leaf(curr_id) => {
                    if &window_id == curr_id {
                        return Some(current);
                    }
                }
                LayoutNode::Split {
                    direction: _,
                    ratio: _,
                    first,
                    second,
                } => {
                    search.push(first);
                    search.push(second);
                }
            }
        }
        None
    }
}
