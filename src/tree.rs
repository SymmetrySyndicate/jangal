use std::collections::{HashMap, HashSet, VecDeque};
use crate::node::{Node, Number, FloatId};

/// A tree structure that manages nodes
#[derive(Debug)]
pub struct Tree<T> {
    nodes: HashMap<FloatId, Node<T>>,
    root_id: Option<FloatId>,
}

impl<T> Tree<T> {
    /// Create a new empty tree
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_id: None,
        }
    }

    /// Add a node to the tree
    pub fn add_node(&mut self, mut node: Node<T>) -> Number {
        let original_id = node.id;
        let id = FloatId::from(original_id);
        node.id = original_id; // Keep the original ID
        self.nodes.insert(id, node);
        if self.root_id.is_none() {
            self.root_id = Some(id);
        }
        original_id
    }

    /// Get a node by ID
    pub fn get_node(&self, id: Number) -> Option<&Node<T>> {
        self.nodes.get(&FloatId::from(id))
    }

    /// Get a mutable reference to a node by ID
    pub fn get_node_mut(&mut self, id: Number) -> Option<&mut Node<T>> {
        self.nodes.get_mut(&FloatId::from(id))
    }

    /// Get the root node
    pub fn root(&self) -> Option<&Node<T>> {
        self.root_id.and_then(|id| self.get_node(id.value()))
    }

    /// Get the root ID
    pub fn root_id(&self) -> Option<Number> {
        self.root_id.map(|id| id.value())
    }

    /// Set the root ID (internal use)
    pub(crate) fn set_root_id(&mut self, id: Option<FloatId>) {
        self.root_id = id;
    }

    /// Remove a node (internal use)
    pub(crate) fn remove_node(&mut self, id: Number) {
        self.nodes.remove(&FloatId::from(id));
    }

    /// Set the root node
    pub fn set_root(&mut self, id: Number) {
        self.root_id = Some(FloatId::from(id));
    }

    /// Get the number of nodes in the tree
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Calculate the height of a node
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
    pub fn is_balanced(&self, node_id: Number) -> bool {
        if let Some(node) = self.get_node(node_id) {
            if node.is_leaf() {
                return true;
            }

            let mut heights = Vec::new();
            for child_id in node.children() {
                heights.push(self.height(child_id));
            }
            heights.sort_by(|a, b| b.cmp(a)); // Sort in descending order

            // Check if all heights are within 1 of each other
            if let Some(&max_height) = heights.first() {
                return heights.iter().all(|&h| max_height - h <= 1);
            }
        }
        true
    }

    /// Get all leaf values in the subtree
    pub fn get_leaves(&self, node_id: Number) -> Vec<&T> {
        if let Some(node) = self.get_node(node_id) {
            if node.is_leaf() {
                return vec![&node.value];
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
    pub fn dfs(&self, node_id: Number) -> Vec<Number> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_recursive(FloatId::from(node_id), &mut visited, &mut result);
        result
    }

    fn dfs_recursive(&self, node_id: FloatId, visited: &mut HashSet<FloatId>, result: &mut Vec<Number>) {
        if visited.contains(&node_id) {
            return;
        }

        visited.insert(node_id);
        result.push(node_id.value());

        if let Some(node) = self.nodes.get(&node_id) {
            for child_id in node.children() {
                self.dfs_recursive(FloatId::from(child_id), visited, result);
            }
        }
    }

    /// Perform breadth-first search traversal
    pub fn bfs(&self, node_id: Number) -> Vec<Number> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        let node_id = FloatId::from(node_id);
        queue.push_back(node_id);
        visited.insert(node_id);

        while let Some(current_id) = queue.pop_front() {
            result.push(current_id.value());

            if let Some(node) = self.nodes.get(&current_id) {
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
    pub fn preorder(&self, node_id: Number) -> Vec<Number> {
        let mut result = Vec::new();
        self.preorder_recursive(FloatId::from(node_id), &mut result);
        result
    }

    fn preorder_recursive(&self, node_id: FloatId, result: &mut Vec<Number>) {
        result.push(node_id.value());

        if let Some(node) = self.nodes.get(&node_id) {
            for child_id in node.children() {
                self.preorder_recursive(FloatId::from(child_id), result);
            }
        }
    }

    /// Perform postorder traversal
    pub fn postorder(&self, node_id: Number) -> Vec<Number> {
        let mut result = Vec::new();
        self.postorder_recursive(FloatId::from(node_id), &mut result);
        result
    }

    fn postorder_recursive(&self, node_id: FloatId, result: &mut Vec<Number>) {
        if let Some(node) = self.nodes.get(&node_id) {
            for child_id in node.children() {
                self.postorder_recursive(FloatId::from(child_id), result);
            }
        }
        result.push(node_id.value());
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;

    #[test]
    fn test_tree_operations() {
        let mut tree = Tree::new();

        let node1 = Node::new(1);
        let node2 = Node::new(2);
        let node3 = Node::new(3);

        let id1 = tree.add_node(node1);
        let id2 = tree.add_node(node2);
        let id3 = tree.add_node(node3);

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
        let mut tree = Tree::new();

        // Create a simple tree: root -> [child1, child2] -> [grandchild1, grandchild2]
        let root = Node::new("root");
        let child1 = Node::new("child1");
        let child2 = Node::new("child2");
        let grandchild1 = Node::new("grandchild1");
        let grandchild2 = Node::new("grandchild2");

        let root_id = tree.add_node(root);
        let child1_id = tree.add_node(child1);
        let child2_id = tree.add_node(child2);
        let grandchild1_id = tree.add_node(grandchild1);
        let grandchild2_id = tree.add_node(grandchild2);

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
        assert_eq!(preorder_result[0], root_id);

        // Verify root is last in postorder
        assert_eq!(postorder_result[4], root_id);
    }

    #[test]
    fn test_tree_properties() {
        let mut tree = Tree::new();

        let root = Node::new("root");
        let child1 = Node::new("child1");
        let child2 = Node::new("child2");

        let root_id = tree.add_node(root);
        let child1_id = tree.add_node(child1);
        let child2_id = tree.add_node(child2);

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
        assert!(leaves.contains(&&"child1"));
        assert!(leaves.contains(&&"child2"));
    }
}
