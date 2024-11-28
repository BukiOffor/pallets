use crate::frame_system::{mocking::MockBlock, GenesisConfig};
use frame::deps::sp_io;
use frame::{
    deps::frame_support::{derive_impl, runtime, weights::constants::RocksDbWeight},
    testing_prelude::*,
};




#[runtime]
mod test_runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    parameter_type!{
        pub const MaxSignatories:u32 = 100;
    }

    #[runtime::pallet_index(0)]
    pub type System = frame_system;
    
    #[runtime::pallet_index(1)]
    pub type Balances = pallet_balances;

    #[runtime::pallet_index(2)]
    pub type MultiAccount = crate;

   

}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = MockBlock<Test>;
    type DbWeight = RocksDbWeight;
    type RuntimeCall = RuntimeCall;
    type AccountData = pallet_balances::AccountData<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
}

impl crate::Config for Test {
    type WeightInfo = ();
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    //type MaxSignatories = frame_support::traits::ConstU32<100>;
    type MaxSignatories = MaxSignatories;

}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
	type AccountStore = frame_system::Pallet<Test>;
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();


}


// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
