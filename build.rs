use std::process::Command;

fn run(command: &str, args: &[&str]) {
    let status = {
        let msg = format!("Failed to run {}. \
                           Please make sure it is installed", command);
        Command::new(command)
        .args(args)
        .status()
        .expect(&msg)
    };
    if !status.success() {
        panic!("{} exited with an error.", command);
    }
}

fn main() {
    std::env::set_current_dir("lemire").unwrap();
    // Rust requires position-independent code for any static library.
    run("gcc", &["-std=c99", "-O3", "-march=native", "-fPIC", "-o", "newlines.o", "-c", "newlines.c"]);
    run("ar", &["rcs", "libnewlines.a", "newlines.o"]);
    //^ For now we are always building the static library. Ideally this should not be done if it
    //  was already built.

    println!("cargo:rustc-link-search=native=lemire");
    println!("cargo:rustc-link-lib=static=newlines");
}
