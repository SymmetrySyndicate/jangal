#ifndef BST_H
#define BST_H

typedef struct BSTNode {
  int value;
  struct BSTNode *left;
  struct BSTNode *right;
} BSTNode;

// Operations
BSTNode *create_node(int value);
void free_node(BSTNode *root);

// Traversal functions
void inorder_print(BSTNode *root);

// Array-based traversals
void inorder(BSTNode *node, BSTNode **output, int *index);
void preorder(BSTNode *node, BSTNode **output, int *index);
void postorder(BSTNode *node, BSTNode **output, int *index);

// Boundary traversal functions
void boundary_traversal(BSTNode *root, BSTNode **output, int *index);

typedef struct {
  BSTNode *root;
  int size;
} BST;

// BST operations
void bst_insert(BST *tree, int value);
BSTNode *search(BST *tree, int value);
void delete_node(BST *tree, int value);
int is_empty(BST *tree);
void free_tree(BST *tree);

#endif
