#include "library.h"
#include "digidocpp/Exception.h"
#include "digidocpp/Signature.h"
#include "eidlib.h"
#include "eidlibException.h"
#include "eidlibdefines.h"
#include "rust/cxx.h"
#include <digidocpp/Container.h>
#include <iostream>
#include <ostream>
#include <string>

long sig_doc(rust::Str file_name, rust::Str sig_file, bool sign, bool cmd,
             rust::Str basicAuthUser, rust::Str basicAuthPassword,
             rust::Str applicationId) {
    std::string file_name_str = std::string(file_name);
    const char *files[] = {file_name_str.data()};

    try {

        if (cmd) {
            eIDMW::PTEID_CMDSignatureClient cmdClient = eIDMW::PTEID_CMDSignatureClient();
            eIDMW::PTEID_CMDSignatureClient::setCredentials(
                    std::string(basicAuthUser).data(),
                    std::string(basicAuthPassword).data(),
                    std::string(applicationId).data());

            if (sign) {
                cmdClient.SignXades(std::string(sig_file).data(), files, 1,
                                    eIDMW::PTEID_LEVEL_BASIC);
            }
        } else {
            eIDMW::PTEID_ReaderContext &readerContext =
                    eIDMW::PTEID_ReaderSet::instance().getReader();
            eIDMW::PTEID_EIDCard &card = readerContext.getEIDCard();

            std::cout << "Signing as: " << card.getID().getGivenName() << " "
                      << card.getID().getSurname() << std::endl;
            if (sign) {
                card.SignXades(std::string(sig_file).data(), files, 1, eIDMW::PTEID_LEVEL_TIMESTAMP);
            }
        }
    } catch (eIDMW::PTEID_Exception &e) {
        eIDMW::PTEID_ReleaseSDK();
        std::cerr << e.GetMessage() << std::endl;
        return e.GetError();
    }

    eIDMW::PTEID_ReleaseSDK();
    return 0;
}


long verify(rust::Str sig_location) {
    digidoc::initialize();

    try {
        auto doc = digidoc::Container::openPtr(std::string(sig_location));
        doc->signatures()[0]->validate();
    } catch (digidoc::Exception &e) {
        for (size_t i = 0; i < e.causes().size(); i++) {
            std::cout << e.causes()[i].msg() << std::endl;
        }
        return 1;
    }
    return 0;
}
