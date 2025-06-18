#include "node.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#define MAX_NODES 100

int visited_values[MAX_NODES];
int visit_index = 0;
int callback_count = 0;

void record_callback(Node *node) {
  visited_values[visit_index++] = *(int *)(node_get_value(node));
  callback_count++;
}

void reset_visited() {
  visit_index = 0;
  callback_count = 0;
  for (int i = 0; i < MAX_NODES; i++)
    visited_values[i] = -1;
}

int arrays_equal(int *a, int *b, int n) {
  for (int i = 0; i < n; i++) {
    if (a[i] != b[i])
      return 0;
  }
  return 1;
}

int compare_ints(const void *a, const void *b) {
  int val_a = *(const int *)a;
  int val_b = *(const int *)b;
  return (val_a > val_b) - (val_a < val_b);
}

//           1
//          / \
//         2   3
//        / \   \
//       4   5   6
//      /     \
//     7       8
Node *build_sample_tree() {
  int *vals = malloc(sizeof(int) * 8);
  for (int i = 0; i < 8; i++)
    vals[i] = i + 1;

  Node *n1 = node_create(&vals[0], 1.0);
  Node *n2 = node_create(&vals[1], 2.0);
  Node *n3 = node_create(&vals[2], 3.0);
  Node *n4 = node_create(&vals[3], 4.0);
  Node *n5 = node_create(&vals[4], 5.0);
  Node *n6 = node_create(&vals[5], 6.0);
  Node *n7 = node_create(&vals[6], 7.0);
  Node *n8 = node_create(&vals[7], 8.0);

  add_child(n1, n2);
  add_child(n1, n3);
  add_child(n2, n4);
  add_child(n2, n5);
  add_child(n3, n6);
  add_child(n4, n7);
  add_child(n5, n8);

  return n1;
}

//      4
//     / \
//    2   6
//   / \ / \
//  1  3 5  7
Node *build_sample_bst() {
  int *vals = malloc(sizeof(int) * 7);
  vals[0] = 4;
  vals[1] = 2;
  vals[2] = 6;
  vals[3] = 1;
  vals[4] = 3;
  vals[5] = 5;
  vals[6] = 7;

  Node *root = node_create(&vals[0], 4.0);
  root->left = node_create(&vals[1], 2.0);
  root->right = node_create(&vals[2], 6.0);
  root->left->left = node_create(&vals[3], 1.0);
  root->left->right = node_create(&vals[4], 3.0);
  root->right->left = node_create(&vals[5], 5.0);
  root->right->right = node_create(&vals[6], 7.0);

  return root;
}

void test_callback() { callback_count++; }

void test_nodeset() {
  NodeSet *set = nodeset_create(2);
  int val = 10;
  Node *node = node_create(&val, 1.0);

  assert(nodeset_size(set) == 0);
  assert(nodeset_is_empty(set) == 1);
  nodeset_add(set, node);
  assert(nodeset_size(set) == 1);
  assert(nodeset_contains(set, node) == 1);
  nodeset_remove(set, node);
  assert(nodeset_size(set) == 0);

  nodeset_destroy(set);
  node_destroy(node);
  printf("NodeSet tests passed!\n");
}

void test_queue() {
  NodeQueue *queue = queue_create(2);
  int val = 10;
  Node *node = node_create(&val, 1.0);

  assert(queue_is_empty(queue) == 1);
  queue_enqueue(queue, node);
  assert(queue_is_empty(queue) == 0);
  assert(queue_dequeue(queue) == node);
  assert(queue_is_empty(queue) == 1);

  queue_destroy(queue);
  node_destroy(node);
  printf("Queue tests passed!\n");
}

void test_tree_structure() {
  int val1 = 1, val2 = 2, val3 = 3;
  Node *root = node_create(&val1, 1.0);
  Node *child1 = node_create(&val2, 2.0);
  Node *child2 = node_create(&val3, 3.0);

  add_child(root, child1);
  add_child(root, child2);

  assert(node_get_parent(child1) == root);
  assert(nodeset_size(node_get_children(root)) == 2);
  assert(is_root(root) == 1);
  assert(is_leaf(child1) == 1);
  assert(height(root) == 1);
  assert(depth(child1) == 1);
  assert(num_nodes(root) == 3);
  assert(num_leaves(root) == 2);

  node_destroy(root);
  node_destroy(child1);
  node_destroy(child2);
  printf("Tree structure tests passed!\n");
}

void test_traversal() {
  int val1 = 1, val2 = 2;
  Node *root = node_create(&val1, 1.0);
  Node *child = node_create(&val2, 2.0);
  add_child(root, child);

  callback_count = 0;
  dfs_traverse(root, test_callback);
  assert(callback_count == 2);

  callback_count = 0;
  bfs_traverse(root, test_callback);
  assert(callback_count == 2);

  callback_count = 0;
  preorder(root, test_callback);
  assert(callback_count == 2);

  callback_count = 0;
  postorder(root, test_callback);
  assert(callback_count == 2);

  node_destroy(root);
  node_destroy(child);
  printf("Traversal callback count tests passed!\n");
}

void test_dfs_output() {
  Node *root = build_sample_tree();
  reset_visited();

  dfs_traverse(root, record_callback);
  int expected[] = {1, 2, 4, 7, 5, 8, 3, 6};

  assert(callback_count == 8);
  assert(arrays_equal(visited_values, expected, 8));
  printf("DFS traversal test passed!\n");

  free(root);
}

void test_bfs_output() {
  Node *root = build_sample_tree();
  reset_visited();

  bfs_traverse(root, record_callback);
  int expected[] = {1, 2, 3, 4, 5, 6, 7, 8};

  assert(callback_count == 8);
  assert(arrays_equal(visited_values, expected, 8));
  printf("BFS traversal test passed!\n");

  free(root);
}

void test_inorder_output() {
  Node *root = build_sample_bst();
  reset_visited();

  inorder(root, record_callback);
  int expected[] = {1, 2, 3, 4, 5, 6, 7};

  assert(callback_count == 7);
  assert(arrays_equal(visited_values, expected, 7));
  printf("Inorder traversal test passed!\n");

  free(root);
}

void test_preorder_output() {
  Node *root = build_sample_tree();
  reset_visited();

  preorder(root, record_callback);
  int expected[] = {1, 2, 4, 7, 5, 8, 3, 6};

  assert(callback_count == 8);
  assert(arrays_equal(visited_values, expected, 8));
  printf("Preorder traversal test passed!\n");

  free(root);
}

void test_postorder_output() {
  Node *root = build_sample_tree();
  reset_visited();

  postorder(root, record_callback);
  int expected[] = {7, 4, 8, 5, 2, 6, 3, 1};

  assert(callback_count == 8);
  assert(arrays_equal(visited_values, expected, 8));
  printf("Postorder traversal test passed!\n");

  free(root);
}

void test_bst_operations() {
  Node *root = NULL;

  // Test insertion
  int vals[] = {4, 2, 6, 1, 3, 5, 7};
  for (int i = 0; i < 7; i++) {
    root = bst_insert(root, &vals[i], (double)vals[i], compare_ints);
  }

  // Test search
  int search_val = 3;
  Node *found = bst_search(root, &search_val, compare_ints);
  assert(found != NULL);
  assert(*(int *)found->value == 3);

  // Test find min/max
  Node *min_node = bst_find_min(root);
  Node *max_node = bst_find_max(root);
  assert(*(int *)min_node->value == 1);
  assert(*(int *)max_node->value == 7);

  // Test inorder traversal gives sorted order
  reset_visited();
  inorder(root, record_callback);
  int expected[] = {1, 2, 3, 4, 5, 6, 7};
  assert(arrays_equal(visited_values, expected, 7));

  printf("BST operations test passed!\n");
}

int main() {
  printf("Running Node tests\n");
  test_nodeset();
  test_queue();
  test_tree_structure();
  test_traversal();
  test_dfs_output();
  test_bfs_output();
  test_inorder_output();
  test_preorder_output();
  test_postorder_output();
  test_bst_operations();
  printf("All tests passed!\n");
  return 0;
}
