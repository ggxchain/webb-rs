//! Deserializer for Webb Proposal binary format.

use serde::de::{self, Deserialize, DeserializeSeed, SeqAccess, Visitor};
use std::io::Cursor;
use std::io::Read;

/// Errors that can occur during deserialization.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum DeserializationError {
    /// Custom error message from [serde].
    #[error("{0}")]
    Custom(String),
    /// Unsupported type encountered.
    #[error("Unsupported type")]
    Unspported,
    /// Reached end of input unexpectedly.
    #[error("Unexpected end of input")]
    Eof,
    /// Invalid bool value.
    #[error("Invalid bool value")]
    InvalidBool,
}

impl de::Error for DeserializationError {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

struct Deserializer<'de> {
    input: Cursor<&'de [u8]>,
}

impl<'de> Deserializer<'de> {
    fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer {
            input: Cursor::new(input),
        }
    }
}

/// Deserialize the given Bytes into `T`.
///
/// # Errors
/// Returns an error if the deserialization failed.
pub fn from_slice<'a, T>(s: &'a [u8]) -> Result<T, DeserializationError>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.get_ref().is_empty() {
        return Err(DeserializationError::Eof);
    }

    Ok(t)
}

macro_rules! impl_nums {
    ($ty:ty, $dser_method:ident, $visitor_method:ident, $reader_method:ident) => {
        #[inline]
        fn $dser_method<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, DeserializationError>
        where
            V: Visitor<'de>,
        {
            let mut bytes = [0u8; core::mem::size_of::<$ty>()];
            // Read bytes from input
            self.input
                .read_exact(&mut bytes)
                .map_err(|_| DeserializationError::Eof)?;
            let value = <$ty>::from_be_bytes(bytes);
            visitor.$visitor_method(value)
        }
    };
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = DeserializationError;

    impl_nums!(u8, deserialize_u8, visit_u8, read_u8);

    impl_nums!(u16, deserialize_u16, visit_u16, read_u16);

    impl_nums!(u32, deserialize_u32, visit_u32, read_u32);

    impl_nums!(u64, deserialize_u64, visit_u64, read_u64);

    impl_nums!(i8, deserialize_i8, visit_i8, read_i8);

    impl_nums!(i16, deserialize_i16, visit_i16, read_i16);

    impl_nums!(i32, deserialize_i32, visit_i32, read_i32);

    impl_nums!(i64, deserialize_i64, visit_i64, read_i64);

    impl_nums!(f32, deserialize_f32, visit_f32, read_f32);

    impl_nums!(f64, deserialize_f64, visit_f64, read_f64);

    fn deserialize_any<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        Err(DeserializationError::Unspported)
    }

    fn deserialize_bool<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        // 0 = false, 1 = true
        // Read one byte
        let mut byte = [0u8; 1];
        self.input
            .read_exact(&mut byte)
            .map_err(|_| DeserializationError::Eof)?;
        match byte[0] {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            _ => Err(DeserializationError::InvalidBool),
        }
    }

    fn deserialize_char<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        Err(DeserializationError::Unspported)
    }

    fn deserialize_str<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        Err(DeserializationError::Unspported)
    }

    fn deserialize_string<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        Err(DeserializationError::Unspported)
    }

    fn deserialize_bytes<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        Visitor::visit_bytes(visitor, self.input.get_ref())
    }

    fn deserialize_byte_buf<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        // We do not support deserializing into a byte buffer (i.e Vec<u8>)
        // since this will need to know the length of the buffer.
        Err(DeserializationError::Unspported)
    }

    fn deserialize_option<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        Err(DeserializationError::Unspported)
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        Visitor::visit_unit(visitor)
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        let len = serde::Deserialize::deserialize(&mut *self)?;

        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(Access { de: self, len })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        Err(DeserializationError::Unspported)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        Err(DeserializationError::Unspported)
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, DeserializationError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct Access<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    len: usize,
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for Access<'a, 'de> {
    type Error = DeserializationError;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, DeserializationError>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            let value = DeserializeSeed::deserialize(seed, &mut *self.de)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_struct_de() {
    use super::*;
    use crate::ProposalHeader;
    use proposal_derive::Proposal;
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Proposal)]
    #[proposal(function_sig = "test(bytes32)")]
    struct MyProposal {
        header: ProposalHeader,
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: [u8; 32],
    }

    let bytes = hex_literal::hex! {
        "0101010101010101010101010101010101010101010101010101010101010101" // rid
        "99372321" // function sig
        "00000001" // nonce
        "01" // a
        "0002" // b
        "00000003" // c
        "0000000000000004" // d
        "0606060606060606060606060606060606060606060606060606060606060606" // e
    };

    let proposal: MyProposal = from_slice(&bytes).unwrap();

    let rid = crate::ResourceId::from([1u8; 32]);
    let header = ProposalHeader::new(rid, MyProposal::function_sig(), Nonce(1));
    let expected = MyProposal {
        header,
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: [6u8; 32],
    };

    assert_eq!(bytes.len(), MyProposal::LENGTH);
    assert_eq!(proposal, expected);
}
