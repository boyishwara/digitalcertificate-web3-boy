#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_register_and_get_certificates() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CertificateRegistryContract);
    let client = CertificateRegistryContractClient::new(&env, &contract_id);

    // Initial state should be empty
    let initial_certs = client.get_certificates();
    assert_eq!(initial_certs.len(), 0);

    // Register a new certificate
    let student_name = String::from_str(&env, "Alice Smith");
    let course_name = String::from_str(&env, "Stellar Smart Contracts 101");
    let cert_id = client.register_certificate(&student_name, &course_name);

    // Fetch certificates again
    let updated_certs = client.get_certificates();
    assert_eq!(updated_certs.len(), 1);

    // Verify the contents
    let cert = updated_certs.get(0).unwrap();
    assert_eq!(cert.student_name, student_name);
    assert_eq!(cert.course_name, course_name);
    assert_eq!(cert.id, cert_id);
}

#[test]
fn test_verify_certificate() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CertificateRegistryContract);
    let client = CertificateRegistryContractClient::new(&env, &contract_id);

    // Register a new certificate
    let student_name = String::from_str(&env, "Bob Johnson");
    let course_name = String::from_str(&env, "Web3 Development");
    let cert_id = client.register_certificate(&student_name, &course_name);

    // Verification should pass for the registered ID
    let is_valid = client.verify_certificate(&cert_id);
    assert_eq!(is_valid, true);

    // Verification should fail for a random ID
    let is_invalid = client.verify_certificate(&9999999);
    assert_eq!(is_invalid, false);
}
