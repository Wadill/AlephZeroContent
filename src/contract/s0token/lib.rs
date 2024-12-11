#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod s0token {
    use ink::{storage::Mapping};

    #[ink(storage)]
    pub struct S0token {
        total_supply: u32,
        balances: Mapping<AccountId, u32>
    }

    impl S0token {
        #[ink(constructor)]
        pub fn new_token(supply: u32) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(&caller, &supply);
            Self {
                total_supply: supply,
                balances
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u32 {
            match self.balances.get(&account) {
                Some(value) => value,
                None => 0,
            }
        }

        #[ink(message)]
        pub fn transfer(&mut self, recipient: AccountId, amount: u32) {
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);
            if sender_balance < amount {
                return;
            }
            self.balances.insert(sender, &(sender_balance - amount));
            let recipient_balance = self.balance_of(recipient);
            self.balances.insert(recipient, &(recipient_balance + amount));
        }
    }
}
