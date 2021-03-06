use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

use crate::encoding::Binary;

// Added Eq and Hash to allow this to be a key in a HashMap (MockQuerier)
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, JsonSchema, Hash)]
pub struct HumanAddr(pub String);

impl HumanAddr {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HumanAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<&str> for HumanAddr {
    fn from(addr: &str) -> Self {
        HumanAddr(addr.to_string())
    }
}

impl From<&HumanAddr> for HumanAddr {
    fn from(addr: &HumanAddr) -> Self {
        HumanAddr(addr.0.to_string())
    }
}

impl From<&&HumanAddr> for HumanAddr {
    fn from(addr: &&HumanAddr) -> Self {
        HumanAddr(addr.0.to_string())
    }
}

impl From<String> for HumanAddr {
    fn from(addr: String) -> Self {
        HumanAddr(addr)
    }
}

/// Just like String, HumanAddr is a smart pointer to str.
/// This implements `*human_address` for us, which is not very valuable directly
/// because str has no known size and cannot be stored in variables. But it allows us to
/// do `&*human_address`, returning a `&str` from a `&HumanAddr`.
/// With [deref coercions](https://doc.rust-lang.org/1.22.1/book/first-edition/deref-coercions.html#deref-coercions),
/// this allows us to use `&human_address` whenever a `&str` is required.
impl Deref for HumanAddr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl PartialEq<str> for HumanAddr {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct CanonicalAddr(pub Binary);

impl From<&[u8]> for CanonicalAddr {
    fn from(source: &[u8]) -> Self {
        Self(source.into())
    }
}

impl From<Vec<u8>> for CanonicalAddr {
    fn from(source: Vec<u8>) -> Self {
        Self(source.into())
    }
}

impl From<CanonicalAddr> for Vec<u8> {
    fn from(source: CanonicalAddr) -> Vec<u8> {
        source.0.into()
    }
}

/// Just like Vec<u8>, CanonicalAddr is a smart pointer to [u8].
/// This implements `*canonical_address` for us and allows us to
/// do `&*canonical_address`, returning a `&[u8]` from a `&CanonicalAddr`.
/// With [deref coercions](https://doc.rust-lang.org/1.22.1/book/first-edition/deref-coercions.html#deref-coercions),
/// this allows us to use `&canonical_address` whenever a `&[u8]` is required.
impl Deref for CanonicalAddr {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl CanonicalAddr {
    pub fn as_slice(&self) -> &[u8] {
        &self.0.as_slice()
    }
}

impl fmt::Display for CanonicalAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.as_slice() {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Test HumanAddr as_str() for each HumanAddr::from input type
    #[test]
    fn human_addr_as_str() {
        // literal string
        let human_addr_from_literal_string = HumanAddr::from("literal-string");
        assert_eq!("literal-string", human_addr_from_literal_string.as_str());

        // String
        let addr = String::from("Hello, world!");
        let human_addr_from_string = HumanAddr::from(addr);
        assert_eq!("Hello, world!", human_addr_from_string.as_str());

        // &HumanAddr
        let human_addr_from_borrow = HumanAddr::from(&human_addr_from_string);
        assert_eq!(
            human_addr_from_borrow.as_str(),
            human_addr_from_string.as_str()
        );

        // &&HumanAddr
        let human_addr_from_borrow_2 = HumanAddr::from(&&human_addr_from_string);
        assert_eq!(
            human_addr_from_borrow_2.as_str(),
            human_addr_from_string.as_str()
        );
    }

    #[test]
    fn human_addr_implements_display() {
        let human_addr = HumanAddr::from("cos934gh9034hg04g0h134");
        let embedded = format!("Address: {}", human_addr);
        assert_eq!(embedded, "Address: cos934gh9034hg04g0h134");
        assert_eq!(human_addr.to_string(), "cos934gh9034hg04g0h134");
    }

    #[test]
    fn human_addr_implements_deref() {
        // We cannot test *human_addr directly since the resulting type str has no known size
        let human_addr = HumanAddr::from("cos934gh9034hg04g0h134");
        assert_eq!(&*human_addr, "cos934gh9034hg04g0h134");

        // This checks deref coercions from &HumanAddr to &str works
        let human_addr = HumanAddr::from("cos934gh9034hg04g0h134");
        assert_eq!(human_addr.len(), 22);
        let human_addr_str: &str = &human_addr;
        assert_eq!(human_addr_str, "cos934gh9034hg04g0h134");
    }

    #[test]
    fn human_addr_implements_partial_eq() {
        let human_addr = HumanAddr::from("cos934gh9034hg04g0h134");
        assert_eq!(&human_addr, "cos934gh9034hg04g0h134");
    }

    #[test]
    fn human_addr_len() {
        let addr = "Hello, world!";
        let human_addr = HumanAddr::from(addr);
        assert_eq!(addr.len(), human_addr.len());
    }

    #[test]
    fn human_addr_is_empty() {
        let human_addr = HumanAddr::from("Hello, world!");
        assert_eq!(false, human_addr.is_empty());
        let empty_human_addr = HumanAddr::from("");
        assert_eq!(true, empty_human_addr.is_empty());
    }

    // Test CanonicalAddr as_slice() for each CanonicalAddr::from input type
    #[test]
    fn canonical_addr_from_slice() {
        // slice
        let bytes: &[u8] = &[0u8, 187, 61, 11, 250, 0];
        let canonical_addr_slice = CanonicalAddr::from(bytes);
        assert_eq!(canonical_addr_slice.as_slice(), &[0u8, 187, 61, 11, 250, 0]);

        // Vector
        let bytes: Vec<u8> = vec![0u8, 187, 61, 11, 250, 0];
        let canonical_addr_vec = CanonicalAddr::from(bytes);
        assert_eq!(canonical_addr_vec.as_slice(), &[0u8, 187, 61, 11, 250, 0]);
    }

    #[test]
    fn canonical_addr_from_vec_works() {
        // Into<CanonicalAddr> for Vec<u8>
        let original = vec![0u8, 187, 61, 11, 250, 0];
        let original_ptr = original.as_ptr();
        let addr: CanonicalAddr = original.into();
        assert_eq!(addr.as_slice(), [0u8, 187, 61, 11, 250, 0]);
        assert_eq!((addr.0).0.as_ptr(), original_ptr, "must not be copied");

        // From<Vec<u8>> for CanonicalAddr
        let original = vec![0u8, 187, 61, 11, 250, 0];
        let original_ptr = original.as_ptr();
        let addr = CanonicalAddr::from(original);
        assert_eq!(addr.as_slice(), [0u8, 187, 61, 11, 250, 0]);
        assert_eq!((addr.0).0.as_ptr(), original_ptr, "must not be copied");
    }

    #[test]
    fn canonical_addr_into_vec_works() {
        // Into<Vec<u8>> for CanonicalAddr
        let original = CanonicalAddr::from(vec![0u8, 187, 61, 11, 250, 0]);
        let original_ptr = (original.0).0.as_ptr();
        let vec: Vec<u8> = original.into();
        assert_eq!(vec.as_slice(), [0u8, 187, 61, 11, 250, 0]);
        assert_eq!(vec.as_ptr(), original_ptr, "must not be copied");

        // From<CanonicalAddr> for Vec<u8>
        let original = CanonicalAddr::from(vec![7u8, 35, 49, 101, 0, 255]);
        let original_ptr = (original.0).0.as_ptr();
        let vec = Vec::<u8>::from(original);
        assert_eq!(vec.as_slice(), [7u8, 35, 49, 101, 0, 255]);
        assert_eq!(vec.as_ptr(), original_ptr, "must not be copied");
    }

    #[test]
    fn canonical_addr_len() {
        let bytes: &[u8] = &[0u8, 187, 61, 11, 250, 0];
        let canonical_addr = CanonicalAddr::from(bytes);
        assert_eq!(canonical_addr.len(), bytes.len());
    }

    #[test]
    fn canonical_addr_is_empty() {
        let bytes: &[u8] = &[0u8, 187, 61, 11, 250, 0];
        let canonical_addr = CanonicalAddr::from(bytes);
        assert_eq!(false, canonical_addr.is_empty());
        let empty_canonical_addr = CanonicalAddr::from(vec![]);
        assert_eq!(true, empty_canonical_addr.is_empty());
    }

    #[test]
    fn canonical_addr_implements_display() {
        let bytes: &[u8] = &[
            0x12, // two hex digits
            0x03, // small values must be padded to two digits
            0xab, // ensure we get upper case
            0x00, // always test extreme values
            0xff,
        ];
        let address = CanonicalAddr::from(bytes);
        let embedded = format!("Address: {}", address);
        assert_eq!(embedded, "Address: 1203AB00FF");
        assert_eq!(address.to_string(), "1203AB00FF");
    }

    #[test]
    fn canonical_addr_implements_deref() {
        // Dereference to [u8]
        let bytes: &[u8] = &[0u8, 187, 61, 11, 250, 0];
        let canonical_addr = CanonicalAddr::from(bytes);
        assert_eq!(*canonical_addr, [0u8, 187, 61, 11, 250, 0]);

        // This checks deref coercions from &CanonicalAddr to &[u8] works
        let bytes: &[u8] = &[0u8, 187, 61, 11, 250, 0];
        let canonical_addr = CanonicalAddr::from(bytes);
        assert_eq!(canonical_addr.len(), 6);
        let canonical_addr_slice: &[u8] = &canonical_addr;
        assert_eq!(canonical_addr_slice, &[0u8, 187, 61, 11, 250, 0]);
    }
}
