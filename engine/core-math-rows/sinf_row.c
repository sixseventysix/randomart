#include <stddef.h>
#include "../../core-math/src/binary32/sin/sinf.c"

__attribute__((flatten)) void cr_sinf_row(float *values, size_t n) {
  for (size_t i = 0; i < n; i++) values[i] = cr_sinf(values[i]);
}
