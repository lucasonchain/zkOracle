pub fn pad_or_trim(bytes: &[u8], size: usize) -> Vec<u8> {
    let len = bytes.len();
    if len == size {
        return bytes.to_vec();
    }
    if len > size {
        return bytes[len - size..].to_vec();
    }
    let mut out = vec![0u8; size];
    out[size - len..].copy_from_slice(bytes);
    out
}
