#ifndef BST_H
#define BST_H

typedef struct BSTNode {
  int value;
  struct BSTNode *left;
  struct BSTNode *right;
} BSTNode;

typedef struct {
  BSTNode *root;
  int size;
} BST;

// Node operations
BSTNode *create_node(int value);
void free_node(BSTNode *root);

// BST operations
void bst_insert(BST *tree, int value);
BSTNode *search(BST *tree, int value);
void delete_node(BST *tree, int value);
int is_empty(BST *tree);
void free_tree(BST *tree);

// Traversal functions
void inorder(BSTNode *root);
void inorder_capture(BSTNode *node, char *output);
void preorder_capture(BSTNode *node, char *output);
void postorder_capture(BSTNode *node, char *output);

// Array-based traversals
void inorder_bst(BSTNode *node, BSTNode **output, int *index);
void preorder_bst(BSTNode *node, BSTNode **output, int *index);
void postorder_bst(BSTNode *node, BSTNode **output, int *index);

// Boundary traversal functions
void print_leaves(BSTNode *root);
void add_leaves_to_output(BSTNode *root, char *output);
void add_left_boundary(BSTNode *node, char *output);
void add_right_boundary(BSTNode *node, char *output);
void boundary_traversal(BSTNode *root, char *output);

#endif
