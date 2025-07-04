//! <https://github.com/KallDrexx/rust-media-libs>
//! License: See ./LICENSE-APACHE and ./LICENSE-MIT

use super::{Amf0Value, errors::Amf0SerializationError, markers};
use byteorder::{BigEndian, WriteBytesExt};
use std::collections::HashMap;

// Serializes values into an amf0 encoded vector of bytes
pub(in crate::formats::sol) fn serialize(
    values: &Vec<Amf0Value>,
) -> Result<Vec<u8>, Amf0SerializationError> {
    let mut bytes = vec![];
    for value in values {
        serialize_value(value, &mut bytes)?;
    }

    Ok(bytes)
}

fn serialize_value(value: &Amf0Value, bytes: &mut Vec<u8>) -> Result<(), Amf0SerializationError> {
    match *value {
        Amf0Value::Boolean(val) => {
            bytes.push(markers::BOOLEAN_MARKER);
            serialize_bool(val, bytes)
        }
        Amf0Value::Null => {
            bytes.push(markers::NULL_MARKER);
            Ok(())
        }
        Amf0Value::Undefined => {
            bytes.push(markers::UNDEFINED_MARKER);
            Ok(())
        }
        Amf0Value::Number(val) => {
            bytes.push(markers::NUMBER_MARKER);
            serialize_number(val, bytes)
        }
        Amf0Value::Utf8String(ref val) => {
            bytes.push(markers::STRING_MARKER);
            serialize_string(val, bytes)
        }
        Amf0Value::Object(ref val) => {
            bytes.push(markers::OBJECT_MARKER);
            serialize_object(val, bytes)
        }
        Amf0Value::StrictArray(ref val) => {
            bytes.push(markers::STRICT_ARRAY_MARKER);
            serialize_strict_array(val, bytes)
        }
        Amf0Value::ECMAArray(ref val) => {
            bytes.push(markers::ECMA_ARRAY_MARKER);
            serialize_ecma_array(val, bytes)
        }
    }
}

fn serialize_number(value: f64, bytes: &mut Vec<u8>) -> Result<(), Amf0SerializationError> {
    bytes.write_f64::<BigEndian>(value)?;
    Ok(())
}

fn serialize_bool(value: bool, bytes: &mut Vec<u8>) -> Result<(), Amf0SerializationError> {
    bytes.push(value as u8);
    Ok(())
}

fn serialize_string(value: &String, bytes: &mut Vec<u8>) -> Result<(), Amf0SerializationError> {
    if value.len() > (u16::max_value() as usize) {
        return Err(Amf0SerializationError::NormalStringTooLong);
    }

    bytes.write_u16::<BigEndian>(value.len() as u16)?;
    bytes.extend(value.as_bytes());
    Ok(())
}

fn serialize_object(
    properties: &HashMap<String, Amf0Value>,
    bytes: &mut Vec<u8>,
) -> Result<(), Amf0SerializationError> {
    for (name, value) in properties {
        bytes.write_u16::<BigEndian>(name.len() as u16)?;
        bytes.extend(name.as_bytes());
        serialize_value(&value, bytes)?;
    }

    bytes.write_u16::<BigEndian>(markers::UTF_8_EMPTY_MARKER)?;
    bytes.push(markers::OBJECT_END_MARKER);
    Ok(())
}

fn serialize_strict_array(
    array: &Vec<Amf0Value>,
    bytes: &mut Vec<u8>,
) -> Result<(), Amf0SerializationError> {
    bytes.write_u32::<BigEndian>(array.len() as u32)?;

    for value in array {
        serialize_value(&value, bytes)?;
    }

    Ok(())
}

fn serialize_ecma_array(
    properties: &HashMap<String, Amf0Value>,
    bytes: &mut Vec<u8>,
) -> Result<(), Amf0SerializationError> {
    bytes.write_u32::<BigEndian>(properties.len() as u32)?;

    serialize_object(properties, bytes)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::Amf0Value;
    use super::Amf0SerializationError;
    use super::markers;
    use super::serialize;
    use byteorder::{BigEndian, WriteBytesExt};
    use std::collections::HashMap;

    #[test]
    fn can_serialize_strict_array() {
        let number: f64 = 332.0;

        let value = Amf0Value::Number(number);

        let input = vec![Amf0Value::StrictArray(vec![value])];

        let result = serialize(&input).unwrap();

        let mut expected = vec![];

        expected.write_u8(markers::STRICT_ARRAY_MARKER).unwrap();
        expected.write_u32::<BigEndian>(1).unwrap();
        expected.write_u8(markers::NUMBER_MARKER).unwrap();
        expected.write_f64::<BigEndian>(number).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_number() {
        let number: f64 = 332.0;

        let input = vec![Amf0Value::Number(number)];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::NUMBER_MARKER).unwrap();
        expected.write_f64::<BigEndian>(number).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_true_boolean() {
        let input = vec![Amf0Value::Boolean(true)];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::BOOLEAN_MARKER).unwrap();
        expected.write_u8(1).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_false_boolean() {
        let input = vec![Amf0Value::Boolean(false)];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::BOOLEAN_MARKER).unwrap();
        expected.write_u8(0).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_string() {
        let value = "test";

        let input = vec![Amf0Value::Utf8String(value.to_string())];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::STRING_MARKER).unwrap();
        expected.write_u16::<BigEndian>(value.len() as u16).unwrap();
        expected.extend(value.as_bytes());

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_null() {
        let input = vec![Amf0Value::Null];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::NULL_MARKER).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_object() {
        const NUMBER: f64 = 332.0;

        let mut properties = HashMap::new();
        properties.insert("test".to_string(), Amf0Value::Number(NUMBER));

        let input = vec![Amf0Value::Object(properties)];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.push(markers::OBJECT_MARKER);
        expected.write_u16::<BigEndian>(4).unwrap();
        expected.extend("test".as_bytes());
        expected.push(markers::NUMBER_MARKER);
        expected.write_f64::<BigEndian>(NUMBER).unwrap();
        expected
            .write_u16::<BigEndian>(markers::UTF_8_EMPTY_MARKER)
            .unwrap();
        expected.push(markers::OBJECT_END_MARKER);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_when_string_length_greater_than_u16() {
        let mut value = String::new();
        let max = (u16::max_value() as u32) + 1;
        for _ in 0..max {
            value.push('a');
        }

        let input = vec![Amf0Value::Utf8String(value)];
        let result = serialize(&input);

        assert!(match result {
            Err(Amf0SerializationError::NormalStringTooLong) => true,
            _ => false,
        });
    }

    #[test]
    fn can_serialize_undefined() {
        let input = vec![Amf0Value::Undefined];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::UNDEFINED_MARKER).unwrap();

        assert_eq!(result, expected);
    }
}
