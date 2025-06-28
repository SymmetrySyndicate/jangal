#include "node.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#define MAX_NODES 100

int visited_values[MAX_NODES];
int visit_index = 0;
int result_count = 0;

Node *build_sample_tree(void) {
  int *vals = malloc(8 * sizeof(int));
  for (int i = 0; i < 8; i++) {
    vals[i] = i + 1;
  }

  Node *root = create_node(&vals[0], 1.0);
  Node *n2 = create_node(&vals[1], 2.0);
  Node *n3 = create_node(&vals[2], 3.0);
  Node *n4 = create_node(&vals[3], 4.0);
  Node *n5 = create_node(&vals[4], 5.0);
  Node *n6 = create_node(&vals[5], 6.0);
  Node *n7 = create_node(&vals[6], 7.0);
  Node *n8 = create_node(&vals[7], 8.0);

  add_child(root, n2);
  add_child(root, n3);
  add_child(n2, n4);
  add_child(n2, n5);
  add_child(n3, n6);
  add_child(n4, n7);
  add_child(n5, n8);

  return root;
}

void record_result(Node *node) {
  if (node && node->value && visit_index < MAX_NODES) {
    visited_values[visit_index++] = *(int *)(get_node_value(node));
    result_count++;
  }
}

void reset_visited() {
  visit_index = 0;
  result_count = 0;
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

void test_nodeset() {
  printf("Testing NodeSet...\n");
  NodeSet *set = nodeset_create(2);
  int val = 10;
  Node *node = create_node(&val, 1.0);

  assert(nodeset_size(set) == 0);
  assert(nodeset_is_empty(set) == 1);

  nodeset_add(set, node);
  assert(nodeset_size(set) == 1);
  assert(nodeset_contains(set, node) == 1);

  nodeset_remove(set, node);
  assert(nodeset_size(set) == 0);

  nodeset_destroy(set);
  destroy_node(node);
  printf("NodeSet tests passed!\n");
}

void test_queue() {
  printf("Testing Queue...\n");
  NodeQueue *queue = queue_create(2);
  int val = 10;
  Node *node = create_node(&val, 1.0);

  assert(queue_is_empty(queue) == 1);

  queue_enqueue(queue, node);
  assert(queue_is_empty(queue) == 0);

  assert(queue_dequeue(queue) == node);
  assert(queue_is_empty(queue) == 1);

  queue_destroy(queue);
  destroy_node(node);
  printf("Queue tests passed!\n");
}

void test_tree_structure() {
  printf("Testing Tree Structure...\n");
  int val1 = 1, val2 = 2, val3 = 3;
  Node *root = create_node(&val1, 1.0);
  Node *child1 = create_node(&val2, 2.0);
  Node *child2 = create_node(&val3, 3.0);

  add_child(root, child1);
  add_child(root, child2);

  assert(get_node_parent(child1) == root);
  assert(nodeset_size(get_node_children(root)) == 2);
  assert(is_root(root) == 1);
  assert(is_leaf(child1) == 1);
  assert(height(root) == 1);
  assert(depth(child1) == 1);
  assert(num_nodes(root) == 3);
  assert(num_leaves(root) == 2);

  destroy_node(root);
  destroy_node(child1);
  destroy_node(child2);
  printf("Tree structure tests passed!\n");
}

void test_traversal() {
  printf("Testing all traversal methods...\n");

  // ============ dfs ============
  printf("Testing DFS traversal...\n");
  Node *root = build_sample_tree();
  reset_visited();
  dfs(root, record_result);
  int dfs_expected[] = {1, 2, 4, 7, 5, 8, 3, 6};
  assert(result_count == 8);
  assert(arrays_equal(visited_values, dfs_expected, 8));
  printf("DFS test passed!\n");

  // ============ bfs ============
  printf("Testing BFS traversal...\n");
  reset_visited();
  bfs(root, record_result);
  int bfs_expected[] = {1, 2, 3, 4, 5, 6, 7, 8};
  assert(result_count == 8);
  assert(arrays_equal(visited_values, bfs_expected, 8));
  printf("BFS test passed!\n");

  // ============ preorder ============
  printf("Testing Preorder traversal...\n");
  reset_visited();
  preorder(root, record_result);
  int preorder_expected[] = {1, 2, 4, 7, 5, 8, 3, 6};
  assert(result_count == 8);
  assert(arrays_equal(visited_values, preorder_expected, 8));
  printf("Preorder test passed!\n");

  // ============ postorder ============
  printf("Testing Postorder traversal...\n");
  reset_visited();
  postorder(root, record_result);
  int postorder_expected[] = {7, 4, 8, 5, 2, 6, 3, 1};
  assert(result_count == 8);
  assert(arrays_equal(visited_values, postorder_expected, 8));
  printf("Postorder test passed!\n");

  printf("All traversal tests passed!\n");
}

int main() {
  printf("==================\n");
  printf("Running Node tests...\n\n");

  test_nodeset();
  test_queue();
  test_tree_structure();
  test_traversal();

  printf("==================\n");
  printf("All Node tests passed!\n");
  return 0;
}
