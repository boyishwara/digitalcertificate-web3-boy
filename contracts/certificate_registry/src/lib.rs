#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

/// Represents a digital certificate issued to a student.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Certificate {
    pub id: u64,
    pub student_name: String,
    pub course_name: String,
    pub issue_date: u64, // Represented as a timestamp
}

// Storage key for the certificates data
const CERT_DATA: Symbol = symbol_short!("CERT_DATA");

#[contract]
pub struct CertificateRegistryContract;

#[contractimpl]
impl CertificateRegistryContract {
    /// Retrieves all issued certificates.
    pub fn get_certificates(env: Env) -> Vec<Certificate> {
        // Retrieve the data from storage or return an empty vector if none exists
        env.storage().instance().get(&CERT_DATA).unwrap_or(Vec::new(&env))
    }

    /// Registers a new certificate to the blockchain.
    pub fn register_certificate(
        env: Env,
        student_name: String,
        course_name: String,
    ) -> u64 {
        // 1. Retrieve the existing certificates
        let mut certificates: Vec<Certificate> = env.storage().instance().get(&CERT_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Generate a pseudo-random ID for the certificate
        let cert_id = env.prng().gen::<u64>();
        let current_timestamp = env.ledger().timestamp();
        
        // 3. Create the new certificate object
        let new_cert = Certificate {
            id: cert_id,
            student_name: student_name,
            course_name: course_name,
            issue_date: current_timestamp,
        };
        
        // 4. Add the new certificate to the list
        certificates.push_back(new_cert);
        
        // 5. Save the updated list to storage
        env.storage().instance().set(&CERT_DATA, &certificates);
        
        // Return the unique certificate ID
        cert_id
    }

    /// Verifies if a certificate exists in the registry based on its ID.
    pub fn verify_certificate(env: Env, id: u64) -> bool {
        let certificates: Vec<Certificate> = env.storage().instance().get(&CERT_DATA).unwrap_or(Vec::new(&env));

        for i in 0..certificates.len() {
            if certificates.get(i).unwrap().id == id {
                return true;
            }
        }

        false
    }
}

mod test;