extern crate once_cell;

use once_cell::sync::OnceCell;
use std::env;
use std::fs;
use std::path::PathBuf;

fn has_docker_env_file() -> bool {
    fs::metadata("/.dockerenv").is_ok()
}

fn has_docker_in_cgroup() -> bool {
    match fs::read_to_string("/proc/self/cgroup") {
        Ok(file_contents) => file_contents.contains("docker"),
        Err(_error) => false,
    }
}

fn has_docker_env_file_rootless() -> bool {
    if let Ok(home) = env::var("HOME") {
        let docker_env_path = PathBuf::from(home).join(".dockerenv");
        fs::metadata(docker_env_path).is_ok()
    } else {
        false
    }
}

pub fn is_docker() -> bool {
    static CACHED_RESULT: OnceCell<bool> = OnceCell::new();

    *CACHED_RESULT.get_or_init(|| {
        has_docker_env_file() || has_docker_in_cgroup() || has_docker_env_file_rootless()
    })
}
