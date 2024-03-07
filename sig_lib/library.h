#ifndef PI_CLI_TESTING_LIBRARY_H
#define PI_CLI_TESTING_LIBRARY_H
#include "rust/cxx.h"
#include <string>

int sig_doc(rust::Str sha);

int handle_err(std::string err_msg);

#endif //PI_CLI_TESTING_LIBRARY_H
