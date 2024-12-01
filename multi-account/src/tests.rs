use crate::{mock::*, *};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::Zero;
//use frame_support::traits::fungible::Mutate;
use frame_support::traits::fungible::Inspect;

const ALICE: u64 = 1337;
const BOB: u64 = 2000;
const CHARLIE: u64 = 3000;
const OSCAR: u64 = 10000;
const FREE_BALANCE: u64 = 10_000_000;

#[test]
fn it_should_create_an_account_in_database() {
    new_test_ext().execute_with(|| {
        let origin = RuntimeOrigin::signed(1);
        let mut other_signatories = vec![2, 3, 4, 5, 6];
        let threshold = 2;
        let id = MultiAccount::multi_account_id(other_signatories.as_slice(), threshold);
        println!("id: {:?}", id);
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
    })
}

#[test]
fn it_should_create_a_call_succesfully_if_signatory() {
    new_test_ext().execute_with(|| {
        let origin = RuntimeOrigin::signed(1);
        let other_signatories = vec![2, 3, 4, 5, 6];
        let threshold = 2;
        let id = MultiAccount::multi_account_id(other_signatories.as_slice(), threshold);
        println!("id: {:?}", id);
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

#[test]
fn should_be_able_to_transfer_to_multi_sig_account() {
    new_test_ext().execute_with(|| {
        let origin = RuntimeOrigin::signed(ALICE);
        let mut other_signatories = vec![BOB, OSCAR, CHARLIE];
        other_signatories.sort();
        let threshold = 3;
        // create a multisig account
        let id = MultiAccount::multi_account_id(other_signatories.as_slice(), threshold);
        // register our multi sig account
        MultiAccount::register_account(origin.clone(), id, other_signatories.clone(), threshold)
            .expect("This should not have failed");
        let root: RuntimeOrigin = RuntimeOrigin::root();

        // Set the balance of our multi account to 10,000,000
        Balances::force_set_balance(root, id, FREE_BALANCE.into())
            .expect("Balance should have been set successfully");

        // confirm that the multi account has the balance that was just set
        let balance = Balances::balance(&id);
        assert_eq!(balance, FREE_BALANCE);

        let transfer_amount = 50000_u64;
        // perform a transfer to another account
        let call: Box<RuntimeCall> = Box::new(RuntimeCall::Balances(
            pallet_balances::Call::<Test>::transfer_keep_alive {
                dest: BOB,
                value: transfer_amount,
            },
        ));
        // create a call to transfer some balance to a BOB
        // first approval ✅ ✅ ✅
        assert_ok!(MultiAccount::account_create_call(
            origin.clone(),
            id,
            call.clone()
        ));
        // Alice created the call, so she should not be able to approve because she approves 
        // automatically at the point of creation.❌ ❌ ❌
        assert_noop!(
            MultiAccount::approve_or_dispatch_call(origin.clone(), id, call.clone()),
            crate::Error::<Test>::SenderInSignatories
        );
        // second approval from ✅ ✅ ✅
        assert_ok!(MultiAccount::approve_or_dispatch_call(RuntimeOrigin::signed(BOB), id, call.clone()));
        // Transaction should fail because bob has approves the call previously
        // ❌ ❌ ❌ 
        assert_noop!(
            MultiAccount::approve_or_dispatch_call(RuntimeOrigin::signed(BOB), id, call.clone()),
            crate::Error::<Test>::SenderInSignatories
        );
        // Charlie approves a transaction to be dispatched
        // ✅ ✅ ✅
        assert_ok!(MultiAccount::approve_or_dispatch_call(RuntimeOrigin::signed(CHARLIE), id, call.clone()));

        // Transaction should fail because the call has already been dispatched regardless of the caller ❌ ❌ ❌ 
        assert_noop!(
            MultiAccount::approve_or_dispatch_call(RuntimeOrigin::signed(BOB), id, call.clone()),
            crate::Error::<Test>::DispatchHasAlreadyOccured
        );

        // Confirm that a transaction actually occured by checking bob's balance ✅ ✅ ✅
        let bob_balance = Balances::balance(&BOB);
        assert_eq!(bob_balance,transfer_amount);

    })
}
