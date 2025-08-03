use crate::node::{Node, Number};
use crate::tree::Tree;

/// A Binary Search Tree implementation
#[derive(Debug)]
pub struct BST<T: Ord + Clone> {
    pub tree: Tree<T>,
}

impl<T: Ord + Clone> BST<T> {
    /// Create a new empty BST
    pub fn new() -> Self {
        Self {
            tree: Tree::new(),
        }
    }

    /// Insert an element into the BST
    pub fn insert(&mut self, element: T) {
        if self.tree.is_empty() {
            let node = Node::new(element);
            let id = self.tree.add_node(node);
            self.tree.set_root(id);
            return;
        }

        let root_id = self.tree.root_id().unwrap();
        self.insert_recursive(root_id, element);
    }

    fn insert_recursive(&mut self, node_id: Number, element: T) {
        if let Some(node) = self.tree.get_node(node_id) {
            let current_value = &node.value;

            match element.cmp(current_value) {
                std::cmp::Ordering::Less => {
                    if let Some(left_id) = node.left() {
                        self.insert_recursive(left_id, element);
                    } else {
                        let new_node = Node::new(element);
                        let new_id = self.tree.add_node(new_node);
                        if let Some(parent) = self.tree.get_node_mut(node_id) {
                            parent.set_left(new_id);
                            parent.add_child(new_id);
                        }
                        if let Some(child) = self.tree.get_node_mut(new_id) {
                            child.set_parent(node_id);
                        }
                    }
                }
                std::cmp::Ordering::Greater => {
                    if let Some(right_id) = node.right() {
                        self.insert_recursive(right_id, element);
                    } else {
                        let new_node = Node::new(element);
                        let new_id = self.tree.add_node(new_node);
                        if let Some(parent) = self.tree.get_node_mut(node_id) {
                            parent.set_right(new_id);
                            parent.add_child(new_id);
                        }
                        if let Some(child) = self.tree.get_node_mut(new_id) {
                            child.set_parent(node_id);
                        }
                    }
                }
                std::cmp::Ordering::Equal => {
                    // Element already exists, don't insert duplicates
                }
            }
        }
    }

    /// Search for an element in the BST
    pub fn search(&self, element: &T) -> Option<Number> {
        if let Some(root_id) = self.tree.root_id() {
            self.search_recursive(root_id, element)
        } else {
            None
        }
    }

    fn search_recursive(&self, node_id: Number, element: &T) -> Option<Number> {
        if let Some(node) = self.tree.get_node(node_id) {
            match element.cmp(&node.value) {
                std::cmp::Ordering::Equal => Some(node_id),
                std::cmp::Ordering::Less => {
                    if let Some(left_id) = node.left() {
                        self.search_recursive(left_id, element)
                    } else {
                        None
                    }
                }
                std::cmp::Ordering::Greater => {
                    if let Some(right_id) = node.right() {
                        self.search_recursive(right_id, element)
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    /// Delete an element from the BST
    pub fn delete(&mut self, element: &T) {
        if let Some(_root_id) = self.tree.root_id() {
            if let Some(node_id) = self.search(element) {
                self.delete_node(node_id);
            }
        }
    }

    fn delete_node(&mut self, node_id: Number) {
        if let Some(node) = self.tree.get_node(node_id) {
            let has_left = node.left().is_some();
            let has_right = node.right().is_some();

            match (has_left, has_right) {
                (false, false) => {
                    // Node with no children
                    self.remove_leaf_node(node_id);
                }
                (true, false) => {
                    // Node with only left child
                    self.remove_node_with_one_child(node_id, true);
                }
                (false, true) => {
                    // Node with only right child
                    self.remove_node_with_one_child(node_id, false);
                }
                (true, true) => {
                    // Node with two children
                    self.remove_node_with_two_children(node_id);
                }
            }
        }
    }

    fn remove_leaf_node(&mut self, node_id: Number) {
        if let Some(node) = self.tree.get_node(node_id) {
            if let Some(parent_id) = node.parent() {
                if let Some(parent) = self.tree.get_node_mut(parent_id) {
                    if parent.left() == Some(node_id) {
                        parent.clear_left();
                    } else if parent.right() == Some(node_id) {
                        parent.clear_right();
                    }
                    parent.remove_child(node_id);
                }
            } else {
                // This is the root node
                self.tree.set_root_id(None);
            }
        }
        self.tree.remove_node(node_id);
    }

    fn remove_node_with_one_child(&mut self, node_id: Number, has_left: bool) {
        // Get the necessary information first
        let (child_id, parent_id) = if let Some(node) = self.tree.get_node(node_id) {
            let child_id = if has_left {
                node.left().unwrap()
            } else {
                node.right().unwrap()
            };
            (child_id, node.parent())
        } else {
            return;
        };

        // Update parent-child relationships
        if let Some(parent_id) = parent_id {
            if let Some(parent) = self.tree.get_node_mut(parent_id) {
                if parent.left() == Some(node_id) {
                    parent.set_left(child_id);
                } else if parent.right() == Some(node_id) {
                    parent.set_right(child_id);
                }
                parent.remove_child(node_id);
                parent.add_child(child_id);
            }
        } else {
            // This is the root node
            self.tree.set_root(child_id);
        }

        // Update child's parent
        if let Some(child) = self.tree.get_node_mut(child_id) {
            child.set_parent(parent_id.unwrap_or(0.0));
        }

        // Remove the node
        self.tree.remove_node(node_id);
    }

    fn remove_node_with_two_children(&mut self, node_id: Number) {
        if let Some(node) = self.tree.get_node(node_id) {
            if let Some(right_id) = node.right() {
                // Find the inorder successor (smallest value in right subtree)
                let successor_id = self.find_min(right_id);

                // Copy successor's value to current node
                if let Some(successor) = self.tree.get_node(successor_id) {
                    let successor_value = successor.value.clone();
                    if let Some(current) = self.tree.get_node_mut(node_id) {
                        current.value = successor_value;
                    }
                }

                // Delete the successor
                self.delete_node(successor_id);
            }
        }
    }

    fn find_min(&self, node_id: Number) -> Number {
        if let Some(node) = self.tree.get_node(node_id) {
            if let Some(left_id) = node.left() {
                self.find_min(left_id)
            } else {
                node_id
            }
        } else {
            node_id
        }
    }

    /// Perform inorder traversal of the BST
    pub fn inorder(&self) -> Vec<Number> {
        if let Some(root_id) = self.tree.root_id() {
            self.inorder_recursive(root_id)
        } else {
            Vec::new()
        }
    }

    fn inorder_recursive(&self, node_id: Number) -> Vec<Number> {
        let mut result = Vec::new();

        if let Some(node) = self.tree.get_node(node_id) {
            // Traverse left subtree
            if let Some(left_id) = node.left() {
                result.extend(self.inorder_recursive(left_id));
            }

            // Visit current node
            result.push(node_id);

            // Traverse right subtree
            if let Some(right_id) = node.right() {
                result.extend(self.inorder_recursive(right_id));
            }
        }

        result
    }

    /// Get the root node ID
    pub fn root(&self) -> Option<Number> {
        self.tree.root_id()
    }

    /// Get the size of the BST
    pub fn size(&self) -> usize {
        self.tree.size()
    }

    /// Check if the BST is empty
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
}

impl<T: Ord + Clone> Default for BST<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_operations() {
        let mut bst = BST::new();

        assert!(bst.is_empty());
        assert_eq!(bst.size(), 0);

        // Insert elements
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(9);

        assert_eq!(bst.size(), 5);
        assert!(!bst.is_empty());

        // Test search
        assert!(bst.search(&5).is_some());
        assert!(bst.search(&3).is_some());
        assert!(bst.search(&10).is_none());

        // Test inorder traversal
        let inorder = bst.inorder();
        assert_eq!(inorder.len(), 5);

        // Verify inorder gives sorted order
        let values: Vec<i32> = inorder
            .iter()
            .map(|&id| bst.tree.get_node(id).unwrap().value)
            .collect();
        assert_eq!(values, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_bst_deletion() {
        let mut bst = BST::new();

        // Insert elements
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(9);

        assert_eq!(bst.size(), 5);

        // Delete leaf node
        bst.delete(&1);
        assert_eq!(bst.size(), 4);
        assert!(bst.search(&1).is_none());

        // Delete node with one child
        bst.delete(&3);
        assert_eq!(bst.size(), 3);
        assert!(bst.search(&3).is_none());

        // Delete node with two children
        bst.delete(&5);
        assert_eq!(bst.size(), 2);
        assert!(bst.search(&5).is_none());

        // Verify remaining elements
        assert!(bst.search(&7).is_some());
        assert!(bst.search(&9).is_some());
    }

    #[test]
    fn test_bst_duplicate_insertion() {
        let mut bst = BST::new();

        bst.insert(5);
        bst.insert(5); // Duplicate

        assert_eq!(bst.size(), 1); // Should only have one element
    }
}
