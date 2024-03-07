#include "library.h"

#include <iostream>

int sig_doc(rust::Str path) {
    // TODO: sign documents with auth.gov
    std::cout << "Signing document: " << path << std::endl;
    return 0;
}
