#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short, String};

#[contracttype]
#[derive(Clone)]
pub struct Bounty {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub is_resolved: bool,
}

#[contracttype]
pub enum BountyBook {
    Bug(u64),
}

const COUNT: Symbol = symbol_short!("COUNT");

#[contract]
pub struct BugBountyContract;

#[contractimpl]
impl BugBountyContract {
    pub fn create_bug(env: Env, title: String, description: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&COUNT).unwrap_or(0);
        count += 1;

        let bug = Bounty {
            id: count,
            title,
            description,
            is_resolved: false,
        };

        env.storage().instance().set(&BountyBook::Bug(count), &bug);
        env.storage().instance().set(&COUNT, &count);

        count
    }

    pub fn resolve_bug(env: Env, id: u64) {
        let key = BountyBook::Bug(id);
        let mut bug: Bounty = env
            .storage()
            .instance()
            .get(&key)
            .expect("Bug not found");

        bug.is_resolved = true;
        env.storage().instance().set(&key, &bug);
    }

    pub fn get_bug(env: Env, id: u64) -> Bounty {
        let key = BountyBook::Bug(id);
        env.storage().instance().get(&key).unwrap_or(Bounty {
            id: 0,
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
            is_resolved: false,
        })
    }

    pub fn total_bugs(env: Env) -> u64 {
        env.storage().instance().get(&COUNT).unwrap_or(0)
    }
}
