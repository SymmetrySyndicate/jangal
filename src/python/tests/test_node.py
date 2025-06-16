import pytest

from src.node import Node


@pytest.fixture
def sample_tree():
    """Create a sample tree structure for testing"""
    #           1
    #          / \
    #         2   3
    #        / \   \
    #       4   5   6
    #      /     \
    #     7       8

    root = Node(1)
    node2 = Node(2)
    node3 = Node(3)
    node4 = Node(4)
    node5 = Node(5)
    node6 = Node(6)
    node7 = Node(7)
    node8 = Node(8)

    root.add_child(node2)
    root.add_child(node3)
    node2.add_child(node4)
    node2.add_child(node5)
    node3.add_child(node6)
    node4.add_child(node7)
    node5.add_child(node8)

    return {
        "root": root,
        "node2": node2,
        "node3": node3,
        "node4": node4,
        "node5": node5,
        "node6": node6,
        "node7": node7,
        "node8": node8,
    }


def test_single_node():
    """Test traversal methods on a single node"""
    root = Node(1)

    assert list(root.dfs()) == [root]
    assert list(root.bfs()) == [root]
    assert list(root.preorder()) == [root]
    assert list(root.postorder()) == [root]


def test_empty_children():
    """Test traversal methods on nodes with no children"""
    root = Node(1)
    child = Node(2)
    root.add_child(child)

    assert list(child.dfs()) == [child]
    assert list(child.bfs()) == [child]
    assert list(child.preorder()) == [child]
    assert list(child.postorder()) == [child]


def test_complex_tree_traversals(sample_tree):
    """Test traversal methods on a more complex tree structure"""
    root = sample_tree["root"]

    # Test DFS
    dfs_result = list(root.dfs())
    expected_dfs = [1, 2, 4, 7, 5, 8, 3, 6]
    assert [node.value for node in dfs_result] == expected_dfs

    # Test BFS
    bfs_result = list(root.bfs())
    expected_bfs = [1, 2, 3, 4, 5, 6, 7, 8]
    assert [node.value for node in bfs_result] == expected_bfs

    # Test Preorder
    preorder_result = list(root.preorder())
    expected_preorder = [1, 2, 4, 7, 5, 8, 3, 6]
    assert [node.value for node in preorder_result] == expected_preorder

    # Test Postorder
    postorder_result = list(root.postorder())
    expected_postorder = [7, 4, 8, 5, 2, 6, 3, 1]
    assert [node.value for node in postorder_result] == expected_postorder


def test_parent_child_relationships():
    """Test that parent-child relationships are correctly maintained"""
    root = Node(1)
    child1 = Node(2)
    child2 = Node(3)

    root.add_child(child1)
    root.add_child(child2)

    assert child1.parent == root
    assert child2.parent == root
    assert root.children == [child1, child2]

    assert len(root.children) == 2
    assert child1 in root.children
    assert child2 in root.children


def test_is_root_property():
    """Test is_root property"""
    root = Node(1)
    child = Node(2)

    assert root.is_root is True
    assert root.is_leaf is True
    assert child.is_root is True  # Initially no parent
    assert child.is_leaf is True  # Initially no children

    root.add_child(child)
    assert root.is_root is True
    assert root.is_leaf is False
    assert child.is_root is False
    assert child.is_leaf is True


def test_length_properties(sample_tree):
    """Test length properties"""
    root = Node(1)
    assert root.height == 0
    assert root.depth == 0

    child = Node(2)
    root.add_child(child)
    assert root.height == 1
    assert child.height == 0
    assert root.depth == 0
    assert child.depth == 1

    assert sample_tree["root"].height == 3
    assert sample_tree["node2"].height == 2
    assert sample_tree["node7"].height == 0
    assert sample_tree["root"].depth == 0
    assert sample_tree["node2"].depth == 1
    assert sample_tree["node7"].depth == 3


def test_num_property(sample_tree):
    root = Node(1)
    assert root.num_leaves == 1
    assert root.num_nodes == 1
    assert root.diameter == 0

    child = Node(2)
    root.add_child(child)
    assert root.num_leaves == 1
    assert root.num_nodes == 2
    assert root.diameter == 1
    assert child.num_leaves == 1
    assert child.num_nodes == 1

    child2 = Node(3)
    root.add_child(child2)
    assert root.num_leaves == 2
    assert root.num_nodes == 3
    assert root.diameter == 2
    assert child.num_leaves == 1
    assert child.num_nodes == 1
    assert child2.num_leaves == 1
    assert child2.num_nodes == 1

    assert sample_tree["root"].num_leaves == 3
    assert sample_tree["root"].diameter == 5
    assert sample_tree["root"].num_nodes == 8
    assert sample_tree["node2"].num_leaves == 2
    assert sample_tree["node7"].num_leaves == 1
    assert sample_tree["node2"].num_nodes == 5
    assert sample_tree["node4"].num_nodes == 2
    assert sample_tree["node7"].num_nodes == 1


def test_is_balanced_property():
    """Test is_balanced property"""
    #   1
    root = Node(1)
    assert root.is_balanced is True

    #    1
    #   /
    #  2
    child = Node(2)
    root.add_child(child)
    assert root.is_balanced is True

    #    1
    #   / \
    #  2   3
    child2 = Node(3)
    root.add_child(child2)
    assert root.is_balanced is True

    #     1
    #    / \
    #   2   3
    #  /
    # 4
    grandchild = Node(4)
    child.add_child(grandchild)
    assert root.is_balanced is True

    #        1
    #       / \
    #      2   3
    #     /
    #    4
    #   /
    #  5
    great_grandchild = Node(5)
    grandchild.add_child(great_grandchild)
    assert root.is_balanced is False
