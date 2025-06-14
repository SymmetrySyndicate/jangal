#include "vEB.h"
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ---------------------------------- */
static uint64_t float_to_bits(float f) {
  union {
    float f;
    uint32_t u32;
  } conv;
  conv.f = f;
  uint32_t bits = conv.u32;
  if (bits & 0x80000000) { // negative
    bits = ~bits;
  } else {
    bits ^= 0x80000000;
  }
  return (uint64_t)bits;
}

static uint64_t double_to_bits(double d) {
  union {
    double d;
    uint64_t u64;
  } conv;
  conv.d = d;
  uint64_t bits = conv.u64;
  if (bits & 0x8000000000000000ULL) { // negative
    bits = ~bits;
  } else {
    bits ^= 0x8000000000000000ULL;
  }
  return bits;
}

static uint64_t int_to_bits(int i) {
  return (uint64_t)((int64_t)i + 0x8000000000000000ULL);
}

static uint64_t standardize_input(void *value, ValueType type) {
  switch (type) {
  case INT_TYPE:
    return int_to_bits(*(int *)value);
  case FLOAT_TYPE:
    return float_to_bits(*(float *)value);
  case DOUBLE_TYPE:
    return double_to_bits(*(double *)value);
  default:
    return 0;
  }
}

static void restore_value(uint64_t x, void *out, ValueType type) {
  switch (type) {
  case INT_TYPE: {
    int64_t val = (int64_t)(x - 0x8000000000000000ULL);
    *(int *)out = (int)val;
    break;
  }
  case FLOAT_TYPE: {
    uint32_t bits = (uint32_t)x;
    if (bits & 0x80000000) {
      bits ^= 0x80000000;
    } else {
      bits = ~bits;
    }
    union {
      float f;
      uint32_t u32;
    } conv;
    conv.u32 = bits;
    *(float *)out = conv.f;
    break;
  }
  case DOUBLE_TYPE: {
    uint64_t bits = x;
    if (bits & 0x8000000000000000ULL) {
      bits ^= 0x8000000000000000ULL;
    } else {
      bits = ~bits;
    }
    union {
      double d;
      uint64_t u64;
    } conv;
    conv.u64 = bits;
    *(double *)out = conv.d;
    break;
  }
  }
}

/* ---------------------------------- */
static uint64_t high(uint64_t x, uint64_t sqrt_size) { return x / sqrt_size; }

static uint64_t low(uint64_t x, uint64_t sqrt_size) { return x % sqrt_size; }

static uint64_t veb_index(uint64_t high, uint64_t low, uint64_t sqrt_size) {
  return high * sqrt_size + low;
}

/* ---------------------------------- */
vEB *create_vEB(uint64_t size) {
  vEB *tree = malloc(sizeof(vEB));
  tree->min = UINT64_MAX;
  tree->max = 0;
  tree->size = size;

  if (size <= 2) {
    tree->cluster = NULL;
    tree->summary = NULL;
  } else {
    uint64_t sqrt_size = (uint64_t)ceil(sqrt(size));
    tree->cluster = calloc(sqrt_size, sizeof(vEB *));
    tree->summary = create_vEB(sqrt_size);
  }
  return tree;
}

static void veb_insert(vEB *tree, uint64_t x) {
  if (tree->min == UINT64_MAX) {
    tree->min = tree->max = x;
    return;
  }

  if (x < tree->min) {
    uint64_t temp = tree->min;
    tree->min = x;
    x = temp;
  }

  if (x > tree->max)
    tree->max = x;

  if (tree->size > 2) {
    uint64_t sqrt_size = (uint64_t)ceil(sqrt(tree->size));
    uint64_t h = high(x, sqrt_size);
    uint64_t l = low(x, sqrt_size);
    if (!tree->cluster[h]) {
      tree->cluster[h] = create_vEB(sqrt_size);
      veb_insert(tree->summary, h);
    }
    veb_insert(tree->cluster[h], l);
  }
}

static int veb_isin(vEB *tree, uint64_t x) {
  if (tree->min == UINT64_MAX)
    return 0;
  if (x == tree->min || x == tree->max)
    return 1;
  if (tree->size <= 2)
    return 0;

  uint64_t sqrt_size = (uint64_t)ceil(sqrt(tree->size));
  uint64_t h = high(x, sqrt_size);
  uint64_t l = low(x, sqrt_size);
  return tree->cluster[h] && veb_isin(tree->cluster[h], l);
}

static int64_t veb_successor(vEB *tree, uint64_t x) {
  if (tree->min == UINT64_MAX || x >= tree->max)
    return -1;
  if (x < tree->min)
    return tree->min;

  if (tree->size <= 2) {
    if (x == 0 && tree->max == 1)
      return 1;
    return -1;
  }

  uint64_t sqrt_size = (uint64_t)ceil(sqrt(tree->size));
  uint64_t h = high(x, sqrt_size);
  uint64_t l = low(x, sqrt_size);

  if (tree->cluster[h] && l < tree->cluster[h]->max) {
    uint64_t offset = veb_successor(tree->cluster[h], l);
    return veb_index(h, offset, sqrt_size);
  } else {
    int64_t next_cluster = veb_successor(tree->summary, h);
    if (next_cluster == -1)
      return -1;
    return veb_index(next_cluster, tree->cluster[next_cluster]->min, sqrt_size);
  }
}

static int64_t veb_predecessor(vEB *tree, uint64_t x) {
  if (tree->min == UINT64_MAX || x <= tree->min)
    return -1;
  if (x > tree->max)
    return tree->max;

  if (tree->size <= 2) {
    if (x == 1 && tree->min == 0)
      return 0;
    return -1;
  }

  uint64_t sqrt_size = (uint64_t)ceil(sqrt(tree->size));
  uint64_t h = high(x, sqrt_size);
  uint64_t l = low(x, sqrt_size);

  if (tree->cluster[h] && l > tree->cluster[h]->min) {
    uint64_t offset = veb_predecessor(tree->cluster[h], l);
    return veb_index(h, offset, sqrt_size);
  } else {
    int64_t prev_cluster = veb_predecessor(tree->summary, h);
    if (prev_cluster == -1)
      return (x > tree->min) ? tree->min : -1;
    return veb_index(prev_cluster, tree->cluster[prev_cluster]->max, sqrt_size);
  }
}

static void veb_delete(vEB *tree, uint64_t x) {
  if (tree->min == tree->max) {
    tree->min = UINT64_MAX;
    tree->max = 0;
    return;
  }

  if (tree->size <= 2) {
    if (x == 0)
      tree->min = 1;
    else
      tree->min = 0;
    tree->max = tree->min;
    return;
  }

  if (x == tree->min) {
    uint64_t next_cluster = tree->summary->min;
    uint64_t sqrt_size = (uint64_t)ceil(sqrt(tree->size));
    x = veb_index(next_cluster, tree->cluster[next_cluster]->min, sqrt_size);
    tree->min = x;
  }

  uint64_t sqrt_size = (uint64_t)ceil(sqrt(tree->size));
  uint64_t h = high(x, sqrt_size);
  uint64_t l = low(x, sqrt_size);

  veb_delete(tree->cluster[h], l);

  if (tree->cluster[h]->min == UINT64_MAX) {
    veb_delete(tree->summary, h);
    free(tree->cluster[h]);
    tree->cluster[h] = NULL;
  }

  if (x == tree->max) {
    if (tree->summary->max == 0 && tree->summary->min == UINT64_MAX) {
      tree->max = tree->min;
    } else {
      uint64_t max_cluster = tree->summary->max;
      tree->max =
          veb_index(max_cluster, tree->cluster[max_cluster]->max, sqrt_size);
    }
  }
}

void free_vEB(vEB *tree) {
  if (!tree)
    return;
  if (tree->size > 2) {
    uint64_t sqrt_size = (uint64_t)ceil(sqrt(tree->size));
    for (uint64_t i = 0; i < sqrt_size; i++) {
      if (tree->cluster[i])
        free_vEB(tree->cluster[i]);
    }
    free(tree->cluster);
    free_vEB(tree->summary);
  }
  free(tree);
}

/* -------------- Public API -------------------- */
void insert(vEB *tree, void *value, ValueType type) {
  uint64_t x = standardize_input(value, type);
  veb_insert(tree, x);
}

int isin(vEB *tree, void *value, ValueType type) {
  uint64_t x = standardize_input(value, type);
  return veb_isin(tree, x);
}

int successor(vEB *tree, void *value, ValueType type, void *result) {
  uint64_t x = standardize_input(value, type);
  int64_t succ = veb_successor(tree, x);
  if (succ == -1)
    return 0;
  restore_value(succ, result, type);
  return 1;
}

int predecessor(vEB *tree, void *value, ValueType type, void *result) {
  uint64_t x = standardize_input(value, type);
  int64_t pred = veb_predecessor(tree, x);
  if (pred == -1)
    return 0;
  restore_value(pred, result, type);
  return 1;
}

void delete(vEB *tree, void *value, ValueType type) {
  uint64_t x = standardize_input(value, type);
  veb_delete(tree, x);
}
