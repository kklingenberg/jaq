use git2::Repository;

fn get_commit_sha() -> Option<String> {
    Repository::open("..")
        .ok()?
        .revparse_single("HEAD")
        .ok()?
        .short_id()
        .ok()?
        .as_str()
        .map(String::from)
}

fn main() {
    // Inject the abbreviated commit SHA as an environment variable
    let commit_sha = get_commit_sha().unwrap_or(String::from("unknown"));
    println!("cargo:rustc-env=COMMIT_SHA={}", commit_sha);
}
