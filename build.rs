use std::env;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    match env::var("CARGO_CFG_TARGET_ENV").as_deref() {
        Ok("msvc") => println!("cargo:rustc-link-arg-bin=recur=/STACK:8388608"),
        Ok("gnu") => println!("cargo:rustc-link-arg-bin=recur=-Wl,--stack,8388608"),
        _ => {}
    }
}
