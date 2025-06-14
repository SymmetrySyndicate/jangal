#ifndef VEB_TREE_H
#define VEB_TREE_H

#include <stdint.h>

typedef enum { INT_TYPE, FLOAT_TYPE, DOUBLE_TYPE } ValueType;

typedef struct vEB {
  uint64_t min;
  uint64_t max;
  uint64_t size;
  struct vEB **cluster;
  struct vEB *summary;
} vEB;

vEB *create_vEB(uint64_t size);
void free_vEB(vEB *tree);

void insert(vEB *tree, void *value, ValueType type);
int isin(vEB *tree, void *value, ValueType type);
int successor(vEB *tree, void *value, ValueType type, void *result);
int predecessor(vEB *tree, void *value, ValueType type, void *result);
void delete(vEB *tree, void *value, ValueType type);

#endif /* VEB_TREE_H */
