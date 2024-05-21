use std::num::Wrapping;

use once_cell::sync::Lazy;
use openssl::{
    pkey::Public,
    rsa::{Padding, Rsa},
};

pub fn unpack(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // "NXS3\x03\x00\x00\x01" | 32-bit unpacked size | 32-bit packed size | 32-bit zipped data size | key sized bytes | data
    if !data.starts_with(b"NXS3\x03\x00\x00\x01") {
        return Err("Invalid header".into());
    }

    static KEY: Lazy<Rsa<Public>> = Lazy::new(|| {
        Rsa::public_key_from_pem_pkcs1(
            "-----BEGIN RSA PUBLIC KEY-----\n\
MIGJAoGBAOZAaZe2qB7dpT9Y8WfZIdDv+ooS1HsFEDW2hFnnvcuFJ4vIuPgKhISm\n\
pY4/jT3aipwPNVTjM6yHbzOLhrnGJh7Ec3CQG/FZu6VKoCqVEtCeh15hjcu6QYtn\n\
YWIEf8qgkylqsOQ3IIn76udV6m0AWC2jDlmLeRcR04w9NNw7+9t9AgMBAAE=\n\
-----END RSA PUBLIC KEY-----\n"
                .as_bytes(),
        )
        .unwrap()
    });

    let mut ephemeral_key = {
        let mut wrapped_key = vec![0; KEY.size() as usize];

        let v = KEY.public_decrypt(
            &data[20..20 + KEY.size() as usize],
            &mut wrapped_key,
            Padding::PKCS1,
        )?;
        wrapped_key.truncate(v);

        if v != 4 {
            return Err("Invalid wrapped key size".into());
        }

        u32::from_le_bytes(wrapped_key.try_into().unwrap())
    };

    // xor the actual data with the wrapped key
    let decrypted = data
        .iter()
        .skip(20 + KEY.size() as usize)
        .enumerate()
        .map(|(i, &x)| {
            let val = x ^ (ephemeral_key >> (i % 4 * 8) & 0xff) as u8;
            if i % 4 == 3 {
                let ror = Wrapping(ephemeral_key.rotate_right(19));
                ephemeral_key = (ror + (ror << 2) + Wrapping::<u32>(0xE6546B64)).0;
            }
            val
        })
        .collect::<Vec<_>>();

    let uncompressed_size = u32::from_le_bytes(data[16..20].try_into().unwrap()) as usize;
    Ok(lz4_flex::decompress(&decrypted, uncompressed_size)?)
}
