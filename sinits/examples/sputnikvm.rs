#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(alloc)]
#![no_std]

#[macro_use]
extern crate system;
extern crate spin;
extern crate selfalloc;
extern crate alloc;
extern crate sputnikvm;
extern crate rlp;
extern crate hexutil;
extern crate bigint;
extern crate block_core;

use alloc::Vec;
use alloc::str::FromStr;

use system::{CAddr};
use sputnikvm::{VM, HeaderParams, Context, SeqContextVM, MainnetEmbeddedPatch, RequireError, AccountCommitment, ValidTransaction, VMStatus, SeqTransactionVM};
use block_core::{TransactionAction};
use bigint::{Gas, H256, U256, Address};
use rlp::Rlp;
use hexutil::*;

#[lang="start"]
#[no_mangle]
fn start(_argc: isize, _argv: *const *const u8) {
    unsafe { system::set_task_buffer_addr(0x90001000); }
    unsafe { selfalloc::setup_allocator(CAddr::from(2), CAddr::from(3), 0x1000000000); }

    let header = HeaderParams {
        beneficiary: Address::default(),
        timestamp: 0x01,
        number: U256::zero(),
        difficulty: U256::zero(),
        gas_limit: Gas::max_value(),
    };

    let context = Context {
        address: Address::default(),
        caller: Address::default(),
        code: read_hex("0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01600055").unwrap(),
        data: Vec::new(),
        gas_limit: Gas::from_str("0x0186a0").unwrap(),
        gas_price: Gas::zero(),
        origin: Address::default(),
        value: U256::zero(),
        apprent_value: U256::zero(),
        is_system: false,
    };

    let mut vm = SeqContextVM::<MainnetEmbeddedPatch>::new(context, header.clone());
    loop {
        match vm.fire() {
            Err(RequireError::Account(address)) => {
                vm.commit_account(AccountCommitment::Nonexist(address));
            },
            Err(RequireError::AccountCode(address)) => {
                vm.commit_account(AccountCommitment::Nonexist(address));
            },
            Err(RequireError::AccountStorage(address, _)) => {
                vm.commit_account(AccountCommitment::Nonexist(address));
            },
            Err(RequireError::Blockhash(number)) => {
                vm.commit_blockhash(number, H256::default());
            },
            Ok(_) => break,
        }
    }

    if vm.available_gas() != Gas::from_str("0x013874").unwrap() {
        system::debug_test_fail();
        return;
    }

    let transaction = ValidTransaction {
        caller: None,
        gas_price: Gas::zero(),
        gas_limit: Gas::max_value(),
        action: TransactionAction::Call(Address::default()),
        value: U256::from_str("0x10000000000").unwrap(),
        input: Vec::new(),
        nonce: U256::zero(),
    };

    let mut vm = SeqTransactionVM::<MainnetEmbeddedPatch>::new(transaction, header.clone());
    loop {
        match vm.fire() {
            Err(RequireError::Account(address)) => {
                vm.commit_account(AccountCommitment::Nonexist(address));
            },
            Err(RequireError::AccountCode(address)) => {
                vm.commit_account(AccountCommitment::Nonexist(address));
            },
            Err(RequireError::AccountStorage(address, _)) => {
                vm.commit_account(AccountCommitment::Nonexist(address));
            },
            Err(RequireError::Blockhash(number)) => {
                vm.commit_blockhash(number, H256::default());
            },
            Ok(_) => break,
        }
    }

    match vm.status() {
        VMStatus::ExitedOk => system::debug_test_succeed(),
        _ => system::debug_test_fail(),
    }
}
