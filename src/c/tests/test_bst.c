#include "../include/bst.h"
#include <stdio.h>

int main()
{
    BST tree = { NULL, 0 };

    printf("is_empty (should be 1): %d\n", is_empty(&tree));

    insert(&tree, 50);
    insert(&tree, 30);
    insert(&tree, 70);
    insert(&tree, 20);
    insert(&tree, 40);
    insert(&tree, 60);
    insert(&tree, 80);

    printf("is_empty (should be 0): %d\n", is_empty(&tree));

    printf("search(30) (should be non-NULL): %p\n", (void*)search(&tree, 30));

    printf("search(100) (should be NULL): %p\n", (void*)search(&tree, 100));

    printf("inorder (should be 20 30 40 50 60 70 80): ");
    inorder(tree.root);
    printf("\n");

    printf("print_leaves (should be 20 40 60 80): ");
    print_leaves(tree.root);
    printf("\n");

    printf("boundary_traversal (should print boundary nodes): ");
    boundary_traversal(&tree);

    delete_node(&tree, 20);
    delete_node(&tree, 30);
    delete_node(&tree, 50);
    printf("inorder after deletions (should be 40 60 70 80): ");
    inorder(tree.root);
    printf("\n");

    // Free tree
    free_tree(&tree);
    printf("is_empty after free (should be 1): %d\n", is_empty(&tree));

    return 0;
}
