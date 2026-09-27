fn main() {
    println!(
        "cargo:rustc-env=FONT_MICRO_OPT_LEVEL={}",
        std::env::var("OPT_LEVEL").unwrap()
    );
}
