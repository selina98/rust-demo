#include <stdlib.h>

int main() {
  int* p1 = (int*) malloc(sizeof(int));
  
  *p1 = 99;
  free(p1);

  *p1 = 100000; // BAD - undefined behavior

  return 0;
}
