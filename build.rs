fn main() {
    println!("cargo:rustc-link-arg-bin=Rusos=--script=src/kernel/kernel.ld");
    //println!("cargo:rerun-if-changed=src/kernel/kernel.ld");
    //println!("cargo:rustc-link-arg=-Tsrc/kernel/kernel.ld");

}
