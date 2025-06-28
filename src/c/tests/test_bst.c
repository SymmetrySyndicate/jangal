#include "bst.h"
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NODES 100

int arrays_equal(int *a, int *b, int n) {
  for (int i = 0; i < n; i++) {
    if (a[i] != b[i]) {
      return 0;
    }
  }
  return 1;
}

void collect_values(BSTNode **nodes, int *values, int count) {
  for (int i = 0; i < count; i++) {
    values[i] = nodes[i]->value;
  }
}

void test_bst_traversals() {
  printf("Testing traversals (inorder, preorder, postorder)...\n");
  BST tree = {NULL, 0};
  bst_insert(&tree, 20);
  bst_insert(&tree, 10);
  bst_insert(&tree, 30);

  BSTNode *inorder_nodes[MAX_NODES];
  BSTNode *preorder_nodes[MAX_NODES];
  BSTNode *postorder_nodes[MAX_NODES];
  int in_index = 0, pre_index = 0, post_index = 0;

  preorder(tree.root, preorder_nodes, &pre_index);
  postorder(tree.root, postorder_nodes, &post_index);

  int inorder_values[MAX_NODES];
  int preorder_values[MAX_NODES];
  int postorder_values[MAX_NODES];

  collect_values(inorder_nodes, inorder_values, in_index);
  collect_values(preorder_nodes, preorder_values, pre_index);
  collect_values(postorder_nodes, postorder_values, post_index);

  int inorder_expected[] = {10, 20, 30};
  int preorder_expected[] = {20, 10, 30};
  int postorder_expected[] = {10, 30, 20};

  assert(in_index == 3);
  assert(pre_index == 3);
  assert(post_index == 3);

  assert(arrays_equal(inorder_values, inorder_expected, 3));
  assert(arrays_equal(preorder_values, preorder_expected, 3));
  assert(arrays_equal(postorder_values, postorder_expected, 3));

  printf("PASS: All array traversals correct\n");
  free_tree(&tree);
  printf("\n");
}

void test_boundary_traversal() {
  printf("Testing boundary traversal...\n");

  /*        1
  //       / \
  //      2   3
  //     / \
  //    4   5
  //   / \   \
  //  6   7   8
  */

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

  BSTNode *boundary_nodes[MAX_NODES];
  int boundary_index = 0;
  boundary_traversal(tree.root, boundary_nodes, &boundary_index);
  int boundary_values[MAX_NODES];
  collect_values(boundary_nodes, boundary_values, boundary_index);
  int boundary_expected[] = {1, 2, 4, 6, 7, 8, 3};
  assert(boundary_index == 7);
  assert(arrays_equal(boundary_values, boundary_expected, 7));
  printf("PASS: Boundary traversal correct\n");

  free(n6);
  free(n7);
  free(n4);
  free(n8);
  free(n5);
  free(n2);
  free(n3);
  free(n1);
}

void test_bst_operations() {
  printf("Testing BST operations...\n");
  BST tree = {NULL, 0};

  // ============ insert ============
  bst_insert(&tree, 50);
  bst_insert(&tree, 30);
  bst_insert(&tree, 70);
  bst_insert(&tree, 20);
  bst_insert(&tree, 40);
  bst_insert(&tree, 60);
  bst_insert(&tree, 80);

  // ============ search ============
  BSTNode *found = search(&tree, 30);
  assert(found != NULL && found->value == 30);
  printf("PASS: Search found correct node (30)\n");

  BSTNode *not_found = search(&tree, 100);
  assert(not_found == NULL);
  printf("PASS: Search correctly returned NULL for non-existent value (100)\n");

  // ============ delete ============
  delete_node(&tree, 20);
  delete_node(&tree, 40);
  delete_node(&tree, 60);
  delete_node(&tree, 80);

  BSTNode *deleted_node = search(&tree, 20);
  assert(deleted_node == NULL);
  printf("PASS: Search correctly returned NULL for deleted value (20)\n");

  // ============ free ============
  free_tree(&tree);
  printf("\n");
}

void test_delete_operations() {
  printf("Testing delete operations...\n");
  BST tree = {NULL, 0};

  /*        50
  //       /  \
  //      30   70
  //     / \   / \
  //    20 40 60 80
  */

  bst_insert(&tree, 50);
  bst_insert(&tree, 30);
  bst_insert(&tree, 70);
  bst_insert(&tree, 20);
  bst_insert(&tree, 40);
  bst_insert(&tree, 60);
  bst_insert(&tree, 80);

  assert(tree.size == 7);
  printf("PASS: Tree size correct after insertion (7)\n");

  // ============ delete ============
  delete_node(&tree, 20);
  assert(search(&tree, 20) == NULL);
  assert(tree.size == 6);
  printf("PASS: Deleted leaf node (20)\n");

  delete_node(&tree, 40);
  assert(search(&tree, 40) == NULL);
  assert(tree.size == 5);
  printf("PASS: Deleted node with one child (40)\n");

  delete_node(&tree, 30);
  assert(search(&tree, 30) == NULL);
  assert(tree.size == 4);
  printf("PASS: Deleted node with two children (30)\n");

  delete_node(&tree, 50);
  assert(search(&tree, 50) == NULL);
  assert(tree.size == 3);
  printf("PASS: Deleted root node (50)\n");

  delete_node(&tree, 60);
  delete_node(&tree, 70);
  delete_node(&tree, 80);
  assert(tree.size == 0);
  assert(tree.root == NULL);
  printf("PASS: Deleted all remaining nodes\n");

  delete_node(&tree, 100);
  assert(tree.size == 0);
  assert(tree.root == NULL);
  printf("PASS: Delete from empty tree handled correctly\n");

  printf("\n");
}

void test_search_edge_cases() {
  printf("Testing search edge cases...\n");
  BST tree = {NULL, 0};

  // ============ search ============
  BSTNode *result = search(&tree, 10);
  assert(result == NULL);
  printf("PASS: Search on empty tree returned NULL\n");

  bst_insert(&tree, 42);
  result = search(&tree, 42);
  assert(result != NULL && result->value == 42);
  printf("PASS: Search found single node (42)\n");

  result = search(&tree, 10);
  assert(result == NULL);
  printf("PASS: Search for non-existent value in single node tree returned "
         "NULL\n");

  bst_insert(&tree, 42);
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

  // ============ is_empty ============
  if (is_empty(&tree) == 1) {
    printf("PASS: Empty tree correctly identified\n");
  } else {
    printf("FAIL: Empty tree not correctly identified\n");
    exit(1);
  }

  // ============ search ============
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

  // ============ boundary_traversal ============
  BSTNode *boundary_nodes[MAX_NODES];
  int boundary_index = 0;
  boundary_traversal(tree.root, boundary_nodes, &boundary_index);

  assert(boundary_index == 1);
  assert(boundary_nodes[0]->value == 42);
  printf("PASS: Single node boundary traversal correct\n");

  free_tree(&tree);
  printf("\n");
}

int main() {
  printf("==================\n");
  printf("Running BST tests...\n\n");

  test_boundary_traversal();
  test_bst_operations();
  test_delete_operations();
  test_search_edge_cases();
  test_empty_tree();
  test_single_node_tree();

  printf("All BST tests passed!\n");
  printf("==================\n");
  return 0;
}
