#![cfg(test)]
use crate::{
    Address::Account,
};

use super::{PetAdoptContract, PetAdoptContractClient};
use soroban_sdk::{testutils::Accounts, Env};
extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PetAdoptContract);
    let client = PetAdoptContractClient::new(&env, &contract_id);

    let adopter = env.accounts().generate();
    let adopter_not = env.accounts().generate();

    // let's adopt pet_id=1 with adopter and see if we get 
    // The id of the pet that will be used for testing
    let expected_pet_id: u32 = 8;


    // Testing the adopt() function
    // To test the adopt() function, recall that upon success it returns the given petId.
    // We can ensure an ID was returned and that it's correct by comparing the return value of adopt() to the ID we passed in.
    let returned_id = client.with_source_account(&adopter).adopt(&expected_pet_id);
    assert_eq!(returned_id, expected_pet_id);


    // Testing retrieval of a single pet's owner
    let returned_adopter = client.adopter(&expected_pet_id);
    //std::println!("The value of returned_adopter is: {returned_adopter}");
    //std::println!("The value of adopter is: {adopter}");
    assert_eq!(returned_adopter, Account(adopter));
    //The assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal. 
    assert_ne!(returned_adopter, Account(adopter_not));
    //assert_eq!(0, 1);

}



/*


    Add the following function within the TestAdoption.sol smart contract, after the declaration of Adoption:

// Testing the adopt() function
function testUserCanAdoptPet() public {
  uint returnedId = adoption.adopt(expectedPetId);

  Assert.equal(returnedId, expectedPetId, "Adoption of the expected pet should match what is returned.");
}

Things to notice:

    We call the smart contract we declared earlier with the ID of expectedPetId.
    Finally, we pass the actual value, the expected value and a failure message (which gets printed to the console if the test does not pass) to Assert.equal().



    Testing retrieval of a single pet's owner¶

Remembering from above that public variables have automatic getter methods, we can retrieve the address stored by our adoption test above. Stored data will persist for the duration of our tests, so our adoption of pet expectedPetId above can be retrieved by other tests.

    Add this function below the previously added function in TestAdoption.sol.

// Testing retrieval of a single pet's owner
function testGetAdopterAddressByPetId() public {
  address adopter = adoption.adopters(expectedPetId);

  Assert.equal(adopter, expectedAdopter, "Owner of the expected pet should be this contract");
}

After getting the adopter address stored by the adoption contract, we assert equality as we did above.


*/