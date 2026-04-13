#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address
};

#[contract]
pub struct WorkerBenefitContract;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
}

#[contractimpl]
impl WorkerBenefitContract {

    // Add worker (initialize with 0)
    pub fn add_worker(env: Env, worker: Address) {
        let key = DataKey::Balance(worker.clone());

        if env.storage().instance().has(&key) {
            return;
        }

        env.storage().instance().set(&key, &0i128);
    }

    // Assign benefit
    pub fn assign_benefit(env: Env, worker: Address, amount: i128) {
        let key = DataKey::Balance(worker.clone());

        let current: i128 = env.storage().instance().get(&key).unwrap_or(0);
        env.storage().instance().set(&key, &(current + amount));
    }

    // Claim benefit
    pub fn claim(env: Env, worker: Address) -> i128 {
        worker.require_auth();

        let key = DataKey::Balance(worker.clone());

        let amount: i128 = env.storage().instance().get(&key).unwrap_or(0);

        env.storage().instance().set(&key, &0);

        amount
    }

    // Check balance
    pub fn get_balance(env: Env, worker: Address) -> i128 {
        let key = DataKey::Balance(worker);

        env.storage().instance().get(&key).unwrap_or(0)
    }
}