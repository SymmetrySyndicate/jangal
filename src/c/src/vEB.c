#include "vEB.h"
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

vEB *create_vEB(int size) {
  vEB *tree = (vEB *)malloc(sizeof(vEB));
  tree->min = -1;
  tree->max = -1;
  tree->size = size;

  if (size <= 2) {
    tree->cluster = NULL;
    tree->summary = NULL;
  } else {
    int sqrt_size = (int)sqrt(size);
    tree->cluster = (vEB **)malloc(sqrt_size * sizeof(vEB *));
    for (int i = 0; i < sqrt_size; i++) {
      tree->cluster[i] = NULL;
    }
    tree->summary = create_vEB(sqrt_size);
  }
  return tree;
}

void insert(vEB *tree, int x) {
  if (tree->min == -1) {
    tree->min = tree->max = x;
    return;
  }

  if (x < tree->min) {
    int temp = tree->min;
    tree->min = x;
    x = temp;
  }

  if (x > tree->max)
    tree->max = x;

  if (tree->size > 2) {
    int sqrt_size = (int)sqrt(tree->size);
    int high = x / sqrt_size;
    int low = x % sqrt_size;

    if (tree->cluster[high] == NULL) {
      tree->cluster[high] = create_vEB(sqrt_size);
      insert(tree->summary, high);
    }
    insert(tree->cluster[high], low);
  }
}

int isin(vEB *tree, int x) {
  if (tree->min == -1)
    return 0;
  if (x == tree->min || x == tree->max)
    return 1;
  if (tree->size <= 2)
    return 0;

  int sqrt_size = (int)sqrt(tree->size);
  int high = x / sqrt_size;
  int low = x % sqrt_size;

  // same min
  if (tree->cluster[high] == NULL)
    return 0;
  return isin(tree->cluster[high], low);
}

int successor(vEB *tree, int x) {
  if (tree->min == -1 || x >= tree->max) {
    return -1;
  }

  if (x < tree->min) {
    return tree->min;
  }

  if (tree->size <= 2) {
    if (x == 0 && tree->max == 1)
      return 1;
    return -1;
  }

  int sqrt_size = (int)sqrt(tree->size);
  int high = x / sqrt_size;
  int low = x % sqrt_size;

  if (tree->cluster[high] != NULL && low < tree->cluster[high]->max) {
    int offset = successor(tree->cluster[high], low);
    return high * sqrt_size + offset;
  } else {
    int next_cluster = successor(tree->summary, high);
    if (next_cluster == -1) {
      return -1;
    }
    return next_cluster * sqrt_size + tree->cluster[next_cluster]->min;
  }
}

int predecessor(vEB *tree, int x) {
  if (tree->min == -1 || x <= tree->min) {
    return -1;
  }

  if (x > tree->max) {
    return tree->max;
  }

  if (tree->size <= 2) {
    if (x == 1 && tree->min == 0)
      return 0;
    return -1;
  }

  int sqrt_size = (int)sqrt(tree->size);
  int high = x / sqrt_size;
  int low = x % sqrt_size;

  if (tree->cluster[high] != NULL && low > tree->cluster[high]->min) {
    int offset = predecessor(tree->cluster[high], low);
    return high * sqrt_size + offset;
  } else {
    int prev_cluster = predecessor(tree->summary, high);
    if (prev_cluster == -1) {
      if (x > tree->min) {
        return tree->min;
      }
      return -1;
    }
    return prev_cluster * sqrt_size + tree->cluster[prev_cluster]->max;
  }
}

void delete(vEB *tree, int x) {
  if (tree->min == -1 || !isin(tree, x))
    return;

  if (tree->min == tree->max) {
    tree->min = tree->max = -1;
    return;
  }

  if (x == tree->min) {
    if (tree->size <= 2) {
      tree->min = tree->max;
      return;
    }

    int sqrt_size = (int)sqrt(tree->size);
    int next_cluster = tree->summary->min;
    tree->min = x = next_cluster * sqrt_size + tree->cluster[next_cluster]->min;
  }

  if (tree->size > 2) {
    int sqrt_size = (int)sqrt(tree->size);
    int high = x / sqrt_size;
    int low = x % sqrt_size;

    if (tree->cluster[high] != NULL) {
      delete (tree->cluster[high], low);

      if (tree->cluster[high]->min == -1) {
        delete (tree->summary, high);
        free(tree->cluster[high]);
        tree->cluster[high] = NULL;
      }
    }
  }

  if (x == tree->max) {
    if (tree->size <= 2) {
      tree->max = tree->min;
    } else {
      int sqrt_size = (int)sqrt(tree->size);
      int max_cluster = tree->summary->max;
      tree->max = max_cluster * sqrt_size + tree->cluster[max_cluster]->max;
    }
  }
}

void free_vEB(vEB *tree) {
  if (tree == NULL)
    return;

  if (tree->size > 2) {
    int sqrt_size = (int)sqrt(tree->size);
    for (int i = 0; i < sqrt_size; i++) {
      if (tree->cluster[i] != NULL) {
        free_vEB(tree->cluster[i]);
      }
    }
    free(tree->cluster);
    free_vEB(tree->summary);
  }

  free(tree);
}
