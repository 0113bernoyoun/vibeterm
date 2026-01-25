//! Binary Split Tree Layout System for VibeTerm
//!
//! This module implements a flexible pane layout system using a binary split tree.
//! Each node is either a Leaf (containing a pane) or a Split (dividing space between two children).

use egui::Rect;
use std::collections::HashMap;

// ============================================================================
// Constants
// ============================================================================

/// Minimum split ratio (10% for the smaller pane)
pub const MIN_SPLIT_RATIO: f32 = 0.1;

/// Maximum split ratio (90% for the larger pane)
pub const MAX_SPLIT_RATIO: f32 = 0.9;

/// Default split ratio (50/50)
pub const DEFAULT_SPLIT_RATIO: f32 = 0.5;

/// Width of the divider between panes in pixels
pub const DIVIDER_WIDTH: f32 = 4.0;

// ============================================================================
// Core Types
// ============================================================================

/// Direction of a split in the layout tree
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// Left | Right split
    Horizontal,
    /// Top / Bottom split
    Vertical,
}

/// Unique identifier for a pane
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaneId(pub u64);

/// A node in the binary split tree layout
pub enum LayoutNode<T> {
    /// A leaf node containing actual content
    Leaf {
        id: PaneId,
        content: T,
    },
    /// A split node dividing space between two children
    Split {
        direction: SplitDirection,
        /// Ratio from 0.0-1.0, representing the first child's portion
        ratio: f32,
        first: Box<LayoutNode<T>>,
        second: Box<LayoutNode<T>>,
    },
}

// ============================================================================
// Layout Computation Types
// ============================================================================

/// Information about a divider for rendering and interaction
pub struct DividerInfo {
    /// Path to this divider's parent split node (false=first child, true=second child)
    pub path: Vec<bool>,
    /// Direction of the split this divider belongs to
    pub direction: SplitDirection,
    /// Screen rectangle of the divider
    pub rect: Rect,
}

/// Result of computing layout for the entire tree
pub struct ComputedLayout {
    /// Map from pane ID to its computed screen rectangle
    pub pane_rects: HashMap<PaneId, Rect>,
    /// All dividers in the layout
    pub dividers: Vec<DividerInfo>,
}

impl ComputedLayout {
    pub fn new() -> Self {
        Self {
            pane_rects: HashMap::new(),
            dividers: Vec::new(),
        }
    }
}

impl Default for ComputedLayout {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Split a rect into two parts with a divider between them
///
/// Returns (first_rect, divider_rect, second_rect)
fn split_rect(
    rect: Rect,
    direction: SplitDirection,
    ratio: f32,
    divider_width: f32,
) -> (Rect, Rect, Rect) {
    let ratio = ratio.clamp(MIN_SPLIT_RATIO, MAX_SPLIT_RATIO);

    match direction {
        SplitDirection::Horizontal => {
            // Left | Right
            let available_width = rect.width() - divider_width;
            let first_width = available_width * ratio;
            let second_width = available_width * (1.0 - ratio);

            let first_rect = Rect::from_min_size(
                rect.min,
                egui::vec2(first_width, rect.height()),
            );
            let divider_rect = Rect::from_min_size(
                egui::pos2(rect.min.x + first_width, rect.min.y),
                egui::vec2(divider_width, rect.height()),
            );
            let second_rect = Rect::from_min_size(
                egui::pos2(rect.min.x + first_width + divider_width, rect.min.y),
                egui::vec2(second_width, rect.height()),
            );

            (first_rect, divider_rect, second_rect)
        }
        SplitDirection::Vertical => {
            // Top / Bottom
            let available_height = rect.height() - divider_width;
            let first_height = available_height * ratio;
            let second_height = available_height * (1.0 - ratio);

            let first_rect = Rect::from_min_size(
                rect.min,
                egui::vec2(rect.width(), first_height),
            );
            let divider_rect = Rect::from_min_size(
                egui::pos2(rect.min.x, rect.min.y + first_height),
                egui::vec2(rect.width(), divider_width),
            );
            let second_rect = Rect::from_min_size(
                egui::pos2(rect.min.x, rect.min.y + first_height + divider_width),
                egui::vec2(rect.width(), second_height),
            );

            (first_rect, divider_rect, second_rect)
        }
    }
}

// ============================================================================
// LayoutNode Implementation
// ============================================================================

impl<T> LayoutNode<T> {
    /// Recursively compute layout rects for all panes and dividers
    pub fn compute_layout(
        &self,
        rect: Rect,
        divider_width: f32,
        path: &mut Vec<bool>,
        output: &mut ComputedLayout,
    ) {
        match self {
            LayoutNode::Leaf { id, .. } => {
                output.pane_rects.insert(*id, rect);
            }
            LayoutNode::Split { direction, ratio, first, second } => {
                let (first_rect, divider_rect, second_rect) =
                    split_rect(rect, *direction, *ratio, divider_width);

                // Record divider with current path
                output.dividers.push(DividerInfo {
                    path: path.clone(),
                    direction: *direction,
                    rect: divider_rect,
                });

                // Recurse into first child
                path.push(false);
                first.compute_layout(first_rect, divider_width, path, output);
                path.pop();

                // Recurse into second child
                path.push(true);
                second.compute_layout(second_rect, divider_width, path, output);
                path.pop();
            }
        }
    }

    /// Count total panes in tree
    pub fn pane_count(&self) -> usize {
        match self {
            LayoutNode::Leaf { .. } => 1,
            LayoutNode::Split { first, second, .. } => {
                first.pane_count() + second.pane_count()
            }
        }
    }

    /// Collect all pane IDs in DFS order
    pub fn collect_pane_ids(&self, out: &mut Vec<PaneId>) {
        match self {
            LayoutNode::Leaf { id, .. } => out.push(*id),
            LayoutNode::Split { first, second, .. } => {
                first.collect_pane_ids(out);
                second.collect_pane_ids(out);
            }
        }
    }

    /// Find path to a specific pane
    pub fn find_path_to_pane(&self, target: PaneId, path: &mut Vec<bool>) -> bool {
        match self {
            LayoutNode::Leaf { id, .. } => *id == target,
            LayoutNode::Split { first, second, .. } => {
                path.push(false);
                if first.find_path_to_pane(target, path) {
                    return true;
                }
                path.pop();

                path.push(true);
                if second.find_path_to_pane(target, path) {
                    return true;
                }
                path.pop();

                false
            }
        }
    }

    /// Get mutable reference to split node at given path
    pub fn get_split_at_path_mut(&mut self, path: &[bool]) -> Option<&mut Self> {
        if path.is_empty() {
            // Return self if it's a Split
            match self {
                LayoutNode::Split { .. } => Some(self),
                LayoutNode::Leaf { .. } => None,
            }
        } else {
            match self {
                LayoutNode::Split { first, second, .. } => {
                    if path[0] {
                        second.get_split_at_path_mut(&path[1..])
                    } else {
                        first.get_split_at_path_mut(&path[1..])
                    }
                }
                LayoutNode::Leaf { .. } => None,
            }
        }
    }

    /// Get the node at given path (for accessing content)
    pub fn get_node_at_path(&self, path: &[bool]) -> Option<&Self> {
        if path.is_empty() {
            Some(self)
        } else {
            match self {
                LayoutNode::Split { first, second, .. } => {
                    if path[0] {
                        second.get_node_at_path(&path[1..])
                    } else {
                        first.get_node_at_path(&path[1..])
                    }
                }
                LayoutNode::Leaf { .. } => None,
            }
        }
    }

    /// Get mutable node at path
    pub fn get_node_at_path_mut(&mut self, path: &[bool]) -> Option<&mut Self> {
        if path.is_empty() {
            Some(self)
        } else {
            match self {
                LayoutNode::Split { first, second, .. } => {
                    if path[0] {
                        second.get_node_at_path_mut(&path[1..])
                    } else {
                        first.get_node_at_path_mut(&path[1..])
                    }
                }
                LayoutNode::Leaf { .. } => None,
            }
        }
    }

    /// Find leaf node by PaneId and return mutable reference to content
    pub fn get_content_mut(&mut self, target: PaneId) -> Option<&mut T> {
        match self {
            LayoutNode::Leaf { id, content } => {
                if *id == target {
                    Some(content)
                } else {
                    None
                }
            }
            LayoutNode::Split { first, second, .. } => {
                first.get_content_mut(target).or_else(|| second.get_content_mut(target))
            }
        }
    }

    /// Find leaf node by PaneId and return reference to content
    pub fn get_content(&self, target: PaneId) -> Option<&T> {
        match self {
            LayoutNode::Leaf { id, content } => {
                if *id == target {
                    Some(content)
                } else {
                    None
                }
            }
            LayoutNode::Split { first, second, .. } => {
                first.get_content(target).or_else(|| second.get_content(target))
            }
        }
    }

    /// Collect all pane contents as mutable references in a single traversal
    /// This is O(n) instead of O(n²) when iterating all panes
    pub fn collect_contents_mut(&mut self) -> Vec<(PaneId, &mut T)> {
        let mut result = Vec::new();
        self.collect_contents_mut_inner(&mut result);
        result
    }

    fn collect_contents_mut_inner<'a>(&'a mut self, out: &mut Vec<(PaneId, &'a mut T)>) {
        match self {
            LayoutNode::Leaf { id, content } => {
                out.push((*id, content));
            }
            LayoutNode::Split { first, second, .. } => {
                first.collect_contents_mut_inner(out);
                second.collect_contents_mut_inner(out);
            }
        }
    }
}

// ============================================================================
// Tree Manipulation Functions (for drag-drop repositioning)
// ============================================================================

/// Extract a pane from the tree, promoting its sibling
/// Returns (new_tree_without_pane, extracted_content) or None if pane not found or is only pane
pub fn extract_pane<T>(
    node: LayoutNode<T>,
    target_id: PaneId,
) -> Option<(LayoutNode<T>, T)> {
    // Helper to check if a pane exists in subtree (without moving)
    fn contains_pane<T>(node: &LayoutNode<T>, target_id: PaneId) -> bool {
        match node {
            LayoutNode::Leaf { id, .. } => *id == target_id,
            LayoutNode::Split { first, second, .. } => {
                contains_pane(first, target_id) || contains_pane(second, target_id)
            }
        }
    }

    match node {
        LayoutNode::Leaf { id, .. } if id == target_id => {
            // Cannot extract the only pane
            None
        }
        LayoutNode::Leaf { .. } => {
            // Not the target
            None
        }
        LayoutNode::Split { direction, ratio, first, second } => {
            // Check if first child IS the target leaf
            if let LayoutNode::Leaf { id, .. } = first.as_ref() {
                if *id == target_id {
                    let content = match *first {
                        LayoutNode::Leaf { content, .. } => content,
                        _ => unreachable!(),
                    };
                    return Some((*second, content));
                }
            }

            // Check if second child IS the target leaf
            if let LayoutNode::Leaf { id, .. } = second.as_ref() {
                if *id == target_id {
                    let content = match *second {
                        LayoutNode::Leaf { content, .. } => content,
                        _ => unreachable!(),
                    };
                    return Some((*first, content));
                }
            }

            // Neither is a direct match, check which subtree contains target
            if contains_pane(&first, target_id) {
                // Target is in first subtree
                if let Some((new_first, content)) = extract_pane(*first, target_id) {
                    return Some((
                        LayoutNode::Split {
                            direction,
                            ratio,
                            first: Box::new(new_first),
                            second,
                        },
                        content,
                    ));
                }
            } else if contains_pane(&second, target_id) {
                // Target is in second subtree
                if let Some((new_second, content)) = extract_pane(*second, target_id) {
                    return Some((
                        LayoutNode::Split {
                            direction,
                            ratio,
                            first,
                            second: Box::new(new_second),
                        },
                        content,
                    ));
                }
            }

            None
        }
    }
}

/// Insert content adjacent to a target pane, creating a new split
/// before=true means new pane becomes first child (top/left)
/// Returns the new tree (unchanged if target not found)
pub fn insert_adjacent<T>(
    node: LayoutNode<T>,
    target_id: PaneId,
    new_id: PaneId,
    new_content: T,
    split_direction: SplitDirection,
    before: bool,
) -> LayoutNode<T> {
    fn insert_impl<T>(
        node: LayoutNode<T>,
        target_id: PaneId,
        new_id: PaneId,
        new_content: Option<T>,
        split_direction: SplitDirection,
        before: bool,
    ) -> (LayoutNode<T>, Option<T>) {
        match node {
            LayoutNode::Leaf { id, content } if id == target_id => {
                if let Some(nc) = new_content {
                    let target_leaf = LayoutNode::Leaf { id, content };
                    let new_leaf = LayoutNode::Leaf { id: new_id, content: nc };

                    let (first, second) = if before {
                        (new_leaf, target_leaf)
                    } else {
                        (target_leaf, new_leaf)
                    };

                    (LayoutNode::Split {
                        direction: split_direction,
                        ratio: DEFAULT_SPLIT_RATIO,
                        first: Box::new(first),
                        second: Box::new(second),
                    }, None)
                } else {
                    (LayoutNode::Leaf { id, content }, None)
                }
            }
            LayoutNode::Leaf { id, content } => {
                (LayoutNode::Leaf { id, content }, new_content)
            }
            LayoutNode::Split { direction, ratio, first, second } => {
                let (new_first, remaining) = insert_impl(*first, target_id, new_id, new_content, split_direction, before);
                if remaining.is_none() {
                    // Inserted in first branch
                    return (LayoutNode::Split {
                        direction,
                        ratio,
                        first: Box::new(new_first),
                        second,
                    }, None);
                }

                let (new_second, remaining) = insert_impl(*second, target_id, new_id, remaining, split_direction, before);
                (LayoutNode::Split {
                    direction,
                    ratio,
                    first: Box::new(new_first),
                    second: Box::new(new_second),
                }, remaining)
            }
        }
    }

    let (result, _) = insert_impl(node, target_id, new_id, Some(new_content), split_direction, before);
    result
}
