#include "eidlib.h"
#include "eidlibException.h"
#include "rust/cxx.h"
#include <eidlibdefines.h>
#include <iostream>
#include <ostream>
#include <string>

using namespace eIDMW;

long sig_doc(rust::Str file_name, rust::Str sig_file, bool sign, bool cmd,
             rust::Str basicAuthUser, rust::Str basicAuthPassword,
             rust::Str applicationId) {
    std::string file_name_str = std::string(file_name);
    const char *files[] = {file_name_str.data()};

    try {
        if (cmd) {
            PTEID_CMDSignatureClient cmdClient = PTEID_CMDSignatureClient();
            PTEID_CMDSignatureClient::setCredentials(
                    std::string(basicAuthUser).data(),
                    std::string(basicAuthPassword).data(),
                    std::string(applicationId).data());

            if (sign) {
                cmdClient.SignXades(std::string(sig_file).data(), files, 1,
                                    PTEID_LEVEL_BASIC);
            }
        } else {
            PTEID_ReaderContext &readerContext =
                    PTEID_ReaderSet::instance().getReader();
            PTEID_EIDCard &card = readerContext.getEIDCard();

            std::cout << "Signing as: " << card.getID().getGivenName() << " "
                      << card.getID().getSurname() << std::endl;
            if (sign) {
                card.SignXadesT(std::string(sig_file).data(), files, 1);
            }
        }
    } catch (PTEID_Exception &e) {
        PTEID_ReleaseSDK();
        std::cerr << e.GetMessage() << std::endl;
        return e.GetError();
    }

    PTEID_ReleaseSDK();
    return 0;
}
