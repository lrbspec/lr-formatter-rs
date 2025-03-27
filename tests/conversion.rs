#[cfg(test)]
mod tests {
    use lr_formatter_rs::{convert, format::Format};

    #[test]
    fn test_convert_json_to_lrb() {
        let json_data = br#"{"label": "My Track"}"#;
        let result = convert(json_data, Format::TrackJson, Format::LRB).expect("Conversion failed");

        // Check for the magic number
        assert_eq!(&result[0..3], b"LRB");

        // Check title length
        let title_len = u16::from_le_bytes(result[3..5].try_into().unwrap());
        assert_eq!(title_len, 8);

        // Check title
        let title = &result[5..];
        assert_eq!(title, b"My Track");
    }

    #[test]
    fn test_convert_lrb_to_json() {
        let mut lrb_data = vec![b'L', b'R', b'B'];
        lrb_data.extend_from_slice(&8u16.to_le_bytes());
        lrb_data.extend_from_slice(b"My Track");

        let result = convert(&lrb_data, Format::LRB, Format::TrackJson).expect("Conversion failed");
        let json_str = String::from_utf8(result).unwrap();
        assert!(json_str.contains("\"label\": \"My Track\""));
    }
}
