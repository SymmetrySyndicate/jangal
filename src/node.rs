use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

/// A wrapper for f64 that implements Hash and Eq
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
        // Handle NaN specially
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

/// Type alias for numeric values
pub type Number = f64;

/// Generic Node Struct
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
    pub fn add_child(&mut self, child_id: Number) {
        self.children.insert(FloatId::from(child_id));
    }

    /// Remove a child node
    pub fn remove_child(&mut self, child_id: Number) {
        self.children.remove(&FloatId::from(child_id));
    }

    /// Set the parent of this node
    pub fn set_parent(&mut self, parent_id: Number) {
        self.parent = Some(FloatId::from(parent_id));
    }

    /// Remove parent relationship
    pub fn remove_parent(&mut self) {
        self.parent = None;
    }

    /// Get the parent ID
    pub fn parent(&self) -> Option<Number> {
        self.parent.map(|id| id.value())
    }

    /// Get children IDs
    pub fn children(&self) -> Vec<Number> {
        self.children.iter().map(|id| id.value()).collect()
    }

    /// Check if this node is a root (no parent)
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Check if this node is a leaf (no children)
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Get the number of children
    pub fn num_children(&self) -> usize {
        self.children.len()
    }

    /// Set left child (for binary trees)
    pub fn set_left(&mut self, left_id: Number) {
        self.left = Some(FloatId::from(left_id));
    }

    /// Set right child (for binary trees)
    pub fn set_right(&mut self, right_id: Number) {
        self.right = Some(FloatId::from(right_id));
    }

    /// Clear left child (for binary trees)
    pub fn clear_left(&mut self) {
        self.left = None;
    }

    /// Clear right child (for binary trees)
    pub fn clear_right(&mut self) {
        self.right = None;
    }

    /// Get left child ID
    pub fn left(&self) -> Option<Number> {
        self.left.map(|id| id.value())
    }

    /// Get right child ID
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
}
