// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, log, near_bindgen, AccountId};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct HelloWorld {
    pub messages: LookupMap<AccountId, String>,
}

impl Default for HelloWorld {
    fn default() -> Self {
        Self {
            messages: LookupMap::new(b"m".to_vec()),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl HelloWorld {
    pub fn hello(&self, account_id: AccountId) -> String {
        match self.messages.get(&account_id) {
            Some(value) => {
                log!("value from lookup map: {}", value);
                value
            }
            None => DEFAULT_MESSAGE.to_string(),
        }
    }

    pub fn set_hello(&mut self, message: String) {
        log!("Saving greeting {}", message);

        let account_id = env::signer_account_id();
        let message = format_args!("Hello {message}!").to_string();

        self.messages.insert(&account_id, &message);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn get_default_hello() {
        let contract = HelloWorld::default();
        // this test did not call hello so should return the default "Hello" greeting
        assert_eq!(contract.hello(accounts(1)), "Hello".to_string());
    }

    #[test]
    fn set_then_get_hello() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());

        let mut contract = HelloWorld::default();
        contract.set_hello("howdy".to_string());
        assert_eq!(contract.hello(accounts(2)), "Hello howdy!".to_string());
    }
}
