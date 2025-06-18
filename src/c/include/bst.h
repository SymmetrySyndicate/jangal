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

// Function declarations
BSTNode *create_node(int value);
void bst_insert(BST *tree, int value);
BSTNode *search(BST *tree, int value);
void delete_node(BST *tree, int value);
void inorder(BSTNode *root);
void inorder_capture(BSTNode *node, char *output);
void preorder_capture(BSTNode *node, char *output);
void postorder_capture(BSTNode *node, char *output);
void print_leaves(BSTNode *root);
void boundary_traversal(BSTNode *root, char *output);
int is_empty(BST *tree);
void free_node(BSTNode *root);
void free_tree(BST *tree);

#endif
