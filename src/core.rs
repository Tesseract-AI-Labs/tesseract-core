#![no_std]
#![allow(unused_attributes)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod hashing_info;

use hashing_info::HashInfo;

static ZERO_ADDRESS: &[u8] = b"zero address";

#[multiversx_sc::contract]
pub trait TesseractContract {
    #[init]
    fn init(&self) {

    }

    #[endpoint]
    fn digest(&self, address: ManagedAddress) {
        let address_mapper = self.logger(&address);
    }


    #[view]
    #[storage_mapper("logger")]
    fn logger(&self, uploader: &ManagedAddress) -> SingleValueMapper<HashInfo<Self::Api>>;

}
