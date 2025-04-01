#include "vEB.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

void test_basic_operations() {
  printf("Testing vEB tree operations...\n");

  vEB *tree = create_vEB(16);
  assert(tree != NULL);
  assert(tree->min == -1);
  assert(tree->max == -1);

  insert(tree, 5);
  insert(tree, 2);
  insert(tree, 8);
  insert(tree, 15);

  assert(isin(tree, 2) == 1);
  assert(isin(tree, 5) == 1);
  assert(isin(tree, 8) == 1);
  assert(isin(tree, 15) == 1);
  assert(isin(tree, 3) == 0);
  assert(tree->min == 2);
  assert(tree->max == 15);

  assert(successor(tree, 2) == 5);
  assert(successor(tree, 5) == 8);
  assert(predecessor(tree, 15) == 8);
  assert(predecessor(tree, 8) == 5);

  delete (tree, 5);
  assert(isin(tree, 5) == 0);
  assert(successor(tree, 2) == 8);

  delete (tree, 2);
  assert(tree->min == 8);

  free_vEB(tree);
  printf("All basic tests passed!\n");
}

int main() {
  printf("Running tests");
  test_basic_operations();
  return 0;
}
