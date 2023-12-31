use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=../allwpilib/hal/src/main/native/include/hal/Main.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_arg("-x")
        .clang_arg("c++")
        // .clang_arg("-include./allwpilib/wpimath/src/main/native/include/units/base.h")
        .clang_arg("-I./allwpilib/wpimath/src/main/native/include/units/")
        // .clang_arg("-include./allwpilib/wpiutil/src/main/native/include/wpi/SymbolExports.h")
        // .clang_arg("-include./allwpilib/wpimath/src/main/native/include/units/formatter.h")
        .clang_arg("-I./allwpilib/wpiutil/src/main/native/thirdparty/llvm/include/wpi/")
        .clang_arg("-I./allwpilib/wpiutil/src/main/native/include/wpi/")
        .clang_arg("-I./allwpilib/wpiutil/src/main/native/include/wpi/sendable/")
        .clang_arg("-I./allwpilib/wpilibc/src/main/native/include/frc/")
        .header("./allwpilib/wpilibc/src/main/native/include/frc/wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
