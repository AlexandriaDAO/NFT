use ciborium::{from_reader, into_writer};
use hmac::Mac;
use serde::Serialize;
use serde_bytes::ByteBuf;
use sha3::Digest;

pub fn sha3_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn mac_256_2(key: &[u8], add1: &[u8], add2: &[u8]) -> [u8; 32] {
    let mut mac =
        hmac::Hmac::<sha3::Sha3_256>::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(add1);
    mac.update(add2);
    mac.finalize().into_bytes().into()
}

pub fn mac_256(key: &[u8], add: &[u8]) -> [u8; 32] {
    let mut mac = hmac::Hmac::<sha3::Sha3_256>::new_from_slice(key).unwrap();
    mac.update(add);
    mac.finalize().into_bytes().into()
}

pub trait Secret {
    fn secret(&self, key: &[u8], timestamp: u64) -> Vec<u8>;
    fn verify(&self, key: &[u8], expire_at: u64, challenge: &[u8]) -> Result<(), String>;
}

pub fn to_cbor_bytes(obj: &impl Serialize) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    into_writer(obj, &mut buf).expect("failed to encode in CBOR format");
    buf
}

impl<T> Secret for T
where
    T: Serialize,
{
    fn secret(&self, key: &[u8], timestamp: u64) -> Vec<u8> {
        let mac = &mac_256_2(key, &to_cbor_bytes(self), &to_cbor_bytes(&timestamp))[0..16];
        to_cbor_bytes(&(timestamp, ByteBuf::from(mac)))
    }

    fn verify(&self, key: &[u8], expire_at: u64, challenge: &[u8]) -> Result<(), String> {
        let arr: (u64, ByteBuf) =
            from_reader(challenge).map_err(|_err| "failed to decode the challenge")?;

        if arr.0 < expire_at {
            return Err("the challenge is expired".to_string());
        }

        let mac = &mac_256_2(key, &to_cbor_bytes(self), &to_cbor_bytes(&arr.0))[0..16];
        if mac != &arr.1[..] {
            return Err("failed to verify the challenge".to_string());
        }

        Ok(())
    }
}
