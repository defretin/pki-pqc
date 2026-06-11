#[cfg(test)]
mod tests {
    use super::* ;


    #[test]
    fn test_hello() {
        hello() ;   
    }
}


pub fn hello() {
    println!( "PKI PQC project starting..." ) ;
}

pub mod crypto ;
pub mod x509 ;
