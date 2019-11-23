use ritual::cli;
use ritual::common::cpp_build_config::{CppBuildConfigData, CppLibraryType};
use ritual::common::errors::{FancyUnwrap, Result};
use ritual::common::target;
use ritual::config::{
    Config, CrateDependencyKind, CrateDependencySource, CrateProperties, GlobalConfig,
};
use ritual::cpp_data::CppPath;
use ritual::rust_info::RustPathScope;
use ritual::rust_type::RustPath;
use std::env;
use std::path::PathBuf;

fn create_config() -> Result<Config> {
    let mut crate_properties = CrateProperties::new("clipper", "0.1.0");
    // Add a dependency on a ritual-based `cpp_std` crate to support working with
    // types like `std::vector`.
    crate_properties.add_dependency(
        "cpp_std",
        CrateDependencyKind::Ritual,
        CrateDependencySource::CratesIo {
            version: "0.1.1".into(),
        },
    )?;
    let mut config = Config::new(crate_properties);
    config.set_cpp_lib_version("6.4.2");

    config
        .set_crate_template_path(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("crate_template"));

    // These options are used by the C++ parser to scan through
    // the API of your library.
    config.add_include_directive("clipper.hpp");
    config.add_target_include_path("/usr/local/include/polyclipping");

    // Link to your C++ library (used by cpp_checker and the build script of
    // the generated crate).
    let mut data = CppBuildConfigData::new();
    data.add_linked_lib("polyclipping");
    data.set_library_type(CppLibraryType::Static);
    config
        .cpp_build_config_mut()
        .add(target::Condition::True, data);

    // Set up a hook to ignore `ClipperLib` C++ namespace when generating the Rust API.
    let namespace = CppPath::from_good_str("ClipperLib");
    config.set_rust_path_scope_hook(move |path| {
        if path == &namespace {
            return Ok(Some(RustPathScope {
                path: RustPath::from_good_str("clipper"),
                prefix: None,
            }));
        }
        Ok(None)
    });

    Ok(config)
}

fn main() {
    let mut config = GlobalConfig::new();
    // Only one crate is targeted by this generator.
    config.set_all_crate_names(vec!["clipper".into()]);
    config.set_create_config_hook(|_crate_name| create_config());

    // Delegate to ritual's command line interface.
    cli::run_from_args(config).fancy_unwrap();
}
