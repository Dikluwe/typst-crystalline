fn main() {
    println!(
        "cargo:rustc-env=TYPST_BENCH_OPT_LEVEL={}",
        std::env::var("OPT_LEVEL").unwrap()
    );
}
