use md5::Md5;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Digest};

pub fn compute_md5(input: Vec<u8>) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();

    format!("{:x}", result) // Format the result as string
}

pub fn compute_sha128(input: Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input);
    let result = hasher.finalize();

    format!("{:x}", result) // Format the result as string
}

pub fn compute_sha224(input: Vec<u8>) -> String {
    let mut hasher = Sha224::new();
    hasher.update(input);
    let result = hasher.finalize();

    format!("{:x}", result) // Format the result as string
}

pub fn compute_sha256(input: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    format!("{:x}", result) // Format the result as string
}

pub fn compute_sha384(input: Vec<u8>) -> String {
    let mut hasher = Sha384::new();
    hasher.update(input);
    let result = hasher.finalize();

    format!("{:x}", result) // Format the result as string
}

pub fn compute_sha512(input: Vec<u8>) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input);
    let result = hasher.finalize();

    format!("{:x}", result) // Format the result as string
}
