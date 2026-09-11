use sha1::{Digest, Sha1};

pub fn blob_hash(content: &[u8]) -> String {
    let mut object = format!("blob {}\0", content.len()).into_bytes();
    object.extend_from_slice(content);

    let hash = Sha1::digest(&object);
    format!("{hash:x}")
}

#[cfg(test)]
mod tests {
    use super::blob_hash;

    #[test]
    fn blob_hash_matches_git_for_hello_file() {
        assert_eq!(
            blob_hash(b"hello\n"),
            "ce013625030ba8dba906f756967f9e9ca394464a"
        );
    }
}
