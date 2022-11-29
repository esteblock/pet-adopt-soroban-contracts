#![no_std]
use soroban_sdk::{contractimpl, contracttype, Env, Address};

#[contracttype]
/*
    Enum keys like DataKey can be a helpful way to store a variety of different types of data
    keyed against the same or similar things.
    In the example the adopter for each pet is stored against DataKey::PetAdopter(u32),
    where u32 is the pet_id 
*/
pub enum DataKey {
    PetAdopter(u32),
}

pub struct PetAdoptContract;

#[contractimpl]
impl PetAdoptContract {
    pub fn adopt(env: Env, pet_id: u32) -> u32 {

        /*
        The invoker is an Address of the account or contract that directly invoked the contract function.
        The env.invoker() always returns the invoker of the currently executing contract.
        It will return either:
            Account with an AccountId if the contract was invoked directly by an account.
            Contract with a BytesN<32> contract ID if the contract was invoked by another contract.
        */
        let invoker = env.invoker();
        let key = DataKey::PetAdopter(pet_id);
        env.data().set(&key, invoker);
        pet_id

    }

    pub fn adopter(env: Env, pet_id: u32) -> Address {
        let key = DataKey::PetAdopter(pet_id);
        env.data().get(key).unwrap_or(Ok(Address::Contract(env.current_contract()))).unwrap()
    }


}
