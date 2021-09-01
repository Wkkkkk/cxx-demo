#include "cxx-demo/include/FiBA.h"
#include <iostream>

std::unique_ptr<FiBA_SUM> create_fiba_with_sum() {
  return std::make_unique<FiBA_SUM>(Sum<uint64_t>());
}
