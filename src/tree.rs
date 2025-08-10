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

/// An improved van Emde Boas tree implementation
///
/// This implementation provides O(log log U) time complexity for insert, delete,
/// search, and predecessor/successor operations, where U is the universe size.
/// It's optimized for integer keys and provides better memory efficiency.
///
/// # Examples
///
/// ```
/// use jangal::VEBTree;
///
/// let mut veb = VEBTree::new(1000);
/// veb.insert(5);
/// veb.insert(10);
/// veb.insert(15);
///
/// assert!(veb.contains(5));
/// assert_eq!(veb.successor(7), Some(10));
/// assert_eq!(veb.predecessor(12), Some(10));
/// ```
#[derive(Debug, Clone)]
pub struct VEBTree {
    /// The universe size (maximum value + 1)
    universe_size: usize,
    /// Minimum value in the tree
    min: Option<usize>,
    /// Maximum value in the tree
    max: Option<usize>,
    /// Summary structure for tracking non-empty clusters
    summary: Option<Box<VEBTree>>,
    /// Clusters for storing actual values
    clusters: Vec<Option<Box<VEBTree>>>,
    /// Number of elements currently in the tree
    size: usize,
}

impl VEBTree {
    /// Creates a new VEB tree with the specified universe size
    ///
    /// # Arguments
    /// * `universe_size` - The maximum value the tree can store (exclusive)
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::VEBTree;
    /// let veb = VEBTree::new(1000);
    /// assert_eq!(veb.size(), 0);
    /// ```
    pub fn new(universe_size: usize) -> Self {
        if universe_size <= 2 {
            Self {
                universe_size,
                min: None,
                max: None,
                summary: None,
                clusters: Vec::new(),
                size: 0,
            }
        } else {
            let cluster_size = (universe_size as f64).sqrt().ceil() as usize;
            let num_clusters = universe_size.div_ceil(cluster_size);

            Self {
                universe_size,
                min: None,
                max: None,
                summary: Some(Box::new(VEBTree::new(num_clusters))),
                clusters: vec![None; num_clusters],
                size: 0,
            }
        }
    }

    /// Returns the number of elements in the tree
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::VEBTree;
    /// let mut veb = VEBTree::new(100);
    /// assert_eq!(veb.size(), 0);
    /// veb.insert(5);
    /// assert_eq!(veb.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Checks if the tree is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::VEBTree;
    /// let mut veb = VEBTree::new(100);
    /// assert!(veb.is_empty());
    /// veb.insert(5);
    /// assert!(!veb.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the minimum value in the tree
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::VEBTree;
    /// let mut veb = VEBTree::new(100);
    /// assert_eq!(veb.minimum(), None);
    /// veb.insert(5);
    /// veb.insert(3);
    /// assert_eq!(veb.minimum(), Some(3));
    /// ```
    pub fn minimum(&self) -> Option<usize> {
        self.min
    }

    /// Returns the maximum value in the tree
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::VEBTree;
    /// let mut veb = VEBTree::new(100);
    /// assert_eq!(veb.maximum(), None);
    /// veb.insert(5);
    /// veb.insert(3);
    /// assert_eq!(veb.maximum(), Some(5));
    /// ```
    pub fn maximum(&self) -> Option<usize> {
        self.max
    }

    /// Checks if a value exists in the tree
    ///
    /// # Arguments
    /// * `value` - The value to check
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::VEBTree;
    /// let mut veb = VEBTree::new(100);
    /// assert!(!veb.contains(5));
    /// veb.insert(5);
    /// assert!(veb.contains(5));
    /// ```
    pub fn contains(&self, value: usize) -> bool {
        if self.universe_size <= 2 {
            self.min == Some(value) || self.max == Some(value)
        } else if let (Some(min_val), Some(max_val)) = (self.min, self.max) {
            if value == min_val || value == max_val {
                return true;
            }
            if value < min_val || value > max_val {
                return false;
            }
            let (cluster, offset) = self.split_value(value);
            if let Some(cluster_tree) = &self.clusters[cluster] {
                cluster_tree.contains(offset)
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Inserts a value into the tree
    ///
    /// # Arguments
    /// * `value` - The value to insert
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::VEBTree;
    /// let mut veb = VEBTree::new(100);
    /// veb.insert(5);
    /// assert!(veb.contains(5));
    /// assert_eq!(veb.size(), 1);
    /// ```
    pub fn insert(&mut self, value: usize) {
        if self.universe_size <= 2 {
            self.insert_simple(value);
        } else {
            self.insert_recursive(value);
        }
    }

    /// Deletes a value from the tree
    ///
    /// # Arguments
    /// * `value` - The value to delete
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::VEBTree;
    /// let mut veb = VEBTree::new(100);
    /// veb.insert(5);
    /// assert!(veb.contains(5));
    /// veb.delete(5);
    /// assert!(!veb.contains(5));
    /// ```
    pub fn delete(&mut self, value: usize) {
        if self.universe_size <= 2 {
            self.delete_simple(value);
        } else {
            self.delete_recursive(value);
        }
    }

    /// Finds the successor of a given value
    ///
    /// # Arguments
    /// * `value` - The value to find the successor of
    ///
    /// # Returns
    /// The smallest value in the tree greater than `value`, or `None` if no such value exists
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::VEBTree;
    /// let mut veb = VEBTree::new(100);
    /// veb.insert(5);
    /// veb.insert(10);
    /// assert_eq!(veb.successor(7), Some(10));
    /// ```
    pub fn successor(&self, value: usize) -> Option<usize> {
        if self.universe_size <= 2 {
            self.successor_simple(value)
        } else {
            self.successor_recursive(value)
        }
    }

    /// Finds the predecessor of a given value
    ///
    /// # Arguments
    /// * `value` - The value to find the predecessor of
    ///
    /// # Returns
    /// The largest value in the tree less than `value`, or `None` if no such value exists
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::VEBTree;
    /// let mut veb = VEBTree::new(100);
    /// veb.insert(5);
    /// veb.insert(10);
    /// assert_eq!(veb.predecessor(7), Some(5));
    /// ```
    pub fn predecessor(&self, value: usize) -> Option<usize> {
        if self.universe_size <= 2 {
            self.predecessor_simple(value)
        } else {
            self.predecessor_recursive(value)
        }
    }

    // Helper methods for base case (universe_size <= 2)
    fn insert_simple(&mut self, value: usize) {
        if self.min.is_none() {
            self.min = Some(value);
            self.max = Some(value);
            self.size = 1;
        } else if value != self.min.unwrap() && value != self.max.unwrap() {
            if value < self.min.unwrap() {
                self.min = Some(value);
            } else {
                self.max = Some(value);
            }
            self.size += 1;
        }
    }

    fn delete_simple(&mut self, value: usize) {
        if let Some(min_val) = self.min {
            if let Some(max_val) = self.max {
                if min_val == max_val {
                    if value == min_val {
                        self.min = None;
                        self.max = None;
                        self.size = 0;
                    }
                } else if value == min_val {
                    self.min = Some(max_val);
                    self.size -= 1;
                } else if value == max_val {
                    self.max = Some(min_val);
                    self.size -= 1;
                }
            }
        }
    }

    fn successor_simple(&self, value: usize) -> Option<usize> {
        if let Some(min_val) = self.min {
            if value < min_val {
                return Some(min_val);
            }
        }
        if let Some(max_val) = self.max {
            if value < max_val && value != max_val {
                return Some(max_val);
            }
        }
        None
    }

    fn predecessor_simple(&self, value: usize) -> Option<usize> {
        if let Some(max_val) = self.max {
            if value > max_val {
                return Some(max_val);
            }
        }
        if let Some(min_val) = self.min {
            if value > min_val && value != min_val {
                return Some(min_val);
            }
        }
        None
    }

    // Helper methods for recursive case (universe_size > 2)
    fn insert_recursive(&mut self, value: usize) {
        if self.min.is_none() {
            self.min = Some(value);
            self.max = Some(value);
            self.size = 1;
            return;
        }

        if value < self.min.unwrap() {
            // Store the old minimum to insert it into clusters
            let old_min = self.min.unwrap();
            self.min = Some(value);

            // Insert the old minimum into clusters
            let (cluster, offset) = self.split_value(old_min);
            self.ensure_cluster(cluster);
            self.clusters[cluster].as_mut().unwrap().insert(offset);
            self.summary.as_mut().unwrap().insert(cluster);
            self.size += 1;
            return;
        }

        if value > self.max.unwrap() {
            self.max = Some(value);
        }

        if value != self.min.unwrap() {
            let (cluster, offset) = self.split_value(value);
            self.ensure_cluster(cluster);
            self.clusters[cluster].as_mut().unwrap().insert(offset);
            self.summary.as_mut().unwrap().insert(cluster);
            self.size += 1;
        }
    }

    fn delete_recursive(&mut self, value: usize) {
        if self.min.is_none() {
            return;
        }

        if self.min == self.max {
            if value == self.min.unwrap() {
                self.min = None;
                self.max = None;
                self.size = 0;
            }
            return;
        }

        if value == self.min.unwrap() {
            let first_cluster = self.summary.as_ref().unwrap().minimum().unwrap();
            let offset = self.clusters[first_cluster]
                .as_ref()
                .unwrap()
                .minimum()
                .unwrap();
            let new_min = self.join_value(first_cluster, offset);
            self.min = Some(new_min);

            let (cluster, _) = self.split_value(new_min);
            self.clusters[cluster].as_mut().unwrap().delete(offset);
            if self.clusters[cluster].as_ref().unwrap().is_empty() {
                self.summary.as_mut().unwrap().delete(cluster);
            }
            self.size -= 1;
        } else if value == self.max.unwrap() {
            let last_cluster = self.summary.as_ref().unwrap().maximum().unwrap();
            let offset = self.clusters[last_cluster]
                .as_ref()
                .unwrap()
                .maximum()
                .unwrap();
            let new_max = self.join_value(last_cluster, offset);
            self.max = Some(new_max);

            let (cluster, _) = self.split_value(new_max);
            self.clusters[cluster].as_mut().unwrap().delete(offset);
            if self.clusters[cluster].as_ref().unwrap().is_empty() {
                self.summary.as_mut().unwrap().delete(cluster);
            }
            self.size -= 1;
        } else {
            let (cluster, offset) = self.split_value(value);
            if let Some(cluster_tree) = &mut self.clusters[cluster] {
                cluster_tree.delete(offset);
                if cluster_tree.is_empty() {
                    self.summary.as_mut().unwrap().delete(cluster);
                }
                self.size -= 1;
            }
        }
    }

    fn successor_recursive(&self, value: usize) -> Option<usize> {
        self.min?;

        if value < self.min.unwrap() {
            return self.min;
        }

        if value >= self.max.unwrap() {
            return None;
        }

        let (cluster, offset) = self.split_value(value);
        let cluster_tree = &self.clusters[cluster];

        if let Some(tree) = cluster_tree {
            if offset < tree.maximum().unwrap() {
                let succ_offset = tree.successor(offset).unwrap();
                return Some(self.join_value(cluster, succ_offset));
            }
        }

        let next_cluster = self.summary.as_ref().unwrap().successor(cluster)?;
        let succ_offset = self.clusters[next_cluster]
            .as_ref()
            .unwrap()
            .minimum()
            .unwrap();
        Some(self.join_value(next_cluster, succ_offset))
    }

    fn predecessor_recursive(&self, value: usize) -> Option<usize> {
        self.min?;

        if value > self.max.unwrap() {
            return self.max;
        }

        if value <= self.min.unwrap() {
            return None;
        }

        let (cluster, offset) = self.split_value(value);
        let cluster_tree = &self.clusters[cluster];

        // First, try to find a predecessor within the current cluster
        if let Some(tree) = cluster_tree {
            if offset > tree.minimum().unwrap() {
                let pred_offset = tree.predecessor(offset).unwrap();
                return Some(self.join_value(cluster, pred_offset));
            }
        }

        // If no predecessor in current cluster, look in previous clusters
        if let Some(prev_cluster) = self.summary.as_ref().unwrap().predecessor(cluster) {
            if let Some(prev_tree) = &self.clusters[prev_cluster] {
                let pred_offset = prev_tree.maximum().unwrap();
                return Some(self.join_value(prev_cluster, pred_offset));
            }
        }

        // If no previous cluster, the predecessor might be the minimum
        if value > self.min.unwrap() {
            return self.min;
        }

        None
    }

    // Utility methods
    fn split_value(&self, value: usize) -> (usize, usize) {
        let cluster_size = (self.universe_size as f64).sqrt().ceil() as usize;
        let cluster = value / cluster_size;
        let offset = value % cluster_size;
        (cluster, offset)
    }

    fn join_value(&self, cluster: usize, offset: usize) -> usize {
        let cluster_size = (self.universe_size as f64).sqrt().ceil() as usize;
        cluster * cluster_size + offset
    }

    fn ensure_cluster(&mut self, cluster: usize) {
        if self.clusters[cluster].is_none() {
            let cluster_size = (self.universe_size as f64).sqrt().ceil() as usize;
            self.clusters[cluster] = Some(Box::new(VEBTree::new(cluster_size)));
        }
    }
}

impl Default for VEBTree {
    fn default() -> Self {
        Self::new(16)
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

    // VEB Tree Tests
    #[test]
    fn test_veb_tree_creation() {
        let veb: VEBTree = VEBTree::new(16);
        assert_eq!(veb.minimum(), None);
        assert_eq!(veb.maximum(), None);
    }

    #[test]
    fn test_veb_tree_small_capacity() {
        let veb: VEBTree = VEBTree::new(2);
        assert_eq!(veb.minimum(), None);
        assert_eq!(veb.maximum(), None);
    }

    #[test]
    fn test_veb_tree_single_insert() {
        let mut veb: VEBTree = VEBTree::new(16);
        veb.insert(5);

        assert_eq!(veb.minimum(), Some(5));
        assert_eq!(veb.maximum(), Some(5));
        assert!(veb.contains(5));
        assert!(!veb.contains(3));
        assert!(!veb.contains(7));
    }

    #[test]
    fn test_veb_tree_multiple_inserts() {
        let mut veb: VEBTree = VEBTree::new(16);
        veb.insert(5);
        veb.insert(3);
        veb.insert(7);

        assert_eq!(veb.minimum(), Some(3));
        assert_eq!(veb.maximum(), Some(7));
        assert!(veb.contains(3));
        assert!(veb.contains(5));
        assert!(veb.contains(7));
        assert!(!veb.contains(4));
        assert!(!veb.contains(6));
    }

    #[test]
    fn test_veb_tree_duplicate_insert() {
        let mut veb: VEBTree = VEBTree::new(16);
        veb.insert(5);
        veb.insert(5); // Duplicate

        assert_eq!(veb.minimum(), Some(5));
        assert_eq!(veb.maximum(), Some(5));
        assert_eq!(veb.contains(5), true);
    }

    #[test]
    fn test_veb_tree_search() {
        let mut veb: VEBTree = VEBTree::new(16);
        veb.insert(5);
        veb.insert(3);
        veb.insert(7);

        assert!(veb.contains(3));
        assert!(veb.contains(5));
        assert!(veb.contains(7));
        assert!(!veb.contains(4));
        assert!(!veb.contains(6));
    }

    #[test]
    fn test_veb_tree_delete() {
        let mut veb: VEBTree = VEBTree::new(16);
        veb.insert(5);
        veb.insert(3);
        veb.insert(7);

        // Delete middle element
        veb.delete(5);
        assert_eq!(veb.minimum(), Some(3));
        assert_eq!(veb.maximum(), Some(7));
        assert!(!veb.contains(5));
        assert!(veb.contains(3));
        assert!(veb.contains(7));

        // Delete minimum
        veb.delete(3);
        assert_eq!(veb.minimum(), Some(7));
        assert_eq!(veb.maximum(), Some(7));
        assert!(!veb.contains(3));

        // Delete maximum
        veb.delete(7);
        assert_eq!(veb.minimum(), None);
        assert_eq!(veb.maximum(), None);
        assert!(!veb.contains(7));
    }

    #[test]
    fn test_veb_tree_findnext() {
        let mut veb: VEBTree = VEBTree::new(16);
        veb.insert(3);
        veb.insert(5);
        veb.insert(7);
        veb.insert(9);

        assert_eq!(veb.successor(2), Some(3));
        assert_eq!(veb.successor(3), Some(5));
        assert_eq!(veb.successor(5), Some(7));
        assert_eq!(veb.successor(7), Some(9));
        assert_eq!(veb.successor(9), None);
        assert_eq!(veb.successor(10), None);
    }

    #[test]
    fn test_veb_tree_findprev() {
        let mut veb: VEBTree = VEBTree::new(16);
        veb.insert(3);
        veb.insert(5);
        veb.insert(7);
        veb.insert(9);

        assert_eq!(veb.predecessor(4), Some(3));
        assert_eq!(veb.predecessor(5), Some(3));
        assert_eq!(veb.predecessor(7), Some(5));
        assert_eq!(veb.predecessor(9), Some(7));
        assert_eq!(veb.predecessor(10), Some(9));
        assert_eq!(veb.predecessor(2), None);
    }

    #[test]
    fn test_veb_tree_large_capacity() {
        let mut veb: VEBTree = VEBTree::new(1000);

        // Insert values across the range
        veb.insert(25);
        veb.insert(50);
        veb.insert(75);

        assert_eq!(veb.minimum(), Some(25));
        assert_eq!(veb.maximum(), Some(75));
        assert!(veb.contains(25));
        assert!(veb.contains(50));
        assert!(veb.contains(75));

        // Test successor and predecessor
        assert_eq!(veb.successor(25), Some(50));
        assert_eq!(veb.successor(50), Some(75));
        assert_eq!(veb.predecessor(75), Some(50));
        assert_eq!(veb.predecessor(50), Some(25));
    }

    #[test]
    fn test_veb_tree_edge_cases() {
        let mut veb: VEBTree = VEBTree::new(16);

        // Test with empty tree
        assert_eq!(veb.minimum(), None);
        assert_eq!(veb.maximum(), None);
        assert_eq!(veb.successor(5), None);
        assert_eq!(veb.predecessor(5), None);

        // Test with single element
        veb.insert(10);
        assert_eq!(veb.minimum(), Some(10));
        assert_eq!(veb.maximum(), Some(10));
        assert_eq!(veb.successor(10), None);
        assert_eq!(veb.predecessor(10), None);

        // Test with two elements
        veb.insert(5);
        assert_eq!(veb.minimum(), Some(5));
        assert_eq!(veb.maximum(), Some(10));
        assert_eq!(veb.successor(5), Some(10));
        assert_eq!(veb.predecessor(10), Some(5));
    }

    #[test]
    fn test_veb_tree_sequential_operations() {
        let mut veb: VEBTree = VEBTree::new(32);

        // Insert sequence
        for i in 0..10 {
            veb.insert(i);
        }

        // Verify all elements are present
        for i in 0..10 {
            assert!(veb.contains(i));
        }

        // Verify min/max
        assert_eq!(veb.minimum(), Some(0));
        assert_eq!(veb.maximum(), Some(9));

        // Verify successor chain
        let mut current = veb.minimum().unwrap();
        for expected in 1..10 {
            current = veb.successor(current).unwrap();
            assert_eq!(current, expected);
        }
        assert_eq!(veb.successor(current), None);

        // Verify predecessor chain
        let mut current = veb.maximum().unwrap();
        for expected in (0..9).rev() {
            current = veb.predecessor(current).unwrap();
            assert_eq!(current, expected);
        }
        assert_eq!(veb.predecessor(current), None);
    }

    #[test]
    fn test_veb_tree_delete_and_reinsert() {
        let mut veb: VEBTree = VEBTree::new(16);

        // Insert elements
        veb.insert(3);
        veb.insert(5);
        veb.insert(7);

        // Delete and verify
        veb.delete(5);
        assert!(!veb.contains(5));
        assert_eq!(veb.minimum(), Some(3));
        assert_eq!(veb.maximum(), Some(7));

        // Reinsert and verify
        veb.insert(5);
        assert!(veb.contains(5));
        assert_eq!(veb.minimum(), Some(3));
        assert_eq!(veb.maximum(), Some(7));

        // Verify successor/predecessor chains are restored
        assert_eq!(veb.successor(3), Some(5));
        assert_eq!(veb.successor(5), Some(7));
        assert_eq!(veb.predecessor(7), Some(5));
        assert_eq!(veb.predecessor(5), Some(3));
    }

    #[test]
    fn test_veb_tree_empty_operations() {
        let veb: VEBTree = VEBTree::new(16);

        // All operations on empty tree should return None/false
        assert_eq!(veb.minimum(), None);
        assert_eq!(veb.maximum(), None);
        assert!(!veb.contains(5));
        assert!(!veb.contains(5));
        assert_eq!(veb.successor(5), None);
        assert_eq!(veb.predecessor(5), None);
    }
}
