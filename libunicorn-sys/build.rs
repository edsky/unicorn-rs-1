#[cfg(windows)]
fn main() {
    vcpkg::Config::new()
        .emit_includes(true)
        .find_package("unicorn").unwrap();
}

#[cfg(macos)]
fn get_libc_path() -> &'static str {
    "/usr/local/lib"
}

#[cfg(not(macos))]
#[allow(dead_code)]
fn get_libc_path() -> &'static str {
    "/usr/lib"
}

#[cfg(not(windows))]
fn main() {
    let path:String;
    if Some(64) == build_helper::target::pointer_width() {
        path = get_libc_path().to_owned() + "64";
    } else {
        path = get_libc_path().to_owned();
    }
    build_helper::rustc::link_search(
        Some(build_helper::SearchKind::Native), path);
    build_helper::rustc::link_lib(Some(build_helper::LibKind::Static), "unicorn");
}