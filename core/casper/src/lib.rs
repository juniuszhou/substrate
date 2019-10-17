extern crate futures;
extern crate futures_timer;

use std::time::{Instant, Duration};
use std::sync::Arc;
use futures::prelude::*;
use futures_timer::Delay;
use substrate_casper_primitives::*;
use sr_primitives::traits::{NumberFor, Block as BlockT, ProvideRuntimeApi};
use sr_primitives::{generic::{BlockId}};
use client::{
	block_builder::api::BlockBuilder as BlockBuilderApi,
	blockchain::{self, HeaderBackend, ProvideCache}, BlockchainEvents, CallExecutor, Client,
	runtime_api::ApiExt, error::Result as ClientResult, backend::{AuxStore, Backend},
	ProvideUncles,
	utils::is_descendent_of,
};

mod casper_slots;

/// Extract current epoch from runtime.
fn finalized_epoch_from_runtime<B, C>(client: &C, at: &BlockId<B>) -> Option<u32> where
	B: BlockT,
	C: ProvideRuntimeApi,
	C::Api: CasperApi<B>,
{
	if client.runtime_api().has_api::<dyn CasperApi<B>>(at).unwrap_or(false) {
		let s = CasperApi::finalized_epoch(&*client.runtime_api(), at).ok()?;
			Some(s)	
	} else {
		None
	}
}

pub struct CasperEngine<C> {
	pub client: Arc<C>,
}

impl<C> CasperEngine<C> {
	pub fn new(client: Arc<C>) -> Self {
		CasperEngine {
			client,
		}
	}

	pub fn try_method(&self) {
		// finalized_epoch_from_runtime(&*self.inner, BlockId::default())
		// CasperApi::finalized_epoch(&*self.inner.runtime_api(), at);

		// if &self.inner.runtime_api().has_api::<dyn CasperApi<B>>(at).unwrap_or(false) {
		// 	finalized_epoch_from_runtime(&self.inner, BlockId::default())
		// }
	}

	// fn apply_finality(hash: Block::Hash, number: NumberFor<Block>) {

	// 	use client::blockchain::HeaderBackend;

	// 	#[allow(deprecated)]
	// 	let blockchain = self.inner.backend().blockchain();
	// 	let info = blockchain.info();
	// 	if number <= info.finalized_number && blockchain.hash(number)? == Some(hash) {
	// 		warn!(target: "afg",
	// 			"Re-finalized block #{:?} ({:?}) in the canonical chain, current best finalized is #{:?}",
	// 			hash,
	// 			number,
	// 			info.finalized_number,
	// 		);

	// 		return Ok(());
	// 	}


	// 	let update_res: Result<_, Error> = client.lock_import_and_run(|import_op| {
	// 	let status = authority_set.apply_standard_changes(
	// 		hash,
	// 		number,
	// 		&is_descendent_of(client, None),
	// 	).map_err(|e| Error::Safety(e.to_string()))?;

	// 	debug!(target: "afg", "Finalizing blocks up to ({:?}, {})", number, hash);

	// 	// ideally some handle to a synchronization oracle would be used
	// 	// to avoid unconditionally notifying.
	// 	client.apply_finality(import_op, BlockId::Hash(hash), justification, true).map_err(|e| {
	// 		warn!(target: "finality", "Error applying finality to block {:?}: {:?}", (hash, number), e);
	// 		e
	// 	})?;
	// 	telemetry!(CONSENSUS_INFO; "afg.finalized_blocks_up_to";
	// 		"number" => ?number, "hash" => ?hash,
	// 	);

	// 	let new_authorities = if let Some((canon_hash, canon_number)) = status.new_set_block {
	// 		// the authority set has changed.
	// 		let (new_id, set_ref) = authority_set.current();

	// 		if set_ref.len() > 16 {
	// 			info!("Applying GRANDPA set change to new set with {} authorities", set_ref.len());
	// 		} else {
	// 			info!("Applying GRANDPA set change to new set {:?}", set_ref);
	// 		}

	// 		telemetry!(CONSENSUS_INFO; "afg.generating_new_authority_set";
	// 			"number" => ?canon_number, "hash" => ?canon_hash,
	// 			"authorities" => ?set_ref.to_vec(),
	// 			"set_id" => ?new_id,
	// 		);
	// 		Some(NewAuthoritySet {
	// 			canon_hash,
	// 			canon_number,
	// 			set_id: new_id,
	// 			authorities: set_ref.to_vec(),
	// 		})
	// 	} else {
	// 		None
	// 	};

	// 	if status.changed {
	// 		let write_result = crate::aux_schema::update_authority_set::<Block, _, _>(
	// 			&authority_set,
	// 			new_authorities.as_ref(),
	// 			|insert| client.apply_aux(import_op, insert, &[]),
	// 		);

	// 		if let Err(e) = write_result {
	// 			warn!(target: "finality", "Failed to write updated authority set to disk. Bailing.");
	// 			warn!(target: "finality", "Node is in a potentially inconsistent state.");

	// 			return Err(e.into());
	// 		}
	// 	}

	// 	Ok(new_authorities.map(VoterCommand::ChangeAuthorities))
	// });
	// }
}


pub fn start_casper() {
	println!("start");
	let slots = casper_slots::CasperSlots::new(30000);
	let f = slots.for_each(|slot_number| {
        println!("Casper current slot is {:?}", slot_number);
		futures::future::ok(())
	});

	tokio::run(f);
	println!("over");
}


