#![allow(dead_code)]
use crate::{fuzz_client::FuzzClient, AccountId};
use anchor_lang::prelude::Clock;
use anchor_lang::solana_program::clock::Epoch;
use anchor_lang::solana_program::stake::state::Lockup;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::collections::HashMap;

pub struct PdaStore {
    pub pubkey: Pubkey,
    pub seeds: Vec<Vec<u8>>,
}
impl PdaStore {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }
}

pub struct TokenStore {
    pub pubkey: Pubkey,
}

pub struct MintStore {
    pub pubkey: Pubkey,
}

pub struct VoteAccountStore {
    pub pubkey: Pubkey,
}

pub struct StakeAccountStore {
    pub pubkey: Pubkey,
}

pub struct ProgramStore {
    pub pubkey: u8,
}

pub struct AccountsStorage<T> {
    accounts: HashMap<AccountId, T>,
    _max_accounts: u8,
}

impl<T> AccountsStorage<T> {
    pub fn new(max_accounts: u8) -> Self {
        let accounts: HashMap<AccountId, T> = HashMap::new();
        Self {
            accounts,
            _max_accounts: max_accounts,
        }
    }

    /// Gets a reference to the account with the given account ID
    pub fn get(&self, account_id: AccountId) -> Option<&T> {
        self.accounts.get(&account_id)
    }

    /// Returns a mutable reference to the underlying HashMap that stores accounts with IDs as keys
    pub fn storage(&mut self) -> &mut HashMap<AccountId, T> {
        &mut self.accounts
    }
}

impl<T> Default for AccountsStorage<T> {
    fn default() -> Self {
        Self::new(2)
    }
}
// TODO Add an easy way to limit the number of created accounts
impl AccountsStorage<Keypair> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        lamports: u64,
    ) -> Keypair {
        let key = self
            .accounts
            .entry(account_id)
            .or_insert_with(|| client.set_account(lamports));
        key.insecure_clone()
    }
}

impl AccountsStorage<TokenStore> {
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Option<Pubkey> {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let key = client.set_token_account(
                mint,
                owner,
                amount,
                delegate,
                is_native,
                delegated_amount,
                close_authority,
            );
            TokenStore { pubkey: key }
        });
        Some(key.pubkey)
    }
}

impl AccountsStorage<MintStore> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Option<Pubkey> {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let key = client.set_mint_account(decimals, owner, freeze_authority);
            MintStore { pubkey: key }
        });
        Some(key.pubkey)
    }
}

impl AccountsStorage<VoteAccountStore> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        node_pubkey: &Pubkey,
        authorized_voter: &Pubkey,
        authorized_withdrawer: &Pubkey,
        commission: u8,
        clock: &Clock,
    ) -> Option<Pubkey> {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let key = client.set_vote_account(
                node_pubkey,
                authorized_voter,
                authorized_withdrawer,
                commission,
                clock,
            );
            VoteAccountStore { pubkey: key }
        });
        Some(key.pubkey)
    }
}

impl AccountsStorage<StakeAccountStore> {
    pub fn get_or_create_delegated_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        voter_pubkey: Pubkey,
        staker: Pubkey,
        withdrawer: Pubkey,
        stake: u64,
        activation_epoch: Epoch,
        deactivation_epoch: Option<Epoch>,
        lockup: Option<Lockup>,
    ) -> Option<Pubkey> {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let key = client.set_delegated_stake_account(
                voter_pubkey,
                staker,
                withdrawer,
                stake,
                activation_epoch,
                deactivation_epoch,
                lockup,
            );
            StakeAccountStore { pubkey: key }
        });
        Some(key.pubkey)
    }

    pub fn get_or_create_initialized_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        staker: Pubkey,
        withdrawer: Pubkey,
        lockup: Option<Lockup>,
    ) -> Option<Pubkey> {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let key = client.set_initialized_stake_account(staker, withdrawer, lockup);
            StakeAccountStore { pubkey: key }
        });
        Some(key.pubkey)
    }
}

impl AccountsStorage<PdaStore> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        seeds: &[&[u8]],
        program_id: &Pubkey,
    ) -> Option<&PdaStore> {
        let key = self.accounts.entry(account_id).or_insert(
            if let Some((key, _)) = Pubkey::try_find_program_address(seeds, program_id) {
                let seeds_vec: Vec<_> = seeds.iter().map(|&s| s.to_vec()).collect();
                PdaStore {
                    pubkey: key,
                    seeds: seeds_vec,
                }
            } else {
                return None;
            },
        );
        Some(key)
    }
}
