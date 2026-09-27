#include <stddef.h>
#include "../../core-math/src/binary32/exp/expf.c"

__attribute__((flatten)) void cr_expf_row(float *values, size_t n) {
  for (size_t i = 0; i < n; i++) values[i] = cr_expf(values[i]);
}
