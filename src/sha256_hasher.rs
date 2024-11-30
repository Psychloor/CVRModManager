use base64::{engine::general_purpose, Engine as _};
use sha2::Digest;
use tokio::task;
use tokio_util::bytes::Bytes;

pub(crate) async fn compute_sha256_hash(data: &Bytes) -> Option<String> {
    let hash = task::block_in_place(move || {
        let mut hasher = sha2::Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        general_purpose::STANDARD.encode(&result)
    });

    Some(hash)
}