use ed25519_dalek::{
	PublicKey,
	Verifier, ed25519::signature::Signature,
};
use hex;



pub fn verify(message: String, signature_hex: String) -> bool {
	// verifies a 64-bit message using 32-bit signature
	let public_key_bytes: [u8; 32] = match hex::decode(dotenv::var("PUBLIC_KEY").unwrap()) {
		Ok(v) => {
			match v.try_into() {
				Ok(b) => b,
				Err(_) => return false,
			}
		},
		Err(_) => return false,
	};

	let signature_bytes: [u8; 64] = match hex::decode(signature_hex) {
		Ok(v) => {
			match v.try_into() {
				Ok(b) => b,
				Err(_) => return false,
			}
		},
		Err(_) => return false,
	};

	let signature = match Signature::from_bytes(&signature_bytes) {
		Ok(s) => s,
		Err(_) => return false,
	};

	let public_key = PublicKey::from_bytes(&public_key_bytes).unwrap();
	public_key.verify(message.as_bytes(), &signature).is_ok()

}