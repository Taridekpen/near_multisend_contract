// Import the NEAR SDK
use near_sdk::{AccountId, env, near_bindgen};

// This line specifies that the contract will be compiled to wasm and
// able to be invoked via the NEAR blockchain.
#[near_bindgen]
// Define the contract struct and its state variables
pub struct TokenSender {
    // The total supply of tokens
    pub total_supply: u64,
    // The balance of tokens for each account
    pub balances: std::collections::HashMap<AccountId, u64>,
    // The recipients of the next token transfer
    pub recipients: Vec<AccountId>,
}

impl TokenSender {
    // A public function that allows the contract owner to send tokens to
    // multiple addresses at once
    pub fn send_tokens(&mut self, amount: u64) {
        // Get the account ID of the caller (i.e. the contract owner)
        let sender = env::predecessor_account_id();
        // Ensure that the caller is the contract owner
        assert_eq!(sender, env::current_account_id(), "Only the contract owner can send tokens");
        // Calculate the total amount of tokens to be sent
        let total_amount = amount * self.recipients.len() as u64;
        // Ensure that the contract has enough tokens to send
        assert!(self.total_supply >= total_amount, "Insufficient token supply");
        // Subtract the total amount of tokens to be sent from the contract's
        // total supply
        self.total_supply -= total_amount;
        // Loop through each recipient and send them the specified amount
        for recipient in &self.recipients {
            // Ensure that the recipient is a valid NEAR account
            assert!(env::is_valid_account_id(recipient.as_bytes()), "Invalid recipient account ID");
            // Subtract the specified amount from the sender's balance
            self.balances.insert(recipient.clone(), amount + self.balances.get(&recipient).unwrap_or(&0));
            // Emit an event to indicate that the tokens have been sent
            env::log_str(&format!("Sent {} tokens to {}", amount, recipient));

        }
    }

    // A public function that allows the contract owner to add a recipient
    pub fn add_recipient(&mut self, recipient: AccountId) {
        // Get the account ID of the caller (i.e. the contract owner)
        let sender = env::predecessor_account_id();
        // Ensure that the caller is the contract owner
        assert_eq!(sender, env::current_account_id(), "Only the contract owner can add recipients");
        // Ensure that the recipient is a valid NEAR account
        assert!(env::is_valid_account_id(recipient.as_bytes()), "Invalid recipient account ID");
        // Add the recipient to the vector
        self.recipients.push(recipient);
    }

    // A public function that allows the contract owner to remove a recipient
    pub fn remove_recipient(&mut self, recipient: AccountId) {
        // Get the account ID of the caller (i.e. the contract owner)
        let sender = env::predecessor_account_id();
        // Ensure that the caller is the contract owner
        assert_eq!(sender, env::current_account_id(), "Only the contract owner can remove recipients");
        // Find the index of the recipient in the vector
        if let Some(index) = self.recipients.iter().position(|r| r == &recipient) {
            // Remove the recipient from the vector
            self.recipients.remove(index);
        }
    }

    pub fn get_recipients(&self) -> Vec<AccountId> {
        self.recipients.clone()
    }
}

// To add a recipient: near call <Contract owner> add_recipient '{"recipient": "zimor.near"}' --accountId <Contract owner>
// Send tokens to multiple recipients: near call <Contract owner> send_tokens '{"recipients": ["taridekpen.near", "zimor.near"], "amount": "10"}' --accountId <Contract owner>
// Remove recipients: near call <Contract owner> remove_recipient '{"recipient": "zimor.near"}' --accountId <Contract owner>
// Get recipients: near view <YOUR_ACCOUNT_ID> get_recipients