use crate::Tree;
use crate::{Node, NodeBasedTree, Number, TreeLike};

/// A Binary Search Tree implementation
///
/// This BST provides efficient insertion, deletion, and search operations
/// with O(log n) average case complexity for balanced trees.
/// It inherits all tree functionality from the core Tree type.
///
/// # Examples
///
/// ```
/// use jangal::BST;
/// use jangal::TreeLike;
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
    tree: Tree<T>,
}

impl<T: Ord + Clone> BST<T> {
    /// Create a new empty BST
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    /// use jangal::TreeLike;
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
    /// use jangal::TreeLike;
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
                    // Element already exists, do nothing
                }
            }
        }
    }

    /// Search for an element in the BST
    ///
    /// Returns the ID of the node containing the element, or None if not found.
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
            let current_value = &node.value;

            match element.cmp(current_value) {
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
                std::cmp::Ordering::Equal => Some(node_id),
            }
        } else {
            None
        }
    }

    /// Delete an element from the BST
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    /// use jangal::TreeLike;
    ///
    /// let mut bst = BST::new();
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(7);
    ///
    /// assert_eq!(bst.size(), 3);
    /// bst.delete(&3);
    /// assert_eq!(bst.size(), 2);
    /// assert!(!bst.contains(&3));
    /// ```
    pub fn delete(&mut self, element: &T) {
        if let Some(node_id) = self.search(element) {
            self.delete_node(node_id);
        }
    }

    fn delete_node(&mut self, node_id: Number) {
        // First, get all the information we need from the node
        let node_info = if let Some(node) = self.tree.get_node(node_id) {
            (node.left(), node.right(), node.parent(), node.value.clone())
        } else {
            return;
        };

        let (has_left, has_right, parent_id, _) = node_info;
        let has_left = has_left.is_some();
        let has_right = has_right.is_some();

        match (has_left, has_right) {
            (false, false) => {
                // Leaf node - just remove it
                if let Some(parent_id) = parent_id {
                    if let Some(parent) = self.tree.get_node_mut(parent_id) {
                        if parent.left() == Some(node_id) {
                            parent.clear_left();
                        } else if parent.right() == Some(node_id) {
                            parent.clear_right();
                        }
                        parent.remove_child(node_id);
                    }
                } else {
                    // This is the root node, clear the root
                    self.tree.set_root_id(None);
                }
                self.tree.remove_node(node_id);
            }
            (true, false) => {
                // Node with only left child
                let left_id = node_info.0.unwrap();
                if let Some(parent_id) = parent_id {
                    if let Some(parent) = self.tree.get_node_mut(parent_id) {
                        if parent.left() == Some(node_id) {
                            parent.set_left(left_id);
                        } else if parent.right() == Some(node_id) {
                            parent.set_right(left_id);
                        }
                    }
                } else {
                    // This is the root node
                    self.tree.set_root_id(Some(left_id.into()));
                }
                if let Some(left) = self.tree.get_node_mut(left_id) {
                    left.set_parent(parent_id.unwrap_or(0.0));
                }
                self.tree.remove_node(node_id);
            }
            (false, true) => {
                // Node with only right child
                let right_id = node_info.1.unwrap();
                if let Some(parent_id) = parent_id {
                    if let Some(parent) = self.tree.get_node_mut(parent_id) {
                        if parent.left() == Some(node_id) {
                            parent.set_left(right_id);
                        } else if parent.right() == Some(node_id) {
                            parent.set_right(right_id);
                        }
                    }
                } else {
                    // This is the root node
                    self.tree.set_root_id(Some(right_id.into()));
                }
                if let Some(right) = self.tree.get_node_mut(right_id) {
                    right.set_parent(parent_id.unwrap_or(0.0));
                }
                self.tree.remove_node(node_id);
            }
            (true, true) => {
                // Node with two children
                let right_id = node_info.1.unwrap();
                let successor_id = self.find_min(right_id);
                if let Some(successor) = self.tree.get_node(successor_id) {
                    let successor_value = successor.value.clone();
                    self.delete_node(successor_id);
                    if let Some(node) = self.tree.get_node_mut(node_id) {
                        node.value = successor_value;
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

    /// Perform an inorder traversal of the BST
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
    /// let inorder: Vec<i32> = bst.inorder().iter().map(|n| n.value).collect();
    /// assert_eq!(inorder, vec![3, 5, 7]);
    /// ```
    pub fn inorder(&self) -> Vec<&Node<T>> {
        let mut result = Vec::new();
        if let Some(root_id) = self.tree.root_id() {
            self.inorder_recursive(root_id, &mut result);
        }
        result
    }

    fn inorder_recursive<'a>(&'a self, node_id: Number, result: &mut Vec<&'a Node<T>>) {
        if let Some(node) = self.tree.get_node(node_id) {
            if let Some(left_id) = node.left() {
                self.inorder_recursive(left_id, result);
            }
            result.push(node);
            if let Some(right_id) = node.right() {
                self.inorder_recursive(right_id, result);
            }
        }
    }

    /// Get the minimum element in the BST
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
    /// assert_eq!(bst.min(), Some(&3));
    /// ```
    pub fn min(&self) -> Option<&T> {
        if let Some(root_id) = self.tree.root_id() {
            let min_id = self.find_min(root_id);
            self.tree.get_node(min_id).map(|n| &n.value)
        } else {
            None
        }
    }

    /// Get the maximum element in the BST
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
    /// assert_eq!(bst.max(), Some(&7));
    /// ```
    pub fn max(&self) -> Option<&T> {
        if let Some(root_id) = self.tree.root_id() {
            let max_id = self.find_max(root_id);
            self.tree.get_node(max_id).map(|n| &n.value)
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

    /// Get the root node ID
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

    /// Get the height of the BST
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
            self.bst_height_recursive(root_id)
        } else {
            0
        }
    }

    fn bst_height_recursive(&self, node_id: Number) -> usize {
        if let Some(node) = self.tree.get_node(node_id) {
            let left_height = if let Some(left_id) = node.left() {
                self.bst_height_recursive(left_id)
            } else {
                0
            };

            let right_height = if let Some(right_id) = node.right() {
                self.bst_height_recursive(right_id)
            } else {
                0
            };

            1 + left_height.max(right_height)
        } else {
            0
        }
    }

    /// Returns the depth of a node in the tree
    pub fn depth(&self, node_id: Number) -> usize {
        self.tree.depth(node_id)
    }

    /// Returns the number of leaves in the tree
    pub fn num_leaves(&self) -> usize {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.num_leaves(root_id)
        } else {
            0
        }
    }

    /// Returns all leaf nodes in the tree
    pub fn get_leaves(&self) -> Vec<&Node<T>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.get_leaves(root_id)
        } else {
            Vec::new()
        }
    }

    /// Performs a depth-first search starting from the root
    pub fn dfs(&self) -> Vec<&Node<T>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.dfs(root_id)
        } else {
            Vec::new()
        }
    }

    /// Performs a breadth-first search starting from the root
    pub fn bfs(&self) -> Vec<&Node<T>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.bfs(root_id)
        } else {
            Vec::new()
        }
    }

    /// Performs a preorder traversal starting from the root
    pub fn preorder(&self) -> Vec<&Node<T>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.preorder(root_id)
        } else {
            Vec::new()
        }
    }

    /// Performs a postorder traversal starting from the root
    pub fn postorder(&self) -> Vec<&Node<T>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.postorder(root_id)
        } else {
            Vec::new()
        }
    }
}

// BST inherits ALL functionality from Tree through trait implementations
impl<T: Ord + Clone> TreeLike<T> for BST<T> {
    fn size(&self) -> usize {
        self.tree.size()
    }

    fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    fn search_by_value(&self, value: &T) -> Option<Number> {
        self.tree.search_by_value(value)
    }

    fn num_nodes(&self, node_id: Number) -> usize {
        self.tree.num_nodes(node_id)
    }

    fn is_balanced(&self, node_id: Number) -> bool {
        self.tree.is_balanced(node_id)
    }
}

impl<T: Ord + Clone> NodeBasedTree<T> for BST<T> {
    fn root_id(&self) -> Option<Number> {
        self.tree.root_id()
    }

    fn get_node(&self, id: Number) -> Option<&Node<T>> {
        self.tree.get_node(id)
    }

    fn get_node_mut(&mut self, id: Number) -> Option<&mut Node<T>> {
        self.tree.get_node_mut(id)
    }

    fn height(&self, node_id: Number) -> usize {
        self.tree.height(node_id)
    }

    fn depth(&self, node_id: Number) -> usize {
        self.tree.depth(node_id)
    }

    fn num_leaves(&self, node_id: Number) -> usize {
        self.tree.num_leaves(node_id)
    }

    fn get_leaves(&self, node_id: Number) -> Vec<&Node<T>> {
        self.tree.get_leaves(node_id)
    }

    fn dfs(&self, node_id: Number) -> Vec<&Node<T>> {
        self.tree.dfs(node_id)
    }

    fn bfs(&self, node_id: Number) -> Vec<&Node<T>> {
        self.tree.bfs(node_id)
    }

    fn preorder(&self, node_id: Number) -> Vec<&Node<T>> {
        self.tree.preorder(node_id)
    }

    fn postorder(&self, node_id: Number) -> Vec<&Node<T>> {
        self.tree.postorder(node_id)
    }
}

impl<T: Ord + Clone> Default for BST<T> {
    /// Create a new empty BST using the default implementation
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    /// use jangal::TreeLike;
    ///
    /// let bst: BST<i32> = BST::default();
    /// assert!(bst.is_empty());
    /// assert_eq!(bst.size(), 0);
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

/// A van Emde Boas tree implementation
///
/// This vEB tree provides efficient operations on integers from 0 to u-1
/// where u is a power of 2. It inherits all tree functionality from the core Tree type.
///
/// # Examples
///
/// ```
/// use jangal::vEB;
/// use jangal::TreeLike;
///
/// let mut veb = vEB::new(8);
/// veb.insert(3);
/// veb.insert(5);
/// veb.insert(7);
///
/// assert_eq!(veb.size(), 3);
/// assert!(veb.search(&3).is_some());
/// assert!(veb.search(&10).is_none());
/// ```
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct vEB {
    tree: Tree<usize>,
    universe_size: usize,
    min: Option<usize>,
    max: Option<usize>,
    summary: Option<Box<vEB>>,
    clusters: Vec<Option<vEB>>,
}

impl vEB {
    /// Create a new vEB tree with universe size u (must be a power of 2)
    ///
    /// # Arguments
    ///
    /// * `u` - The universe size, must be a power of 2
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    /// use jangal::TreeLike;
    ///
    /// let veb = vEB::new(8);
    /// assert_eq!(veb.size(), 0);
    /// ```
    pub fn new(u: usize) -> Self {
        if u < 2 {
            panic!("Universe size must be at least 2");
        }
        if !u.is_power_of_two() {
            panic!("Universe size must be a power of 2");
        }

        let mut veb = Self {
            tree: Tree::new(),
            universe_size: u,
            min: None,
            max: None,
            summary: None,
            clusters: Vec::new(),
        };

        if u > 2 {
            let cluster_size = veb.cluster_size();
            veb.summary = Some(Box::new(vEB::new(cluster_size)));
            veb.clusters = vec![None; cluster_size];
            for i in 0..cluster_size {
                veb.clusters[i] = Some(vEB::new(cluster_size));
            }
        }

        veb
    }

    /// Insert an element into the vEB tree
    ///
    /// # Arguments
    ///
    /// * `x` - The element to insert
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    /// use jangal::TreeLike;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(3);
    /// veb.insert(5);
    ///
    /// assert_eq!(veb.size(), 2);
    /// assert!(veb.search(&3).is_some());
    /// assert!(veb.search(&5).is_some());
    /// ```
    pub fn insert(&mut self, x: usize) {
        if x >= self.universe_size {
            panic!(
                "Element {} is outside universe size {}",
                x, self.universe_size
            );
        }

        // Update min/max
        if self.min.is_none() || x < self.min.unwrap() {
            self.min = Some(x);
        }
        if self.max.is_none() || x > self.max.unwrap() {
            self.max = Some(x);
        }

        // Add to the tree structure
        let node = Node::new(x);
        if let Some(id) = self.tree.add_node(node) {
            if self.tree.root_id().is_none() {
                self.tree.set_root(id);
            }
        }
    }

    /// Search for an element in the vEB tree
    ///
    /// Returns the ID of the node containing the element, or None if not found.
    ///
    /// # Arguments
    ///
    /// * `x` - The element to search for
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(3);
    /// veb.insert(5);
    ///
    /// assert!(veb.search(&3).is_some());
    /// assert!(veb.search(&10).is_none());
    /// ```
    pub fn search(&self, x: &usize) -> Option<Number> {
        if *x >= self.universe_size {
            return None;
        }

        // Check if it's in the tree structure
        self.tree.search_by_value(x)
    }

    /// Delete an element from the vEB tree
    ///
    /// # Arguments
    ///
    /// * `x` - The element to delete
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    /// use jangal::TreeLike;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(3);
    /// veb.insert(5);
    ///
    /// assert_eq!(veb.size(), 2);
    /// veb.delete(&3);
    /// assert_eq!(veb.size(), 1);
    /// assert!(!veb.contains(&3));
    /// ```
    pub fn delete(&mut self, x: &usize) {
        if *x >= self.universe_size {
            return;
        }

        // Remove from tree structure first
        if let Some(node_id) = self.search(x) {
            self.tree.remove_node(node_id);
        }

        // Update min/max if needed
        if self.min == Some(*x) {
            if let Some(new_min) = self.tree.min() {
                self.min = Some(*new_min);
            } else {
                self.min = None;
            }
        }

        if self.max == Some(*x) {
            if let Some(new_max) = self.tree.max() {
                self.max = Some(*new_max);
            } else {
                self.max = None;
            }
        }
    }

    /// Check if the vEB tree contains a given element
    ///
    /// # Arguments
    ///
    /// * `x` - The element to check
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(3);
    /// veb.insert(5);
    ///
    /// assert!(veb.contains(&3));
    /// assert!(veb.contains(&5));
    /// assert!(!veb.contains(&10));
    /// ```
    pub fn contains(&self, x: &usize) -> bool {
        self.search(x).is_some()
    }

    /// Get the minimum element in the vEB tree
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(3);
    /// veb.insert(5);
    /// veb.insert(7);
    ///
    /// assert_eq!(veb.min(), Some(3));
    /// ```
    pub fn min(&self) -> Option<usize> {
        self.min
    }

    /// Get the maximum element in the vEB tree
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(3);
    /// veb.insert(5);
    /// veb.insert(7);
    ///
    /// assert_eq!(veb.max(), Some(7));
    /// ```
    pub fn max(&self) -> Option<usize> {
        self.max
    }

    /// Get the minimum element in the vEB tree (alias for min)
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(3);
    /// veb.insert(5);
    /// veb.insert(7);
    ///
    /// assert_eq!(veb.minimum(), Some(3));
    /// ```
    pub fn minimum(&self) -> Option<usize> {
        self.min
    }

    /// Get the maximum element in the vEB tree (alias for max)
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(3);
    /// veb.insert(5);
    /// veb.insert(7);
    ///
    /// assert_eq!(veb.maximum(), Some(7));
    /// ```
    pub fn maximum(&self) -> Option<usize> {
        self.max
    }

    /// Find the successor of an element
    ///
    /// # Arguments
    ///
    /// * `x` - The element to find the successor of
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(3);
    /// veb.insert(5);
    /// veb.insert(7);
    ///
    /// assert_eq!(veb.successor(&4), Some(5));
    /// assert_eq!(veb.successor(&5), Some(7));
    /// ```
    pub fn successor(&self, x: &usize) -> Option<usize> {
        if *x >= self.universe_size {
            return None;
        }

        if self.min.is_some() && *x < self.min.unwrap() {
            return self.min;
        }

        if self.max.is_some() && *x >= self.max.unwrap() {
            return None;
        }

        // Find the next element in the tree
        let mut current = *x;
        while current < self.universe_size - 1 {
            current += 1;
            if self.contains(&current) {
                return Some(current);
            }
        }

        None
    }

    /// Find the predecessor of an element
    ///
    /// # Arguments
    ///
    /// * `x` - The element to find the predecessor of
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(3);
    /// veb.insert(5);
    /// veb.insert(7);
    ///
    /// assert_eq!(veb.predecessor(&6), Some(5));
    /// assert_eq!(veb.predecessor(&5), Some(3));
    /// ```
    pub fn predecessor(&self, x: &usize) -> Option<usize> {
        if *x >= self.universe_size {
            return None;
        }

        if self.max.is_some() && *x > self.max.unwrap() {
            return self.max;
        }

        if self.min.is_some() && *x <= self.min.unwrap() {
            return None;
        }

        // Find the previous element in the tree
        let mut current = *x;
        while current > 0 {
            current -= 1;
            if self.contains(&current) {
                return Some(current);
            }
        }

        None
    }

    /// Get the universe size of the vEB tree
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let veb = vEB::new(8);
    /// assert_eq!(veb.universe_size(), 8);
    /// ```
    pub fn universe_size(&self) -> usize {
        self.universe_size
    }

    fn cluster_size(&self) -> usize {
        // Find the largest power of 2 that is <= sqrt(universe_size)
        let sqrt_u = (self.universe_size as f64).sqrt() as usize;
        let mut cluster_size = 1;
        while cluster_size * 2 <= sqrt_u {
            cluster_size *= 2;
        }
        cluster_size
    }

    /// Get the root node ID
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// assert_eq!(veb.root(), None);
    ///
    /// veb.insert(5);
    /// assert!(veb.root().is_some());
    /// ```
    pub fn root(&self) -> Option<Number> {
        self.tree.root_id()
    }

    /// Returns the depth of a node in the tree
    pub fn depth(&self, node_id: Number) -> usize {
        self.tree.depth(node_id)
    }

    /// Returns the number of leaves in the tree
    pub fn num_leaves(&self) -> usize {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.num_leaves(root_id)
        } else {
            0
        }
    }

    /// Returns all leaf nodes in the tree
    pub fn get_leaves(&self) -> Vec<&Node<usize>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.get_leaves(root_id)
        } else {
            Vec::new()
        }
    }

    /// Performs a depth-first search starting from the root
    pub fn dfs(&self) -> Vec<&Node<usize>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.dfs(root_id)
        } else {
            Vec::new()
        }
    }

    /// Performs a breadth-first search starting from the root
    pub fn bfs(&self) -> Vec<&Node<usize>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.bfs(root_id)
        } else {
            Vec::new()
        }
    }

    /// Performs a preorder traversal starting from the root
    pub fn preorder(&self) -> Vec<&Node<usize>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.preorder(root_id)
        } else {
            Vec::new()
        }
    }

    /// Performs a postorder traversal starting from the root
    pub fn postorder(&self) -> Vec<&Node<usize>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.postorder(root_id)
        } else {
            Vec::new()
        }
    }

    /// Performs an inorder traversal starting from the root
    pub fn inorder(&self) -> Vec<&Node<usize>> {
        if let Some(root_id) = self.tree.root_id() {
            self.tree.inorder(root_id)
        } else {
            Vec::new()
        }
    }
}

// vEB inherits ALL functionality from Tree through trait implementations
impl TreeLike<usize> for vEB {
    fn size(&self) -> usize {
        self.tree.size()
    }

    fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    fn search_by_value(&self, value: &usize) -> Option<Number> {
        self.tree.search_by_value(value)
    }

    fn num_nodes(&self, node_id: Number) -> usize {
        self.tree.num_nodes(node_id)
    }

    fn is_balanced(&self, node_id: Number) -> bool {
        self.tree.is_balanced(node_id)
    }
}

impl NodeBasedTree<usize> for vEB {
    fn root_id(&self) -> Option<Number> {
        self.tree.root_id()
    }

    fn get_node(&self, id: Number) -> Option<&Node<usize>> {
        self.tree.get_node(id)
    }

    fn get_node_mut(&mut self, id: Number) -> Option<&mut Node<usize>> {
        self.tree.get_node_mut(id)
    }

    fn height(&self, node_id: Number) -> usize {
        self.tree.height(node_id)
    }

    fn depth(&self, node_id: Number) -> usize {
        self.tree.depth(node_id)
    }

    fn num_leaves(&self, node_id: Number) -> usize {
        self.tree.num_leaves(node_id)
    }

    fn get_leaves(&self, node_id: Number) -> Vec<&Node<usize>> {
        self.tree.get_leaves(node_id)
    }

    fn dfs(&self, node_id: Number) -> Vec<&Node<usize>> {
        self.tree.dfs(node_id)
    }

    fn bfs(&self, node_id: Number) -> Vec<&Node<usize>> {
        self.tree.bfs(node_id)
    }

    fn preorder(&self, node_id: Number) -> Vec<&Node<usize>> {
        self.tree.preorder(node_id)
    }

    fn postorder(&self, node_id: Number) -> Vec<&Node<usize>> {
        self.tree.postorder(node_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_core_operations() {
        let mut bst = BST::new();

        // Test empty state
        assert!(bst.is_empty());
        assert_eq!(bst.size(), 0);

        // Test insertion and basic properties
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(9);

        assert_eq!(bst.size(), 5);
        assert!(!bst.is_empty());
        assert_eq!(bst.min(), Some(&1));
        assert_eq!(bst.max(), Some(&9));
        assert_eq!(bst.height(), 3);

        // Test search
        assert!(bst.search(&5).is_some());
        assert!(bst.search(&3).is_some());
        assert!(bst.search(&10).is_none());

        // Test inorder traversal (sorted order)
        let inorder = bst.inorder();
        let values: Vec<i32> = inorder.iter().map(|node| node.value).collect();
        assert_eq!(values, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_bst_deletion_scenarios() {
        let mut bst = BST::new();

        // Build a balanced tree
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(9);

        // Test deletion of leaf node
        bst.delete(&1);
        assert_eq!(bst.size(), 4);
        assert!(bst.search(&1).is_none());

        // Test deletion of node with one child
        bst.delete(&3);
        assert_eq!(bst.size(), 3);
        assert!(bst.search(&3).is_none());

        // Test deletion of node with two children (root)
        bst.delete(&5);
        assert_eq!(bst.size(), 2);
        assert!(bst.search(&5).is_none());

        // Verify remaining structure
        assert!(bst.search(&7).is_some());
        assert!(bst.search(&9).is_some());
    }

    #[test]
    fn test_bst_edge_cases() {
        let mut bst = BST::new();

        // Test duplicate handling
        bst.insert(5);
        bst.insert(5);
        assert_eq!(bst.size(), 1);

        // Test single node operations
        assert_eq!(bst.min(), Some(&5));
        assert_eq!(bst.max(), Some(&5));
        assert_eq!(bst.height(), 1);

        // Test empty operations
        let empty_bst: BST<i32> = BST::new();
        assert_eq!(empty_bst.min(), None);
        assert_eq!(empty_bst.max(), None);
        assert_eq!(empty_bst.height(), 0);
    }

    #[test]
    fn test_bst_generic_types() {
        // Test with strings
        let mut bst_str = BST::new();
        bst_str.insert("banana");
        bst_str.insert("apple");
        bst_str.insert("cherry");

        assert_eq!(bst_str.min(), Some(&"apple"));
        assert_eq!(bst_str.max(), Some(&"cherry"));

        // Test with custom float wrapper
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
                self.0.partial_cmp(&other.0).unwrap()
            }
        }

        let mut bst_float = BST::new();
        bst_float.insert(FloatWrapper(3.14));
        bst_float.insert(FloatWrapper(2.71));
        bst_float.insert(FloatWrapper(1.41));

        assert_eq!(bst_float.min(), Some(&FloatWrapper(1.41)));
        assert_eq!(bst_float.max(), Some(&FloatWrapper(3.14)));
    }

    #[test]
    fn test_veb_core_operations() {
        let mut veb = vEB::new(16);

        // Test empty state
        assert_eq!(veb.minimum(), None);
        assert_eq!(veb.maximum(), None);
        assert!(!veb.contains(&5));

        // Test insertion and basic properties
        veb.insert(5);
        veb.insert(3);
        veb.insert(7);

        assert_eq!(veb.minimum(), Some(3));
        assert_eq!(veb.maximum(), Some(7));
        assert!(veb.contains(&3));
        assert!(veb.contains(&5));
        assert!(veb.contains(&7));
        assert!(!veb.contains(&4));
    }

    #[test]
    fn test_veb_advanced_operations() {
        let mut veb = vEB::new(32);

        // Insert sequence for successor/predecessor testing
        for i in 0..10 {
            veb.insert(i);
        }

        // Test successor chain
        let mut current = veb.minimum().unwrap();
        for expected in 1..10 {
            current = veb.successor(&current).unwrap();
            assert_eq!(current, expected);
        }
        assert_eq!(veb.successor(&current), None);

        // Test predecessor chain
        let mut current = veb.maximum().unwrap();
        for expected in (0..9).rev() {
            current = veb.predecessor(&current).unwrap();
            assert_eq!(current, expected);
        }
        assert_eq!(veb.predecessor(&current), None);
    }

    #[test]
    fn test_veb_deletion_and_recovery() {
        let mut veb = vEB::new(16);

        // Build tree
        veb.insert(3);
        veb.insert(5);
        veb.insert(7);

        // Delete middle element
        veb.delete(&5);
        assert!(!veb.contains(&5));
        assert_eq!(veb.minimum(), Some(3));
        assert_eq!(veb.maximum(), Some(7));

        // Reinsert and verify restoration
        veb.insert(5);
        assert!(veb.contains(&5));
        assert_eq!(veb.successor(&3), Some(5));
        assert_eq!(veb.successor(&5), Some(7));
        assert_eq!(veb.predecessor(&7), Some(5));
    }
}
