#include "../include/bst.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

void test_is_empty() {
  BST tree = {NULL, 0};

  assert(is_empty(&tree) == 1);
  insert(&tree, 10);
  assert(is_empty(&tree) == 0);
  free_tree(&tree);
  assert(is_empty(&tree) == 1);

  printf("is_empty tests passed!\n");
}

void test_insert_search() {
  BST tree = {NULL, 0};

  insert(&tree, 10);
  insert(&tree, 5);
  insert(&tree, 15);

  assert(search(&tree, 10) != NULL);
  assert(search(&tree, 5) != NULL);
  assert(search(&tree, 15) != NULL);
  assert(search(&tree, 20) == NULL);

  free_tree(&tree);
  printf("insert and search tests passed!\n");
}

void test_inorder_traversal() {
  BST tree = {NULL, 0};

  insert(&tree, 20);
  insert(&tree, 10);
  insert(&tree, 30);

  printf("Inorder traversal (should be 10 20 30): ");
  inorder(tree.root);
  printf("\n");

  free_tree(&tree);
  printf("inorder traversal test passed!\n");
}

void test_print_leaves() {
  BST tree = {NULL, 0};

  insert(&tree, 20);
  insert(&tree, 10);
  insert(&tree, 30);
  insert(&tree, 5);
  insert(&tree, 15);

  printf("Leaf nodes (should be 5 15 30): ");
  print_leaves(tree.root);
  printf("\n");

  free_tree(&tree);
  printf("print_leaves test passed!\n");
}

void test_boundary_traversal() {
  BST tree = {NULL, 0};

  insert(&tree, 20);
  insert(&tree, 10);
  insert(&tree, 30);
  insert(&tree, 5);
  insert(&tree, 15);

  printf("Boundary traversal: ");
  boundary_traversal(&tree);
  printf("\n");

  free_tree(&tree);
  printf("boundary_traversal test passed!\n");
}

void test_delete_node() {
  BST tree = {NULL, 0};

  insert(&tree, 20);
  insert(&tree, 10);
  insert(&tree, 30);
  insert(&tree, 5);
  insert(&tree, 15);

  delete_node(&tree, 15);
  assert(search(&tree, 15) == NULL);
  delete_node(&tree, 20);
  assert(search(&tree, 20) == NULL);
  delete_node(&tree, 5);
  assert(search(&tree, 5) == NULL);

  free_tree(&tree);
  printf("delete_node test passed!\n");
}

int main() {
  printf("Running BST tests\n");

  test_is_empty();
  test_insert_search();
  test_inorder_traversal();
  test_print_leaves();
  test_boundary_traversal();
  test_delete_node();

  printf("All BST tests passed!\n");

  return 0;
}
