#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Result, String, Vec };

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    ClaimID,
}

#[derive(Clone)]
#[contracttype]
pub struct InsuranceClaim {
    claimant: Address,
    policy_number: String,
    documents: Vec<String>,
    status: ClaimStatus,
}

#[derive(Clone)]
#[contracttype]
pub enum ClaimStatus {
    Pending,
    InProgress,
    Approved,
    Denied,
}

#[contract]
pub struct InsuranceContract;

#[contractimpl]
impl InsuranceContract {
    // Event for logging claim creation
    pub fn event_claim_created(claim_id: u64, claimant: Address, policy_number: &str) {
        log("Claim created".into());
        log(format!("Claim ID: {}", claim_id));
        log(format!("Claimant: {}", claimant));
        log(format!("Policy Number: {}", policy_number));
    }

    // Event for logging document addition
    pub fn event_document_added(claim_id: u64, document_url: &str) {
        log("Document added to claim".into());
        log(format!("Claim ID: {}", claim_id));
        log(format!("Document URL: {}", document_url));
    }

    // Event for logging claim status update
    pub fn event_claim_status_updated(claim_id: u64, new_status: ClaimStatus) {
        log("Claim status updated".into());
        log(format!("Claim ID: {}", claim_id));
        log(format!("New Status: {:?}", new_status));
    }

    // Utility function for logging
    fn log(message: String) {
        // You need to implement the logging mechanism according to your blockchain platform
        // This could involve emitting events or writing logs to storage
        // For demonstration purposes, we'll just print to the console
        println!("{}", message);
    }

    // Function to create a new insurance claim
    pub fn create_claim(
        env: Env,
        from: Address,
        claimant: Address,
        policy_number: String,
        initial_documents: Vec<String>
    ) -> Result<()> {
        // Ensure that the transaction sender is authorized to create claims
        from.require_auth()?;

        // Generate a new unique claim ID
        let claim_id = env.storage().instance().get::<_, u64>(&StorageKey::ClaimID).unwrap_or(0) + 1;

        // Create the insurance claim
        let claim = InsuranceClaim {
            claimant,
            policy_number: policy_number.clone(), // Cloning for event logging
            documents: initial_documents,
            status: ClaimStatus::Pending,
        };

        // Store the claim in the contract storage
        env.storage().instance().set(&StorageKey::ClaimID, &claim_id)?;
        env.storage().instance().set(&claim_id, &claim)?;

        // Log the creation of the claim
        Self::event_claim_created(claim_id, claimant, &policy_number);

        Ok(())
    }

    // Function to add a document to an existing claim
    pub fn add_document(env: Env, from: Address, claim_id: u64, document_url: String) -> Result<()> {
        // Ensure that the transaction sender is authorized to add documents
        from.require_auth()?;

        // Retrieve the claim from the contract storage
        let mut claim: InsuranceClaim = env.storage().instance().get(&claim_id).ok_or("Claim not found")?;

        // Add the document to the claim
        claim.documents.push(document_url.clone()); // Cloning for event logging

        // Update the claim in the contract storage
        env.storage().instance().set(&claim_id, &claim)?;

        // Log the addition of the document
        Self::event_document_added(claim_id, &document_url);

        Ok(())
    }

    // Function to process an insurance claim
    pub fn process_claim(env: Env, from: Address, claim_id: u64, status: ClaimStatus) -> Result<()> {
        // Ensure that the transaction sender is authorized to process claims
        from.require_auth()?;

        // Retrieve the claim from the contract storage
        let mut claim: InsuranceClaim = env.storage().instance().get(&claim_id).ok_or("Claim not found")?;

        // Update the claim status
        claim.status = status;

        // Update the claim in the contract storage
        env.storage().instance().set(&claim_id, &claim)?;

        // Log the status update of the claim
        Self::event_claim_status_updated(claim_id, status);

        Ok(())
    }
}

