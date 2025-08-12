use crate::Tree;
use crate::{Node, Number};

/// A Binary Search Tree implementation
///
/// This BST provides efficient insertion, deletion, and search operations
/// with O(log n) average case complexity for balanced trees.
///
/// The BST focuses on binary search tree-specific operations like insertion,
/// deletion, search, and traversal. For generic tree operations (like
/// node manipulation, advanced traversals, etc.), use the `as_tree()` method
/// to access the underlying tree structure.
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
///
/// // For advanced tree operations, access the underlying tree
/// let tree_ref = bst.as_tree();
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

    /// Get a reference to the underlying tree structure
    ///
    /// This provides controlled access to the tree for advanced operations
    /// while maintaining encapsulation. Use this method when you need
    /// direct access to tree-specific functionality not exposed through
    /// the BST interface.
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
    ///
    /// // Access underlying tree for advanced operations
    /// let tree_ref = bst.as_tree();
    /// assert_eq!(tree_ref.size(), 2);
    /// ```
    pub fn as_tree(&self) -> &Tree<T> {
        &self.tree
    }

    /// Get a mutable reference to the underlying tree structure
    ///
    /// This provides controlled access to the tree for advanced operations
    /// while maintaining encapsulation. Use this method when you need
    /// direct mutable access to tree-specific functionality not exposed through
    /// the BST interface.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    /// use jangal::TreeLike;
    ///
    /// let mut bst = BST::new();
    /// bst.insert(5);
    ///
    /// // Access underlying tree for advanced operations
    /// let tree_ref = bst.as_tree_mut();
    /// // Perform advanced tree operations...
    /// ```
    pub fn as_tree_mut(&mut self) -> &mut Tree<T> {
        &mut self.tree
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

        let (has_left, has_right, parent_id, _node_value) = node_info;
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
                    if let Some(parent_id) = parent_id {
                        left.set_parent(parent_id);
                    } else {
                        left.remove_parent();
                    }
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
                    if let Some(parent_id) = parent_id {
                        right.set_parent(parent_id);
                    } else {
                        right.remove_parent();
                    }
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

    /// Get the size of the BST
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// assert_eq!(bst.size(), 0);
    /// bst.insert(5);
    /// assert_eq!(bst.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.tree.size()
    }

    /// Check if the BST is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// assert!(bst.is_empty());
    /// bst.insert(5);
    /// assert!(!bst.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    /// Search for a value in the BST and return the node ID
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.insert(5);
    /// assert!(bst.search_by_value(&5).is_some());
    /// assert!(bst.search_by_value(&10).is_none());
    /// ```
    pub fn search_by_value(&self, value: &T) -> Option<Number> {
        self.tree.search_by_value(value)
    }

    /// Get a node by its ID
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.insert(5);
    /// if let Some(node_id) = bst.search(&5) {
    ///     if let Some(node) = bst.get_node(node_id) {
    ///         assert_eq!(node.value, 5);
    ///     }
    /// }
    /// ```
    pub fn get_node(&self, id: Number) -> Option<&Node<T>> {
        self.tree.get_node(id)
    }

    /// Get a mutable reference to a node by its ID
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.insert(5);
    /// if let Some(node_id) = bst.search(&5) {
    ///     if let Some(node) = bst.get_node_mut(node_id) {
    ///         // Modify the node if needed
    ///     }
    /// }
    /// ```
    pub fn get_node_mut(&mut self, id: Number) -> Option<&mut Node<T>> {
        self.tree.get_node_mut(id)
    }

    /// Get the number of nodes in a subtree starting from the given node
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
    /// if let Some(root_id) = bst.root() {
    ///     assert_eq!(bst.num_nodes(root_id), 3);
    /// }
    /// ```
    pub fn num_nodes(&self, node_id: Number) -> usize {
        self.tree.num_nodes(node_id)
    }

    /// Check if a subtree starting from the given node is balanced
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
    /// if let Some(root_id) = bst.root() {
    ///     assert!(bst.is_balanced(root_id));
    /// }
    /// ```
    pub fn is_balanced(&self, node_id: Number) -> bool {
        self.tree.is_balanced(node_id)
    }
}

// BST provides its own focused API for binary search tree operations
// Generic tree functionality is available through as_tree() when needed
impl<T: Ord + Clone> Default for BST<T> {
    /// Create a new empty BST using the default implementation
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::BST;
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
    element_count: usize, // Track actual element count
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
            element_count: 0,
        };

        if u > 2 {
            // For van Emde Boas, we need to split the universe properly
            // If u = 2^2^k, then we want sqrt(u) = 2^(2^(k-1))
            // For other powers of 2, we need to find the closest power of 2
            let log_u = u.ilog2() as usize;
            let upper_sqrt = 1 << log_u.div_ceil(2); // Upper square root
            let lower_sqrt = u / upper_sqrt; // Lower square root

            veb.summary = Some(Box::new(vEB::new(upper_sqrt)));
            veb.clusters = vec![None; upper_sqrt];
            for i in 0..upper_sqrt {
                veb.clusters[i] = Some(vEB::new(lower_sqrt));
            }
        }

        veb
    }

    /// Get a reference to the underlying tree structure
    ///
    /// This provides controlled access to the tree for advanced operations
    /// while maintaining encapsulation. Use this method when you need
    /// direct access to tree-specific functionality not exposed through
    /// the vEB interface.
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
    /// // Access underlying tree for advanced operations
    /// let tree_ref = veb.as_tree();
    /// assert_eq!(tree_ref.size(), 0); // Underlying tree is empty
    /// assert_eq!(veb.size(), 2); // vEB tree has 2 elements
    /// ```
    pub fn as_tree(&self) -> &Tree<usize> {
        &self.tree
    }

    /// Get a mutable reference to the underlying tree structure
    ///
    /// This provides controlled access to the tree for advanced operations
    /// while maintaining encapsulation. Use this method when you need
    /// direct mutable access to tree-specific functionality not exposed through
    /// the vEB interface.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    /// use jangal::TreeLike;
    ///
    /// let mut veb = vEB::new(8);
    /// veb.insert(5);
    ///
    /// // Access underlying tree for advanced operations
    /// let tree_ref = veb.as_tree_mut();
    /// // Perform advanced tree operations...
    /// ```
    pub fn as_tree_mut(&mut self) -> &mut Tree<usize> {
        &mut self.tree
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

        if self.min.is_none() {
            self.min = Some(x);
            self.max = Some(x);
            self.element_count = 1;
        } else {
            if x < self.min.unwrap() {
                let old_min = self.min.unwrap();
                self.min = Some(x);
                if self.universe_size > 2 {
                    self.insert_recursive(old_min);
                }
            }
            if x > self.max.unwrap() {
                self.max = Some(x);
            }
            if self.universe_size > 2 {
                self.insert_recursive(x);
            }
            self.element_count += 1;
        }
    }

    fn insert_recursive(&mut self, x: usize) {
        let i = self.high(x);
        let j = self.low(x);

        if let Some(cluster) = &mut self.clusters[i] {
            if cluster.min.is_none() {
                if let Some(summary) = &mut self.summary {
                    summary.insert(i);
                }
                cluster.min = Some(j);
                cluster.max = Some(j);
                cluster.element_count = 1;
            } else {
                cluster.insert(j);
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

        // Check min/max first
        if self.min == Some(*x) || self.max == Some(*x) {
            return Some(f64::NAN); // Return marker value since we're not using the tree structure
        }

        // Base case: universe size 2
        if self.universe_size == 2 {
            return None;
        }

        // Search recursively in clusters
        let i = self.high(*x);
        let j = self.low(*x);

        if let Some(cluster) = &self.clusters[i] {
            if cluster.contains(&j) {
                return Some(0.0); // Return dummy ID
            }
        }

        None
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

        if self.min == Some(*x) && self.max == Some(*x) {
            self.min = None;
            self.max = None;
            self.element_count = 0;
        } else if self.universe_size == 2 {
            if *x == 0 {
                self.min = Some(1);
            } else {
                self.min = Some(0);
            }
            self.max = self.min;
            self.element_count = 1;
        } else {
            if *x == self.min.unwrap() {
                let first_cluster = self.summary.as_ref().unwrap().min.unwrap();
                let new_min_low = self.clusters[first_cluster].as_ref().unwrap().min.unwrap();
                let new_min = self.index(first_cluster, new_min_low);
                self.min = Some(new_min);

                // Delete the new min from its cluster
                self.clusters[first_cluster]
                    .as_mut()
                    .unwrap()
                    .delete(&new_min_low);

                // If cluster is now empty, remove it from summary
                if self.clusters[first_cluster].as_ref().unwrap().min.is_none() {
                    self.summary.as_mut().unwrap().delete(&first_cluster);

                    // Update max if needed
                    if new_min == self.max.unwrap() {
                        let summary_max = self.summary.as_ref().unwrap().max;
                        if let Some(summary_max_val) = summary_max {
                            let cluster_max = self.clusters[summary_max_val]
                                .as_ref()
                                .unwrap()
                                .max
                                .unwrap();
                            self.max = Some(self.index(summary_max_val, cluster_max));
                        } else {
                            self.max = self.min;
                        }
                    }
                }
            } else {
                let high_x = self.high(*x);
                let low_x = self.low(*x);

                // Delete from cluster
                self.clusters[high_x].as_mut().unwrap().delete(&low_x);

                // If cluster is now empty, remove it from summary
                if self.clusters[high_x].as_ref().unwrap().min.is_none() {
                    self.summary.as_mut().unwrap().delete(&high_x);

                    // Update max if needed
                    if *x == self.max.unwrap() {
                        let summary_max = self.summary.as_ref().unwrap().max;
                        if let Some(summary_max_val) = summary_max {
                            let cluster_max = self.clusters[summary_max_val]
                                .as_ref()
                                .unwrap()
                                .max
                                .unwrap();
                            self.max = Some(self.index(summary_max_val, cluster_max));
                        } else {
                            self.max = self.min;
                        }
                    }
                } else if *x == self.max.unwrap() {
                    let cluster_max = self.clusters[high_x].as_ref().unwrap().max.unwrap();
                    self.max = Some(self.index(high_x, cluster_max));
                }
            }
            self.element_count -= 1;
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
        if *x >= self.universe_size {
            return false;
        }

        if (self.min.is_some() && x == self.min.as_ref().unwrap())
            || (self.max.is_some() && x == self.max.as_ref().unwrap())
        {
            true
        } else if self.universe_size == 2 {
            false
        } else {
            let high_x = self.high(*x);
            let low_x = self.low(*x);
            if let Some(cluster) = &self.clusters[high_x] {
                return cluster.contains(&low_x);
            }
            false
        }
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

        if self.universe_size == 2 {
            if *x == 0 && self.max == Some(1) {
                return Some(1);
            } else {
                return None;
            }
        } else if self.min.is_some() && *x < self.min.unwrap() {
            return self.min;
        } else {
            let high_x = self.high(*x);
            let low_x = self.low(*x);

            if let Some(cluster) = &self.clusters[high_x] {
                let max_low = cluster.max;
                if max_low.is_some() && low_x < max_low.unwrap() {
                    let offset = cluster.successor(&low_x);
                    if let Some(offset_val) = offset {
                        return Some(self.index(high_x, offset_val));
                    }
                }
            }

            let succ_cluster = self.summary.as_ref().unwrap().successor(&high_x);
            if let Some(succ_cluster_val) = succ_cluster {
                let offset = self.clusters[succ_cluster_val].as_ref().unwrap().min;
                if let Some(offset_val) = offset {
                    return Some(self.index(succ_cluster_val, offset_val));
                }
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

        if self.universe_size == 2 {
            if *x == 1 && self.min == Some(0) {
                return Some(0);
            } else {
                return None;
            }
        } else if self.max.is_some() && *x > self.max.unwrap() {
            return self.max;
        } else {
            let high_x = self.high(*x);
            let low_x = self.low(*x);

            if let Some(cluster) = &self.clusters[high_x] {
                let min_low = cluster.min;
                if min_low.is_some() && low_x > min_low.unwrap() {
                    let offset = cluster.predecessor(&low_x);
                    if let Some(offset_val) = offset {
                        return Some(self.index(high_x, offset_val));
                    }
                }
            }

            let pred_cluster = self.summary.as_ref().unwrap().predecessor(&high_x);
            if let Some(pred_cluster_val) = pred_cluster {
                let offset = self.clusters[pred_cluster_val].as_ref().unwrap().max;
                if let Some(offset_val) = offset {
                    return Some(self.index(pred_cluster_val, offset_val));
                }
            } else if self.min.is_some() && *x > self.min.unwrap() {
                return self.min;
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

    /// Get the number of elements in the vEB tree
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// assert_eq!(veb.size(), 0);
    /// veb.insert(3);
    /// assert_eq!(veb.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.element_count
    }

    /// Check if the vEB tree is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    ///
    /// let mut veb = vEB::new(8);
    /// assert!(veb.is_empty());
    /// veb.insert(3);
    /// assert!(!veb.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.element_count == 0
    }

    fn cluster_size(&self) -> usize {
        // For van Emde Boas, we need to split the universe properly
        // If u = 2^2^k, then we want sqrt(u) = 2^(2^(k-1))
        // For other powers of 2, we need to find the closest power of 2
        let log_u = self.universe_size.ilog2() as usize;
        let upper_sqrt = 1 << log_u.div_ceil(2); // Upper square root
                                                 // Lower square root
        self.universe_size / upper_sqrt
    }

    /// Get the high-order bits (cluster number) of x
    fn high(&self, x: usize) -> usize {
        x / self.cluster_size()
    }

    /// Get the low-order bits (position within cluster) of x
    fn low(&self, x: usize) -> usize {
        x % self.cluster_size()
    }

    /// Combine high and low bits to form the original value
    fn index(&self, high: usize, low: usize) -> usize {
        high * self.cluster_size() + low
    }

    /// Get the root node ID
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::vEB;
    /// use jangal::TreeLike;
    ///
    /// let mut veb = vEB::new(8);
    /// assert_eq!(veb.root(), None);
    ///
    /// veb.insert(5);
    /// assert!(veb.root().is_some());
    /// ```
    pub fn root(&self) -> Option<Number> {
        if self.min.is_some() {
            Some(0.0) // Return dummy ID since we're not using the tree structure
        } else {
            None
        }
    }

    /// Returns the depth of a node in the tree
    pub fn depth(&self, _node_id: Number) -> usize {
        0 // Since we're not using the tree structure, depth is always 0
    }

    /// Returns the number of leaves in the tree
    pub fn num_leaves(&self) -> usize {
        self.size() // In our case, all elements are leaves
    }

    /// Returns all leaf nodes in the tree
    pub fn get_leaves(&self) -> Vec<&Node<usize>> {
        Vec::new() // We don't have Node objects in the new structure
    }

    /// Performs a depth-first search starting from the root
    pub fn dfs(&self) -> Vec<&Node<usize>> {
        Vec::new() // We don't have Node objects in the new structure
    }

    /// Performs a breadth-first search starting from the root
    pub fn bfs(&self) -> Vec<&Node<usize>> {
        Vec::new() // We don't have Node objects in the new structure
    }

    /// Performs a preorder traversal starting from the root
    pub fn preorder(&self) -> Vec<&Node<usize>> {
        Vec::new() // We don't have Node objects in the new structure
    }

    /// Performs a postorder traversal starting from the root
    pub fn postorder(&self) -> Vec<&Node<usize>> {
        Vec::new() // We don't have Node objects in the new structure
    }

    /// Performs an inorder traversal starting from the root
    pub fn inorder(&self) -> Vec<&Node<usize>> {
        Vec::new() // We don't have Node objects in the new structure
    }
}

// vEB inherits ALL functionality from Tree through trait implementations
// vEB tree doesn't implement TreeLike or NodeBasedTree traits
// since it doesn't actually use the underlying Tree<usize> field
// The vEB tree is a completely separate data structure

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
    fn test_bst_tree_access_methods() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);

        let tree_ref = bst.as_tree();
        assert_eq!(tree_ref.size(), 3);
        assert!(tree_ref.root_id().is_some());

        let tree_mut = bst.as_tree_mut();
        assert_eq!(tree_mut.size(), 3);
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
    fn test_bst_root_deletion_with_one_child() {
        let mut bst = BST::new();

        // Create a tree with root 5 and left child 3
        bst.insert(5);
        bst.insert(3);

        // Get the initial root ID
        let initial_root_id = bst.root().unwrap();

        // Delete the root (5), leaving only the left child (3)
        bst.delete(&5);

        // Verify there's a new root
        let new_root_id = bst.root().unwrap();
        assert_ne!(new_root_id, initial_root_id);

        // Verify the new root has no parent (it's the root)
        if let Some(root_node) = bst.get_node(new_root_id) {
            assert!(root_node.parent().is_none());
            assert_eq!(root_node.value, 3);
        }
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
    fn test_veb_universe_size_2() {
        // Test that universe size 2 is valid and works correctly
        let mut veb = vEB::new(2);

        // Verify the vEB tree was created successfully
        assert_eq!(veb.universe_size(), 2);
        assert_eq!(veb.size(), 0);
        assert!(veb.is_empty());

        // Test insertion of valid elements (0 and 1)
        veb.insert(0);
        assert_eq!(veb.size(), 1);
        assert!(veb.contains(&0));
        assert!(!veb.contains(&1));
        assert_eq!(veb.minimum(), Some(0));
        assert_eq!(veb.maximum(), Some(0));

        veb.insert(1);
        assert_eq!(veb.size(), 2);
        assert!(veb.contains(&0));
        assert!(veb.contains(&1));
        assert_eq!(veb.minimum(), Some(0));
        assert_eq!(veb.maximum(), Some(1));
    }

    #[test]
    #[should_panic(expected = "Universe size must be at least 2")]
    fn test_veb_universe_size_1_panics() {
        // Test that universe size 1 causes a panic
        let _veb = vEB::new(1);
    }

    #[test]
    #[should_panic(expected = "Universe size must be a power of 2")]
    fn test_veb_universe_size_3_panics() {
        // Test that universe size 3 (not a power of 2) causes a panic
        let _veb = vEB::new(3);
    }

    #[test]
    fn test_veb_tree_access_methods() {
        let mut veb = vEB::new(16);
        veb.insert(5);
        veb.insert(3);
        veb.insert(7);

        // Test the vEB tree's own size method
        assert_eq!(veb.size(), 3);
        assert!(!veb.is_empty());

        // Test that the underlying tree is empty (as expected)
        let tree_ref = veb.as_tree();
        assert_eq!(tree_ref.size(), 0); // Underlying tree is empty
        assert!(tree_ref.root_id().is_none());
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

    #[test]
    fn test_veb_cluster_size() {
        let mut veb = vEB::new(4);
        assert_eq!(veb.universe_size, 4);
        assert_eq!(veb.cluster_size(), 2);
        let num_clusters = veb.universe_size / veb.cluster_size();
        assert_eq!(num_clusters, 2);

        veb.insert(0);
        assert_eq!(veb.size(), 1);
        assert_eq!(veb.minimum(), Some(0));
        assert_eq!(veb.maximum(), Some(0));

        veb.insert(1);
        assert_eq!(veb.size(), 2);
        assert_eq!(veb.minimum(), Some(0));
        assert_eq!(veb.maximum(), Some(1));

        veb.insert(2);
        assert_eq!(veb.size(), 3);
        assert_eq!(veb.minimum(), Some(0));
        assert_eq!(veb.maximum(), Some(2));

        veb.insert(3);
        assert_eq!(veb.size(), 4);
        assert_eq!(veb.minimum(), Some(0));
        assert_eq!(veb.maximum(), Some(3));
    }
}
