use crate::frame_system::{mocking::MockBlock, GenesisConfig};
use frame::deps::sp_io;
use frame::{
    deps::frame_support::{derive_impl, runtime, weights::constants::RocksDbWeight},
    runtime::prelude::*,
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

    #[runtime::pallet_index(0)]
    pub type System = frame_system;

    #[runtime::pallet_index(1)]
    pub type MultiAccount = crate;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Nonce = u64;
    type Block = MockBlock<Test>;
    type BlockHashCount = ConstU64<250>;
    type DbWeight = RocksDbWeight;
}

impl crate::Config for Test {
    type WeightInfo = ();
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
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
