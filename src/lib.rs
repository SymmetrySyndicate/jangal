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

use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

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
        } else {
            if bidirectional {
                self.edges.insert(other_id);
                // Note: The other node's edge would need to be added separately
            } else {
                self.edges.insert(other_id);
            }
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
        map.insert(id2, "second"); // Should overwrite first
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

        assert_eq!(nan1, nan2); // NaN should equal NaN in our implementation
        assert_ne!(nan1, regular);
    }

    #[test]
    fn test_binary_tree_operations() {
        let mut root = Node::new(10);
        let left = Node::new(5);
        let right = Node::new(15);

        // Test setting children
        root.set_left(left.id);
        root.set_right(right.id);

        assert_eq!(root.left(), Some(left.id));
        assert_eq!(root.right(), Some(right.id));

        // Test clearing children
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

        // Test adding edges (basic functionality)
        node1.add_edge(node2.id, None, None, None);
        node1.add_edge(node2.id, None, Some(true), None);

        // This test mainly checks that the methods don't panic
        // Full edge functionality would require a graph structure
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
}
