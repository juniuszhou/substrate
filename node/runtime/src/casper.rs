/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as CasperModule {
		Epoch get(epoch): u32;
		Validators get(validators): map T::AccountId => bool;
		LastEpochValidator get(last_epoch_validator): u32;
		CurrentEpochValidator get(current_epoch_validator): u32;
		ExitedValidator get(exited_validator): u32;
		LastFinalizedEpoch get(last_finalized_epoch): u32;
		FinalizedHash get(finalized_hash): map u32 => Option<T::Hash>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		pub fn register(origin) -> Result {
			let _ = ensure_signed(origin)?;
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
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		NewValidator(AccountId),
	}
);

