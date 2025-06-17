#include "../include/node.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

static int callback_count = 0;
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
  printf("Traversal tests passed!\n");
}

void test_edges() {
  int val1 = 1, val2 = 2;
  Node *node1 = node_create(&val1, 1.0);
  Node *node2 = node_create(&val2, 2.0);

  add_edge(node1, node2, 1, 0);
  assert(nodeset_contains(node1->outgoing, node2) == 1);
  assert(nodeset_contains(node2->incoming, node1) == 1);

  add_edge(node1, node2, 0, 1);
  assert(nodeset_contains(node1->edges, node2) == 1);
  assert(nodeset_contains(node2->edges, node1) == 1);

  node_destroy(node1);
  node_destroy(node2);
  printf("Edge cases passed!\n");
}

int main() {
  printf("Running Node tests\n");
  test_nodeset();
  test_queue();
  test_tree_structure();
  test_traversal();
  test_edges();
  printf("All tests passed!\n");
  return 0;
}
