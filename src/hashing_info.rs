multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct HashInfo<M: ManagedTypeApi> {
    pub uploader_address: ManagedAddress<M>,
    pub hash_id: BigInt<M>
}

impl<M> HashInfo<M>
where 
    M: ManagedTypeApi,
{
    // pub fn get_total_data_points(&self) -> usize {
    //     let mut num = hash_total;
    //     if self.hash_total > 0 {
    //         num += 1;
    //     }

    //     num
    // }
}

// pub hash_total: BigInt; 