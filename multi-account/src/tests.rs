use crate::{mock::*, *};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::Zero;



pub type MultiSigAccount = (u64, Vec<u64>, u16);

#[test]
fn it_should_create_an_account_in_database() {
    new_test_ext().execute_with(|| {
        let origin = RuntimeOrigin::signed(1);
        let mut other_signatories = vec![2, 3, 4, 5, 6];
        let threshold = 2;
        let id = MultiAccount::multi_account_id(other_signatories.as_slice(), threshold);
        //frame::log::debug!("id: {:?}", id);

        assert_ok!(MultiAccount::register_account(
            origin,
            id,
            other_signatories.clone(),
            threshold
        ));
        other_signatories.insert(0, 1);
        let event = Event::Account {
            id,
            signatories: other_signatories,
            threshold,
        };
        assert!(
            !frame_system::Pallet::<Test>::block_number().is_zero(),
            "The genesis block has no events"
        );
        frame_system::Pallet::<Test>::finalize();
        frame_system::Pallet::<Test>::assert_has_event(event.clone().into());
        frame_system::Pallet::<Test>::assert_last_event(event.into());
        create_a_multisig_account();
    })
}

#[test]
fn should_be_able_to_transfer_to_multi_sig_account(){
    new_test_ext().execute_with(|| {
        //frame_system::Pallet::<Test>
    })
}

#[test]
fn it_should_create_a_call_succesfully_if_signatory() {
    new_test_ext().execute_with(|| {
        let origin = RuntimeOrigin::signed(1);
        let other_signatories = vec![2, 3, 4, 5, 6];
        let threshold = 2;
        let id = MultiAccount::multi_account_id(other_signatories.as_slice(), threshold);
        //frame::log::debug!("id: {:?}", id);
        // register an account and its signatories
        MultiAccount::register_account(origin.clone(), id, other_signatories.clone(), threshold)
            .expect("This should not fail under no circumstance");
        // the derive call macro creates an enum named Call in a pallet crate that takes a generic of T <runtime>;
        // so the way to access a runtime call from the crate, is to chain the series of enums.
        let call = Box::new(RuntimeCall::System(frame_system::Call::<Test>::remark {
            remark: vec![42, 34, 23, 78],
        }));
        //let runtime_call = RuntimeCall::MultiAccount(crate::Call::<Test>::register_account { id: (), other_signatories: (), threshold: () });
        assert_ok!(MultiAccount::account_create_call(origin, id, call));
    })
}

#[test]
fn should_fail_to_create_a_call_if_not_signatory() {
    new_test_ext().execute_with(|| {
        let origin = RuntimeOrigin::signed(1);
        let other_signatories = vec![2, 3, 4, 5, 6];
        let threshold = 2;
        let id = MultiAccount::multi_account_id(other_signatories.as_slice(), threshold);
        // register an account and its signatories
        MultiAccount::register_account(origin.clone(), id, other_signatories.clone(), threshold)
            .expect("This should not fail under no circumstance");
        let call = Box::new(RuntimeCall::System(frame_system::Call::<Test>::remark {
            remark: vec![42, 34, 23, 78],
        }));
        let attacker = RuntimeOrigin::signed(89);
        assert!(
            MultiAccount::account_create_call(attacker.clone(), id.clone(), call.clone()).is_err()
        );
        assert_noop!(
            MultiAccount::account_create_call(attacker, id, call),
            crate::Error::<Test>::SignerIsNotApproved
        );
    })
}

fn create_a_multisig_account() -> MultiSigAccount {
    new_test_ext().execute_with(|| {
        let origin = RuntimeOrigin::signed(1);
        let other_signatories = vec![2, 3, 4, 5, 6];
        let threshold = 2;
        let id = MultiAccount::multi_account_id(other_signatories.as_slice(), threshold);
        //debug!("id: {:?}", id);
        MultiAccount::register_account(origin, id, other_signatories.clone(), threshold)
            .expect("This should not have failed");
        (id, other_signatories, threshold)
    })
}
