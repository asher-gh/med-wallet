use rand::Rng;
use ring::{
    aead::{
        Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM,
        NONCE_LEN,
    },
    error::Unspecified,
    rand::{SecureRandom, SystemRandom},
};

// This will keep track of nonce. Since data is sent for storage, nonce is only incremented upon
// encryption.
pub struct AES {
    sealing_key: SealingKey<CounterNonceSequence>,
    opening_key: OpeningKey<CounterNonceSequence>,
}

impl AES {
    pub fn new() -> Result<Self, Unspecified> {
        let mut key_bytes = vec![0; AES_256_GCM.key_len()];
        SystemRandom::new().fill(&mut key_bytes)?;
        let nonce_ctr = rand::thread_rng().gen_range(1..250);

        let sealing_key = SealingKey::new(
            UnboundKey::new(&AES_256_GCM, &key_bytes)?,
            CounterNonceSequence(nonce_ctr),
        );

        let opening_key = OpeningKey::new(
            UnboundKey::new(&AES_256_GCM, &key_bytes)?,
            CounterNonceSequence(nonce_ctr),
        );

        Ok(AES {
            sealing_key,
            opening_key,
        })
    }

    // pub fn from(key_bytes: &[u8], nonce_ctr: u32) -> Result<Self, Unspecified> {
    //     Ok(AES {
    //         key_bytes: key_bytes.to_vec(),
    //         nonce_ctr,
    //     })
    // }

    pub fn encrypt(&mut self, data: &[u8], aad: Option<&[u8]>) -> Result<Vec<u8>, Unspecified> {
        // Create a mutable copy of the data that will be encrypted in place
        let mut in_out = data.to_owned();

        // Associated data will be authenticated but not encrypted
        if let Some(data) = aad {
            self.sealing_key
                .seal_in_place_append_tag(Aad::from(data), &mut in_out)?;
        } else {
            self.sealing_key
                .seal_in_place_append_tag(Aad::empty(), &mut in_out)?;
        }

        Ok(in_out.to_vec())
    }

    pub fn decrypt(&mut self, tagged_cypher: &[u8]) -> Result<Vec<u8>, Unspecified> {
        // Decrypt the data by passing in the associated data and the cypher text with the authentication tag appended
        let mut tagged_cypher = tagged_cypher.to_owned();
        let decrypted_data = self
            .opening_key
            .open_in_place(Aad::empty(), &mut tagged_cypher)?;

        #[cfg(debug_assertions)]
        println!(
            "decrypted_data = {}",
            String::from_utf8(decrypted_data.to_vec()).unwrap()
        );

        Ok(decrypted_data.to_owned())
    }
}

struct CounterNonceSequence(u32);

impl NonceSequence for CounterNonceSequence {
    // called once for each seal operation
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = vec![0; NONCE_LEN];

        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);

        #[cfg(debug_assertions)]
        println!("nonce_bytes = {}", hex::encode(&nonce_bytes));

        self.0 += 1; // advance the counter
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonce_handling() {
        let mut aes = AES::new().unwrap();

        let cypher1 = aes.encrypt(b"hello", None).unwrap();
        let cypher2 = aes.encrypt(b" world", None).unwrap();

        assert_eq!(aes.decrypt(&cypher1).unwrap(), b"hello".to_vec());
        assert_eq!(aes.decrypt(&cypher2).unwrap(), b" world".to_vec());
    }
}
