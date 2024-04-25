const c = @cImport({
    @cInclude("rust/cxx.h");
    @cInclude("include/eidlib.h");
    @cInclude("include/eidlibExceptions.h");
});

export fn sig_doc() i32 {
    return 0;
}
