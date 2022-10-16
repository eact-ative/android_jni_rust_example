## problem
https://github.com/rust-lang/rust/pull/85806  
r23-beta3 and up will case `note: ld: error: unable to find library -lgcc`, use api21  
openssl complile failed when cross compile, enbable feature `openssl-sys = { version = "0.9.76", features = ["vendored"] }` and 
set path for android clang like this `export PATH=/Users/niko/Library/Android/sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/darwin-x86_64/bin/:$PATH`