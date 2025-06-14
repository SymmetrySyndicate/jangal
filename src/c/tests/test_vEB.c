#include "vEB.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void test_int_operations() {
  printf("Testing vEB tree operations with integers");

  vEB *tree = create_vEB(16);
  assert(tree != NULL);
  assert(tree->min == UINT64_MAX);
  assert(tree->max == 0);

  int val1 = 5, val2 = 2, val3 = 8, val4 = 15;
  insert(tree, &val1, INT_TYPE);
  insert(tree, &val2, INT_TYPE);
  insert(tree, &val3, INT_TYPE);
  insert(tree, &val4, INT_TYPE);

  assert(isin(tree, &val2, INT_TYPE) == 1);
  assert(isin(tree, &val1, INT_TYPE) == 1);
  assert(isin(tree, &val3, INT_TYPE) == 1);
  assert(isin(tree, &val4, INT_TYPE) == 1);

  int test_val = 3;
  assert(isin(tree, &test_val, INT_TYPE) == 0);

  assert(tree->min != UINT64_MAX);
  assert(tree->max != 0);

  int result;
  assert(successor(tree, &val2, INT_TYPE, &result) == 1);
  assert(result == 5);
  assert(successor(tree, &val1, INT_TYPE, &result) == 1);
  assert(result == 8);
  assert(predecessor(tree, &val4, INT_TYPE, &result) == 1);
  assert(result == 8);
  assert(predecessor(tree, &val3, INT_TYPE, &result) == 1);
  assert(result == 5);

  delete (tree, &val1, INT_TYPE);
  assert(isin(tree, &val1, INT_TYPE) == 0);
  assert(successor(tree, &val2, INT_TYPE, &result) == 1);
  assert(result == 8);

  delete (tree, &val2, INT_TYPE);

  free_vEB(tree);
  printf(" [OK]\n");
}

void test_float_operations() {
  printf("Testing vEB tree operations with floats");

  vEB *tree = create_vEB(4294967296ULL); // 2^32
  assert(tree != NULL);

  float val1 = 5.5f, val2 = 2.3f, val3 = 8.7f, val4 = 15.2f;
  insert(tree, &val1, FLOAT_TYPE);
  insert(tree, &val2, FLOAT_TYPE);
  insert(tree, &val3, FLOAT_TYPE);
  insert(tree, &val4, FLOAT_TYPE);

  assert(isin(tree, &val2, FLOAT_TYPE) == 1);
  assert(isin(tree, &val1, FLOAT_TYPE) == 1);
  assert(isin(tree, &val3, FLOAT_TYPE) == 1);
  assert(isin(tree, &val4, FLOAT_TYPE) == 1);

  float test_val = 3.0f;
  assert(isin(tree, &test_val, FLOAT_TYPE) == 0);

  float result;
  assert(successor(tree, &val2, FLOAT_TYPE, &result) == 1);
  assert(result == 5.5f);
  assert(successor(tree, &val1, FLOAT_TYPE, &result) == 1);
  assert(result == 8.7f);
  assert(predecessor(tree, &val4, FLOAT_TYPE, &result) == 1);
  assert(result == 8.7f);
  assert(predecessor(tree, &val3, FLOAT_TYPE, &result) == 1);
  assert(result == 5.5f);

  delete (tree, &val1, FLOAT_TYPE);
  assert(isin(tree, &val1, FLOAT_TYPE) == 0);
  assert(successor(tree, &val2, FLOAT_TYPE, &result) == 1);
  assert(result == 8.7f);

  delete (tree, &val2, FLOAT_TYPE);

  free_vEB(tree);
  printf(" [OK]\n");
}

void test_double_operations() {
  printf("Testing vEB tree operations with doubles");

  vEB *tree = create_vEB(18446744073709551615ULL); // 2^64-1
  assert(tree != NULL);

  double val1 = 5.5, val2 = 2.3, val3 = 8.7, val4 = 15.2;
  insert(tree, &val1, DOUBLE_TYPE);
  insert(tree, &val2, DOUBLE_TYPE);
  insert(tree, &val3, DOUBLE_TYPE);
  insert(tree, &val4, DOUBLE_TYPE);

  assert(isin(tree, &val2, DOUBLE_TYPE) == 1);
  assert(isin(tree, &val1, DOUBLE_TYPE) == 1);
  assert(isin(tree, &val3, DOUBLE_TYPE) == 1);
  assert(isin(tree, &val4, DOUBLE_TYPE) == 1);

  double test_val = 3.0;
  assert(isin(tree, &test_val, DOUBLE_TYPE) == 0);

  double result;
  assert(successor(tree, &val2, DOUBLE_TYPE, &result) == 1);
  assert(result == 5.5);
  assert(successor(tree, &val1, DOUBLE_TYPE, &result) == 1);
  assert(result == 8.7);
  assert(predecessor(tree, &val4, DOUBLE_TYPE, &result) == 1);
  assert(result == 8.7);
  assert(predecessor(tree, &val3, DOUBLE_TYPE, &result) == 1);
  assert(result == 5.5);

  delete (tree, &val1, DOUBLE_TYPE);
  assert(isin(tree, &val1, DOUBLE_TYPE) == 0);
  assert(successor(tree, &val2, DOUBLE_TYPE, &result) == 1);
  assert(result == 8.7);

  delete (tree, &val2, DOUBLE_TYPE);

  free_vEB(tree);
  printf(" [OK]\n");
}

void test_mixed_operations() {
  printf("Testing vEB tree operations with mixed data types");

  vEB *tree =
      create_vEB(18446744073709551615ULL); // 2^64-1 to accommodate all types
  assert(tree != NULL);

  int int_val = 10;
  float float_val = 10.5f;
  double double_val = 10.25;

  insert(tree, &int_val, INT_TYPE);
  insert(tree, &float_val, FLOAT_TYPE);
  insert(tree, &double_val, DOUBLE_TYPE);

  assert(isin(tree, &int_val, INT_TYPE) == 1);
  assert(isin(tree, &float_val, FLOAT_TYPE) == 1);
  assert(isin(tree, &double_val, DOUBLE_TYPE) == 1);

  // assert cross-type lookups fail
  assert(isin(tree, &int_val, FLOAT_TYPE) == 0);
  assert(isin(tree, &float_val, DOUBLE_TYPE) == 0);
  assert(isin(tree, &double_val, INT_TYPE) == 0);

  int int_val2 = 5;
  float float_val2 = 5.5f;
  double double_val2 = 15.75;

  insert(tree, &int_val2, INT_TYPE);
  insert(tree, &float_val2, FLOAT_TYPE);
  insert(tree, &double_val2, DOUBLE_TYPE);

  int int_result;
  float float_result;
  double double_result;

  // assert integer ordering
  assert(successor(tree, &int_val2, INT_TYPE, &int_result) == 1);
  assert(int_result == 10);
  assert(predecessor(tree, &int_val, INT_TYPE, &int_result) == 1);
  assert(int_result == 5);

  // assert float ordering
  assert(successor(tree, &float_val2, FLOAT_TYPE, &float_result) == 1);
  assert(float_result == 10.5f);
  assert(predecessor(tree, &float_val, FLOAT_TYPE, &float_result) == 1);
  assert(float_result == 5.5f);

  // assert double ordering
  assert(successor(tree, &double_val, DOUBLE_TYPE, &double_result) == 1);
  assert(double_result == 15.75);
  assert(predecessor(tree, &double_val2, DOUBLE_TYPE, &double_result) == 1);
  assert(double_result == 10.25);

  // assert cross-type ordering: 5 < 5.5f < 5.75
  int test_int = 5;
  float test_float = 5.5f;
  double test_double = 5.75;

  insert(tree, &test_int, INT_TYPE);
  insert(tree, &test_float, FLOAT_TYPE);
  insert(tree, &test_double, DOUBLE_TYPE);

  assert(isin(tree, &test_int, INT_TYPE) == 1);
  assert(isin(tree, &test_float, FLOAT_TYPE) == 1);
  assert(isin(tree, &test_double, DOUBLE_TYPE) == 1);

  if (successor(tree, &test_int, INT_TYPE, &int_result) == 1) {
    assert(int_result > 5);
  }

  if (successor(tree, &test_float, FLOAT_TYPE, &float_result) == 1) {
    assert(float_result > 5.5f);
  }

  if (predecessor(tree, &test_double, DOUBLE_TYPE, &double_result) == 1) {
    assert(double_result < 5.75);
  }

  // assert that the values are correctly restored to their original types
  assert(isin(tree, &int_result, INT_TYPE) == 1);
  assert(isin(tree, &float_result, FLOAT_TYPE) == 1);
  assert(isin(tree, &double_result, DOUBLE_TYPE) == 1);

  free_vEB(tree);
  printf(" [OK]\n");
}

int main() {
  printf("Running vEB tests\n");
  test_int_operations();
  test_float_operations();
  test_double_operations();
  test_mixed_operations();
  return 0;
}
