#include "bst.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Create a new node
BSTNode *create_node(int value) {
  BSTNode *new_node = (BSTNode *)malloc(sizeof(BSTNode));
  new_node->value = value;
  new_node->left = NULL;
  new_node->right = NULL;
  return new_node;
}

void bst_insert(BST *tree, int value) {
  if (tree->root == NULL) {
    tree->root = create_node(value);
    tree->size = 1;
    return;
  }
  BSTNode *current = tree->root;
  while (1) {
    if (value < current->value) {
      if (current->left == NULL) {
        current->left = create_node(value);
        tree->size++;
        break;
      } else {
        current = current->left;
      }
    } else if (value > current->value) {
      if (current->right == NULL) {
        current->right = create_node(value);
        tree->size++;
        break;
      } else {
        current = current->right;
      }
    } else {
      // Value already exists, don't insert duplicate
      break;
    }
  }
}

BSTNode *search(BST *tree, int value) {
  BSTNode *current = tree->root;
  while (current) {
    if (current->value == value) {
      return current;
    } else if (value < current->value) {
      current = current->left;
    } else {
      current = current->right;
    }
  }
  return NULL;
}

// Fixed array-based traversal functions
void inorder_bst(BSTNode *node, BSTNode **output, int *index) {
  if (!node)
    return;
  inorder_bst(node->left, output, index);
  output[(*index)++] = node;
  inorder_bst(node->right, output, index);
}

void preorder_bst(BSTNode *node, BSTNode **output, int *index) {
  if (!node)
    return;
  output[(*index)++] = node;
  preorder_bst(node->left, output, index);
  preorder_bst(node->right, output, index);
}

void postorder_bst(BSTNode *node, BSTNode **output, int *index) {
  if (!node)
    return;
  postorder_bst(node->left, output, index);
  postorder_bst(node->right, output, index);
  output[(*index)++] = node;
}

// String capture functions for traversals
void inorder_capture(BSTNode *node, char *output) {
  if (!node)
    return;
  inorder_capture(node->left, output);
  char buf[16];
  sprintf(buf, "%d ", node->value);
  strcat(output, buf);
  inorder_capture(node->right, output);
}

void preorder_capture(BSTNode *node, char *output) {
  if (!node)
    return;
  char buf[16];
  sprintf(buf, "%d ", node->value);
  strcat(output, buf);
  preorder_capture(node->left, output);
  preorder_capture(node->right, output);
}

void postorder_capture(BSTNode *node, char *output) {
  if (!node)
    return;
  postorder_capture(node->left, output);
  postorder_capture(node->right, output);
  char buf[16];
  sprintf(buf, "%d ", node->value);
  strcat(output, buf);
}

// Helper function to add left boundary (excluding leaves)
void add_left_boundary(BSTNode *node, char *output) {
  if (node == NULL || (node->left == NULL && node->right == NULL))
    return;
  char buf[16];
  sprintf(buf, "%d ", node->value);
  strcat(output, buf);
  if (node->left)
    add_left_boundary(node->left, output);
  else
    add_left_boundary(node->right, output);
}

void add_right_boundary(BSTNode *node, char *output) {
  if (node == NULL || (node->left == NULL && node->right == NULL))
    return;
  if (node->right)
    add_right_boundary(node->right, output);
  else
    add_right_boundary(node->left, output);
  char buf[16];
  sprintf(buf, "%d ", node->value);
  strcat(output, buf);
}

// Add leaves to output (needed for boundary traversal)
void add_leaves_to_output(BSTNode *node, char *output) {
  if (node == NULL)
    return;

  if (node->left == NULL && node->right == NULL) {
    char buf[16];
    sprintf(buf, "%d ", node->value);
    strcat(output, buf);
    return;
  }

  add_leaves_to_output(node->left, output);
  add_leaves_to_output(node->right, output);
}

// Fixed boundary traversal function
void boundary_traversal(BSTNode *root, char *output) {
  if (root == NULL) {
    output[0] = '\0';
    return;
  }

  output[0] = '\0';

  // Add root
  char buf[16];
  sprintf(buf, "%d ", root->value);
  strcat(output, buf);

  // If root is not a leaf, add left boundary, leaves, and right boundary
  if (root->left != NULL || root->right != NULL) {
    // Add left boundary (excluding leaves)
    add_left_boundary(root->left, output);

    // Add leaf nodes
    add_leaves_to_output(root, output);

    // Add right boundary (excluding leaves) in reverse
    add_right_boundary(root->right, output);
  }
}

int is_empty(BST *tree) { return tree->root == NULL; }

// Fixed free_node function for BST
void free_node(BSTNode *root) {
  if (root == NULL)
    return;
  free_node(root->left);
  free_node(root->right);
  free(root);
}

void free_tree(BST *tree) {
  free_node(tree->root);
  tree->root = NULL;
  tree->size = 0;
}

// Simple traversal functions
void inorder(BSTNode *root) {
  if (root == NULL)
    return;
  inorder(root->left);
  printf("%d ", root->value);
  inorder(root->right);
}

void print_leaves(BSTNode *root) {
  if (root == NULL)
    return;
  if (root->left == NULL && root->right == NULL) {
    printf("%d ", root->value);
    return;
  }
  print_leaves(root->left);
  print_leaves(root->right);
}
