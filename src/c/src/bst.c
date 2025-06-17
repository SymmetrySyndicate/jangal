#include "../include/bst.h"
#include <stdio.h>
#include <stdlib.h>

// Forward declarations
void print_leaves(BSTNode* root);

// Create a new node
BSTNode* create_node(int value) {
    BSTNode* new_node = (BSTNode*)malloc(sizeof(BSTNode));
    new_node->value = value;
    new_node->left = NULL;
    new_node->right = NULL;
    return new_node;
}

void bst_insert(BST* tree, int value) {
    if (tree->root == NULL) {
        tree->root = create_node(value);
        tree->size++;
        return;
    }

    BSTNode* current = tree->root;

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
            // Already exists
            break;
        }
    }
}

BSTNode* search(BST* tree, int value) {
    BSTNode* current = tree->root;

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

void delete_node(BST* tree, int value) {
    BSTNode** current = &tree->root;

    while (*current) {
        if (value == (*current)->value) {
            break;
        }
        if (value < (*current)->value) {
            current = &(*current)->left;
        } else {
            current = &(*current)->right;
        }
    }

    if (*current == NULL) {
        return;
    }

    BSTNode* to_delete = *current;

    if (to_delete->left == NULL && to_delete->right == NULL) {
        *current = NULL;
        free(to_delete);
        tree->size--;
    } else if (to_delete->left == NULL) {
        *current = to_delete->right;
        free(to_delete);
        tree->size--;
    } else if (to_delete->right == NULL) {
        *current = to_delete->left;
        free(to_delete);
        tree->size--;
    } else {
        BSTNode* successor_parent = to_delete;
        BSTNode* successor = to_delete->right;

        while (successor->left) {
            successor_parent = successor;
            successor = successor->left;
        }

        to_delete->value = successor->value;

        if (successor_parent == to_delete) {
            successor_parent->right = successor->right;
        } else {
            successor_parent->left = successor->right;
        }
        free(successor);
        tree->size--;
    }
}

void inorder(BSTNode* root) {
    if (root == NULL) return;

    inorder(root->left);
    printf("%d ", root->value);
    inorder(root->right);
}

void print_leaves(BSTNode* root) {
    if (root == NULL) return;

    print_leaves(root->left);
    if (root->left == NULL && root->right == NULL) {
        printf("%d ", root->value);
    }
    print_leaves(root->right);
}

void boundary_traversal(BST* tree) {
    if (tree->root == NULL) return;

    // print root
    printf("%d ", tree->root->value);

    // left boundary
    BSTNode* current = tree->root->left;
    while (current) {
        if (current->left || current->right) {
            printf("%d ", current->value);
        }
        if (current->left) current = current->left;
        else current = current->right;
    }

    // leaf nodes (inorder)
    print_leaves(tree->root);

    // right boundary (in reverse)
    BSTNode* stack[100];
    int top = 0;

    current = tree->root->right;

    while (current) {
        if (current->left || current->right) {
            stack[top++] = current;
        }
        if (current->right) current = current->right;
        else current = current->left;
    }

    while (top > 0) {
        current = stack[--top];
        printf("%d ", current->value);
    }
    printf("\n");

}

int is_empty(BST* tree) {
    return tree->root == NULL;
}

void free_node(BSTNode* root) {
    if (root == NULL) return;

    free_node(root->left);
    free_node(root->right);
    free(root);
}

void free_tree(BST* tree) {
    free_node(tree->root);
    tree->root = NULL;
    tree->size = 0;
}
