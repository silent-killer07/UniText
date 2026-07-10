use crate::grapheme_table::GraphemeTable;
use crate::normalizer::Normalizer;
use std::fmt;

const MAGIC: [u8; 4] = [b'U', b'N', b'I', 0x01];
const VERSION_FLAGS: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

#[derive(Debug, PartialEq)]
pub enum WireFormatError {
    InvalidMagic,
    UnsupportedVersion,
    InvalidUtf8,
    BufferTooSmall,
}

impl fmt::Display for WireFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMagic => write!(f, "Invalid magic bytes"),
            Self::UnsupportedVersion => write!(f, "Unsupported format version"),
            Self::InvalidUtf8 => write!(f, "Invalid UTF-8 payload"),
            Self::BufferTooSmall => write!(f, "Buffer too small"),
        }
    }
}

pub fn encode(table: &GraphemeTable) -> Vec<u8> {
    let mut text_payload = String::new();
    for g in &table.graphemes {
        if let Some(vid) = g.visual_id {
            if vid < table.visuals.len() {
                text_payload.push_str(&table.visuals[vid]);
            }
        } else {
            text_payload.push_str(&g.canonical_form);
        }
    }
    
    let text_bytes = text_payload.as_bytes();
    let text_len = text_bytes.len() as u32;
    
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&MAGIC);
    buffer.extend_from_slice(&VERSION_FLAGS);
    buffer.extend_from_slice(&text_len.to_le_bytes());
    buffer.extend_from_slice(text_bytes);
    
    // Visual payload length = 0 for now
    buffer.extend_from_slice(&0u32.to_le_bytes());
    
    buffer
}

pub fn decode(bytes: &[u8]) -> Result<GraphemeTable, WireFormatError> {
    if bytes.len() < 16 {
        return Err(WireFormatError::BufferTooSmall);
    }
    
    if bytes[0..4] != MAGIC {
        return Err(WireFormatError::InvalidMagic);
    }
    
    if bytes[4] != VERSION_FLAGS[0] {
        return Err(WireFormatError::UnsupportedVersion);
    }
    
    let mut text_len_bytes = [0u8; 4];
    text_len_bytes.copy_from_slice(&bytes[8..12]);
    let text_len = u32::from_le_bytes(text_len_bytes) as usize;
    
    if bytes.len() < 16 + text_len {
        return Err(WireFormatError::BufferTooSmall);
    }
    
    let text_slice = &bytes[12..12 + text_len];
    let text_str = std::str::from_utf8(text_slice).map_err(|_| WireFormatError::InvalidUtf8)?;
    
    // Reconstruct deterministically
    Ok(Normalizer::process(text_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_format_roundtrip() {
        let original = Normalizer::process("Hello 👨‍👩‍👧‍👦 Café");
        let bytes = encode(&original);
        let decoded = decode(&bytes).unwrap();
        assert_eq!(original, decoded);
    }
    
    #[test]
    fn test_wire_format_errors() {
        let mut bad_magic = encode(&Normalizer::process("test"));
        bad_magic[0] = b'X';
        assert_eq!(decode(&bad_magic), Err(WireFormatError::InvalidMagic));
        
        let mut bad_version = encode(&Normalizer::process("test"));
        bad_version[4] = 0x02;
        assert_eq!(decode(&bad_version), Err(WireFormatError::UnsupportedVersion));
    }
}
