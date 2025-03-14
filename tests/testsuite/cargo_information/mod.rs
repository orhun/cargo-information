use cargo_test_support::{compare, TestEnv};

mod basic;
mod features;
mod help;
mod not_found;
mod specify_empty_version_with_url;
mod specify_version_outside_ws;
mod specify_version_with_url_but_registry_is_not_matched;
mod specify_version_with_ws_and_match_multiple_versions;
mod specify_version_within_ws_and_conflict_with_lockfile;
mod specify_version_within_ws_and_match_with_lockfile;
mod with_frozen_outside_ws;
mod with_frozen_within_ws;
mod with_locked_outside_ws;
mod with_locked_within_ws;
mod with_offline;
mod with_quiet;
mod within_ws;
mod within_ws_and_pick_ws_package;
mod within_ws_with_alternative_registry;

// Invoke `cargo-info info` with the test environment.
pub(crate) fn cargo_info() -> snapbox::cmd::Command {
    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin("cargo-info"))
        .with_assert(compare::assert_ui())
        .test_env()
        .arg("info")
        .arg("--color=never")
}

// Initialize the registry without a token.
// Otherwise, it will try to list owners of the crate and fail.
pub(crate) fn init_registry_without_token() {
    let _reg = cargo_test_support::registry::RegistryBuilder::new()
        .no_configure_token()
        .build();
}
