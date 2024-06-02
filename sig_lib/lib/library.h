#ifndef PI_CLI_TESTING_LIBRARY_H
#define PI_CLI_TESTING_LIBRARY_H
#include "rust/cxx.h"

long sig_doc(rust::Str file_name, rust::Str sig_file, bool sign, bool cmd,
             rust::Str basicAuthUser, rust::Str basicAuthPassword,
             rust::Str applicationId);

long verify(rust::Str sig_location);

#endif// PI_CLI_TESTING_LIBRARY_H
