#include "node.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

Node *create_node(void *value, double node_id) {
  Node *node = (Node *)malloc(sizeof(Node));
  if (!node)
    return NULL;

  node->value = value;
  node->node_id = node_id;
  node->edges = nodeset_create(4);
  node->incoming = nodeset_create(2);
  node->outgoing = nodeset_create(2);
  node->parent = NULL;
  node->children = nodeset_create(4);
  return node;
}

void destroy_node(Node *node) {
  if (node) {
    nodeset_destroy(node->edges);
    nodeset_destroy(node->incoming);
    nodeset_destroy(node->outgoing);
    nodeset_destroy(node->children);
    free(node);
  }
}

void *get_node_value(Node *node) { return node ? node->value : NULL; }

Node *get_node_parent(Node *node) { return node ? node->parent : NULL; }

NodeSet *get_node_children(Node *node) { return node ? node->children : NULL; }

void add_edge(Node *self, Node *other, bool directed, bool bidirectional) {
  if (!self || !other)
    return;
  nodeset_add(self->edges, other);
  if (directed) {
    nodeset_add(self->outgoing, other);
    nodeset_add(other->incoming, self);
  }
  if (bidirectional) {
    nodeset_add(other->edges, self);
  }
}

void add_child(Node *self, Node *child) {
  if (!self || !child)
    return;
  child->parent = self;
  nodeset_add(self->children, child);
}

bool is_root(Node *node) {
  if (!node)
    return false;
  return node->parent == NULL;
}

bool is_leaf(Node *node) {
  if (!node)
    return true;
  return nodeset_size(node->children) == 0;
}

int height(Node *node) {
  if (!node || is_leaf(node))
    return 0;
  int max = 0;
  for (size_t i = 0; i < nodeset_size(node->children); i++) {
    int h = height(node->children->nodes[i]);
    if (h > max)
      max = h;
  }
  return max + 1;
}

int depth(Node *node) {
  if (!node)
    return -1;
  int d = 0;
  while (node->parent) {
    d++;
    node = node->parent;
  }
  return d;
}

int num_leaves(Node *node) {
  if (!node)
    return 0;
  if (is_leaf(node))
    return 1;
  int count = 0;
  for (size_t i = 0; i < nodeset_size(node->children); i++) {
    count += num_leaves(node->children->nodes[i]);
  }
  return count;
}

int num_nodes(Node *node) {
  if (!node)
    return 0;
  int count = 1;
  for (size_t i = 0; i < nodeset_size(node->children); i++) {
    count += num_nodes(node->children->nodes[i]);
  }
  return count;
}

int diameter(Node *node) {
  if (!node)
    return 0;
  int max1 = 0, max2 = 0;
  for (size_t i = 0; i < nodeset_size(node->children); i++) {
    int h = height(node->children->nodes[i]);
    if (h > max1) {
      max2 = max1;
      max1 = h;
    } else if (h > max2) {
      max2 = h;
    }
  }
  int max_child_dia = 0;
  for (size_t i = 0; i < nodeset_size(node->children); i++) {
    int d = diameter(node->children->nodes[i]);
    if (d > max_child_dia)
      max_child_dia = d;
  }
  int through_node = max1 + max2 + 1;
  if (through_node > max_child_dia)
    return through_node;
  else
    return max_child_dia;
}

void dfs(Node *start, void (*result)(Node *)) {
  if (!start || !result)
    return;
  result(start);
  for (size_t i = 0; i < nodeset_size(start->children); i++) {
    dfs(start->children->nodes[i], result);
  }
}

void bfs(Node *start, void (*result)(Node *)) {
  if (!start || !result)
    return;
  NodeQueue *queue = queue_create(32);
  queue_enqueue(queue, start);
  while (!queue_is_empty(queue)) {
    Node *current = queue_dequeue(queue);
    result(current);
    for (size_t i = 0; i < nodeset_size(current->children); i++) {
      queue_enqueue(queue, current->children->nodes[i]);
    }
  }
  queue_destroy(queue);
}

void preorder_node(Node *node, void (*result)(Node *)) {
  if (!node || !result)
    return;
  result(node);
  for (size_t i = 0; i < nodeset_size(node->children); i++) {
    preorder_node(node->children->nodes[i], result);
  }
}

void postorder_node(Node *node, void (*result)(Node *)) {
  if (!node || !result)
    return;
  for (size_t i = 0; i < nodeset_size(node->children); i++) {
    postorder_node(node->children->nodes[i], result);
  }
  result(node);
}

void inorder_node(Node *node, void (*result)(Node *)) {
  if (!node || !result)
    return;

  size_t num_children = nodeset_size(node->children);

  if (num_children == 0) {
    result(node);
  } else {
    // Visit first half of children
    for (size_t i = 0; i < num_children / 2; i++) {
      inorder_node(node->children->nodes[i], result);
    }

    // Visit the node
    result(node);

    // Visit second half of children
    for (size_t i = num_children / 2; i < num_children; i++) {
      inorder_node(node->children->nodes[i], result);
    }
  }
}

// Comparison function for integers
int compare_ints(const void *a, const void *b) {
  int ia = *(const int *)a;
  int ib = *(const int *)b;
  return (ia > ib) - (ia < ib);
}

// Helper functions for building test trees
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

// NodeSet implementation
NodeSet *nodeset_create(size_t initial_capacity) {
  NodeSet *set = (NodeSet *)malloc(sizeof(NodeSet));
  if (!set)
    return NULL;

  set->nodes = (Node **)malloc(sizeof(Node *) * initial_capacity);
  if (!set->nodes) {
    free(set);
    return NULL;
  }

  set->size = 0;
  set->capacity = initial_capacity;
  return set;
}

void nodeset_destroy(NodeSet *set) {
  if (set) {
    free(set->nodes);
    free(set);
  }
}

bool nodeset_contains(NodeSet *set, Node *node) {
  if (!set || !node)
    return false;
  for (size_t i = 0; i < set->size; i++) {
    if (set->nodes[i] == node)
      return true;
  }
  return false;
}

void nodeset_add(NodeSet *set, Node *node) {
  if (!set || !node || nodeset_contains(set, node))
    return;

  if (set->size == set->capacity) {
    set->capacity *= 2;
    set->nodes = (Node **)realloc(set->nodes, set->capacity * sizeof(Node *));
    if (!set->nodes)
      return;
  }
  set->nodes[set->size++] = node;
}

void nodeset_remove(NodeSet *set, Node *node) {
  if (!set || !node)
    return;

  for (size_t i = 0; i < set->size; i++) {
    if (set->nodes[i] == node) {
      memmove(&set->nodes[i], &set->nodes[i + 1],
              sizeof(Node *) * (set->size - i - 1));
      set->size--;
      return;
    }
  }
}

size_t nodeset_size(NodeSet *set) { return set ? set->size : 0; }

bool nodeset_is_empty(NodeSet *set) { return set ? set->size == 0 : true; }

// NodeQueue implementation
NodeQueue *queue_create(size_t capacity) {
  NodeQueue *queue = (NodeQueue *)malloc(sizeof(NodeQueue));
  if (!queue)
    return NULL;

  queue->nodes = (Node **)malloc(sizeof(Node *) * capacity);
  if (!queue->nodes) {
    free(queue);
    return NULL;
  }

  queue->front = 0;
  queue->rear = 0;
  queue->capacity = capacity;
  return queue;
}

void queue_destroy(NodeQueue *queue) {
  if (queue) {
    free(queue->nodes);
    free(queue);
  }
}

bool queue_is_empty(NodeQueue *queue) {
  return queue ? queue->front == queue->rear : true;
}

void queue_enqueue(NodeQueue *queue, Node *node) {
  if (!queue || !node)
    return;

  if (queue->rear == queue->capacity) {
    queue->capacity *= 2;
    queue->nodes = realloc(queue->nodes, sizeof(Node *) * queue->capacity);
    if (!queue->nodes)
      return;
  }
  queue->nodes[queue->rear++] = node;
}

Node *queue_dequeue(NodeQueue *queue) {
  if (!queue || queue_is_empty(queue))
    return NULL;
  return queue->nodes[queue->front++];
}
