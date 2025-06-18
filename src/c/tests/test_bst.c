#include "bst.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_OUTPUT 1024

// Assertion helper
void assert_equal(const char *expected, const char *actual, const char *label) {
  if (strcmp(expected, actual) != 0) {
    printf("❌ %s failed!\nExpected: %s\nActual:   %s\n", label, expected,
           actual);
    exit(1);
  } else {
    printf("✅ %s passed.\n", label);
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

  char output[MAX_OUTPUT] = "";
  inorder_capture(tree.root, output);
  printf("Actual inorder: %s\n", output);

  assert_equal("10 20 30 ", output, "Inorder traversal");

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

  inorder_capture(tree.root, in_out);
  preorder_capture(tree.root, pre_out);
  postorder_capture(tree.root, post_out);

  printf("Expected inorder:  1 3 4 5 8 9\n");
  printf("Actual inorder:    %s\n", in_out);
  assert_equal("1 3 4 5 8 9 ", in_out, "Complex inorder");

  printf("Expected preorder: 5 3 1 4 8 9\n");
  printf("Actual preorder:   %s\n", pre_out);
  assert_equal("5 3 1 4 8 9 ", pre_out, "Complex preorder");

  printf("Expected postorder: 1 4 3 9 8 5\n");
  printf("Actual postorder:   %s\n", post_out);
  assert_equal("1 4 3 9 8 5 ", post_out, "Complex postorder");

  free_node(tree.root);
  printf("\n");
}

void test_boundary_traversal() {
  printf("Testing boundary traversal...\n");
  //         1
  //       /   \
    //      2     3
  //     / \     \
    //    4   5     7
  //       / \   /
  //      8   9 10

  BSTNode *n1 = create_node(1);
  BSTNode *n2 = create_node(2);
  BSTNode *n3 = create_node(3);
  BSTNode *n4 = create_node(4);
  BSTNode *n5 = create_node(5);
  BSTNode *n7 = create_node(7);
  BSTNode *n8 = create_node(8);
  BSTNode *n9 = create_node(9);
  BSTNode *n10 = create_node(10);

  n1->left = n2;
  n1->right = n3;
  n2->left = n4;
  n2->right = n5;
  n5->left = n8;
  n5->right = n9;
  n3->right = n7;
  n7->left = n10;

  char output[MAX_OUTPUT] = "";
  boundary_traversal(n1, output);

  assert_equal("1 2 4 8 9 10 7 3 ", output, "Boundary traversal");

  free_node(n1);
  printf("\n");
}

void test_bst_operations() {
  printf("Testing BST operations...\n");
  BST tree = {NULL, 0};

  // Test insertion
  printf("Inserting values: 50, 30, 70, 20, 40, 60, 80\n");
  bst_insert(&tree, 50);
  bst_insert(&tree, 30);
  bst_insert(&tree, 70);
  bst_insert(&tree, 20);
  bst_insert(&tree, 40);
  bst_insert(&tree, 60);
  bst_insert(&tree, 80);

  // Test search
  BSTNode *found = search(&tree, 40);
  if (found && found->value == 40) {
    printf("Search for 40: Found (value = %d)\n", found->value);
  } else {
    printf("Search for 40: Not found\n");
    exit(1);
  }

  // Test search for non-existent value
  BSTNode *not_found = search(&tree, 99);
  if (not_found == NULL) {
    printf("✅ Search for 99: Correctly not found\n");
  } else {
    printf("❌ Search for 99: Incorrectly found\n");
    exit(1);
  }

  // Test inorder traversal
  char output[MAX_OUTPUT] = "";
  inorder_capture(tree.root, output);
  printf("Expected inorder: 20 30 40 50 60 70 80\n");
  printf("Actual inorder:   %s\n", output);
  assert_equal("20 30 40 50 60 70 80 ", output, "BST inorder traversal");

  // Test deletion
  printf("Deleting node with value 30...\n");
  delete_node(&tree, 30);
  memset(output, 0, sizeof(output));
  inorder_capture(tree.root, output);
  printf("Expected after deletion: 20 40 50 60 70 80\n");
  printf("Actual after deletion:   %s\n", output);
  assert_equal("20 40 50 60 70 80 ", output, "BST after deletion");

  free_tree(&tree);
  printf("\n");
}

void test_empty_tree() {
  printf("Testing empty tree operations...\n");
  BST tree = {NULL, 0};

  if (is_empty(&tree)) {
    printf("Empty tree check: Tree is correctly identified as empty\n");
  } else {
    printf("Empty tree check: Tree should be empty\n");
    exit(1);
  }

  // Test operations on empty tree
  BSTNode *result = search(&tree, 10);
  if (result == NULL) {
    printf("Search in empty tree: Correctly returns NULL\n");
  } else {
    printf("Search in empty tree: Should return NULL\n");
    exit(1);
  }

  // Test boundary traversal on empty tree
  char output[MAX_OUTPUT] = "";
  boundary_traversal(tree.root, output);
  printf("Expected boundary traversal of empty tree: (empty string)\n");
  printf("Actual boundary traversal: '%s'\n", output);
  assert_equal("", output, "Boundary traversal of empty tree");
  printf("\n");
}

int main() {
  printf("Running BST tests...\n\n");

  test_bst_inorder();
  test_complex_traversals();
  test_boundary_traversal();
  test_bst_operations();
  test_empty_tree();

  printf("\n All tests passed!\n");
  return 0;
}
