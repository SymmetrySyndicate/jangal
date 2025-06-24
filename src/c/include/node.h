#ifndef NODE_H
#define NODE_H
#include <stdbool.h>
#include <stddef.h>

typedef struct Node {
  void *value;
  double node_id;
  // Graph connections
  struct NodeSet *edges;
  struct NodeSet *incoming;
  struct NodeSet *outgoing;
  // Tree structure
  struct Node *parent;
  struct NodeSet *children;

} Node;

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

// Node creation and destruction
Node *create_node(void *value, double node_id);
void destroy_node(Node *node);

// Node accessors
void *get_node_value(Node *node);
Node *get_node_parent(Node *node);
NodeSet *get_node_children(Node *node);

// Graph operations
void add_edge(Node *self, Node *other, bool directed, bool bidirectional);
void add_child(Node *self, Node *child);

// Tree properties
bool is_root(Node *node);
bool is_leaf(Node *node);
int height(Node *node);
int depth(Node *node);
int num_leaves(Node *node);
int num_nodes(Node *node);
int diameter(Node *node);

// Traversal with callbacks
void dfs(Node *start, void (*result)(Node *));
void bfs(Node *start, void (*result)(Node *));
void preorder_node(Node *node, void (*result)(Node *));
void postorder_node(Node *node, void (*result)(Node *));
void inorder_node(Node *node, void (*result)(Node *));

// Comparison function for integers
int compare_ints(const void *a, const void *b);

// NodeSet operations
NodeSet *nodeset_create(size_t initial_capacity);
void nodeset_destroy(NodeSet *set);
bool nodeset_contains(NodeSet *set, Node *node);
void nodeset_add(NodeSet *set, Node *node);
void nodeset_remove(NodeSet *set, Node *node);
size_t nodeset_size(NodeSet *set);
bool nodeset_is_empty(NodeSet *set);

// NodeQueue operations
NodeQueue *queue_create(size_t capacity);
void queue_destroy(NodeQueue *queue);
bool queue_is_empty(NodeQueue *queue);
void queue_enqueue(NodeQueue *queue, Node *node);
Node *queue_dequeue(NodeQueue *queue);

// Helper functions for tests
Node *build_sample_tree(void);
Node *build_sample_bst(void);

#endif // NODE_H
