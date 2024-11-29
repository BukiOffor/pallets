# Multi-Signature Pallet Documentation

## Overview

This pallet facilitates multi-signature account management, allowing multiple signatories to collectively control an account. The pallet ensures that certain operations, such as dispatching calls, are only executed if the required signature threshold is met. 

Key features include:
- Creation of multi-signature accounts.
- Storage of account metadata (signatories and thresholds).
- Tracking of call approvals.
- Dispatch of approved calls.

## Configuration

The pallet requires the following types and constants to be defined in the runtime configuration:

### `Config` Trait

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    type WeightInfo;  // Represents the weight information.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type RuntimeCall: Parameter
        + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin, PostInfo = PostDispatchInfo>
        + GetDispatchInfo
        + From<frame_system::Call<Self>>;
    type MaxSignatories: Get<u32>;  // Maximum number of allowed signatories.
}
```

## Storage Items

1. **`Account`**: Stores the list of signatories for each multi-signature account.
2. **`Threshold`**: Stores the signature threshold for each multi-signature account.
3. **`Calls`**: Tracks approvals for pending calls.
4. **`Executed`**: Tracks executed calls to prevent re-execution.

```rust
#[pallet::storage]
#[pallet::getter(fn get_account)]
pub type Account<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    BoundedVec<T::AccountId, T::MaxSignatories>,
    ValueQuery,
>;

#[pallet::storage]
#[pallet::getter(fn get_threshold)]
pub type Threshold<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u16, ValueQuery>;

#[pallet::storage]
pub type Calls<T: Config> = StorageDoubleMap<
    _,
    Twox64Concat,
    T::AccountId,
    Blake2_128Concat,
    CallHash,
    BoundedVec<T::AccountId, ConstU32<100>>,
    ValueQuery,
>;

#[pallet::storage]
pub type Executed<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    CallHash,
    (),
    ValueQuery,
>;
```

## Events

The pallet emits events to signal key actions:

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    Account {
        id: T::AccountId,
        signatories: Vec<T::AccountId>,
        threshold: u16,
    },
}
```

## Errors

Custom errors for handling failure cases:

```rust
#[pallet::error]
pub enum Error<T> {
    MinimumThreshold,
    TooFewSignatories,
    TooManySignatories,
    SignatoriesOutOfOrder,
    SenderInSignatories,
    NotFound,
    SignerIsNotApproved,
}
```

## Callable Functions

1. **`register_account`**: Registers a multi-signature account with a set of signatories and a signature threshold.
   
2. **`account_create_call`**: Records a call initiated by a multi-signature account.

3. **`approve_or_dispatch_call`**: Approves a call and dispatches it if the signature threshold is met.

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn register_account(origin: OriginFor<T>, id: T::AccountId, other_signatories: Vec<T::AccountId>, threshold: u16) -> DispatchResult { ... }

    pub fn account_create_call(origin: OriginFor<T>, id: T::AccountId, call: Box<<T as Config>::RuntimeCall>) -> DispatchResult { ... }

    pub fn approve_or_dispatch_call(origin: OriginFor<T>, id: T::AccountId, call: Box<<T as Config>::RuntimeCall>) -> DispatchResult { ... }
}
```

## Utility Functions

1. **`multi_account_id`**: Derives a unique account ID from signatories and threshold.
2. **`ensure_sorted_and_insert`**: Ensures signatories are sorted and inserts a new signatory.

```rust
impl<T: Config> Pallet<T> {
    pub fn multi_account_id(who: &[T::AccountId], threshold: u16) -> T::AccountId { ... }

    fn ensure_sorted_and_insert(other_signatories: Vec<T::AccountId>, who: T::AccountId) -> Result<Vec<T::AccountId>, DispatchError> { ... }
}
```

## Conclusion

This pallet ensures secure, threshold-based multi-signature account management. It stores account metadata, tracks call approvals, and dispatches calls upon reaching the required approval threshold.