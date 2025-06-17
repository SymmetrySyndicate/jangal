#ifndef NODE_H
#define NODE_H
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Node Node;
typedef struct NodeSet NodeSet;
typedef struct NodeQueue NodeQueue;
typedef void (*NodeCallback)(Node *node);

typedef struct NodeSet {
  Node **nodes;
  size_t size;
  size_t capacity;
} NodeSet;

typedef struct NodeQueue {
  Node **nodes;
  size_t front;
  size_t rear;
  size_t capacity;
} NodeQueue;

typedef struct Node {
  void *value;
  double node_id;
  NodeSet *edges;
  NodeSet *incoming;
  NodeSet *outgoing;
  Node *parent;
  NodeSet *children;
} Node;

NodeSet *nodeset_create(size_t initial_capacity);
void nodeset_destroy(NodeSet *set);
bool nodeset_contains(NodeSet *set, Node *node);
void nodeset_add(NodeSet *set, Node *node);
void nodeset_remove(NodeSet *set, Node *node);

static inline size_t nodeset_size(NodeSet *set) { return set ? set->size : 0; }

static inline bool nodeset_is_empty(NodeSet *set) {
  return set ? (set->size == 0) : true;
}

NodeQueue *queue_create(size_t capacity);
void queue_destroy(NodeQueue *queue);
bool queue_is_empty(NodeQueue *queue);
void queue_enqueue(NodeQueue *queue, Node *node);
Node *queue_dequeue(NodeQueue *queue);

Node *node_create(void *value, double node_id);
void node_destroy(Node *node);
void add_edge(Node *self, Node *other, bool directed, bool bidirectional);
void add_child(Node *self, Node *child);

static inline Node *node_get_parent(Node *node) {
  return node ? node->parent : NULL;
}

static inline NodeSet *node_get_children(Node *node) {
  return node ? node->children : NULL;
}

static inline void *node_get_value(Node *node) {
  return node ? node->value : NULL;
}

static inline double node_get_id(Node *node) {
  return node ? node->node_id : 0.0;
}

bool is_root(Node *node);
bool is_leaf(Node *node);
int height(Node *node);
int depth(Node *node);
int num_leaves(Node *node);
int num_nodes(Node *node);
int diameter(Node *node);

void dfs_traverse(Node *start, NodeCallback callback);
void bfs_traverse(Node *start, NodeCallback callback);
void preorder(Node *node, NodeCallback callback);
void postorder(Node *node, NodeCallback callback);
void print_node(Node *node);

static inline int max(int a, int b) { return (a > b) ? a : b; }

static inline int min(int a, int b) { return (a < b) ? a : b; }

#endif // NODE_H
