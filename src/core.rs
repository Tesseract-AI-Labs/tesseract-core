#![no_std]
#![allow(unused_attributes)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod hashing_info;

use hashing_info::HashInfo;

static ZERO_ADDRESS: &[u8] = b"zero address";
static INVALID_ADDRESS: &[u8] = b"invalid address";

#[multiversx_sc::contract]
pub trait TesseractContract {
    #[init]
    fn init(&self) {

    }

    #[endpoint]
    fn digest(
        &self, 
        address: ManagedAddress, 
        uri: ManagedBuffer,
        omnikey: ManagedAddress
    ) {
        let data_mapper = self.logger(&address);
        require!(!data_mapper.is_empty(), INVALID_ADDRESS);

        let hash = self.generate_hash(uri);

        data_mapper.update(|logger| {
                logger.omnikey_address = omnikey;
                logger.hash = hash;
            }
        )
    }

    fn generate_hash(
        &self, 
        data: ManagedBuffer
    ) -> ManagedByteArray<Self::Api, 32> {
        self.crypto().keccak256(data)
    }


    #[view]
    #[storage_mapper("logger")]
    fn logger(&self, uploader: &ManagedAddress) -> SingleValueMapper<HashInfo<Self::Api>>;

}
