#![cfg_attr(not(feature = "bluetooth"), no_std)]
#![doc = include_str!("../README.md")]

mod bit_reader;
mod bluetooth;
mod err;
mod model;
mod record;

pub use crate::err::*;
#[cfg(feature = "bluetooth")]
pub use bluetooth::open_stream;
pub use model::*;
use record::Record;

/// One AES-128 block encryption: the only cipher primitive a record needs.
///
/// A record's payload is a single AES-CTR block, so its keystream is
/// `E_k(IV)` and this crate does the XOR itself. Supplying the primitive
/// rather than the mode lets a host hand over an AES accelerator (on an ESP32,
/// `esp_hal::aes`) instead of the software cipher, which costs kilobytes of
/// code and re-expands the key schedule for every advertisement.
///
/// `block` is the counter block on the way in and the keystream on the way
/// out: encryption and decryption are the same operation in CTR mode.
pub trait Aes128Encrypt {
    fn encrypt_block(&mut self, key: &[u8; 16], block: &mut [u8; 16]);
}

/// The software cipher, for a host without an accelerator.
#[cfg(feature = "soft-aes")]
pub struct SoftAes;

#[cfg(feature = "soft-aes")]
impl Aes128Encrypt for SoftAes {
    fn encrypt_block(&mut self, key: &[u8; 16], block: &mut [u8; 16]) {
        use aes::cipher::{BlockEncrypt, KeyInit};
        aes::Aes128::new(key.into()).encrypt_block(block.into());
    }
}

/// Decrypt and parse the content of the manufacturer data published by a
/// Victron device, using `aes` for the one block it has to decrypt.
pub fn parse_manufacturer_data_with<A: Aes128Encrypt>(
    manufacturer_data: &[u8],
    device_encryption_key: &[u8],
    aes: &mut A,
) -> Result<DeviceState> {
    let record = Record::new(manufacturer_data, device_encryption_key)?;
    let data = record.decrypt_with(aes)?;
    DeviceState::parse(record.record_type(), &data)
}

/// Decrypt and parse the content of the manufacturer data published by a Victron device.
#[cfg(feature = "soft-aes")]
pub fn parse_manufacturer_data(
    manufacturer_data: &[u8],
    device_encryption_key: &[u8],
) -> Result<DeviceState> {
    parse_manufacturer_data_with(manufacturer_data, device_encryption_key, &mut SoftAes)
}
