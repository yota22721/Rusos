fn main() {
    println!("cargo:rustc-link-arg-bin=Rusos=--script=src/kernel/kernel.ld");
}