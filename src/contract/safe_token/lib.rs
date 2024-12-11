#![cfg_attr(not(feature = "std"), no_std)]

/// デフォルトアドレスへのトークン転送を防止
fn get_default_accounts() -> [AccountId; 8] {
    let zero = [0; 32];
    let one = [1; 32];
    let two = [2; 32];
    let three = [3; 32];
    let four = [4; 32];
    let five = [5; 32];
    let six = [6; 32];
    let empty = [0x46, 0xEB, 0xDD, 0xEF, 0x8C, 0xD9, 0xBB, 0x16,
                 0x7D, 0xC3, 0x08, 0x78, 0xD7, 0x11, 0x3B, 0x7E,
                 0x16, 0x8E, 0x6F, 0x06, 0x46, 0xBE, 0xFF, 0xD7,
                 0x7D, 0x69, 0xD3, 0x9B, 0xAD, 0x76, 0xB4, 0x7A];

    [AccountId::from(zero), AccountId::from(one), AccountId::from(two),
     AccountId::from(three), AccountId::from(four), AccountId::from(five),
     AccountId::from(six), AccountId::from(empty)]
}

#[ink::contract]
mod safe_token {
    use ink::{storage::Mapping};

    #[ink(storage)]
    pub struct SafeToken {
        total_supply: u32,
        balances: Mapping<AccountId, u32>
    }

    impl SafeToken {
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

            // 受信者アドレス検証
            if cfg!(not(debug_assertions)) {
                let default_accounts = SafeToken::get_default_accounts();
                if default_accounts.contains(&recipient) {
                    return;
                }
            }

            self.balances.insert(sender, &(sender_balance - amount));
            let recipient_balance = self.balance_of(recipient);
            self.balances.insert(recipient, &(recipient_balance + amount));
        }
    }
}
