#include "eidlib.h"
#include "eidlibException.h"
#include "rust/cxx.h"
#include <iostream>
#include <ostream>
#include <string>

using namespace eIDMW;

int handle_err(std::string err_msg) {
  PTEID_ReleaseSDK();
  std::cerr << err_msg << std::endl;
  return 1;
}

int sig_doc(rust::Str sha, rust::Str file_name, bool sign, bool cmd,
            rust::Str basicAuthUser, rust::Str basicAuthPassword,
            rust::Str applicationId) {
  if (cmd) {
    PTEID_CMDSignatureClient cmdClient = PTEID_CMDSignatureClient();
    cmdClient.setCredentials(std::string(basicAuthUser).data(),
                             std::string(basicAuthPassword).data(),
                             std::string(applicationId).data());

    std::string file_name_str = std::string(file_name);
    const char *files[] = {file_name_str.data()};
    cmdClient.SignXades(std::string(file_name).data(), files, 1);

    return 0;
  } else {
    try {
      PTEID_ReaderContext &readerContext =
          PTEID_ReaderSet::instance().getReader();
      PTEID_EIDCard &card = readerContext.getEIDCard();

      PTEID_ByteArray sha_arr((unsigned char *)std::string(sha).data(), 64);
      if (sign) {
        std::string file_name_str = std::string(sha);
        const char *files[] = {file_name_str.data()};
        card.SignXadesT(std::string(file_name).data(), files, 1);
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
