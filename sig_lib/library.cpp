#include "eidlib.h"
#include "eidlibException.h"
#include "rust/cxx.h"
#include <iostream>
#include <ostream>
#include <string>
// #include <string>

using namespace eIDMW;

int handle_err(std::string err_msg) {
  PTEID_ReleaseSDK();
  std::cerr << err_msg << std::endl;
  return 1;
}

int sig_doc(rust::Str sha, rust::Str file_name, bool sign, bool cmd) {
  if (cmd) {
    std::cout << "Not implemented" << std::endl;
    return 255;
  } else {
    try {
      PTEID_ReaderContext &readerContext =
          PTEID_ReaderSet::instance().getReader();
      PTEID_EIDCard &card = readerContext.getEIDCard();

      PTEID_ByteArray sha_arr((unsigned char *)std::string(sha).data(), 64);
      if (sign) {
        sha_arr.writeToFile("sha.txt");
        PTEID_ByteArray sig_sha = card.SignSHA256(sha_arr);
        sig_sha.writeToFile(std::string(file_name).data());
      } else {
        std::cout << card.getID().getGivenName() << std::endl;
        sha_arr.writeToFile(std::string(file_name).data());
      }
    } catch (const PTEID_ExNoReader &e) {
      return handle_err("No reader found");
    } catch (const PTEID_ExNoCardPresent &e) {
      return handle_err("No card present");
    } catch (const PTEID_Exception &e) {
      std::cerr << e.GetError() << std::endl;
      return handle_err("Unkown error ocurred");
    }

    PTEID_ReleaseSDK();
    std::cout << "Signing Sha256 Hash: " << sha << std::endl;
    return 0;
  }
}
