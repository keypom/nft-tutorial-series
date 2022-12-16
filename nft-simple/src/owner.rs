use crate::*;

#[near_bindgen]
impl Contract {
    //add a token to the set of tokens an owner has
    pub(crate) fn assert_contract_owner(&mut self) {
        assert!(
            self.owner_id == env::predecessor_account_id(),
            "only contract owner"
        )
    }
    /// approved minters
    pub fn add_approved_minter(&mut self, account_id: AccountId) {
        self.assert_contract_owner();
        self.approved_minters.insert(&account_id);
    }

    pub fn remove_approved_minter(&mut self, account_id: AccountId) {
        self.assert_contract_owner();
        self.approved_minters.remove(&account_id);
    }

    pub fn is_approved_minter(&self, account_id: AccountId) -> bool {
        self.approved_minters.contains(&account_id)
    }
}
