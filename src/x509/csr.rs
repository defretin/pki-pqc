use crate::crypto::keys::KeyPair ;
use ed25519_dalek::Signature ;

pub struct Csr {
    pub subject : String,
    pub public_key: Vec<u8>,
    pub signature:Signature,
}

impl Csr {
    pub fn new( subject: String, key_pair: &KeyPair ) -> Self {
        let public_key  = key_pair.public.to_bytes().to_vec() ;
        // On construit les data à signer
        let data = subject.as_bytes() ;
        let signature   = key_pair.sign( data ) ;

        Self {  subject, 
                public_key, 
                signature, }
    }

}

#[cfg(test)]
mod tests {
    use super::* ;
    use crate::crypto::keys::KeyPair ;

    #[test]
    fn test_create_csr() {
        let kp = KeyPair::generate() ;
        let csr = Csr::new( "Lionel".to_string(), &kp ) ;
        println!( "CSR subject: {}", csr.subject ) ;
        println!( "CSR public key: {:?}", csr.public_key ) ;
        println!( "CSR signature: {:?}", csr.signature ) ;  
    }
}   