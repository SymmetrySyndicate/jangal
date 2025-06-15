from typing import Any, Generator, Optional

from ..node import NUMBER, Node
from .base import Tree


class BSTNode(Node):
    """Binary Search Tree Node with explicit left and right children"""

    def __init__(self, value: Any, node_id: Optional[NUMBER] = None):
        super().__init__(value, node_id)
        self._left: Optional[BSTNode] = None
        self._right: Optional[BSTNode] = None

    @property
    def left(self) -> Optional["BSTNode"]:
        return self._left

    @left.setter
    def left(self, node: Optional["BSTNode"]) -> None:
        self._left = node

    @property
    def right(self) -> Optional["BSTNode"]:
        return self._right

    @right.setter
    def right(self, node: Optional["BSTNode"]) -> None:
        self._right = node


class BST(Tree):
    """Binary Search Tree implementation"""

    def __init__(self):
        super().__init__()
        self._root: Optional[BSTNode] = None
        self._size: int = 0

    def insert(self, element: Any) -> None:
        """Insert an element into the BST"""
        if self._root is None:
            self._root = BSTNode(element)
            self._size += 1
            return

        current = self._root
        while True:
            if element < current.value:
                if current.left is None:
                    current.left = BSTNode(element)
                    self._size += 1
                    break
                else:
                    current = current.left
            elif element > current.value:
                if current.right is None:
                    current.right = BSTNode(element)
                    self._size += 1
                    break
                else:
                    current = current.right
            else:
                # Element already exists, don't insert duplicates
                break

    def search(self, element: Any) -> Optional[BSTNode]:
        """Search for an element in the BST"""
        current = self._root
        while current is not None:
            if element == current.value:
                return current
            elif element < current.value:
                current = current.left
            else:
                current = current.right
        return None

    def delete(self, element: Any) -> None:
        """Delete an element from the BST"""
        current = self.search(element)

        # Element not found
        if current is None:
            return

        # Find parent of the node to delete
        parent = None
        if current != self._root:
            temp = self._root
            while temp is not None:
                if (temp.left == current) or (temp.right == current):
                    parent = temp
                    break
                elif current.value < temp.value:
                    temp = temp.left
                else:
                    temp = temp.right

        # Case 1: Node with no children
        if current.left is None and current.right is None:
            if parent is None:
                # Deleting root node
                self._root = None
            elif parent.left == current:
                parent.left = None
            else:
                parent.right = None
            self._size -= 1

        # Case 2: Node with one child
        elif current.left is None:
            if parent is None:
                # Deleting root node
                self._root = current.right
            elif parent.left == current:
                parent.left = current.right
            else:
                parent.right = current.right
            self._size -= 1

        elif current.right is None:
            if parent is None:
                # Deleting root node
                self._root = current.left
            elif parent.left == current:
                parent.left = current.left
            else:
                parent.right = current.left
            self._size -= 1

        # Case 3: Node with two children
        else:
            # Find the inorder successor (smallest value in right subtree)
            successor_parent = current
            successor = current.right

            while successor.left is not None:
                successor_parent = successor
                successor = successor.left

            # Copy successor's value to current node
            current.value = successor.value

            # Delete the successor (which has at most one child)
            if successor_parent == current:
                successor_parent.right = successor.right
            else:
                successor_parent.left = successor.right

            self._size -= 1

    def inorder(self) -> Generator[BSTNode, None, None]:
        """Perform inorder traversal of the BST"""
        if self._root is None:
            return

        stack = []
        current = self._root

        while current is not None or stack:
            # Reach the leftmost node
            while current is not None:
                stack.append(current)
                current = current.left

            # Process current node
            current = stack.pop()
            yield current

            # Move to right subtree
            current = current.right

    def boundary_traversal(self) -> Generator[BSTNode, None, None]:
            """
            Perform boundary traversal of the BST.
            """
            if self._root is None:
                return

            if self._root.left is None and self._root.right is None:
                yield self._root
                return

            yield self._root

            left_boundary = self._get_left_boundary(self._root.left)
            for node in left_boundary:
                yield node

            leaf_nodes = list(self._get_leaf_nodes(self._root))
            for node in leaf_nodes:
                yield node

            right_boundary = self._get_right_boundary(self._root.right)
            for node in reversed(right_boundary):
                yield node

    def _get_left_boundary(self, node: Optional[BSTNode]):
            """Get left boundary nodes (excluding leaf nodes)"""
            boundary = []
            current = node

            while current is not None:
                if not self._is_leaf(current):
                    boundary.append(current)

                if current.left is not None:
                    current = current.left
                else:
                    current = current.right

            return boundary

    def _get_right_boundary(self, node: Optional[BSTNode]):
        """Get right boundary nodes (excluding leaf nodes)"""
        boundary = []
        current = node

        while current is not None:
            if not self._is_leaf(current):
                boundary.append(current)

            if current.right is not None:
                current = current.right
            else:
                current = current.left

        return boundary

    def _get_leaf_nodes(self, node: Optional[BSTNode]) -> Generator[BSTNode, None, None]:
        """Get all leaf nodes using inorder traversal"""
        if node is None:
            return

        if self._is_leaf(node):
            yield node
        else:
            yield from self._get_leaf_nodes(node.left)
            yield from self._get_leaf_nodes(node.right)

    def _is_leaf(self, node: Optional[BSTNode]) -> bool:
        """Check if a node is a leaf node"""
        return node is not None and node.left is None and node.right is None

    @property
    def root(self) -> Optional[BSTNode]:
        return self._root

    @property
    def size(self) -> int:
        return self._size

    def is_empty(self) -> bool:
        return self._root is None
