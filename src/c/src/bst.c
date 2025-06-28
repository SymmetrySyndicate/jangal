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

// Fixed free_node function for BST
void free_node(BSTNode *root) {
  if (root == NULL)
    return;
  free_node(root->left);
  free_node(root->right);
  free(root);
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

// Helper function to find the minimum value node in a subtree
BSTNode *find_min(BSTNode *node) {
  while (node->left != NULL) {
    node = node->left;
  }
  return node;
}

// Helper function to delete a node recursively
BSTNode *delete_node_recursive(BSTNode *root, int value) {
  if (root == NULL) {
    return NULL;
  }

  if (value < root->value) {
    root->left = delete_node_recursive(root->left, value);
  } else if (value > root->value) {
    root->right = delete_node_recursive(root->right, value);
  } else {
    // Node to be deleted found

    // Case 1: Node with no children (leaf node)
    if (root->left == NULL && root->right == NULL) {
      free(root);
      return NULL;
    }
    // Case 2: Node with only one child
    else if (root->left == NULL) {
      BSTNode *temp = root->right;
      free(root);
      return temp;
    } else if (root->right == NULL) {
      BSTNode *temp = root->left;
      free(root);
      return temp;
    }
    // Case 3: Node with two children
    else {
      BSTNode *temp = find_min(root->right);
      root->value = temp->value;
      root->right = delete_node_recursive(root->right, temp->value);
    }
  }
  return root;
}

void delete_node(BST *tree, int value) {
  if (tree == NULL || tree->root == NULL) {
    return;
  }

  BSTNode *node_to_delete = search(tree, value);
  if (node_to_delete == NULL) {
    return;
  }

  tree->root = delete_node_recursive(tree->root, value);
  tree->size--;
}

int is_empty(BST *tree) { return tree->root == NULL; }

void free_tree(BST *tree) {
  free_node(tree->root);
  tree->root = NULL;
  tree->size = 0;
}

// Fixed array-based traversal functions
void inorder(BSTNode *node, BSTNode **output, int *index) {
  if (!node)
    return;
  inorder(node->left, output, index);
  output[(*index)++] = node;
  inorder(node->right, output, index);
}

void preorder(BSTNode *node, BSTNode **output, int *index) {
  if (!node)
    return;
  output[(*index)++] = node;
  preorder(node->left, output, index);
  preorder(node->right, output, index);
}

void postorder(BSTNode *node, BSTNode **output, int *index) {
  if (!node)
    return;
  postorder(node->left, output, index);
  postorder(node->right, output, index);
  output[(*index)++] = node;
}

// Array-based boundary traversal function
void boundary_traversal(BSTNode *root, BSTNode **output, int *index) {
  if (root == NULL) {
    return;
  }

  output[(*index)++] = root;

  if (root->left != NULL || root->right != NULL) {
    BSTNode *current = root->left;
    while (current != NULL) {
      if (current->left != NULL || current->right != NULL) {
        output[(*index)++] = current;
        if (current->left != NULL) {
          current = current->left;
        } else {
          current = current->right;
        }
      } else {
        break; // Reached a leaf
      }
    }

    BSTNode **stack = malloc(1000 * sizeof(BSTNode *));
    int stack_top = 0;
    stack[stack_top++] = root;

    while (stack_top > 0) {
      BSTNode *node = stack[--stack_top];

      if (node->right != NULL) {
        stack[stack_top++] = node->right;
      }
      if (node->left != NULL) {
        stack[stack_top++] = node->left;
      }

      if (node->left == NULL && node->right == NULL) {
        output[(*index)++] = node;
      }
    }
    free(stack);

    // Add right boundary (excluding leaves) in reverse
    current = root->right;
    BSTNode **right_boundary = malloc(1000 * sizeof(BSTNode *));
    int right_count = 0;

    while (current != NULL) {
      if (current->left != NULL || current->right != NULL) {
        right_boundary[right_count++] = current;
        if (current->right != NULL) {
          current = current->right;
        } else {
          current = current->left;
        }
      } else {
        break;
      }
    }

    for (int i = right_count - 1; i >= 0; i--) {
      output[(*index)++] = right_boundary[i];
    }
    free(right_boundary);
  }
}
