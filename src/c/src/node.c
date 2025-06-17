#include "../include/node.h"
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

NodeSet *nodeset_create(size_t initial_capacity) {
  NodeSet *set = malloc(sizeof(NodeSet));
  set->nodes = malloc(sizeof(Node *) * initial_capacity);
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
  for (size_t i = 0; i < set->size; i++) {
    if (set->nodes[i] == node) {
      return true;
    }
  }
  return false;
}

void nodeset_add(NodeSet *set, Node *node) {
  if (nodeset_contains(set, node)) {
    return;
  }

  if (set->size >= set->capacity) {
    set->capacity *= 2;
    set->nodes = realloc(set->nodes, sizeof(Node *) * set->capacity);
  }

  set->nodes[set->size++] = node;
}

void nodeset_remove(NodeSet *set, Node *node) {
  for (size_t i = 0; i < set->size; i++) {
    if (set->nodes[i] == node) {
      for (size_t j = i; j < set->size - 1; j++) {
        set->nodes[j] = set->nodes[j + 1];
      }
      set->size--;
      return;
    }
  }
}

NodeQueue *queue_create(size_t capacity) {
  NodeQueue *queue = malloc(sizeof(NodeQueue));
  queue->nodes = malloc(sizeof(Node *) * capacity);
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

bool queue_is_empty(NodeQueue *queue) { return queue->front == queue->rear; }

void queue_enqueue(NodeQueue *queue, Node *node) {
  if ((queue->rear + 1) % queue->capacity == queue->front) {
    size_t new_capacity = queue->capacity * 2;
    Node **new_nodes = malloc(sizeof(Node *) * new_capacity);

    size_t i = 0;
    size_t current = queue->front;
    while (current != queue->rear) {
      new_nodes[i++] = queue->nodes[current];
      current = (current + 1) % queue->capacity;
    }

    free(queue->nodes);
    queue->nodes = new_nodes;
    queue->front = 0;
    queue->rear = i;
    queue->capacity = new_capacity;
  }

  queue->nodes[queue->rear] = node;
  queue->rear = (queue->rear + 1) % queue->capacity;
}

Node *queue_dequeue(NodeQueue *queue) {
  if (queue_is_empty(queue)) {
    return NULL;
  }

  Node *node = queue->nodes[queue->front];
  queue->front = (queue->front + 1) % queue->capacity;
  return node;
}

Node *node_create(void *value, double node_id) {
  Node *node = malloc(sizeof(Node));
  node->value = value;
  node->node_id = (node_id != 0.0) ? node_id : (double)(uintptr_t)node;

  node->edges = nodeset_create(4);
  node->incoming = nodeset_create(4);
  node->outgoing = nodeset_create(4);
  node->parent = NULL;
  node->children = nodeset_create(4);

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

void add_edge(Node *self, Node *other, bool directed, bool bidirectional) {
  assert(other != NULL);

  if (directed) {
    nodeset_add(self->outgoing, other);
    nodeset_add(other->incoming, self);
  } else {
    if (bidirectional) {
      nodeset_add(self->edges, other);
      nodeset_add(other->edges, self);
    } else {
      nodeset_add(self->edges, other);
    }
  }
}

void add_child(Node *self, Node *child) {
  assert(child != NULL);

  if (child->parent) {
    nodeset_remove(child->parent->children, child);
  }

  child->parent = self;
  nodeset_add(self->children, child);
}

bool is_root(Node *node) { return node->parent == NULL; }

bool is_leaf(Node *node) { return node->children->size == 0; }

int height(Node *node) {
  if (is_leaf(node)) {
    return 0;
  }

  int max_height = 0;
  for (size_t i = 0; i < node->children->size; i++) {
    int child_height = height(node->children->nodes[i]);
    if (child_height > max_height) {
      max_height = child_height;
    }
  }

  return 1 + max_height;
}

int depth(Node *node) {
  if (is_root(node)) {
    return 0;
  }

  return 1 + depth(node->parent);
}

int num_leaves(Node *node) {
  if (is_leaf(node)) {
    return 1;
  }

  int total = 0;
  for (size_t i = 0; i < node->children->size; i++) {
    total += num_leaves(node->children->nodes[i]);
  }

  return total;
}

int num_nodes(Node *node) {
  int total = 1;
  for (size_t i = 0; i < node->children->size; i++) {
    total += num_nodes(node->children->nodes[i]);
  }

  return total;
}

// Helper function for max - using a different name to avoid conflicts
static int max_int(int a, int b) { return (a > b) ? a : b; }

int diameter(Node *node) {
  if (is_leaf(node)) {
    return 0;
  }

  size_t num_children = node->children->size;

  if (num_children == 0) {
    return 0;
  } else if (num_children == 1) {
    return 1 + height(node->children->nodes[0]);
  } else {
    // Get heights of all children
    int *heights = malloc(sizeof(int) * num_children);
    for (size_t i = 0; i < num_children; i++) {
      heights[i] = height(node->children->nodes[i]);
    }

    // Sort heights in descending order (bubble sort)
    for (size_t i = 0; i < num_children - 1; i++) {
      for (size_t j = 0; j < num_children - 1 - i; j++) {
        if (heights[j] < heights[j + 1]) {
          int temp = heights[j];
          heights[j] = heights[j + 1];
          heights[j + 1] = temp;
        }
      }
    }

    int root_diameter = 2 + heights[0] + heights[1];

    int max_child_diameter = 0;
    for (size_t i = 0; i < num_children; i++) {
      int child_diameter = diameter(node->children->nodes[i]);
      if (child_diameter > max_child_diameter) {
        max_child_diameter = child_diameter;
      }
    }

    free(heights);
    return max_int(root_diameter, max_child_diameter);
  }
}

// DFS traversal with callback function
void dfs(Node *node, NodeSet *visited, void (*callback)(Node *)) {
  if (nodeset_contains(visited, node)) {
    return;
  }

  nodeset_add(visited, node);
  callback(node);

  for (size_t i = 0; i < node->children->size; i++) {
    dfs(node->children->nodes[i], visited, callback);
  }
}

void dfs_traverse(Node *start, void (*callback)(Node *)) {
  NodeSet *visited = nodeset_create(16);
  dfs(start, visited, callback);
  nodeset_destroy(visited);
}

// BFS traversal with callback function
void bfs_traverse(Node *start, void (*callback)(Node *)) {
  NodeSet *visited = nodeset_create(16);
  NodeQueue *queue = queue_create(16);

  nodeset_add(visited, start);
  queue_enqueue(queue, start);

  while (!queue_is_empty(queue)) {
    Node *current = queue_dequeue(queue);
    callback(current);

    for (size_t i = 0; i < current->children->size; i++) {
      Node *child = current->children->nodes[i];
      if (!nodeset_contains(visited, child)) {
        nodeset_add(visited, child);
        queue_enqueue(queue, child);
      }
    }
  }

  nodeset_destroy(visited);
  queue_destroy(queue);
}

// Preorder traversal
void preorder(Node *node, void (*callback)(Node *)) {
  callback(node);
  for (size_t i = 0; i < node->children->size; i++) {
    preorder(node->children->nodes[i], callback);
  }
}

// Postorder traversal
void postorder(Node *node, void (*callback)(Node *)) {
  for (size_t i = 0; i < node->children->size; i++) {
    postorder(node->children->nodes[i], callback);
  }
  callback(node);
}

void print_node(Node *node) {
  printf("Node(id=%.0f, value=%p)\n", node->node_id, node->value);
}
