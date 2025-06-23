pub(crate) fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("\\x{:02X}", b)).collect()
}
