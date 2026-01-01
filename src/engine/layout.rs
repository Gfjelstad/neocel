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
    pub fn remove_window(self, target: &WindowId) -> Option<LayoutNode> {
        match self {
            LayoutNode::Leaf(id) => {
                if &id == target {
                    None // delete this leaf
                } else {
                    Some(LayoutNode::Leaf(id))
                }
            }

            LayoutNode::Split {
                direction,
                ratio,
                first,
                second,
            } => {
                let left = first.remove_window(target);
                let right = second.remove_window(target);

                match (left, right) {
                    (None, None) => None, // entire subtree vanished
                    (Some(node), None) | (None, Some(node)) => {
                        // Collapse the split — sibling takes over
                        Some(node)
                    }
                    (Some(l), Some(r)) => {
                        // Both sides still exist, keep the split
                        Some(LayoutNode::Split {
                            direction,
                            ratio,
                            first: Box::new(l),
                            second: Box::new(r),
                        })
                    }
                }
            }
        }
    }
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
