
#include <iostream>
#include "rust/cxx.h"

int sig_doc(rust::Str path) {
  // TODO: sign documents with auth.gov
  std::cout << "Signing document: " << path << std::endl;
  return 0;
}
