#ifndef VEB_TREE_H
#define VEB_TREE_H

typedef struct vEB {
  int min;
  int max;
  struct vEB **cluster;
  struct vEB *summary;
  int size;
} vEB;

vEB *create_vEB(int size);
void insert(vEB *tree, int x);
int isin(vEB *tree, int x);
int successor(vEB *tree, int x);
int predecessor(vEB *tree, int x);
void delete(vEB *tree, int x);
void free_vEB(vEB *tree);

#endif /* VEB_TREE_H */
