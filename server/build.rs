fn main() {
    let linker = std::env::var("DEP_EXECUTION_LINKER_SCRIPT")
        .expect("DEP_EXECUTION_LINKER_SCRIPT from execution");
    println!("cargo:rustc-link-arg-bin=iommu=-T{linker}");
    println!("cargo:rerun-if-changed={linker}");
}
