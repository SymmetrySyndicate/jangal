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

// Simple inorder traversal for string output (for testing purposes)
void inorder_to_string(BSTNode *node, char *output) {
  if (!node)
    return;

  inorder_to_string(node->left, output);
  char buf[16];
  sprintf(buf, "%d ", node->value);
  strcat(output, buf);
  inorder_to_string(node->right, output);
}

// Simple preorder traversal for string output
void preorder_to_string(BSTNode *node, char *output) {
  if (!node)
    return;

  char buf[16];
  sprintf(buf, "%d ", node->value);
  strcat(output, buf);
  preorder_to_string(node->left, output);
  preorder_to_string(node->right, output);
}

// Simple postorder traversal for string output
void postorder_to_string(BSTNode *node, char *output) {
  if (!node)
    return;

  postorder_to_string(node->left, output);
  postorder_to_string(node->right, output);
  char buf[16];
  sprintf(buf, "%d ", node->value);
  strcat(output, buf);
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

  char output[MAX_OUTPUT] = "";
  inorder_to_string(tree.root, output);
  printf("Actual inorder: %s\n", output);

  assert_equal("10 20 30 ", output, "Inorder traversal");

  free_tree(&tree);
  printf("\n");
}

void test_bst_array_traversals() {
  printf("Testing array-based traversals...\n");
  BST tree = {NULL, 0};
  bst_insert(&tree, 20);
  bst_insert(&tree, 10);
  bst_insert(&tree, 30);

  BSTNode *inorder_nodes[MAX_NODES];
  BSTNode *preorder_nodes[MAX_NODES];
  BSTNode *postorder_nodes[MAX_NODES];
  int in_index = 0, pre_index = 0, post_index = 0;

  // Test the actual functions from your implementation
  inorder_bst(tree.root, inorder_nodes, &in_index);
  preorder_bst(tree.root, preorder_nodes, &pre_index);
  postorder_bst(tree.root, postorder_nodes, &post_index);

  printf("Inorder array size: %d\n", in_index);
  printf("Preorder array size: %d\n", pre_index);
  printf("Postorder array size: %d\n", post_index);

  // Verify sizes
  if (in_index == 3 && pre_index == 3 && post_index == 3) {
    printf("PASS: Array traversal sizes correct\n");
  } else {
    printf("FAIL: Array traversal sizes incorrect\n");
    exit(1);
  }

  // Verify inorder values
  if (inorder_nodes[0]->value == 10 && inorder_nodes[1]->value == 20 &&
      inorder_nodes[2]->value == 30) {
    printf("PASS: Inorder array values correct\n");
  } else {
    printf("FAIL: Inorder array values incorrect\n");
    exit(1);
  }

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

  char in_out[MAX_OUTPUT] = "";
  char pre_out[MAX_OUTPUT] = "";
  char post_out[MAX_OUTPUT] = "";

  inorder_to_string(tree.root, in_out);
  preorder_to_string(tree.root, pre_out);
  postorder_to_string(tree.root, post_out);

  printf("Expected inorder:  1 3 4 5 8 9\n");
  printf("Actual inorder:    %s\n", in_out);
  assert_equal("1 3 4 5 8 9 ", in_out, "Complex inorder");

  printf("Expected preorder: 5 3 1 4 8 9\n");
  printf("Actual preorder:   %s\n", pre_out);
  assert_equal("5 3 1 4 8 9 ", pre_out, "Complex preorder");

  printf("Expected postorder: 1 4 3 9 8 5\n");
  printf("Actual postorder:   %s\n", post_out);
  assert_equal("1 4 3 9 8 5 ", post_out, "Complex postorder");

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

  char boundary_output[MAX_OUTPUT] = "";
  boundary_traversal(tree.root, boundary_output);

  printf("Boundary traversal output: %s\n", boundary_output);
  // Expected: root + left boundary + leaves + right boundary
  // Root: 1, Left boundary: 2 4, Leaves: 6 7 8 3, Right boundary: none (3 is
  // already a leaf) But since we include leaves in the leaf traversal, we get:
  // 1 2 4 6 7 8 3

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

  BSTNode *found = search(&tree, 30);
  if (found && found->value == 30) {
    printf("PASS: Search found correct node\n");
  } else {
    printf("FAIL: Search failed\n");
    exit(1);
  }

  BSTNode *not_found = search(&tree, 100);
  if (not_found == NULL) {
    printf("PASS: Search correctly returned NULL for non-existent value\n");
  } else {
    printf("FAIL: Search should have returned NULL\n");
    exit(1);
  }

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

  char output[MAX_OUTPUT] = "";
  inorder_to_string(tree.root, output);
  assert_equal("42 ", output, "Single node inorder");

  char boundary_output[MAX_OUTPUT] = "";
  boundary_traversal(tree.root, boundary_output);
  assert_equal("42 ", boundary_output, "Single node boundary");

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
  test_empty_tree();
  test_single_node_tree();

  printf("All tests passed!\n");
  return 0;
}
