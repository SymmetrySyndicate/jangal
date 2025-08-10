use crate::Tree;
use crate::{Node, Number};

/// A Binary Search Tree implementation
///
/// This BST provides efficient insertion, deletion, and search operations
/// with O(log n) average case complexity for balanced trees.
///
/// # Examples
///
/// ```
/// use jangal::BST;
///
/// let mut bst = BST::new();
/// bst.insert(5);
/// bst.insert(3);
/// bst.insert(7);
///
/// assert_eq!(bst.size(), 3);
/// assert!(bst.search(&5).is_some());
/// assert!(bst.search(&10).is_none());
/// ```
#[derive(Debug)]
pub struct BST<T: Ord + Clone> {
    pub tree: Tree<T>,
}

impl<T: Ord + Clone> BST<T> {
    /// Create a new empty BST
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let bst: BST<i32> = BST::new();
    /// assert!(bst.is_empty());
    /// assert_eq!(bst.size(), 0);
    /// ```
    pub fn new() -> Self {
        Self { tree: Tree::new() }
    }

    /// Insert an element into the BST
    ///
    /// If the element already exists, it will not be inserted (no duplicates).
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(7);
    ///
    /// assert_eq!(bst.size(), 3);
    /// assert!(bst.search(&5).is_some());
    /// assert!(bst.search(&3).is_some());
    /// assert!(bst.search(&7).is_some());
    /// ```
    pub fn insert(&mut self, element: T) {
        if self.tree.is_empty() {
            let node = Node::new(element);
            if let Some(id) = self.tree.add_node(node) {
                self.tree.set_root(id);
            }
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
                        if let Some(new_id) = self.tree.add_node(new_node) {
                            if let Some(parent) = self.tree.get_node_mut(node_id) {
                                parent.set_left(new_id);
                                parent.add_child(new_id);
                            }
                            if let Some(child) = self.tree.get_node_mut(new_id) {
                                child.set_parent(node_id);
                            }
                        }
                    }
                }
                std::cmp::Ordering::Greater => {
                    if let Some(right_id) = node.right() {
                        self.insert_recursive(right_id, element);
                    } else {
                        let new_node = Node::new(element);
                        if let Some(new_id) = self.tree.add_node(new_node) {
                            if let Some(parent) = self.tree.get_node_mut(node_id) {
                                parent.set_right(new_id);
                                parent.add_child(new_id);
                            }
                            if let Some(child) = self.tree.get_node_mut(new_id) {
                                child.set_parent(node_id);
                            }
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
    ///
    /// Returns `Some(node_id)` if the element is found, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(7);
    ///
    /// assert!(bst.search(&5).is_some());
    /// assert!(bst.search(&3).is_some());
    /// assert!(bst.search(&10).is_none());
    /// ```
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
    ///
    /// If the element is not found, no action is taken.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(7);
    ///
    /// assert_eq!(bst.size(), 3);
    /// bst.delete(&3);
    /// assert_eq!(bst.size(), 2);
    /// assert!(bst.search(&3).is_none());
    /// ```
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
                    // Node with no children - remove leaf node
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
                    self.tree.remove_node(node_id);
                }
                (true, false) | (false, true) => {
                    // Node with only one child
                    let child_id = if has_left {
                        node.left().unwrap()
                    } else {
                        node.right().unwrap()
                    };
                    let parent_id = node.parent();

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
                (true, true) => {
                    // Node with two children
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
    ///
    /// Returns a vector of references to nodes in ascending order (sorted).
    /// This is the main traversal method for BSTs that provides sorted output.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(7);
    /// bst.insert(1);
    /// bst.insert(9);
    ///
    /// let inorder = bst.inorder();
    /// let values: Vec<i32> = inorder
    ///     .iter()
    ///     .map(|node| node.value)
    ///     .collect();
    /// assert_eq!(values, vec![1, 3, 5, 7, 9]);
    /// ```
    pub fn inorder(&self) -> Vec<&Node<T>> {
        if let Some(root_id) = self.tree.root_id() {
            self.inorder_recursive(root_id)
        } else {
            Vec::new()
        }
    }

    fn inorder_recursive(&self, node_id: Number) -> Vec<&Node<T>> {
        let mut result = Vec::new();

        if let Some(node) = self.tree.get_node(node_id) {
            // Traverse left subtree
            if let Some(left_id) = node.left() {
                result.extend(self.inorder_recursive(left_id));
            }

            // Visit current node
            result.push(node);

            // Traverse right subtree
            if let Some(right_id) = node.right() {
                result.extend(self.inorder_recursive(right_id));
            }
        }

        result
    }

    /// Find the minimum value in the BST
    ///
    /// Returns `Some(min_value)` if the BST is not empty, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// assert_eq!(bst.min(), None);
    ///
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(7);
    /// bst.insert(1);
    ///
    /// assert_eq!(bst.min(), Some(&1));
    /// ```
    pub fn min(&self) -> Option<&T> {
        if let Some(root_id) = self.tree.root_id() {
            let min_id = self.find_min(root_id);
            self.tree.get_node(min_id).map(|node| &node.value)
        } else {
            None
        }
    }

    /// Find the maximum value in the BST
    ///
    /// Returns `Some(max_value)` if the BST is not empty, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// assert_eq!(bst.max(), None);
    ///
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(7);
    /// bst.insert(9);
    ///
    /// assert_eq!(bst.max(), Some(&9));
    /// ```
    pub fn max(&self) -> Option<&T> {
        if let Some(root_id) = self.tree.root_id() {
            let max_id = self.find_max(root_id);
            self.tree.get_node(max_id).map(|node| &node.value)
        } else {
            None
        }
    }

    fn find_max(&self, node_id: Number) -> Number {
        if let Some(node) = self.tree.get_node(node_id) {
            if let Some(right_id) = node.right() {
                self.find_max(right_id)
            } else {
                node_id
            }
        } else {
            node_id
        }
    }

    /// Check if the BST contains a given element
    ///
    /// Returns `true` if the element is found, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(7);
    ///
    /// assert!(bst.contains(&5));
    /// assert!(bst.contains(&3));
    /// assert!(!bst.contains(&10));
    /// ```
    pub fn contains(&self, element: &T) -> bool {
        self.search(element).is_some()
    }

    // Delegate to Tree methods for common operations
    /// Get the root node ID
    ///
    /// Returns `Some(root_id)` if the BST is not empty, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// assert_eq!(bst.root(), None);
    ///
    /// bst.insert(5);
    /// assert!(bst.root().is_some());
    /// ```
    pub fn root(&self) -> Option<Number> {
        self.tree.root_id()
    }

    /// Get the size of the BST
    ///
    /// Returns the number of nodes in the BST.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// assert_eq!(bst.size(), 0);
    ///
    /// bst.insert(5);
    /// bst.insert(3);
    /// assert_eq!(bst.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.tree.size()
    }

    /// Check if the BST is empty
    ///
    /// Returns `true` if the BST contains no nodes, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// assert!(bst.is_empty());
    ///
    /// bst.insert(5);
    /// assert!(!bst.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    /// Get the height of the BST
    ///
    /// Returns the maximum depth of any leaf node from the root.
    /// An empty tree has height 0, a single node has height 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// assert_eq!(bst.height(), 0);
    ///
    /// bst.insert(5);
    /// assert_eq!(bst.height(), 1);
    ///
    /// bst.insert(3);
    /// bst.insert(7);
    /// assert_eq!(bst.height(), 2);
    /// ```
    pub fn height(&self) -> usize {
        if let Some(root_id) = self.tree.root_id() {
            self.calculate_height(root_id)
        } else {
            0
        }
    }

    fn calculate_height(&self, node_id: Number) -> usize {
        if let Some(node) = self.tree.get_node(node_id) {
            let left_height = if let Some(left_id) = node.left() {
                self.calculate_height(left_id)
            } else {
                0
            };

            let right_height = if let Some(right_id) = node.right() {
                self.calculate_height(right_id)
            } else {
                0
            };

            1 + left_height.max(right_height)
        } else {
            0
        }
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
        let values: Vec<i32> = inorder.iter().map(|node| node.value).collect();
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

    #[test]
    fn test_bst_min_max() {
        let mut bst = BST::new();

        assert_eq!(bst.min(), None);
        assert_eq!(bst.max(), None);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(9);

        assert_eq!(bst.min(), Some(&1));
        assert_eq!(bst.max(), Some(&9));
    }

    #[test]
    fn test_bst_contains() {
        let mut bst = BST::new();

        assert!(!bst.contains(&5));

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);

        assert!(bst.contains(&5));
        assert!(bst.contains(&3));
        assert!(bst.contains(&7));
        assert!(!bst.contains(&10));
    }

    #[test]
    fn test_bst_height() {
        let mut bst = BST::new();

        assert_eq!(bst.height(), 0);

        bst.insert(5);
        assert_eq!(bst.height(), 1);

        bst.insert(3);
        bst.insert(7);
        assert_eq!(bst.height(), 2);

        bst.insert(1);
        bst.insert(9);
        assert_eq!(bst.height(), 3);
    }

    #[test]
    fn test_bst_empty_operations() {
        let bst: BST<i32> = BST::new();

        assert!(bst.is_empty());
        assert_eq!(bst.size(), 0);
        assert_eq!(bst.root(), None);
        assert_eq!(bst.min(), None);
        assert_eq!(bst.max(), None);
        assert_eq!(bst.inorder(), Vec::<&Node<i32>>::new());
    }

    #[test]
    fn test_bst_single_node() {
        let mut bst = BST::new();
        bst.insert(42);

        assert_eq!(bst.size(), 1);
        assert!(!bst.is_empty());
        assert!(bst.root().is_some());
        assert_eq!(bst.min(), Some(&42));
        assert_eq!(bst.max(), Some(&42));
        assert_eq!(bst.height(), 1);

        let inorder = bst.inorder();
        assert_eq!(inorder.len(), 1);
        assert_eq!(inorder[0].value, 42);
    }

    #[test]
    fn test_bst_left_heavy() {
        let mut bst = BST::new();

        // Create a left-heavy tree: 5 -> 3 -> 1
        bst.insert(5);
        bst.insert(3);
        bst.insert(1);

        assert_eq!(bst.size(), 3);
        assert_eq!(bst.height(), 3);
        assert_eq!(bst.min(), Some(&1));
        assert_eq!(bst.max(), Some(&5));

        let inorder = bst.inorder();
        let values: Vec<i32> = inorder.iter().map(|node| node.value).collect();
        assert_eq!(values, vec![1, 3, 5]);
    }

    #[test]
    fn test_bst_right_heavy() {
        let mut bst = BST::new();

        // Create a right-heavy tree: 1 -> 3 -> 5
        bst.insert(1);
        bst.insert(3);
        bst.insert(5);

        assert_eq!(bst.size(), 3);
        assert_eq!(bst.height(), 3);
        assert_eq!(bst.min(), Some(&1));
        assert_eq!(bst.max(), Some(&5));

        let inorder = bst.inorder();
        let values: Vec<i32> = inorder.iter().map(|node| node.value).collect();
        assert_eq!(values, vec![1, 3, 5]);
    }

    #[test]
    fn test_bst_delete_root_with_one_child() {
        let mut bst = BST::new();

        bst.insert(5);
        bst.insert(3);

        // Delete root (5) which has only left child (3)
        bst.delete(&5);

        assert_eq!(bst.size(), 1);
        assert!(bst.search(&5).is_none());
        assert!(bst.search(&3).is_some());
        assert_eq!(bst.root(), bst.search(&3));
    }

    #[test]
    fn test_bst_delete_root_with_two_children() {
        let mut bst = BST::new();

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);

        // Delete root (5) which has two children
        bst.delete(&5);

        assert_eq!(bst.size(), 2);
        assert!(bst.search(&5).is_none());
        assert!(bst.search(&3).is_some());
        assert!(bst.search(&7).is_some());

        // The inorder successor (7) should replace 5
        let inorder = bst.inorder();
        let values: Vec<i32> = inorder.iter().map(|node| node.value).collect();
        assert_eq!(values, vec![3, 7]);
    }

    #[test]
    fn test_bst_string_values() {
        let mut bst = BST::new();

        bst.insert("banana");
        bst.insert("apple");
        bst.insert("cherry");

        assert_eq!(bst.size(), 3);
        assert_eq!(bst.min(), Some(&"apple"));
        assert_eq!(bst.max(), Some(&"cherry"));

        let inorder = bst.inorder();
        let values: Vec<&str> = inorder.iter().map(|node| node.value).collect();
        assert_eq!(values, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_bst_float_values() {
        // Use a custom type that wraps f64 and implements Ord
        #[derive(Clone, Debug)]
        struct FloatWrapper(f64);

        impl PartialEq for FloatWrapper {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Eq for FloatWrapper {}

        impl PartialOrd for FloatWrapper {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl Ord for FloatWrapper {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        let mut bst = BST::new();

        bst.insert(FloatWrapper(3.14));
        bst.insert(FloatWrapper(2.71));
        bst.insert(FloatWrapper(1.41));

        assert_eq!(bst.size(), 3);
        assert_eq!(bst.min(), Some(&FloatWrapper(1.41)));
        assert_eq!(bst.max(), Some(&FloatWrapper(3.14)));

        let inorder = bst.inorder();
        let values: Vec<f64> = inorder.iter().map(|node| node.value.0).collect();
        assert_eq!(values, vec![1.41, 2.71, 3.14]);
    }

    #[test]
    fn test_bst_default() {
        let bst: BST<i32> = BST::default();
        assert!(bst.is_empty());
        assert_eq!(bst.size(), 0);
    }
}
