use support::{decl_module, decl_storage, decl_event, dispatch::Result, traits::Currency};
use system::{ensure_signed, ensure_root};
use node_primitives::{
	AccountId
};
// use balances;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Currency: Currency<Self::AccountId>;
}

pub const DEPOSIT_VALUE: u128 = 1_000_000_000_000_000_000;

decl_storage! {
	trait Store for Module<T: Trait> as CasperModule {
		pub Owner get(owner): T::AccountId;
		pub Epoch get(epoch): u32;
		pub Validators get(validators): map T::AccountId => bool;
		pub LastEpochValidator get(last_epoch_validator): u32;
		pub CurrentEpochValidator get(current_epoch_validator): u32;
		pub ExitedValidator get(exited_validator): u32;
		pub LastFinalizedEpoch get(last_finalized_epoch): u32;
		pub FinalizedHash get(finalized_hash): map u32 => Option<T::Hash>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		pub fn register(origin) -> Result {
			let sender = ensure_signed(origin)?;
			T::Currency::transfer(&sender, &Self::owner(), 1_000_000_000.into())?;
			Ok(())
		}
		pub fn vote(origin) -> Result {
			let _ = ensure_signed(origin)?;
			Ok(())
		}
		pub fn exit(origin) -> Result {
			let _ = ensure_signed(origin)?;
			Ok(())
		}
		pub fn change_owner(origin, new_owner: T::AccountId) -> Result {
			let _ = ensure_root(origin)?;
			<Owner<T>>::put(new_owner);
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		NewValidator(AccountId),
	}
);

