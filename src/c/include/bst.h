#pragma once

typedef struct BSTNode {
    int value;
    struct BSTNode* left;
    struct BSTNode* right;
} BSTNode;

typedef struct BST {
    BSTNode* root;
    int size;
} BST;

// Func prototypes
BSTNode* create_node(int value);
void insert(BST* tree, int value);
BSTNode* search(BST* tree, int value);
void delete_node(BST* tree, int value);
void inorder(BSTNode* root);
void print_leaves(BSTNode* root);
void boundary_traversal(BST* tree);
int is_empty(BST* tree);
void free_node(BSTNode* root);
void free_tree(BST* tree);
