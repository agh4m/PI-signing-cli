pub mod blockchain;
pub mod communication;
pub mod util;
mod test;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("disa_lib/lib/library.h");

        // file_name is the path of the file that contains the hashes
        // sig_file is the path of the file that will contain the signature
        fn sig_doc(
            file_name: &str,
            sig_file: &str,
            sign: bool,
            cmd: bool,
            basic_auth_user: &str,
            basicAuthPassword: &str,
            applicationID: &str,
        ) -> i64;
    }
}
