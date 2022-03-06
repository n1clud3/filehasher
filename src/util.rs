pub fn compute_md5(input: Vec<u8>) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

pub fn compute_sha256(input: Vec<u8>) -> String {
    sha256::digest_bytes(&input)
}
