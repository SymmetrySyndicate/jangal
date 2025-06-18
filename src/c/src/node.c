#include "node.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

Node *node_create(void *value, double node_id) {
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
  node->left = NULL;
  node->right = NULL;
  return node;
}

void node_destroy(Node *node) {
  if (node) {
    nodeset_destroy(node->edges);
    nodeset_destroy(node->incoming);
    nodeset_destroy(node->outgoing);
    nodeset_destroy(node->children);
    free(node);
  }
}

void *node_get_value(Node *node) { return node ? node->value : NULL; }

Node *node_get_parent(Node *node) { return node ? node->parent : NULL; }

NodeSet *node_get_children(Node *node) { return node ? node->children : NULL; }

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

bool is_root(Node *node) { return node ? node->parent == NULL : false; }

bool is_leaf(Node *node) {
  return node ? nodeset_size(node->children) == 0 : true;
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
  return max1 + max2 + 1 > max_child_dia ? max1 + max2 + 1 : max_child_dia;
}

// BST Operations
Node *bst_insert(Node *root, void *value, double node_id,
                 int (*compare)(const void *a, const void *b)) {
  if (!root) {
    return node_create(value, node_id);
  }

  int cmp = compare(value, root->value);
  if (cmp < 0) {
    root->left = bst_insert(root->left, value, node_id, compare);
  } else if (cmp > 0) {
    root->right = bst_insert(root->right, value, node_id, compare);
  }
  // If equal, don't insert duplicate

  return root;
}

Node *bst_search(Node *root, void *value,
                 int (*compare)(const void *a, const void *b)) {
  if (!root)
    return NULL;

  int cmp = compare(value, root->value);
  if (cmp == 0)
    return root;
  else if (cmp < 0)
    return bst_search(root->left, value, compare);
  else
    return bst_search(root->right, value, compare);
}

Node *bst_find_min(Node *root) {
  if (!root)
    return NULL;
  while (root->left) {
    root = root->left;
  }
  return root;
}

Node *bst_find_max(Node *root) {
  if (!root)
    return NULL;
  while (root->right) {
    root = root->right;
  }
  return root;
}

Node *bst_delete(Node *root, void *value,
                 int (*compare)(const void *a, const void *b)) {
  if (!root)
    return NULL;

  int cmp = compare(value, root->value);
  if (cmp < 0) {
    root->left = bst_delete(root->left, value, compare);
  } else if (cmp > 0) {
    root->right = bst_delete(root->right, value, compare);
  } else {
    // Node to be deleted found
    if (!root->left) {
      Node *temp = root->right;
      node_destroy(root);
      return temp;
    } else if (!root->right) {
      Node *temp = root->left;
      node_destroy(root);
      return temp;
    }

    // Node with two children
    Node *temp = bst_find_min(root->right);
    root->value = temp->value;
    root->node_id = temp->node_id;
    root->right = bst_delete(root->right, temp->value, compare);
  }

  return root;
}

void dfs_traverse(Node *start, void (*callback)(Node *)) {
  if (!start || !callback)
    return;
  callback(start);
  for (size_t i = 0; i < nodeset_size(start->children); i++) {
    dfs_traverse(start->children->nodes[i], callback);
  }
}

void bfs_traverse(Node *start, void (*callback)(Node *)) {
  if (!start || !callback)
    return;
  NodeQueue *queue = queue_create(32);
  queue_enqueue(queue, start);
  while (!queue_is_empty(queue)) {
    Node *current = queue_dequeue(queue);
    callback(current);
    for (size_t i = 0; i < nodeset_size(current->children); i++) {
      queue_enqueue(queue, current->children->nodes[i]);
    }
  }
  queue_destroy(queue);
}

void inorder(Node *node, void (*callback)(Node *)) {
  if (!node || !callback)
    return;
  inorder(node->left, callback);
  callback(node);
  inorder(node->right, callback);
}

void preorder(Node *node, void (*callback)(Node *)) {
  if (!node || !callback)
    return;
  callback(node);
  preorder(node->left, callback);
  preorder(node->right, callback);
}

void postorder(Node *node, void (*callback)(Node *)) {
  if (!node || !callback)
    return;
  postorder(node->left, callback);
  postorder(node->right, callback);
  callback(node);
}

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
      return; // Handle realloc failure
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
      return; // Handle realloc failure
  }
  queue->nodes[queue->rear++] = node;
}

Node *queue_dequeue(NodeQueue *queue) {
  if (!queue || queue_is_empty(queue))
    return NULL;
  return queue->nodes[queue->front++];
}
