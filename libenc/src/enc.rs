use ring::{
    aead::{
        Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, Tag, UnboundKey, AES_256_GCM,
        MAX_TAG_LEN, NONCE_LEN,
    },
    error::Unspecified,
    rand::{SecureRandom, SystemRandom},
};
// This will keep track of nonce. Since data is sent for storage, nonce is only incremented upon
// encryption.
pub struct AES {
    key_bytes: Vec<u8>,
    nonce_ctr: u32,
    tag: Tag,
}

impl AES {
    pub fn new() -> Result<Self, Unspecified> {
        let mut key_bytes = vec![0; AES_256_GCM.key_len()];
        SystemRandom::new().fill(&mut key_bytes)?;
        let nonce_ctr = 1;

        Ok(AES {
            key_bytes,
            nonce_ctr,
            tag: [0; MAX_TAG_LEN].into(),
        })
    }

    pub fn encrypt(&mut self, data: &[u8], aad: Option<&[u8]>) -> Result<Vec<u8>, Unspecified> {
        #[cfg(debug_assertions)] // don't print this in release
        println!("key_bytes = {}", hex::encode(&self.key_bytes));
        println!("data = {}", String::from_utf8(data.to_vec()).unwrap());

        // Create a new AEAD key without a designated role or nonce sequence
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key_bytes)?;

        // Create a new AEAD key for encrypting and signing ("sealing"), bound to a nonce sequence
        // The SealingKey can be used multiple times, each time a new nonce will be used
        let mut sealing_key = SealingKey::new(unbound_key, CounterNonceSequence(self.nonce_ctr));
        // Increment the nonce_ctr
        self.nonce_ctr += 1;

        // Create a mutable copy of the data that will be encrypted in place
        let mut in_out = data.to_owned();

        // Associated data will be authenticated but not encrypted
        if let Some(data) = aad {
            let associated_data = Aad::from(data); // is optional so can be empty
            self.tag = sealing_key.seal_in_place_separate_tag(associated_data, &mut in_out)?;
        } else {
            let associated_data = Aad::empty(); // is optional so can be empty
            self.tag = sealing_key.seal_in_place_separate_tag(associated_data, &mut in_out)?;
        }

        Ok(in_out.to_vec())
    }

    pub fn decrypt(&self, in_out: &[u8]) -> Result<Vec<u8>, Unspecified> {
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key_bytes)?;
        let mut opening_key =
            OpeningKey::new(unbound_key, CounterNonceSequence(self.nonce_ctr - 1));

        // Decrypt the data by passing in the associated data and the cypher text with the authentication tag appended
        let mut cypher_text_with_tag = [in_out, self.tag.as_ref()].concat();

        let decrypted_data = opening_key.open_in_place(Aad::empty(), &mut cypher_text_with_tag)?;

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
