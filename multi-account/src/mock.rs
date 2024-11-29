use frame_system::{mocking::MockBlock, GenesisConfig};
use frame_support::derive_impl;
use frame_support::sp_runtime::BuildStorage;
pub use crate as multi_account;
use pallet_balances;
use frame_system;
use frame_support::parameter_types;


type Block = MockBlock<Test>;

parameter_types!{
    pub const MaxSignatories:u32 = 100;
}

frame_support::construct_runtime!(
	pub enum Test{
		System: frame_system,
		Balances: pallet_balances,
        MultiAccount: multi_account,
	}
);


#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<u64>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
	type AccountStore = System;
}

impl multi_account::Config for Test {
	type WeightInfo = ();
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    //type MaxSignatories = frame_support::traits::ConstU32<100>;
    type MaxSignatories = MaxSignatories;

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
