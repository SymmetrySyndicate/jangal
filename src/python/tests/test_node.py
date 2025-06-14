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
