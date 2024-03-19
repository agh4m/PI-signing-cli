#ifndef PI_CLI_TESTING_LIBRARY_H
#define PI_CLI_TESTING_LIBRARY_H
#include "rust/cxx.h"
#include <string>

int sig_doc(rust::Str sha, rust::Str file_name, bool sign, bool cmd,
            rust::Str basicAuthUser, rust::Str basicAuthPassword,
            rust::Str applicationId);

int handle_err(std::string err_msg);

#endif // PI_CLI_TESTING_LIBRARY_H
