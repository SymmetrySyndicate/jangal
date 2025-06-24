#include "bst.h"
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_OUTPUT 1024
#define MAX_NODES 100

// Assertion helper
void assert_equal(const char *expected, const char *actual, const char *label) {
  if (strcmp(expected, actual) != 0) {
    printf("FAIL: %s failed!\nExpected: %s\nActual:   %s\n", label, expected,
           actual);
    exit(1);
  } else {
    printf("PASS: %s passed.\n", label);
  }
}

// Helper function to compare arrays
int arrays_equal(int *a, int *b, int n) {
  for (int i = 0; i < n; i++) {
    if (a[i] != b[i]) {
      return 0;
    }
  }
  return 1;
}

// Helper function to collect values from nodes array into values array
void collect_values(BSTNode **nodes, int *values, int count) {
  for (int i = 0; i < count; i++) {
    values[i] = nodes[i]->value;
  }
}

// Tests
void test_bst_inorder() {
  printf("Testing inorder traversal...\n");
  BST tree = {NULL, 0};
  bst_insert(&tree, 20);
  bst_insert(&tree, 10);
  bst_insert(&tree, 30);

  printf("Tree structure: 20 (root), 10 (left), 30 (right)\n");
  printf("Expected inorder: 10 20 30\n");

  BSTNode *inorder_nodes[MAX_NODES];
  int in_index = 0;
  inorder_bst(tree.root, inorder_nodes, &in_index);

  // Convert nodes to values array
  int inorder_values[MAX_NODES];
  collect_values(inorder_nodes, inorder_values, in_index);

  // Expected values
  int inorder_expected[] = {10, 20, 30};

  assert(in_index == 3);
  assert(arrays_equal(inorder_values, inorder_expected, 3));

  printf("PASS: Inorder traversal correct\n");
  free_tree(&tree);
  printf("\n");
}

void test_bst_preorder() {
  printf("Testing preorder traversal...\n");
  BST tree = {NULL, 0};
  bst_insert(&tree, 20);
  bst_insert(&tree, 10);
  bst_insert(&tree, 30);

  printf("Tree structure: 20 (root), 10 (left), 30 (right)\n");
  printf("Expected preorder: 20 10 30\n");

  BSTNode *preorder_nodes[MAX_NODES];
  int pre_index = 0;
  preorder_bst(tree.root, preorder_nodes, &pre_index);

  // Convert nodes to values array
  int preorder_values[MAX_NODES];
  collect_values(preorder_nodes, preorder_values, pre_index);

  // Expected values
  int preorder_expected[] = {20, 10, 30};

  assert(pre_index == 3);
  assert(arrays_equal(preorder_values, preorder_expected, 3));

  printf("PASS: Preorder traversal correct\n");
  free_tree(&tree);
  printf("\n");
}

void test_bst_postorder() {
  printf("Testing postorder traversal...\n");
  BST tree = {NULL, 0};
  bst_insert(&tree, 20);
  bst_insert(&tree, 10);
  bst_insert(&tree, 30);

  printf("Tree structure: 20 (root), 10 (left), 30 (right)\n");
  printf("Expected postorder: 10 30 20\n");

  BSTNode *postorder_nodes[MAX_NODES];
  int post_index = 0;
  postorder_bst(tree.root, postorder_nodes, &post_index);

  // Convert nodes to values array
  int postorder_values[MAX_NODES];
  collect_values(postorder_nodes, postorder_values, post_index);

  // Expected values
  int postorder_expected[] = {10, 30, 20};

  assert(post_index == 3);
  assert(arrays_equal(postorder_values, postorder_expected, 3));

  printf("PASS: Postorder traversal correct\n");
  free_tree(&tree);
  printf("\n");
}

void test_bst_array_traversals() {
  printf("Testing all array-based traversals together...\n");
  BST tree = {NULL, 0};
  bst_insert(&tree, 20);
  bst_insert(&tree, 10);
  bst_insert(&tree, 30);

  BSTNode *inorder_nodes[MAX_NODES];
  BSTNode *preorder_nodes[MAX_NODES];
  BSTNode *postorder_nodes[MAX_NODES];
  int in_index = 0, pre_index = 0, post_index = 0;

  // Get all traversals
  inorder_bst(tree.root, inorder_nodes, &in_index);
  preorder_bst(tree.root, preorder_nodes, &pre_index);
  postorder_bst(tree.root, postorder_nodes, &post_index);

  // Convert to values arrays
  int inorder_values[MAX_NODES];
  int preorder_values[MAX_NODES];
  int postorder_values[MAX_NODES];

  collect_values(inorder_nodes, inorder_values, in_index);
  collect_values(preorder_nodes, preorder_values, pre_index);
  collect_values(postorder_nodes, postorder_values, post_index);

  // Expected values
  int inorder_expected[] = {10, 20, 30};
  int preorder_expected[] = {20, 10, 30};
  int postorder_expected[] = {10, 30, 20};

  // Verify sizes
  assert(in_index == 3);
  assert(pre_index == 3);
  assert(post_index == 3);

  // Verify values
  assert(arrays_equal(inorder_values, inorder_expected, 3));
  assert(arrays_equal(preorder_values, preorder_expected, 3));
  assert(arrays_equal(postorder_values, postorder_expected, 3));

  printf("PASS: All array traversals correct\n");
  free_tree(&tree);
  printf("\n");
}
void test_complex_traversals() {
  printf("Testing complex traversals...\n");
  //        5
  //       / \
    //      3   8
  //     / \   \
    //    1   4   9

  BSTNode *n1 = create_node(1);
  BSTNode *n4 = create_node(4);
  BSTNode *n3 = create_node(3);
  n3->left = n1;
  n3->right = n4;

  BSTNode *n9 = create_node(9);
  BSTNode *n8 = create_node(8);
  n8->right = n9;

  BSTNode *n5 = create_node(5);
  n5->left = n3;
  n5->right = n8;

  BST tree = {n5, 6};

  printf("Tree structure:\n");
  printf("        5\n");
  printf("       / \\\n");
  printf("      3   8\n");
  printf("     / \\   \\\n");
  printf("    1   4   9\n\n");

  // Test inorder traversal (BST)
  BSTNode *inorder_nodes[MAX_NODES];
  int in_index = 0;
  inorder_bst(tree.root, inorder_nodes, &in_index);

  int inorder_values[MAX_NODES];
  collect_values(inorder_nodes, inorder_values, in_index);

  int inorder_expected[] = {1, 3, 4, 5, 8, 9};
  printf("Expected inorder: 1 3 4 5 8 9\n");
  printf("Actual inorder: ");
  for (int i = 0; i < in_index; i++) {
    printf("%d ", inorder_values[i]);
  }
  printf("\n");
  assert(in_index == 6);
  assert(arrays_equal(inorder_values, inorder_expected, 6));
  printf("PASS: Complex inorder traversal correct\n");

  // Test preorder traversal (BST)
  BSTNode *preorder_nodes[MAX_NODES];
  int pre_index = 0;
  preorder_bst(tree.root, preorder_nodes, &pre_index);

  int preorder_values[MAX_NODES];
  collect_values(preorder_nodes, preorder_values, pre_index);

  int preorder_expected[] = {5, 3, 1, 4, 8, 9};
  printf("Expected preorder: 5 3 1 4 8 9\n");
  printf("Actual preorder: ");
  for (int i = 0; i < pre_index; i++) {
    printf("%d ", preorder_values[i]);
  }
  printf("\n");
  assert(pre_index == 6);
  assert(arrays_equal(preorder_values, preorder_expected, 6));
  printf("PASS: Complex preorder traversal correct\n");

  // Test postorder traversal (BST)
  BSTNode *postorder_nodes[MAX_NODES];
  int post_index = 0;
  postorder_bst(tree.root, postorder_nodes, &post_index);

  int postorder_values[MAX_NODES];
  collect_values(postorder_nodes, postorder_values, post_index);

  int postorder_expected[] = {1, 4, 3, 9, 8, 5};
  printf("Expected postorder: 1 4 3 9 8 5\n");
  printf("Actual postorder: ");
  for (int i = 0; i < post_index; i++) {
    printf("%d ", postorder_values[i]);
  }
  printf("\n");
  assert(post_index == 6);
  assert(arrays_equal(postorder_values, postorder_expected, 6));
  printf("PASS: Complex postorder traversal correct\n");

  // Clean up manually created tree
  free(n1);
  free(n4);
  free(n3);
  free(n9);
  free(n8);
  free(n5);
  printf("\n");
}

void test_boundary_traversal() {
  printf("Testing boundary traversal...\n");
  //        1
  //       / \
    //      2   3
  //     / \
    //    4   5
  //   / \   \
    //  6   7   8

  BSTNode *n6 = create_node(6);
  BSTNode *n7 = create_node(7);
  BSTNode *n4 = create_node(4);
  n4->left = n6;
  n4->right = n7;

  BSTNode *n8 = create_node(8);
  BSTNode *n5 = create_node(5);
  n5->right = n8;

  BSTNode *n2 = create_node(2);
  n2->left = n4;
  n2->right = n5;

  BSTNode *n3 = create_node(3);

  BSTNode *n1 = create_node(1);
  n1->left = n2;
  n1->right = n3;

  BST tree = {n1, 8};

  // Use array-based boundary traversal
  BSTNode *boundary_nodes[MAX_NODES];
  int boundary_index = 0;
  boundary_traversal_bst(tree.root, boundary_nodes, &boundary_index);

  // Convert nodes to values array
  int boundary_values[MAX_NODES];
  collect_values(boundary_nodes, boundary_values, boundary_index);

  int boundary_expected[] = {1, 2, 4, 6, 7, 8, 3};

  assert(boundary_index == 7);
  assert(arrays_equal(boundary_values, boundary_expected, 7));
  printf("PASS: Boundary traversal correct\n");

  // Clean up
  free(n6);
  free(n7);
  free(n4);
  free(n8);
  free(n5);
  free(n2);
  free(n3);
  free(n1);
  printf("PASS: Boundary traversal completed\n\n");
}

void test_inorder_bst() {
  printf("Testing inorder_bst function...\n");
  BST tree = {NULL, 0};
  bst_insert(&tree, 15);
  bst_insert(&tree, 10);
  bst_insert(&tree, 20);
  bst_insert(&tree, 8);
  bst_insert(&tree, 12);

  BSTNode *nodes[MAX_NODES];
  int index = 0;
  inorder_bst(tree.root, nodes, &index);

  if (index == 5 && nodes[0]->value == 8 && nodes[1]->value == 10 &&
      nodes[2]->value == 12 && nodes[3]->value == 15 && nodes[4]->value == 20) {
    printf("PASS: inorder_bst function correct\n");
  } else {
    printf("FAIL: inorder_bst function incorrect\n");
    exit(1);
  }

  free_tree(&tree);
  printf("\n");
}

void test_preorder_bst() {
  printf("Testing preorder_bst function...\n");
  BST tree = {NULL, 0};
  bst_insert(&tree, 15);
  bst_insert(&tree, 10);
  bst_insert(&tree, 20);

  BSTNode *nodes[MAX_NODES];
  int index = 0;
  preorder_bst(tree.root, nodes, &index);

  if (index == 3 && nodes[0]->value == 15 && nodes[1]->value == 10 &&
      nodes[2]->value == 20) {
    printf("PASS: preorder_bst function correct\n");
  } else {
    printf("FAIL: preorder_bst function incorrect\n");
    exit(1);
  }

  free_tree(&tree);
  printf("\n");
}

void test_bst_operations() {
  printf("Testing BST operations...\n");
  BST tree = {NULL, 0};

  // Test insertion and search
  bst_insert(&tree, 50);
  bst_insert(&tree, 30);
  bst_insert(&tree, 70);
  bst_insert(&tree, 20);
  bst_insert(&tree, 40);
  bst_insert(&tree, 60);
  bst_insert(&tree, 80);

  // Test search for existing values
  BSTNode *found = search(&tree, 30);
  assert(found != NULL && found->value == 30);
  printf("PASS: Search found correct node (30)\n");

  found = search(&tree, 50);
  assert(found != NULL && found->value == 50);
  printf("PASS: Search found correct node (50)\n");

  found = search(&tree, 80);
  assert(found != NULL && found->value == 80);
  printf("PASS: Search found correct node (80)\n");

  // Test search for non-existent values
  BSTNode *not_found = search(&tree, 100);
  assert(not_found == NULL);
  printf("PASS: Search correctly returned NULL for non-existent value (100)\n");

  not_found = search(&tree, 25);
  assert(not_found == NULL);
  printf("PASS: Search correctly returned NULL for non-existent value (25)\n");

  free_tree(&tree);
  printf("\n");
}

void test_delete_operations() {
  printf("Testing delete operations...\n");
  BST tree = {NULL, 0};

  // Build a tree:       50
  //                   /  \
  //                  30   70
  //                 / \   / \
  //                20 40 60 80

  bst_insert(&tree, 50);
  bst_insert(&tree, 30);
  bst_insert(&tree, 70);
  bst_insert(&tree, 20);
  bst_insert(&tree, 40);
  bst_insert(&tree, 60);
  bst_insert(&tree, 80);

  assert(tree.size == 7);
  printf("PASS: Tree size correct after insertion (7)\n");

  // Test deleting a leaf node
  delete_node(&tree, 20);
  assert(search(&tree, 20) == NULL);
  assert(tree.size == 6);
  printf("PASS: Deleted leaf node (20)\n");

  // Test deleting a node with one child
  delete_node(&tree, 40);
  assert(search(&tree, 40) == NULL);
  assert(tree.size == 5);
  printf("PASS: Deleted node with one child (40)\n");

  // Test deleting a node with two children
  delete_node(&tree, 30);
  assert(search(&tree, 30) == NULL);
  assert(tree.size == 4);
  printf("PASS: Deleted node with two children (30)\n");

  // Test deleting the root
  delete_node(&tree, 50);
  assert(search(&tree, 50) == NULL);
  assert(tree.size == 3);
  printf("PASS: Deleted root node (50)\n");

  // Test deleting remaining nodes
  delete_node(&tree, 60);
  delete_node(&tree, 70);
  delete_node(&tree, 80);
  assert(tree.size == 0);
  assert(tree.root == NULL);
  printf("PASS: Deleted all remaining nodes\n");

  // Test deleting from empty tree
  delete_node(&tree, 100);
  assert(tree.size == 0);
  assert(tree.root == NULL);
  printf("PASS: Delete from empty tree handled correctly\n");

  printf("\n");
}

void test_search_edge_cases() {
  printf("Testing search edge cases...\n");
  BST tree = {NULL, 0};

  // Test search on empty tree
  BSTNode *result = search(&tree, 10);
  assert(result == NULL);
  printf("PASS: Search on empty tree returned NULL\n");

  // Test search on single node tree
  bst_insert(&tree, 42);
  result = search(&tree, 42);
  assert(result != NULL && result->value == 42);
  printf("PASS: Search found single node (42)\n");

  result = search(&tree, 10);
  assert(result == NULL);
  printf("PASS: Search for non-existent value in single node tree returned "
         "NULL\n");

  bst_insert(&tree, 42); // This should not be inserted due to duplicate
  result = search(&tree, 42);
  assert(result != NULL && result->value == 42);
  printf(
      "PASS: Search found existing value after duplicate insertion attempt\n");

  free_tree(&tree);
  printf("\n");
}

void test_empty_tree() {
  printf("Testing empty tree operations...\n");
  BST tree = {NULL, 0};

  // Test that tree is correctly identified as empty
  if (is_empty(&tree) == 1) {
    printf("PASS: Empty tree correctly identified\n");
  } else {
    printf("FAIL: Empty tree not correctly identified\n");
    exit(1);
  }

  // Test search operation on empty tree
  BSTNode *result = search(&tree, 10);
  if (result == NULL) {
    printf("PASS: Search on empty tree returned NULL\n");
  } else {
    printf("FAIL: Search on empty tree should return NULL\n");
    exit(1);
  }

  printf("Empty tree tests passed!\n\n");
}

void test_single_node_tree() {
  printf("Testing single node tree...\n");
  BST tree = {NULL, 0};
  bst_insert(&tree, 42);

  if (tree.size == 1) {
    printf("PASS: Single node tree size correct\n");
  } else {
    printf("FAIL: Single node tree size incorrect\n");
    exit(1);
  }

  // Test array-based boundary traversal for single node
  BSTNode *boundary_nodes[MAX_NODES];
  int boundary_index = 0;
  boundary_traversal_bst(tree.root, boundary_nodes, &boundary_index);

  assert(boundary_index == 1);
  assert(boundary_nodes[0]->value == 42);
  printf("PASS: Single node boundary traversal correct\n");

  free_tree(&tree);
  printf("\n");
}

int main() {
  printf("Running BST tests...\n\n");

  test_bst_inorder();
  test_bst_array_traversals();
  test_complex_traversals();
  test_boundary_traversal();
  test_inorder_bst();
  test_preorder_bst();
  test_bst_operations();
  test_delete_operations();
  test_search_edge_cases();
  test_empty_tree();
  test_single_node_tree();

  printf("All tests passed!\n");
  return 0;
}
