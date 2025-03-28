fn main() {
    println!("cargo::rustc-link-search=framework=/Library/Frameworks",);
    println!("cargo::rustc-link-arg=-Wl,-rpath,/Library/Frameworks");
    println!("cargo::rustc-link-lib=framework=Syphilm");
}
