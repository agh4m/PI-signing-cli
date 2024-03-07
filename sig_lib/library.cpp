#include "eidlib.h"
#include "eidlibException.h"
#include "rust/cxx.h"
#include <iostream>

using namespace eIDMW;

int sig_doc(rust::Str path) {
  try {
    PTEID_ReaderContext &readerContext =
        PTEID_ReaderSet::instance().getReader();
    PTEID_EIDCard &card = readerContext.getEIDCard();

    std::string path_str = std::string(path);
    char *path_c = path_str.data();

    PTEID_PDFSignature signature(path_c);

  } catch (PTEID_ExNoReader) {
    PTEID_ReleaseSDK();
    std::cerr << "No reader found" << std::endl;
    return 1;
  }

  PTEID_ReleaseSDK();
  std::cout << "Signing document: " << path << std::endl;
  return 0;
}
