#ifndef NODE_H
#define NODE_H

#include <stdbool.h>
#include <stddef.h>

typedef struct Node Node;
typedef struct NodeSet NodeSet;
typedef struct NodeQueue NodeQueue;

// Structure Definitions
struct Node {
  void *value;
  double node_id;

  // Graph connections
  NodeSet *edges;
  NodeSet *incoming;
  NodeSet *outgoing;

  // Tree structure
  Node *parent;
  NodeSet *children;

  // Binary tree structure
  Node *left;
  Node *right;
};

struct NodeSet {
  Node **nodes;
  size_t size;
  size_t capacity;
};

struct NodeQueue {
  Node **nodes;
  size_t front;
  size_t rear;
  size_t capacity;
};

// Node creation and destruction
Node *node_create(void *value, double node_id);
void node_destroy(Node *node);

// Node accessors
void *node_get_value(Node *node);
Node *node_get_parent(Node *node);
NodeSet *node_get_children(Node *node);

// Graph operations
void add_edge(Node *self, Node *other, bool directed, bool bidirectional);
void add_child(Node *self, Node *child);

// Graph traversal
void dfs_traverse(Node *start, void (*callback)(Node *));
void bfs_traverse(Node *start, void (*callback)(Node *));

// Tree properties
bool is_root(Node *node);
bool is_leaf(Node *node);
int height(Node *node);
int depth(Node *node);
int num_leaves(Node *node);
int num_nodes(Node *node);
int diameter(Node *node);

// Tree traversal
void inorder(Node *node, void (*callback)(Node *));
void preorder(Node *node, void (*callback)(Node *));
void postorder(Node *node, void (*callback)(Node *));

// BST
Node *bst_insert(Node *root, void *value, double node_id,
                 int (*compare)(const void *a, const void *b));
Node *bst_search(Node *root, void *value,
                 int (*compare)(const void *a, const void *b));
Node *bst_delete(Node *root, void *value,
                 int (*compare)(const void *a, const void *b));
Node *bst_find_min(Node *root);
Node *bst_find_max(Node *root);

// NodeSet
NodeSet *nodeset_create(size_t initial_capacity);
void nodeset_destroy(NodeSet *set);
bool nodeset_contains(NodeSet *set, Node *node);
void nodeset_add(NodeSet *set, Node *node);
void nodeset_remove(NodeSet *set, Node *node);
size_t nodeset_size(NodeSet *set);
bool nodeset_is_empty(NodeSet *set);

// NodeQueue
NodeQueue *queue_create(size_t capacity);
void queue_destroy(NodeQueue *queue);
bool queue_is_empty(NodeQueue *queue);
void queue_enqueue(NodeQueue *queue, Node *node);
Node *queue_dequeue(NodeQueue *queue);

#endif // NODE_H
