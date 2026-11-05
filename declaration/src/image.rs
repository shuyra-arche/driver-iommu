//! Build coordinates owned by this provider; composition supplies its checkout prefix.

pub const BUILD: unit::ImageBuild = unit::ImageBuild {
    image: "iommu",
    manifest: "server/Cargo.toml",
    package: "iommu-server",
    bin: "iommu",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};
