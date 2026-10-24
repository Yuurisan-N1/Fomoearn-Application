
pub fn length_bytes(len: usize) -> Vec<u8> {
    if len < 0x80 {
        return vec![len as u8];
    }
    let mut bytes = Vec::new();
    let mut value = len;
    while value > 0 {
        bytes.push((value & 0xFF) as u8);
        value >>= 8;
    }
    bytes.reverse();
    let mut out = vec![0x80 | bytes.len() as u8];
    out.extend_from_slice(&bytes);
    out
}

pub fn tlv(tag: u8, payload: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    out.extend_from_slice(&length_bytes(payload.len()));
    out.extend_from_slice(payload);
    out
}

pub fn integer(value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut rest = value;
    loop {
        bytes.push((rest & 0xFF) as u8);
        rest >>= 8;
        if rest == 0 {
            break;
        }
    }
    if bytes.last().map(|byte| byte & 0x80 != 0).unwrap_or(false) {
        bytes.push(0);
    }
    bytes.reverse();
    tlv(0x02, &bytes)
}

pub fn integer_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut payload = bytes.to_vec();
    while payload.len() > 1 && payload[0] == 0 {
        payload.remove(0);
    }
    if payload.first().map(|byte| byte & 0x80 != 0).unwrap_or(false) {
        payload.insert(0, 0);
    }
    tlv(0x02, &payload)
}

pub fn enumerated(value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut rest = value;
    loop {
        bytes.push((rest & 0xFF) as u8);
        rest >>= 8;
        if rest == 0 {
            break;
        }
    }
    bytes.reverse();
    tlv(0x0A, &bytes)
}

pub fn octets(bytes: &[u8]) -> Vec<u8> {
    tlv(0x04, bytes)
}

pub fn boolean(value: bool) -> Vec<u8> {
    tlv(0x01, &[if value { 0xFF } else { 0x00 }])
}

pub fn null() -> Vec<u8> {
    tlv(0x05, &[])
}

pub fn sequence(items: &[Vec<u8>]) -> Vec<u8> {
    let mut body = Vec::new();
    for item in items {
        body.extend_from_slice(item);
    }
    tlv(0x30, &body)
}

pub fn set(items: &[Vec<u8>]) -> Vec<u8> {
    let mut body = Vec::new();
    for item in items {
        body.extend_from_slice(item);
    }
    tlv(0x31, &body)
}

pub fn utf8(text: &str) -> Vec<u8> {
    tlv(0x0C, text.as_bytes())
}

pub fn utc_time(text: &str) -> Vec<u8> {
    tlv(0x17, text.as_bytes())
}

pub fn bit_string(bytes: &[u8]) -> Vec<u8> {
    let mut payload = vec![0x00];
    payload.extend_from_slice(bytes);
    tlv(0x03, &payload)
}

pub fn object_id(content: &[u8]) -> Vec<u8> {
    tlv(0x06, content)
}

pub fn explicit(tag: u64, inner: &[u8]) -> Vec<u8> {
    if tag < 0x1F {
        return tlv(0xA0 | tag as u8, inner);
    }
    let mut digits = Vec::new();
    let mut value = tag;
    loop {
        digits.push((value & 0x7F) as u8);
        value >>= 7;
        if value == 0 {
            break;
        }
    }
    digits.reverse();
    let last = digits.len() - 1;
    let mut tag_bytes = vec![0xBF];
    for (index, digit) in digits.iter().enumerate() {
        tag_bytes.push(if index < last { digit | 0x80 } else { *digit });
    }
    let mut out = tag_bytes;
    out.extend_from_slice(&length_bytes(inner.len()));
    out.extend_from_slice(inner);
    out
}

pub struct Node<'a> {
    pub tag: u8,
    pub body: &'a [u8],
    pub raw: &'a [u8],
}

pub fn head(input: &[u8]) -> Option<Node<'_>> {
    if input.len() < 2 {
        return None;
    }
    let tag = input[0];
    let mut index = 1;
    let first = input[index];
    index += 1;
    let len = if first & 0x80 == 0 {
        first as usize
    } else {
        let count = (first & 0x7F) as usize;
        if count == 0 || count > 4 || input.len() < index + count {
            return None;
        }
        let mut value = 0usize;
        for _ in 0..count {
            value = (value << 8) | input[index] as usize;
            index += 1;
        }
        value
    };
    if input.len() < index + len {
        return None;
    }
    Some(Node {
        tag,
        body: &input[index..index + len],
        raw: &input[..index + len],
    })
}

pub fn walk(input: &[u8]) -> Vec<Node<'_>> {
    let mut nodes = Vec::new();
    let mut rest = input;
    while !rest.is_empty() {
        match head(rest) {
            Some(node) => {
                let consumed = node.raw.len();
                nodes.push(node);
                rest = &rest[consumed..];
            }
            None => break,
        }
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_short_length_stays_one_byte() {
        assert_eq!(length_bytes(127), vec![0x7F]);
    }

    #[test]
    fn a_long_length_carries_its_octets() {
        assert_eq!(length_bytes(200), vec![0x81, 0xC8]);
        assert_eq!(length_bytes(749), vec![0x82, 0x02, 0xED]);
    }

    #[test]
    fn a_high_bit_integer_gains_a_zero() {
        assert_eq!(integer(0x80), vec![0x02, 0x02, 0x00, 0x80]);
        assert_eq!(integer(300), vec![0x02, 0x02, 0x01, 0x2C]);
    }

    #[test]
    fn a_context_tag_wraps_its_body() {
        let inner = integer(2);
        let wrapped = explicit(0, &inner);
        assert_eq!(wrapped[0], 0xA0);
        assert_eq!(&wrapped[2..], &inner[..]);
    }

    #[test]
    fn a_multi_octet_context_tag_keeps_its_number() {
        let wrapped = explicit(709, &octets(b"x"));
        assert_eq!(wrapped[0], 0xBF);
        assert_eq!(wrapped[1] & 0x80, 0x80);
    }

    #[test]
    fn the_reader_returns_the_body_and_the_whole_slice() {
        let encoded = integer(300);
        let node = head(&encoded).unwrap();
        assert_eq!(node.tag, 0x02);
        assert_eq!(node.body, &[0x01, 0x2C]);
        assert_eq!(node.raw.len(), 4);
    }
}
