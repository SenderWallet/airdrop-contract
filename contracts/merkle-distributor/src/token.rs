use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{env, json_types::U128, AccountId, PromiseOrValue};
use crate::constant::GAS_FOR_FT_TRANSFER;


#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}


#[near_bindgen]
impl FungibleTokenReceiver for MerkleDistributor {
    // Callback on receiving tokens by this contract.
    // Returns zero.
    #[allow(unused_variables)]
    #[payable]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_id = env::predecessor_account_id();
        let sender_id = AccountId::from(sender_id);

        self.deposit_token(token_id, amount.into());
        return PromiseOrValue::Value(U128(0));
    }
}

#[near_bindgen]
impl MerkleDistributor {
    // Record deposit of some number of tokens to this contract.
    pub(crate) fn deposit_token(&mut self, token_id: AccountId, amount: Balance) {
        require!(
            token_id.to_string() == self.token_id.to_string(),
            "Wrong token on deposit"
        );
        env_log!("Deposit {} of {} token", amount, self.token_id);
        self.balance += amount
    }

    // Withdraws tokens
    #[payable]
    pub fn withdraw_token(&mut self, receiver_id: AccountId, amount: Balance) -> Promise {
        ext_fungible_token::ext(self.token_id.clone())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .with_static_gas(GAS_FOR_FT_TRANSFER)
            .ft_transfer(
                receiver_id.clone(), //caller to refund the FTs to
                amount.into(), //amount to transfer
                Some("Withdraw token".to_string()), //memo (to include some context)
            )

    }
}
