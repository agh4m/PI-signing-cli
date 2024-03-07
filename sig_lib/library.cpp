#include "eidlib.h"
#include "eidlibException.h"
#include "rust/cxx.h"
#include <iostream>
#include <string>

using namespace eIDMW;

int sig_doc(rust::Str path) {
  try {
    PTEID_ReaderContext &readerContext =
        PTEID_ReaderSet::instance().getReader();
    PTEID_EIDCard &card = readerContext.getEIDCard();

    PTEID_PDFSignature signature((char *)std::string(path).data());

  } catch (const PTEID_ExNoReader &e) {
    PTEID_ReleaseSDK();
    std::cerr << "No reader found" << std::endl;
    return 1;
  }

  PTEID_ReleaseSDK();
  std::cout << "Signing document: " << path << std::endl;
  return 0;
}
