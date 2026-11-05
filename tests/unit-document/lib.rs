#[cfg(test)]
mod tests {
    use iommu_declaration::{BUILD, PROGRAM};
    use unit::{document as d, record};

    fn document() -> d::Unit<String> {
        toml_edit::de::from_str(include_str!("../../unit.toml")).unwrap()
    }

    #[test]
    fn document_preserves_the_driver_authority_and_provider_contract() {
        let declaration = document();
        declaration.validate().unwrap();
        assert!(PROGRAM.valid());
        assert_eq!(declaration.schema, 1);
        assert_eq!(declaration.id, PROGRAM.name);
        assert_eq!(declaration.loader.image, PROGRAM.loader.image);
        assert!(matches!(PROGRAM.loader.space, env::ProgramKind::Supervisor));
        assert_eq!(declaration.loader.space, d::Space::Supervisor);
        assert_eq!(
            declaration.loader.args,
            PROGRAM
                .loader
                .args
                .iter()
                .map(|arg| *arg as u64)
                .collect::<Vec<_>>()
        );
        assert_eq!(declaration.loader.stack, PROGRAM.loader.stack as u64);

        assert_eq!(
            declaration.operator.namespace.len(),
            PROGRAM.operator.namespace.len()
        );
        for (current, previous) in declaration
            .operator
            .namespace
            .iter()
            .zip(PROGRAM.operator.namespace)
        {
            assert_eq!(current.key, previous.key.name());
            assert!(matches!(
                previous.placement,
                legacy_unit::operator::Placement::Shared
            ));
            assert_eq!(current.placement, d::Placement::Shared);
            assert_eq!(current.path, previous.path);
            assert!(matches!(
                previous.scope,
                legacy_unit::operator::Scope::Subtree
            ));
            assert!(matches!(current.scope, d::Scope::Subtree));
            assert_eq!(current.ops, previous.ops.bits());
            assert_eq!(previous.permit, Some(system_api::operator::Permit::Public));
            assert_eq!(current.permit, Some(d::Permit::Public));
            assert_eq!(current.source_mark, previous.source_mark);
            assert!(previous.bindings.is_empty());
            assert!(current.bindings.is_empty());
        }
        assert!(PROGRAM.identity.aliases.is_empty() && declaration.identity.aliases.is_empty());
        assert!(PROGRAM.identity.grants.is_empty() && declaration.identity.grants.is_empty());
        assert_eq!(
            PROGRAM.control.after.unwrap(),
            [
                legacy_unit::Dependency::Bootstrap(legacy_unit::Bootstrap::Operator),
                legacy_unit::Dependency::Bootstrap(legacy_unit::Bootstrap::Identity),
                legacy_unit::Dependency::Unit("hub"),
            ]
        );
        assert_eq!(declaration.control.creation, d::Creation::Startup);
        assert_eq!(declaration.control.requires, ["hub"]);
        assert_eq!(declaration.control.after, ["hub"]);
        match (declaration.control.ready, PROGRAM.control.ready) {
            (d::Ready::Signal(current), legacy_unit::Ready::Signal(previous)) => {
                assert_eq!(current, previous.get());
                assert_eq!(current, unit::READY_MARK.get());
            }
            _ => panic!("ready policy changed"),
        }
        assert!(matches!(
            PROGRAM.control.ending,
            Some(legacy_unit::Ending::Resident)
        ));
        assert_eq!(declaration.control.ending, d::Ending::Resident);
        assert_eq!(
            declaration.control.supervision.as_deref(),
            PROGRAM.control.supervision
        );
        assert!(
            PROGRAM.control.construction.is_empty() && declaration.control.construction.is_empty()
        );
        assert!(declaration.control.groups.is_empty());
        assert!(!declaration.control.guarded);
        assert_eq!(
            declaration.control.retirement.len(),
            PROGRAM.control.retirement.len()
        );
        for (current, previous) in declaration
            .control
            .retirement
            .iter()
            .zip(PROGRAM.control.retirement)
        {
            assert_eq!(current.provider, previous.provider);
            assert_eq!(current.namespace as usize, previous.namespace);
            assert_eq!(current.face, previous.face);
        }
        assert_eq!(
            declaration.supply.bindings.len(),
            PROGRAM.supply.bindings.len()
        );
        for (current, previous) in declaration
            .supply
            .bindings
            .iter()
            .zip(PROGRAM.supply.bindings)
        {
            assert_eq!(current.source, previous.source.name());
            assert_eq!(current.resource, previous.resource.name());
            assert_eq!(current.slot, previous.slot.get());
            assert_eq!(current.permission, previous.permission.bits());
        }
    }

    #[test]
    fn binary_record_and_runtime_keep_the_manifest_supplies_and_hub_retirement() {
        let declaration = document();
        let bytes = record::encode(&declaration).unwrap();
        let decoded = record::decode(&bytes).unwrap();
        let text = toml_edit::ser::to_string(&decoded).unwrap();
        assert_eq!(
            toml_edit::de::from_str::<d::Unit<String>>(&text).unwrap(),
            declaration
        );

        static RECORD: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
        let bytes = RECORD.get_or_init(|| record::encode(&document()).unwrap());
        let native = unit::runtime::UnitFile::try_from(record::decode(bytes).unwrap()).unwrap();
        assert!(native.valid());
        assert!(matches!(native.loader.space, env::ProgramKind::Supervisor));
        assert_eq!(
            native.control.after.as_deref(),
            Some(&[unit::Dependency::Unit("hub")][..])
        );
        assert_eq!(native.control.retirement[0].provider, "hub");
        assert_eq!(native.control.retirement[0].face, "retire");
        assert_eq!(native.supply.bindings.as_slice(), PROGRAM.supply.bindings);
        assert_eq!(
            iommu_declaration::bindings::REGION_SUBSET,
            env::Permission::FETCH | env::Permission::STORE | env::Permission::ONLY
        );
    }

    #[test]
    fn image_names_the_existing_server_binary() {
        assert_eq!(document().loader.image, BUILD.image);
        assert_eq!(BUILD.manifest, "server/Cargo.toml");
        let manifest: toml_edit::DocumentMut =
            include_str!("../../server/Cargo.toml").parse().unwrap();
        assert_eq!(manifest["package"]["name"].as_str(), Some(BUILD.package));
        let bins = manifest["bin"].as_array_of_tables().unwrap();
        assert_eq!(bins.len(), 1);
        let bin = bins.get(0).unwrap();
        assert_eq!(bin["name"].as_str(), Some(BUILD.bin));
        assert_eq!(BUILD.bin, BUILD.image);
        assert!(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../server")
                .join(bin["path"].as_str().unwrap())
                .is_file()
        );
        assert!(BUILD.features.is_empty());
        assert!(BUILD.default_features);
        assert_eq!(BUILD.target, "riscv64gc-unknown-none-elf");
        assert_eq!(BUILD.profile, "release");
    }
}
