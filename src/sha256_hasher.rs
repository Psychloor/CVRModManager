use base64::{engine::general_purpose, Engine as _};
use sha2::Digest;
use tokio_util::bytes::Bytes;

pub(crate) fn compute_sha256_hash(data: &Bytes) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    general_purpose::STANDARD.encode(result)
}
