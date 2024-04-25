#ifndef PI_CLI_TESTING_LIBRARY_H
#define PI_CLI_TESTING_LIBRARY_H
#include "rust/cxx.h"
#include <string>

long sig_doc(rust::Str sha, rust::Str file_name, bool sign, bool cmd,
            rust::Str basicAuthUser, rust::Str basicAuthPassword,
            rust::Str applicationId);

#endif // PI_CLI_TESTING_LIBRARY_H
