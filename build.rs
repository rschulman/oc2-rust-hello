fn main() {
    println!("cargo:rustc-link-search=/usr/local/riscv64-linux-musl/lib");
    println!("cargo:rustc-link-search=/usr/local/lib/gcc/riscv64-linux-musl/9.2.0");
}
