cargo build
cargo lipo --release
cbindgen src/lib.rs -l c > librcc_ios.h