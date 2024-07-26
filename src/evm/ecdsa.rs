use alloc::vec::Vec;

use alloy_primitives::{address, Address};
use alloy_sol_types::sol;
use stylus_proc::{sol_interface, sol_storage, SolidityError};
use stylus_sdk::{
    call::Call, console, prelude::external, storage::TopLevelStorage,
};

const ECRECOVER_ADDR: Address =
    address!("0000000000000000000000000000000000000001");

sol! {
    /// The signature derives the `Address::ZERO`.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ECDSAInvalidSignature();
}

/// An error that occurred in the implementation of an [`ECDSA`] library.
#[derive(SolidityError, Debug)]
pub enum Error {
    /// The signature derives the `Address::ZERO`.
    InvalidSignature(ECDSAInvalidSignature),
}

sol_interface! {
    /// EVM Precompiles interface.
    ///
    /// Interface for any contract that wants to call `ecrecover` precompile .
    interface EVMPrecompile {
        #[allow(missing_docs)]
        function ecrecover(
            bytes32 hash,
            uint8 v,
            bytes32 r,
            bytes32 s
        ) returns (address);
    }
}

sol_storage! {
    /// ECDSA contract.
    #[derive(Debug)]
    pub struct ECDSA {}
}

/// NOTE: Implementation of [`TopLevelStorage`] to be able use `&mut self` when
/// calling other contracts and not `&mut (impl TopLevelStorage +
/// BorrowMut<Self>)`. Should be fixed in the future by the Stylus team.
unsafe impl TopLevelStorage for ECDSA {}

#[external]
impl ECDSA {
    /// Calls `ecrecover` EVM precompile.
    /// The `ecrecover` EVM precompile allows for malleable (non-unique)
    /// signatures: this function rejects them by requiring the `s` value to
    /// be in the lower half order, and the `v` value to be either 27 or 28.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `hash` - Hash of the message.
    /// * `v` - `v` value from the signature.
    /// * `r` - `r` value from the signature.
    /// * `s` - `s` value from the signature.
    ///
    /// # Errors
    ///
    /// * If the `s` value is grater than `EIP2_VALUE`, then the error
    /// [`Error::ECDSAInvalidSignatureS`] is returned.
    /// * If the recovered address is `Address::ZERO`, then the error
    /// [`Error::InvalidSignature`] is returned.
    ///
    /// # Panics
    ///
    /// * If `ecrecover` precompile fail to execute.
    fn recover(
        &mut self,
        // hash: FixedBytes<32>,
        // v: u8,
        // r: FixedBytes<32>,
        // s: FixedBytes<32>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        // cast abi-encode
        //     "ecrecover(bytes32,uint8,bytes32,bytes32)(address)"
        //     0xa1de988600a42c4b4ab089b619297c17d53cffae5d5120d82d8a92d0bb3b78f2
        //     28
        //     0x65e72b1cf8e189569963750e10ccb88fe89389daeeb8b735277d59cd6885ee82
        //     0x3eb5a6982b540f185703492dab77b863a88ce01f27e21ade8b2879c10fc9e653
        let data = alloy_primitives::hex!("698c7e0dcbe3ddba9ddb8e76443f81da443d0432e532271e875dc79ab9e21568000000000000000000000000000000000000000000000000000000000000001bb814eaab5953337fed2cf504a5b887cddd65a54b7429d7b191ff1331ca0726b1264de2660d307112075c15f08ba9c25c9a0cc6f8119aff3e7efb0a942773abb0");
        console!("{:?}", &data);
        console!("{:?}", Call::new_in(self));
        let recovered = stylus_sdk::call::static_call(
            Call::new_in(self),
            ECRECOVER_ADDR,
            &data,
        )?;
        console!("{:?}", recovered);
        Ok(recovered)

        // let recovered = Address::from_slice(recovered.as_slice());
        // Ok(recovered)
    }
}
