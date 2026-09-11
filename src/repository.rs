use std::fs;
use std::io;
use std::path::Path;

pub fn init(worktree: &Path) -> io::Result<()> {
    let git_dir = worktree.join(".git");

    fs::create_dir_all(git_dir.join("objects"))?;
    fs::create_dir_all(git_dir.join("refs").join("heads"))?;
    fs::write(git_dir.join("HEAD"), b"ref: refs/heads/main\n")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::init;
    use std::fs;

    #[test]
    fn init_creates_basic_git_directory_layout() {
        let worktree = std::env::temp_dir().join(format!("rgit-init-test-{}", std::process::id()));

        if worktree.exists() {
            fs::remove_dir_all(&worktree).unwrap();
        }

        fs::create_dir(&worktree).unwrap();
        init(&worktree).unwrap();

        assert!(worktree.join(".git").is_dir());
        assert!(worktree.join(".git").join("objects").is_dir());
        assert!(worktree.join(".git").join("refs").join("heads").is_dir());
        assert_eq!(
            fs::read_to_string(worktree.join(".git").join("HEAD")).unwrap(),
            "ref: refs/heads/main\n"
        );

        fs::remove_dir_all(&worktree).unwrap();
    }
}
