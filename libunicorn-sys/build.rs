#[cfg(windows)]
fn main() {
    vcpkg::Config::new()
        .emit_includes(true)
        .find_package("unicorn").unwrap();
}

#[cfg(macos)]
fn get_libc_path() -> &'static str {
    "/usr/local/lib64"
}

#[cfg(not(macos))]
#[allow(dead_code)]
fn get_libc_path() -> &'static str {
    "/usr/lib64"
}

#[cfg(not(windows))]
fn main() {
    build_helper::rustc::link_search(
        Some(build_helper::SearchKind::Native), get_libc_path());
    build_helper::rustc::link_lib(Some(build_helper::LibKind::Static), "unicorn");
}
