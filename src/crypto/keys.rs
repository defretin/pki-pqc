use ed25519_dalek::{SigningKey, VerifyingKey, Verifier, Signature, Signer};
use rand::rngs::OsRng;
use rand::RngCore;

pub struct KeyPair {
    pub private: SigningKey,
    pub public: VerifyingKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let mut csprng = OsRng;

        // Générer 32 bytes aléatoires (seed)
        let mut seed = [0u8; 32];
        csprng.fill_bytes(&mut seed);

        // Créer la clé privée depuis la seed
        let private = SigningKey::from_bytes(&seed);
        let public = private.verifying_key();

        Self { private, public }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.private.sign(message)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let kp = KeyPair::generate();
        assert!(kp.public.to_bytes().len() > 0);
    }

    #[test]
    fn test_signing() {
        let kp = KeyPair::generate();
        let message = b"test message";
        let signature = kp.sign(message);
        assert!(kp.public.verify(message, &signature).is_ok());
    }

    #[test]
    fn test_verification() {
        let kp          = KeyPair::generate() ;
        let message     = b"Hello PKI" ;
        let signature   = kp.sign( message ) ;
        println!( "Signature: {:?}", signature.to_bytes() ) ;
    }
}