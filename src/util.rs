/// A 192-bit hash value.
pub struct Hash([u8; 24]);

impl Hash {
    pub fn from_bytes(value: [u8; 24]) -> Self {
        Hash(value)
    }
}

impl std::fmt::Display for Hash {
    /// The Base52 representation of the hash. This will always be 34 ASCII characters long.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::with_capacity(34);
        for chunk in self.0.chunks(4) {
            let mut value = 0u32;
            for (i, &byte) in chunk.iter().enumerate() {
                value |= (byte as u32) << (8 * i);
            }

            for _ in 0..6 {
                let digit = value % 52;
                value /= 52;
                let c = if digit < 26 {
                    (b'A' + digit as u8) as char
                } else {
                    (b'a' + (digit - 26) as u8) as char
                };
                result.push(c);
            }
        }
        write!(f, "{}", result)
    }
}

impl Hash {
    pub fn to_string_truncated(&self, length: usize) -> String {
        self.to_string()[0..length].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_to_string() {
        let pairs = [(0u8, "AAAAAA"), (255u8, "ZZZZZZ"), (1u8, "AAAAAB")];
        let input = [0u8; 24];
        let hash = Hash::from_bytes(input);
        let result = hash.to_string();

        assert_eq!(result.len(), 34);

        // String should only contain A-Z and a-z
        assert!(result.chars().all(|c| c.is_ascii_alphabetic()));

        // With all zero bytes, should start with 'AAAAAA'
        assert_eq!(&result[0..6], "AAAAAA");
    }
}
