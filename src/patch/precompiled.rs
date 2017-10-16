use bigint::Gas;
use core::cmp::min;
use alloc::Vec;

use errors::{RuntimeError, OnChainError};

/// Represent a precompiled contract.
pub trait Precompiled: Sync {
    /// Step a precompiled contract based on the gas required.
    fn step(&self, _: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
    /// Gas needed for a given computation.
    fn gas(&self, _: &[u8]) -> Gas {
        unimplemented!()
    }
    /// Combine step and gas together, given the gas limit.
    fn gas_and_step(&self, data: &[u8], gas_limit: Gas) -> Result<(Gas, Vec<u8>), RuntimeError> {
        let gas = self.gas(data);
        if gas > gas_limit {
            Err(RuntimeError::OnChain(OnChainError::EmptyGas))
        } else {
            Ok((gas, self.step(data)))
        }
    }
}

fn gas_div_ceil(a: Gas, b: Gas) -> Gas {
    if a % b == Gas::zero() {
        a / b
    } else {
        a / b + Gas::from(1u64)
    }
}
