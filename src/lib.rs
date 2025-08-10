//! jangal - trees, forests, graphs
//!
//! A Rust library for working with nodes and graph-like data structures.
//! This library provides a flexible `Node` structure that can be used to
//! build trees, graphs, and other connected data structures.
//!
//! # Examples
//!
//! ## Creating a simple node
//!
//! ```
//! use jangal::{Node, Number};
//!
//! let node = Node::new(42);
//! assert_eq!(node.value, 42);
//! assert!(node.is_root());
//! assert!(node.is_leaf());
//! ```
//!
//! ## Building a tree structure
//!
//! ```
//! use jangal::{Node, Number};
//!
//! let mut parent = Node::new("parent");
//! let mut child1 = Node::new("child1");
//! let mut child2 = Node::new("child2");
//!
//! // Set up parent-child relationships
//! parent.add_child(child1.id);
//! parent.add_child(child2.id);
//! child1.set_parent(parent.id);
//! child2.set_parent(parent.id);
//!
//! assert_eq!(parent.num_children(), 2);
//! assert!(!parent.is_leaf());
//! assert!(child1.is_leaf());
//! assert!(!child1.is_root());
//! ```

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};

pub mod tree;
pub use tree::{VEBTree, BST};

#[derive(Debug, Clone, Copy)]
pub struct FloatId(f64);

impl FloatId {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Hash for FloatId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use the bit representation for hashing
        self.0.to_bits().hash(state);
    }
}

impl PartialEq for FloatId {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            true
        } else {
            self.0 == other.0
        }
    }
}

impl Eq for FloatId {}

impl From<f64> for FloatId {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<FloatId> for f64 {
    fn from(id: FloatId) -> Self {
        id.0
    }
}

pub type Number = f64;

/// Generic Node Struct
///
/// A flexible node structure that can represent various types of graph and tree nodes.
/// Each node has a unique ID and can maintain relationships with other nodes through
/// various connection types: undirected edges, directed edges, parent-child relationships,
/// and binary tree left-right relationships.
///
/// # Examples
///
/// ## Basic node creation and properties
///
/// ```
/// use jangal::Node;
///
/// let node = Node::new("Hello, World!");
/// assert_eq!(node.value, "Hello, World!");
/// assert!(node.is_root());
/// assert!(node.is_leaf());
/// assert_eq!(node.num_children(), 0);
/// ```
///
/// ## Creating a node with a specific ID
///
/// ```
/// use jangal::Node;
///
/// let node = Node::with_id(42, 100.0);
/// assert_eq!(node.value, 42);
/// assert_eq!(node.id, 100.0);
/// ```
///
/// ## Building relationships between nodes
///
/// ```
/// use jangal::Node;
///
/// let mut parent = Node::new("parent");
/// let mut child = Node::new("child");
///
/// // Create parent-child relationship
/// parent.add_child(child.id);
/// child.set_parent(parent.id);
///
/// assert_eq!(parent.children(), vec![child.id]);
/// assert_eq!(child.parent(), Some(parent.id));
/// assert!(!parent.is_leaf());
/// assert!(!child.is_root());
/// ```
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Node<T> {
    pub value: T,
    pub id: Number,

    // Undirected edges
    edges: HashSet<FloatId>,

    // Directed edges
    incoming: HashSet<FloatId>,
    outgoing: HashSet<FloatId>,

    // Tree structure
    parent: Option<FloatId>,
    children: HashSet<FloatId>,

    // BST specific
    left: Option<FloatId>,
    right: Option<FloatId>,
}

impl<T> Node<T> {
    /// Create a new node with the given value
    ///
    /// The node is assigned a unique ID automatically. The new node starts
    /// with no connections to other nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let string_node = Node::new("hello");
    /// let number_node = Node::new(42);
    /// let bool_node = Node::new(true);
    ///
    /// assert_eq!(string_node.value, "hello");
    /// assert_eq!(number_node.value, 42);
    /// assert_eq!(bool_node.value, true);
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            value,
            id: Self::generate_id(),
            edges: HashSet::new(),
            incoming: HashSet::new(),
            outgoing: HashSet::new(),
            parent: None,
            children: HashSet::new(),
            left: None,
            right: None,
        }
    }

    /// Create a new node with a specific ID
    ///
    /// This allows you to control the ID assignment, which can be useful
    /// when reconstructing data structures from serialized data.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let node = Node::with_id("custom", 999.0);
    /// assert_eq!(node.id, 999.0);
    /// assert_eq!(node.value, "custom");
    /// ```
    pub fn with_id(value: T, id: Number) -> Self {
        Self {
            value,
            id,
            edges: HashSet::new(),
            incoming: HashSet::new(),
            outgoing: HashSet::new(),
            parent: None,
            children: HashSet::new(),
            left: None,
            right: None,
        }
    }

    /// Generate a unique ID for the node
    fn generate_id() -> Number {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed) as Number
    }

    /// Add an edge to another node
    ///
    /// This method allows you to create various types of connections:
    /// - Undirected edges (default): bidirectional connections
    /// - Directed edges: one-way connections from this node to another
    ///
    /// # Parameters
    ///
    /// * `other_id` - The ID of the node to connect to
    /// * `weight` - Optional weight for the edge (currently unused)
    /// * `directed` - Whether the edge is directed (default: false)
    /// * `bidirectional` - Whether to create a bidirectional connection (default: false)
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut node1 = Node::new("A");
    /// let node2 = Node::new("B");
    ///
    /// // Add an undirected edge
    /// node1.add_edge(node2.id, None, None, None);
    ///
    /// // Add a directed edge
    /// node1.add_edge(node2.id, None, Some(true), None);
    /// ```
    #[allow(unused_variables)]
    pub fn add_edge(
        &mut self,
        other_id: Number,
        weight: Option<Number>,
        directed: Option<bool>,
        bidirectional: Option<bool>,
    ) {
        let directed = directed.unwrap_or(false);
        let bidirectional = bidirectional.unwrap_or(false);
        let other_id = FloatId::from(other_id);

        if directed {
            self.outgoing.insert(other_id);
            // Note: The other node's incoming edge would need to be added separately
        } else if bidirectional {
            self.edges.insert(other_id);
            // Note: The other node's edge would need to be added separately
        } else {
            self.edges.insert(other_id);
        }
    }

    /// Add a child node
    ///
    /// Adds a node as a child of this node. This is used for tree structures
    /// where nodes have parent-child relationships.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut parent = Node::new("parent");
    /// let child = Node::new("child");
    ///
    /// parent.add_child(child.id);
    /// assert_eq!(parent.num_children(), 1);
    /// assert!(parent.children().contains(&child.id));
    /// ```
    pub fn add_child(&mut self, child_id: Number) {
        self.children.insert(FloatId::from(child_id));
    }

    /// Remove a child node
    ///
    /// Removes a node from this node's children. The child node's parent
    /// relationship should be updated separately.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut parent = Node::new("parent");
    /// let child = Node::new("child");
    ///
    /// parent.add_child(child.id);
    /// assert_eq!(parent.num_children(), 1);
    ///
    /// parent.remove_child(child.id);
    /// assert_eq!(parent.num_children(), 0);
    /// ```
    pub fn remove_child(&mut self, child_id: Number) {
        self.children.remove(&FloatId::from(child_id));
    }

    /// Set the parent of this node
    ///
    /// Establishes a parent relationship. The parent node should be updated
    /// separately to include this node as a child.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let parent = Node::new("parent");
    /// let mut child = Node::new("child");
    ///
    /// child.set_parent(parent.id);
    /// assert_eq!(child.parent(), Some(parent.id));
    /// assert!(!child.is_root());
    /// ```
    pub fn set_parent(&mut self, parent_id: Number) {
        self.parent = Some(FloatId::from(parent_id));
    }

    /// Remove parent relationship
    ///
    /// Makes this node a root node by removing its parent relationship.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let parent = Node::new("parent");
    /// let mut child = Node::new("child");
    ///
    /// child.set_parent(parent.id);
    /// assert!(!child.is_root());
    ///
    /// child.remove_parent();
    /// assert!(child.is_root());
    /// assert_eq!(child.parent(), None);
    /// ```
    pub fn remove_parent(&mut self) {
        self.parent = None;
    }

    /// Get the parent ID
    ///
    /// Returns the ID of this node's parent, or `None` if this is a root node.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let parent = Node::new("parent");
    /// let mut child = Node::new("child");
    ///
    /// assert_eq!(child.parent(), None);
    ///
    /// child.set_parent(parent.id);
    /// assert_eq!(child.parent(), Some(parent.id));
    /// ```
    pub fn parent(&self) -> Option<Number> {
        self.parent.map(|id| id.value())
    }

    /// Get children IDs
    ///
    /// Returns a vector containing the IDs of all child nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut parent = Node::new("parent");
    /// let child1 = Node::new("child1");
    /// let child2 = Node::new("child2");
    ///
    /// parent.add_child(child1.id);
    /// parent.add_child(child2.id);
    ///
    /// let children = parent.children();
    /// assert_eq!(children.len(), 2);
    /// assert!(children.contains(&child1.id));
    /// assert!(children.contains(&child2.id));
    /// ```
    pub fn children(&self) -> Vec<Number> {
        self.children.iter().map(|id| id.value()).collect()
    }

    /// Check if this node is a root (no parent)
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut node = Node::new("test");
    /// assert!(node.is_root());
    ///
    /// let parent = Node::new("parent");
    /// node.set_parent(parent.id);
    /// assert!(!node.is_root());
    /// ```
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Check if this node is a leaf (no children)
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut node = Node::new("test");
    /// assert!(node.is_leaf());
    ///
    /// let child = Node::new("child");
    /// node.add_child(child.id);
    /// assert!(!node.is_leaf());
    /// ```
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Get the number of children
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut node = Node::new("test");
    /// assert_eq!(node.num_children(), 0);
    ///
    /// let child1 = Node::new("child1");
    /// let child2 = Node::new("child2");
    /// node.add_child(child1.id);
    /// node.add_child(child2.id);
    /// assert_eq!(node.num_children(), 2);
    /// ```
    pub fn num_children(&self) -> usize {
        self.children.len()
    }

    /// Set left child (for binary trees)
    ///
    /// Sets the left child reference for binary tree structures.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut root = Node::new(10);
    /// let left = Node::new(5);
    ///
    /// root.set_left(left.id);
    /// assert_eq!(root.left(), Some(left.id));
    /// ```
    pub fn set_left(&mut self, left_id: Number) {
        self.left = Some(FloatId::from(left_id));
    }

    /// Set right child (for binary trees)
    ///
    /// Sets the right child reference for binary tree structures.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut root = Node::new(10);
    /// let right = Node::new(15);
    ///
    /// root.set_right(right.id);
    /// assert_eq!(root.right(), Some(right.id));
    /// ```
    pub fn set_right(&mut self, right_id: Number) {
        self.right = Some(FloatId::from(right_id));
    }

    /// Clear left child (for binary trees)
    ///
    /// Removes the left child reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut root = Node::new(10);
    /// let left = Node::new(5);
    ///
    /// root.set_left(left.id);
    /// assert_eq!(root.left(), Some(left.id));
    ///
    /// root.clear_left();
    /// assert_eq!(root.left(), None);
    /// ```
    pub fn clear_left(&mut self) {
        self.left = None;
    }

    /// Clear right child (for binary trees)
    ///
    /// Removes the right child reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut root = Node::new(10);
    /// let right = Node::new(15);
    ///
    /// root.set_right(right.id);
    /// assert_eq!(root.right(), Some(right.id));
    ///
    /// root.clear_right();
    /// assert_eq!(root.right(), None);
    /// ```
    pub fn clear_right(&mut self) {
        self.right = None;
    }

    /// Get left child ID
    ///
    /// Returns the ID of the left child, or `None` if there is no left child.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut root = Node::new(10);
    /// assert_eq!(root.left(), None);
    ///
    /// let left = Node::new(5);
    /// root.set_left(left.id);
    /// assert_eq!(root.left(), Some(left.id));
    /// ```
    pub fn left(&self) -> Option<Number> {
        self.left.map(|id| id.value())
    }

    /// Get right child ID
    ///
    /// Returns the ID of the right child, or `None` if there is no right child.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Node;
    ///
    /// let mut root = Node::new(10);
    /// assert_eq!(root.right(), None);
    ///
    /// let right = Node::new(15);
    /// root.set_right(right.id);
    /// assert_eq!(root.right(), Some(right.id));
    /// ```
    pub fn right(&self) -> Option<Number> {
        self.right.map(|id| id.value())
    }
}

impl<T> Hash for Node<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        FloatId::from(self.id).hash(state);
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Node<T> {}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node(value={})", self.value)
    }
}

/// A tree structure that manages nodes
///
/// A flexible tree structure that can represent various types of hierarchical data.
/// Each tree maintains a collection of nodes and their relationships, providing
/// methods for traversal, analysis, and manipulation.
///
/// # Examples
///
/// ## Creating a simple tree
///
/// ```
/// use jangal::{Tree, Node};
///
/// let mut tree = Tree::new();
/// let root = Node::new("root");
/// let root_id = tree.add_node(root).unwrap();
/// tree.set_root(root_id);
///
/// assert_eq!(tree.size(), 1);
/// assert_eq!(tree.root_id(), Some(root_id));
/// ```
///
/// ## Building a tree with parent-child relationships
///
/// ```
/// use jangal::{Tree, Node};
///
/// let mut tree = Tree::new();
///
/// let root = Node::new("root");
/// let child1 = Node::new("child1");
/// let child2 = Node::new("child2");
///
/// let root_id = tree.add_node(root).unwrap();
/// let child1_id = tree.add_node(child1).unwrap();
/// let child2_id = tree.add_node(child2).unwrap();
///
/// // Set up relationships
/// if let Some(root_node) = tree.get_node_mut(root_id) {
///     root_node.add_child(child1_id);
///     root_node.add_child(child2_id);
/// }
///
/// if let Some(child1_node) = tree.get_node_mut(child1_id) {
///     child1_node.set_parent(root_id);
/// }
///
/// if let Some(child2_node) = tree.get_node_mut(child2_id) {
///     child2_node.set_parent(root_id);
/// }
///
/// tree.set_root(root_id);
///
/// assert_eq!(tree.size(), 3);
/// assert_eq!(tree.height(root_id), 1);
/// assert_eq!(tree.num_leaves(root_id), 2);
/// ```
///
/// ## Tree traversal
///
/// ```
/// use jangal::{Tree, Node};
///
/// let mut tree = Tree::new();
/// let root = Node::new("root");
/// let child = Node::new("child");
///
/// let root_id = tree.add_node(root).unwrap();
/// let child_id = tree.add_node(child).unwrap();
///
/// if let Some(root_node) = tree.get_node_mut(root_id) {
///     root_node.add_child(child_id);
/// }
///
/// if let Some(child_node) = tree.get_node_mut(child_id) {
///     child_node.set_parent(root_id);
/// }
///
/// tree.set_root(root_id);
///
/// let dfs_result = tree.dfs(root_id);
/// let bfs_result = tree.bfs(root_id);
/// let preorder_result = tree.preorder(root_id);
/// let postorder_result = tree.postorder(root_id);
///
/// assert_eq!(dfs_result.len(), 2);
/// assert_eq!(bfs_result.len(), 2);
/// assert_eq!(preorder_result.len(), 2);
/// assert_eq!(postorder_result.len(), 2);
/// ```
#[derive(Debug)]
pub struct Tree<T> {
    nodes: HashMap<FloatId, Node<T>>,
    root_id: Option<FloatId>,
}

impl<T> Tree<T> {
    /// Create a new empty tree
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Tree;
    ///
    /// let tree: Tree<i32> = Tree::new();
    /// assert!(tree.is_empty());
    /// assert_eq!(tree.size(), 0);
    /// assert_eq!(tree.root_id(), None);
    /// ```
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_id: None,
        }
    }

    /// Add a node to the tree
    ///
    /// Adds a node to the tree and returns its ID. If this is the first node
    /// added to the tree, it will automatically be set as the root.
    ///
    /// Users can choose whether to handle the returned ID or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let node = Node::new("test");
    ///
    /// // When you need the ID
    /// let node_id = tree.add_node(node).unwrap();
    /// assert_eq!(tree.size(), 1);
    /// assert_eq!(tree.root_id(), Some(node_id));
    ///
    /// // When you don't need the ID
    /// let another_node = Node::new("another");
    /// tree.add_node(another_node);
    /// assert_eq!(tree.size(), 2);
    /// ```
    pub fn add_node(&mut self, node: Node<T>) -> Option<Number> {
        let id = FloatId::from(node.id);
        self.nodes.insert(id, node);
        if self.root_id.is_none() {
            self.root_id = Some(id);
        }
        Some(id.value())
    }

    /// Get a node by ID
    ///
    /// Returns a reference to the node with the given ID, or `None` if no such
    /// node exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let node = Node::new("test");
    /// let node_id = tree.add_node(node).unwrap();
    ///
    /// let retrieved_node = tree.get_node(node_id);
    /// assert!(retrieved_node.is_some());
    /// assert_eq!(retrieved_node.unwrap().value, "test");
    ///
    /// let non_existent = tree.get_node(999.0);
    /// assert!(non_existent.is_none());
    /// ```
    pub fn get_node(&self, id: Number) -> Option<&Node<T>> {
        self.nodes.get(&FloatId::from(id))
    }

    /// Get a mutable reference to a node by ID
    ///
    /// Returns a mutable reference to the node with the given ID, or `None` if
    /// no such node exists. This allows you to modify the node's properties.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let node = Node::new("test");
    /// let node_id = tree.add_node(node).unwrap();
    ///
    /// if let Some(node_mut) = tree.get_node_mut(node_id) {
    ///     node_mut.add_child(42.0);
    ///     assert_eq!(node_mut.num_children(), 1);
    /// }
    /// ```
    pub fn get_node_mut(&mut self, id: Number) -> Option<&mut Node<T>> {
        self.nodes.get_mut(&FloatId::from(id))
    }

    /// Get the root node
    ///
    /// Returns a reference to the root node of the tree, or `None` if the tree
    /// is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// assert_eq!(tree.root(), None);
    ///
    /// let node = Node::new("root");
    /// let node_id = tree.add_node(node).unwrap();
    /// tree.set_root(node_id);
    ///
    /// let root = tree.root();
    /// assert!(root.is_some());
    /// assert_eq!(root.unwrap().value, "root");
    /// ```
    pub fn root(&self) -> Option<&Node<T>> {
        self.root_id.and_then(|id| self.get_node(id.value()))
    }

    /// Get the root ID
    ///
    /// Returns the ID of the root node, or `None` if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// assert_eq!(tree.root_id(), None);
    ///
    /// let node = Node::new("root");
    /// let node_id = tree.add_node(node).unwrap();
    /// tree.set_root(node_id);
    ///
    /// assert_eq!(tree.root_id(), Some(node_id));
    /// ```
    pub fn root_id(&self) -> Option<Number> {
        self.root_id.map(|id| id.value())
    }

    /// Set the root ID
    #[allow(dead_code)]
    pub(crate) fn set_root_id(&mut self, id: Option<FloatId>) {
        self.root_id = id;
    }

    /// Remove a node
    #[allow(dead_code)]
    pub(crate) fn remove_node(&mut self, id: Number) {
        self.nodes.remove(&FloatId::from(id));
    }

    /// Set the root node
    ///
    /// Sets the node with the given ID as the root of the tree. The node must
    /// already exist in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let node = Node::new("root");
    /// let node_id = tree.add_node(node).unwrap();
    ///
    /// tree.set_root(node_id);
    /// assert_eq!(tree.root_id(), Some(node_id));
    /// ```
    pub fn set_root(&mut self, id: Number) {
        self.root_id = Some(FloatId::from(id));
    }

    /// Get the number of nodes in the tree
    ///
    /// Returns the total number of nodes currently in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// assert_eq!(tree.size(), 0);
    ///
    /// let node1 = Node::new("first");
    /// let node2 = Node::new("second");
    /// tree.add_node(node1);
    /// tree.add_node(node2);
    ///
    /// assert_eq!(tree.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the tree is empty
    ///
    /// Returns `true` if the tree contains no nodes, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// assert!(tree.is_empty());
    ///
    /// let node = Node::new("test");
    /// tree.add_node(node);
    /// assert!(!tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Calculate the height of a node
    ///
    /// The height of a node is the length of the longest path from the node
    /// to a leaf. A leaf node has height 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let root = Node::new("root");
    /// let child = Node::new("child");
    /// let grandchild = Node::new("grandchild");
    ///
    /// let root_id = tree.add_node(root).unwrap();
    /// let child_id = tree.add_node(child).unwrap();
    /// let grandchild_id = tree.add_node(grandchild).unwrap();
    ///
    /// // Set up relationships
    /// if let Some(root_node) = tree.get_node_mut(root_id) {
    ///     root_node.add_child(child_id);
    /// }
    /// if let Some(child_node) = tree.get_node_mut(child_id) {
    ///     child_node.set_parent(root_id);
    ///     child_node.add_child(grandchild_id);
    /// }
    /// if let Some(grandchild_node) = tree.get_node_mut(grandchild_id) {
    ///     grandchild_node.set_parent(child_id);
    /// }
    ///
    /// tree.set_root(root_id);
    ///
    /// assert_eq!(tree.height(root_id), 2);
    /// assert_eq!(tree.height(child_id), 1);
    /// assert_eq!(tree.height(grandchild_id), 0);
    /// ```
    pub fn height(&self, node_id: Number) -> usize {
        if let Some(node) = self.get_node(node_id) {
            if node.is_leaf() {
                return 0;
            }
            let mut max_height = 0;
            for child_id in node.children() {
                let child_height = self.height(child_id);
                max_height = max_height.max(child_height);
            }
            return 1 + max_height;
        }
        0
    }

    /// Calculate the depth of a node
    ///
    /// The depth of a node is the length of the path from the root to the node.
    /// The root node has depth 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let root = Node::new("root");
    /// let child = Node::new("child");
    /// let grandchild = Node::new("grandchild");
    ///
    /// let root_id = tree.add_node(root).unwrap();
    /// let child_id = tree.add_node(child).unwrap();
    /// let grandchild_id = tree.add_node(grandchild).unwrap();
    ///
    /// // Set up relationships
    /// if let Some(root_node) = tree.get_node_mut(root_id) {
    ///     root_node.add_child(child_id);
    /// }
    /// if let Some(child_node) = tree.get_node_mut(child_id) {
    ///     child_node.set_parent(root_id);
    ///     child_node.add_child(grandchild_id);
    /// }
    /// if let Some(grandchild_node) = tree.get_node_mut(grandchild_id) {
    ///     grandchild_node.set_parent(child_id);
    /// }
    ///
    /// tree.set_root(root_id);
    ///
    /// assert_eq!(tree.depth(root_id), 0);
    /// assert_eq!(tree.depth(child_id), 1);
    /// assert_eq!(tree.depth(grandchild_id), 2);
    /// ```
    pub fn depth(&self, node_id: Number) -> usize {
        self.depth_recursive(FloatId::from(node_id), &mut HashSet::new())
    }

    fn depth_recursive(&self, node_id: FloatId, visited: &mut HashSet<FloatId>) -> usize {
        if visited.contains(&node_id) {
            return 0; // Prevent infinite recursion
        }

        visited.insert(node_id);

        if let Some(node) = self.nodes.get(&node_id) {
            if node.is_root() {
                return 0;
            }
            if let Some(parent_id) = node.parent() {
                return 1 + self.depth_recursive(FloatId::from(parent_id), visited);
            }
        }
        0
    }

    /// Count the number of leaves in the subtree rooted at the given node
    ///
    /// A leaf is a node with no children. This method recursively counts all
    /// leaf nodes in the subtree.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let root = Node::new("root");
    /// let child1 = Node::new("child1");
    /// let child2 = Node::new("child2");
    /// let grandchild = Node::new("grandchild");
    ///
    /// let root_id = tree.add_node(root).unwrap();
    /// let child1_id = tree.add_node(child1).unwrap();
    /// let child2_id = tree.add_node(child2).unwrap();
    /// let grandchild_id = tree.add_node(grandchild).unwrap();
    ///
    /// // Set up relationships
    /// if let Some(root_node) = tree.get_node_mut(root_id) {
    ///     root_node.add_child(child1_id);
    ///     root_node.add_child(child2_id);
    /// }
    /// if let Some(child1_node) = tree.get_node_mut(child1_id) {
    ///     child1_node.set_parent(root_id);
    ///     child1_node.add_child(grandchild_id);
    /// }
    /// if let Some(child2_node) = tree.get_node_mut(child2_id) {
    ///     child2_node.set_parent(root_id);
    /// }
    /// if let Some(grandchild_node) = tree.get_node_mut(grandchild_id) {
    ///     grandchild_node.set_parent(child1_id);
    /// }
    ///
    /// tree.set_root(root_id);
    ///
    /// assert_eq!(tree.num_leaves(root_id), 2);
    /// assert_eq!(tree.num_leaves(child1_id), 1);
    /// assert_eq!(tree.num_leaves(child2_id), 1);
    /// ```
    pub fn num_leaves(&self, node_id: Number) -> usize {
        if let Some(node) = self.get_node(node_id) {
            if node.is_leaf() {
                return 1;
            }
            let mut count = 0;
            for child_id in node.children() {
                count += self.num_leaves(child_id);
            }
            return count;
        }
        0
    }

    /// Count the total number of nodes in the subtree rooted at the given node
    ///
    /// This method recursively counts all nodes in the subtree, including the
    /// root node itself.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let root = Node::new("root");
    /// let child1 = Node::new("child1");
    /// let child2 = Node::new("child2");
    ///
    /// let root_id = tree.add_node(root).unwrap();
    /// let child1_id = tree.add_node(child1).unwrap();
    /// let child2_id = tree.add_node(child2).unwrap();
    ///
    /// // Set up relationships
    /// if let Some(root_node) = tree.get_node_mut(root_id) {
    ///     root_node.add_child(child1_id);
    ///     root_node.add_child(child2_id);
    /// }
    /// if let Some(child1_node) = tree.get_node_mut(child1_id) {
    ///     child1_node.set_parent(root_id);
    /// }
    /// if let Some(child2_node) = tree.get_node_mut(child2_id) {
    ///     child2_node.set_parent(root_id);
    /// }
    ///
    /// tree.set_root(root_id);
    ///
    /// assert_eq!(tree.num_nodes(root_id), 3);
    /// assert_eq!(tree.num_nodes(child1_id), 1);
    /// assert_eq!(tree.num_nodes(child2_id), 1);
    /// ```
    pub fn num_nodes(&self, node_id: Number) -> usize {
        if let Some(node) = self.get_node(node_id) {
            let mut count = 1;
            for child_id in node.children() {
                count += self.num_nodes(child_id);
            }
            return count;
        }
        0
    }

    /// Check if the tree is balanced (all leaf nodes are at most one level apart)
    ///
    /// A tree is considered balanced if the heights of all subtrees differ by
    /// at most 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let root = Node::new("root");
    /// let child1 = Node::new("child1");
    /// let child2 = Node::new("child2");
    ///
    /// let root_id = tree.add_node(root).unwrap();
    /// let child1_id = tree.add_node(child1).unwrap();
    /// let child2_id = tree.add_node(child2).unwrap();
    ///
    /// // Set up relationships
    /// if let Some(root_node) = tree.get_node_mut(root_id) {
    ///     root_node.add_child(child1_id);
    ///     root_node.add_child(child2_id);
    /// }
    /// if let Some(child1_node) = tree.get_node_mut(child1_id) {
    ///     child1_node.set_parent(root_id);
    /// }
    /// if let Some(child2_node) = tree.get_node_mut(child2_id) {
    ///     child2_node.set_parent(root_id);
    /// }
    ///
    /// tree.set_root(root_id);
    ///
    /// // This tree is balanced: both children are at the same level
    /// assert!(tree.is_balanced(root_id));
    /// ```
    pub fn is_balanced(&self, node_id: Number) -> bool {
        if let Some(node) = self.get_node(node_id) {
            if node.is_leaf() {
                return true;
            }

            let mut heights = Vec::new();
            for child_id in node.children() {
                heights.push(self.height(child_id));
            }
            heights.sort_by(|a, b| b.cmp(a));

            if let Some(&max_height) = heights.first() {
                return heights.iter().all(|&h| max_height - h <= 1);
            }
        }
        true
    }

    /// Get all leaf values in the subtree
    ///
    /// Returns a vector containing references to all leaf nodes
    /// in the subtree rooted at the given node.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let root = Node::new("root");
    /// let child1 = Node::new("child1");
    /// let child2 = Node::new("child2");
    /// let grandchild = Node::new("grandchild");
    ///
    /// let root_id = tree.add_node(root).unwrap();
    /// let child1_id = tree.add_node(child1).unwrap();
    /// let child2_id = tree.add_node(child2).unwrap();
    /// let grandchild_id = tree.add_node(grandchild).unwrap();
    ///
    /// // Set up relationships
    /// if let Some(root_node) = tree.get_node_mut(root_id) {
    ///     root_node.add_child(child1_id);
    ///     root_node.add_child(child2_id);
    /// }
    /// if let Some(child1_node) = tree.get_node_mut(child1_id) {
    ///     child1_node.set_parent(root_id);
    ///     child1_node.add_child(grandchild_id);
    /// }
    /// if let Some(child2_node) = tree.get_node_mut(child2_id) {
    ///     child2_node.set_parent(root_id);
    /// }
    /// if let Some(grandchild_node) = tree.get_node_mut(grandchild_id) {
    ///     grandchild_node.set_parent(child1_id);
    /// }
    ///
    /// tree.set_root(root_id);
    ///
    /// let leaves = tree.get_leaves(root_id);
    /// assert_eq!(leaves.len(), 2);
    /// assert!(leaves.iter().any(|node| node.value == "child2"));
    /// assert!(leaves.iter().any(|node| node.value == "grandchild"));
    /// ```
    pub fn get_leaves(&self, node_id: Number) -> Vec<&Node<T>> {
        if let Some(node) = self.get_node(node_id) {
            if node.is_leaf() {
                return vec![node];
            }
            let mut leaves = Vec::new();
            for child_id in node.children() {
                leaves.extend(self.get_leaves(child_id));
            }
            return leaves;
        }
        Vec::new()
    }

    /// Perform depth-first search traversal
    ///
    /// Traverses the subtree in depth-first order, visiting nodes as deep as
    /// possible before backtracking. Returns a vector of nodes in traversal order.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let root = Node::new("root");
    /// let child1 = Node::new("child1");
    /// let child2 = Node::new("child2");
    /// let grandchild = Node::new("grandchild");
    ///
    /// let root_id = tree.add_node(root).unwrap();
    /// let child1_id = tree.add_node(child1).unwrap();
    /// let child2_id = tree.add_node(child2).unwrap();
    /// let grandchild_id = tree.add_node(grandchild).unwrap();
    ///
    /// // Set up relationships
    /// if let Some(root_node) = tree.get_node_mut(root_id) {
    ///     root_node.add_child(child1_id);
    ///     root_node.add_child(child2_id);
    /// }
    /// if let Some(child1_node) = tree.get_node_mut(child1_id) {
    ///     child1_node.set_parent(root_id);
    ///     child1_node.add_child(grandchild_id);
    /// }
    /// if let Some(child2_node) = tree.get_node_mut(child2_id) {
    ///     child2_node.set_parent(root_id);
    /// }
    /// if let Some(grandchild_node) = tree.get_node_mut(grandchild_id) {
    ///     grandchild_node.set_parent(child1_id);
    /// }
    ///
    /// tree.set_root(root_id);
    ///
    /// let dfs_result = tree.dfs(root_id);
    /// assert_eq!(dfs_result.len(), 4);
    /// ```
    pub fn dfs(&self, node_id: Number) -> Vec<&Node<T>> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_recursive(FloatId::from(node_id), &mut visited, &mut result);
        result
    }

    fn dfs_recursive<'a>(
        &'a self,
        node_id: FloatId,
        visited: &mut HashSet<FloatId>,
        result: &mut Vec<&'a Node<T>>,
    ) {
        if visited.contains(&node_id) {
            return;
        }

        visited.insert(node_id);

        if let Some(node) = self.nodes.get(&node_id) {
            result.push(node);
            for child_id in node.children() {
                self.dfs_recursive(FloatId::from(child_id), visited, result);
            }
        }
    }

    /// Perform breadth-first search traversal
    ///
    /// Traverses the subtree level by level, visiting all nodes at the current
    /// level before moving to the next level. Returns a vector of nodes in traversal order.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let root = Node::new("root");
    /// let child1 = Node::new("child1");
    /// let child2 = Node::new("child2");
    /// let grandchild = Node::new("grandchild");
    ///
    /// let root_id = tree.add_node(root).unwrap();
    /// let child1_id = tree.add_node(child1).unwrap();
    /// let child2_id = tree.add_node(child2).unwrap();
    /// let grandchild_id = tree.add_node(grandchild).unwrap();
    ///
    /// // Set up relationships
    /// if let Some(root_node) = tree.get_node_mut(root_id) {
    ///     root_node.add_child(child1_id);
    ///     root_node.add_child(child2_id);
    /// }
    /// if let Some(child1_node) = tree.get_node_mut(child1_id) {
    ///     child1_node.set_parent(root_id);
    ///     child1_node.add_child(grandchild_id);
    /// }
    /// if let Some(child2_node) = tree.get_node_mut(child2_id) {
    ///     child2_node.set_parent(root_id);
    /// }
    /// if let Some(grandchild_node) = tree.get_node_mut(grandchild_id) {
    ///     grandchild_node.set_parent(child1_id);
    /// }
    ///
    /// tree.set_root(root_id);
    ///
    /// let bfs_result = tree.bfs(root_id);
    /// assert_eq!(bfs_result.len(), 4);
    /// ```
    pub fn bfs(&self, node_id: Number) -> Vec<&Node<T>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        let node_id = FloatId::from(node_id);
        queue.push_back(node_id);
        visited.insert(node_id);

        while let Some(current_id) = queue.pop_front() {
            if let Some(node) = self.nodes.get(&current_id) {
                result.push(node);
                for child_id in node.children() {
                    let child_id = FloatId::from(child_id);
                    if !visited.contains(&child_id) {
                        visited.insert(child_id);
                        queue.push_back(child_id);
                    }
                }
            }
        }

        result
    }

    /// Perform preorder traversal
    ///
    /// Traverses the subtree in preorder: root, left subtree, right subtree.
    /// Returns a vector of nodes in traversal order.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let root = Node::new("root");
    /// let child1 = Node::new("child1");
    /// let child2 = Node::new("child2");
    ///
    /// let root_id = tree.add_node(root).unwrap();
    /// let child1_id = tree.add_node(child1).unwrap();
    /// let child2_id = tree.add_node(child2).unwrap();
    ///
    /// // Set up relationships
    /// if let Some(root_node) = tree.get_node_mut(root_id) {
    ///     root_node.add_child(child1_id);
    ///     root_node.add_child(child2_id);
    /// }
    /// if let Some(child1_node) = tree.get_node_mut(child1_id) {
    ///     child1_node.set_parent(root_id);
    /// }
    /// if let Some(child2_node) = tree.get_node_mut(child2_id) {
    ///     child2_node.set_parent(root_id);
    /// }
    ///
    /// tree.set_root(root_id);
    ///
    /// let preorder_result = tree.preorder(root_id);
    /// assert_eq!(preorder_result.len(), 3);
    /// ```
    pub fn preorder(&self, node_id: Number) -> Vec<&Node<T>> {
        let mut result = Vec::new();
        self.preorder_recursive(FloatId::from(node_id), &mut result);
        result
    }

    fn preorder_recursive<'a>(&'a self, node_id: FloatId, result: &mut Vec<&'a Node<T>>) {
        if let Some(node) = self.nodes.get(&node_id) {
            result.push(node);
            for child_id in node.children() {
                self.preorder_recursive(FloatId::from(child_id), result);
            }
        }
    }

    /// Perform postorder traversal
    ///
    /// Traverses the subtree in postorder: left subtree, right subtree, root.
    /// Returns a vector of nodes in traversal order.
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::{Tree, Node};
    ///
    /// let mut tree = Tree::new();
    /// let root = Node::new("root");
    /// let child1 = Node::new("child1");
    /// let child2 = Node::new("child2");
    ///
    /// let root_id = tree.add_node(root).unwrap();
    /// let child1_id = tree.add_node(child1).unwrap();
    /// let child2_id = tree.add_node(child2).unwrap();
    ///
    /// // Set up relationships
    /// if let Some(root_node) = tree.get_node_mut(root_id) {
    ///     root_node.add_child(child1_id);
    ///     root_node.add_child(child2_id);
    /// }
    /// if let Some(child1_node) = tree.get_node_mut(child1_id) {
    ///     child1_node.set_parent(root_id);
    /// }
    /// if let Some(child2_node) = tree.get_node_mut(child2_id) {
    ///     child2_node.set_parent(root_id);
    /// }
    ///
    /// tree.set_root(root_id);
    ///
    /// let postorder_result = tree.postorder(root_id);
    /// assert_eq!(postorder_result.len(), 3);
    /// ```
    pub fn postorder(&self, node_id: Number) -> Vec<&Node<T>> {
        let mut result = Vec::new();
        self.postorder_recursive(FloatId::from(node_id), &mut result);
        result
    }

    fn postorder_recursive<'a>(&'a self, node_id: FloatId, result: &mut Vec<&'a Node<T>>) {
        if let Some(node) = self.nodes.get(&node_id) {
            for child_id in node.children() {
                self.postorder_recursive(FloatId::from(child_id), result);
            }
        }
        if let Some(node) = self.nodes.get(&node_id) {
            result.push(node);
        }
    }
}

impl<T> Default for Tree<T> {
    /// Create a new empty tree using the default implementation
    ///
    /// # Examples
    ///
    /// ```
    /// use jangal::Tree;
    ///
    /// let tree: Tree<String> = Tree::default();
    /// assert!(tree.is_empty());
    /// assert_eq!(tree.size(), 0);
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(42);
        assert_eq!(node.value, 42);
        assert!(node.is_root());
        assert!(node.is_leaf());
        assert_eq!(node.num_children(), 0);
    }

    #[test]
    fn test_node_with_id() {
        let node = Node::with_id("test", 123.0);
        assert_eq!(node.value, "test");
        assert_eq!(node.id, 123.0);
    }

    #[test]
    fn test_node_relationships() {
        let mut parent = Node::new("parent");
        let mut child = Node::new("child");

        parent.add_child(child.id);
        child.set_parent(parent.id);

        assert_eq!(parent.children(), vec![child.id]);
        assert_eq!(child.parent(), Some(parent.id));
        assert!(!parent.is_leaf());
        assert!(child.is_leaf());
        assert!(!child.is_root());
    }

    #[test]
    fn test_node_display() {
        let node = Node::new("test_value");
        let display_str = format!("{}", node);
        assert_eq!(display_str, "Node(value=test_value)");
    }

    #[test]
    fn test_node_equality() {
        let node1 = Node::with_id(42, 1.0);
        let node2 = Node::with_id(42, 1.0);
        let node3 = Node::with_id(42, 2.0);

        assert_eq!(node1, node2);
        assert_ne!(node1, node3);
    }

    #[test]
    fn test_float_id_hash_and_eq() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let id1 = FloatId::new(1.5);
        let id2 = FloatId::new(1.5);
        let id3 = FloatId::new(2.5);

        map.insert(id1, "first");
        map.insert(id2, "second");
        map.insert(id3, "third");

        assert_eq!(map.get(&id1), Some(&"second"));
        assert_eq!(map.get(&id2), Some(&"second"));
        assert_eq!(map.get(&id3), Some(&"third"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_float_id_nan_handling() {
        let nan1 = FloatId::new(f64::NAN);
        let nan2 = FloatId::new(f64::NAN);
        let regular = FloatId::new(1.0);

        assert_eq!(nan1, nan2);
        assert_ne!(nan1, regular);
    }

    #[test]
    fn test_binary_tree_operations() {
        let mut root = Node::new(10);
        let left = Node::new(5);
        let right = Node::new(15);

        root.set_left(left.id);
        root.set_right(right.id);

        assert_eq!(root.left(), Some(left.id));
        assert_eq!(root.right(), Some(right.id));

        root.clear_left();
        root.clear_right();

        assert_eq!(root.left(), None);
        assert_eq!(root.right(), None);
    }

    #[test]
    fn test_multiple_children() {
        let mut parent = Node::new("parent");
        let child1 = Node::new("child1");
        let child2 = Node::new("child2");
        let child3 = Node::new("child3");

        parent.add_child(child1.id);
        parent.add_child(child2.id);
        parent.add_child(child3.id);

        assert_eq!(parent.num_children(), 3);
        let children = parent.children();
        assert!(children.contains(&child1.id));
        assert!(children.contains(&child2.id));
        assert!(children.contains(&child3.id));

        // Test removing a child
        parent.remove_child(child2.id);
        assert_eq!(parent.num_children(), 2);
        let children = parent.children();
        assert!(children.contains(&child1.id));
        assert!(!children.contains(&child2.id));
        assert!(children.contains(&child3.id));
    }

    #[test]
    fn test_parent_child_relationship() {
        let mut parent = Node::new("parent");
        let mut child = Node::new("child");

        // Initially both are roots and leaves
        assert!(parent.is_root());
        assert!(parent.is_leaf());
        assert!(child.is_root());
        assert!(child.is_leaf());

        // Create relationship
        parent.add_child(child.id);
        child.set_parent(parent.id);

        // Check parent state
        assert!(parent.is_root());
        assert!(!parent.is_leaf());
        assert_eq!(parent.num_children(), 1);

        // Check child state
        assert!(!child.is_root());
        assert!(child.is_leaf());
        assert_eq!(child.parent(), Some(parent.id));

        // Remove relationship
        parent.remove_child(child.id);
        child.remove_parent();

        // Both should be roots and leaves again
        assert!(parent.is_root());
        assert!(parent.is_leaf());
        assert!(child.is_root());
        assert!(child.is_leaf());
    }

    #[test]
    fn test_edge_operations() {
        let mut node1 = Node::new("A");
        let node2 = Node::new("B");

        node1.add_edge(node2.id, None, None, None);
        node1.add_edge(node2.id, None, Some(true), None);
    }

    #[test]
    fn test_unique_ids() {
        let node1 = Node::new("first");
        let node2 = Node::new("second");
        let node3 = Node::new("third");

        // Each node should have a unique ID
        assert_ne!(node1.id, node2.id);
        assert_ne!(node2.id, node3.id);
        assert_ne!(node1.id, node3.id);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashMap;

        let node1 = Node::with_id("test", 42.0);
        let node2 = Node::with_id("different_value", 42.0); // Same ID, different value

        let mut map = HashMap::new();
        map.insert(node1.clone(), "first");
        map.insert(node2.clone(), "second"); // Should overwrite because same ID

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&node1), Some(&"second"));
        assert_eq!(map.get(&node2), Some(&"second"));
    }

    #[test]
    fn test_float_id_conversion() {
        let value = 3.14159;
        let float_id = FloatId::new(value);

        assert_eq!(float_id.value(), value);

        let converted_to_f64: f64 = float_id.into();
        assert_eq!(converted_to_f64, value);

        let converted_from_f64 = FloatId::from(value);
        assert_eq!(converted_from_f64, float_id);
    }

    #[test]
    fn test_tree_operations() {
        let mut tree = Tree::<i32>::new();

        let node1 = Node::new(1);
        let node2 = Node::new(2);
        let node3 = Node::new(3);

        let id1 = tree.add_node(node1).unwrap();
        let id2 = tree.add_node(node2).unwrap();
        let id3 = tree.add_node(node3).unwrap();

        // Set up parent-child relationships
        if let Some(parent) = tree.get_node_mut(id1) {
            parent.add_child(id2);
            parent.add_child(id3);
        }

        if let Some(child1) = tree.get_node_mut(id2) {
            child1.set_parent(id1);
        }

        if let Some(child2) = tree.get_node_mut(id3) {
            child2.set_parent(id1);
        }

        tree.set_root(id1);

        assert_eq!(tree.size(), 3);
        assert_eq!(tree.height(id1), 1);
        assert_eq!(tree.depth(id2), 1);
        assert_eq!(tree.num_leaves(id1), 2);
        assert_eq!(tree.num_nodes(id1), 3);
        assert!(tree.is_balanced(id1));
    }

    #[test]
    fn test_tree_traversals() {
        let mut tree = Tree::<&str>::new();

        // Create a simple tree: root -> [child1, child2] -> [grandchild1, grandchild2]
        let root = Node::new("root");
        let child1 = Node::new("child1");
        let child2 = Node::new("child2");
        let grandchild1 = Node::new("grandchild1");
        let grandchild2 = Node::new("grandchild2");

        let root_id = tree.add_node(root).unwrap();
        let child1_id = tree.add_node(child1).unwrap();
        let child2_id = tree.add_node(child2).unwrap();
        let grandchild1_id = tree.add_node(grandchild1).unwrap();
        let grandchild2_id = tree.add_node(grandchild2).unwrap();

        // Set up relationships
        if let Some(root_node) = tree.get_node_mut(root_id) {
            root_node.add_child(child1_id);
            root_node.add_child(child2_id);
        }

        if let Some(child1_node) = tree.get_node_mut(child1_id) {
            child1_node.set_parent(root_id);
            child1_node.add_child(grandchild1_id);
        }

        if let Some(child2_node) = tree.get_node_mut(child2_id) {
            child2_node.set_parent(root_id);
            child2_node.add_child(grandchild2_id);
        }

        if let Some(grandchild1_node) = tree.get_node_mut(grandchild1_id) {
            grandchild1_node.set_parent(child1_id);
        }

        if let Some(grandchild2_node) = tree.get_node_mut(grandchild2_id) {
            grandchild2_node.set_parent(child2_id);
        }

        tree.set_root(root_id);

        // Test traversals
        let dfs_result = tree.dfs(root_id);
        let bfs_result = tree.bfs(root_id);
        let preorder_result = tree.preorder(root_id);
        let postorder_result = tree.postorder(root_id);

        assert_eq!(dfs_result.len(), 5);
        assert_eq!(bfs_result.len(), 5);
        assert_eq!(preorder_result.len(), 5);
        assert_eq!(postorder_result.len(), 5);

        // Verify root is first in preorder
        assert_eq!(preorder_result[0].id, root_id);

        // Verify root is last in postorder
        assert_eq!(postorder_result[4].id, root_id);
    }

    #[test]
    fn test_tree_properties() {
        let mut tree = Tree::<&str>::new();

        let root = Node::new("root");
        let child1 = Node::new("child1");
        let child2 = Node::new("child2");

        let root_id = tree.add_node(root).unwrap();
        let child1_id = tree.add_node(child1).unwrap();
        let child2_id = tree.add_node(child2).unwrap();

        // Set up relationships
        if let Some(root_node) = tree.get_node_mut(root_id) {
            root_node.add_child(child1_id);
            root_node.add_child(child2_id);
        }

        if let Some(child1_node) = tree.get_node_mut(child1_id) {
            child1_node.set_parent(root_id);
        }

        if let Some(child2_node) = tree.get_node_mut(child2_id) {
            child2_node.set_parent(root_id);
        }

        tree.set_root(root_id);

        // Test properties
        assert_eq!(tree.height(root_id), 1);
        assert_eq!(tree.depth(child1_id), 1);
        assert_eq!(tree.num_leaves(root_id), 2);
        assert_eq!(tree.num_nodes(root_id), 3);
        assert!(tree.is_balanced(root_id));

        let leaves = tree.get_leaves(root_id);
        assert_eq!(leaves.len(), 2);
        assert!(leaves.iter().any(|node| node.value == "child1"));
        assert!(leaves.iter().any(|node| node.value == "child2"));
    }
}
