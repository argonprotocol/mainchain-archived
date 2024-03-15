#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use weights::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
// const LOG_TARGET: &str = "runtime::notaries";

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use log::warn;
	use sp_core::H256;
	use sp_runtime::{app_crypto::RuntimePublic, BoundedBTreeMap, Saturating};
	use sp_std::vec::Vec;

	use ulx_primitives::notary::{
		GenesisNotary, NotaryId, NotaryMeta, NotaryProvider, NotaryPublic, NotaryRecord,
		NotarySignature,
	};

	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	type NotaryRecordOf<T> = NotaryRecord<
		<T as frame_system::Config>::AccountId,
		BlockNumberFor<T>,
		<T as Config>::MaxNotaryHosts,
	>;
	type NotaryMetaOf<T> = NotaryMeta<<T as Config>::MaxNotaryHosts>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		/// The maximum active notaries allowed
		#[pallet::constant]
		type MaxActiveNotaries: Get<u32>;

		/// The maximum blocks a proposal can sit unapproved
		#[pallet::constant]
		type MaxProposalHoldBlocks: Get<u32>;

		#[pallet::constant]
		type MaxProposalsPerBlock: Get<u32>;

		/// Number of blocks to delay changing a notaries' meta
		#[pallet::constant]
		type MetaChangesBlockDelay: Get<u32>;

		/// Number of blocks to maintain key history for each notary
		/// NOTE: only pruned when new keys are added
		#[pallet::constant]
		type MaxBlocksForKeyHistory: Get<u32>;

		/// Maximum hosts a notary can supply
		#[pallet::constant]
		type MaxNotaryHosts: Get<u32>;
	}

	#[pallet::storage]
	pub(super) type NextNotaryId<T: Config> = StorageValue<_, u32, OptionQuery>;

	#[pallet::storage]
	pub(super) type ProposedNotaries<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		(NotaryMetaOf<T>, BlockNumberFor<T>),
		OptionQuery,
	>;

	#[pallet::storage]
	pub(super) type ExpiringProposals<T: Config> = StorageMap<
		_,
		Twox64Concat,
		BlockNumberFor<T>,
		BoundedVec<T::AccountId, T::MaxProposalsPerBlock>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn notaries)]
	pub(super) type ActiveNotaries<T: Config> =
		StorageValue<_, BoundedVec<NotaryRecordOf<T>, T::MaxActiveNotaries>, ValueQuery>;

	#[pallet::storage]
	pub(super) type NotaryKeyHistory<T: Config> = StorageMap<
		_,
		Twox64Concat,
		NotaryId,
		BoundedVec<(BlockNumberFor<T>, NotaryPublic), T::MaxBlocksForKeyHistory>,
		ValueQuery,
	>;

	#[pallet::storage]
	pub(super) type QueuedNotaryMetaChanges<T: Config> = StorageMap<
		_,
		Twox64Concat,
		BlockNumberFor<T>,
		BoundedBTreeMap<NotaryId, NotaryMetaOf<T>, T::MaxActiveNotaries>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user has proposed operating as a notary
		NotaryProposed {
			operator_account: T::AccountId,
			meta: NotaryMetaOf<T>,
			expires: BlockNumberFor<T>,
		},
		/// A notary proposal has been accepted
		NotaryActivated { notary: NotaryRecordOf<T> },
		/// Notary metadata queued for update
		NotaryMetaUpdateQueued {
			notary_id: NotaryId,
			meta: NotaryMetaOf<T>,
			effective_block: BlockNumberFor<T>,
		},
		/// Notary metadata updated
		NotaryMetaUpdated { notary_id: NotaryId, meta: NotaryMetaOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		ProposalNotFound,
		MaxNotariesExceeded,
		MaxProposalsPerBlockExceeded,
		NotAnActiveNotary,
		InvalidNotaryOperator,
		NoMoreNotaryIds,
	}

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub list: Vec<GenesisNotary<T::AccountId>>,
		pub _phantom: PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			for notary in self.list.iter() {
				Pallet::<T>::activate_notary(
					notary.account_id.clone(),
					NotaryMeta {
						public: notary.public.clone(),
						hosts: BoundedVec::truncate_from(notary.hosts.clone()),
					},
					0u32.into(),
				)
				.unwrap();
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			let meta_changes = QueuedNotaryMetaChanges::<T>::take(n);
			if meta_changes.len() > 0 {
				let old_block_to_preserve =
					n.saturating_sub(T::MaxBlocksForKeyHistory::get().into());
				let _ = <ActiveNotaries<T>>::try_mutate(|active| -> DispatchResult {
					for (notary_id, meta) in meta_changes.into_iter() {
						if let Some(pos) = active.iter().position(|n| n.notary_id == notary_id) {
							active[pos].meta = meta.clone();
							active[pos].meta_updated_block = n;
							if let Err(e) =
								<NotaryKeyHistory<T>>::try_mutate(notary_id, |history| {
									history.retain(|(block, _)| *block >= old_block_to_preserve);
									history.try_push((n, meta.public.clone()))
								}) {
								warn!("Failed to update notary key history: {:?} {notary_id:?}", e);
							}
							Self::deposit_event(Event::NotaryMetaUpdated { notary_id, meta });
						} else {
							warn!(
								"Invalid notary meta queued (id={:?}) at block {:?}",
								notary_id, n
							);
						}
					}
					Ok(())
				})
				.map_err(|err| warn!("Failed to update notary meta: {:?} at block {:?}", err, n));
			}
			T::DbWeight::get().reads_writes(2, 1)
		}

		fn on_finalize(n: BlockNumberFor<T>) {
			let expiring = ExpiringProposals::<T>::take(n);
			for proposed_operator in expiring.into_iter() {
				ProposedNotaries::<T>::remove(proposed_operator);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn propose(origin: OriginFor<T>, meta: NotaryMetaOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let already_exists = <ProposedNotaries<T>>::take(&who);

			let expiration_block = match already_exists {
				Some(x) => x.1,
				None => {
					let expires = frame_system::Pallet::<T>::block_number() +
						T::MaxProposalHoldBlocks::get().into();
					<ExpiringProposals<T>>::try_append(&expires, &who)
						.map_err(|_| Error::<T>::MaxProposalsPerBlockExceeded)?;
					expires
				},
			};

			<ProposedNotaries<T>>::insert(&who, (&meta, &expiration_block));

			Self::deposit_event(Event::NotaryProposed {
				operator_account: who,
				meta,
				expires: expiration_block,
			});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn activate(origin: OriginFor<T>, operator_account: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			let (proposal, expiration_block) = <ProposedNotaries<T>>::take(&operator_account)
				.ok_or(Error::<T>::ProposalNotFound)?;

			<ExpiringProposals<T>>::try_mutate(expiration_block, |proposals| {
				if let Some(pos) = proposals.iter().position(|x| x == &operator_account) {
					proposals.remove(pos);
				}
				Ok::<_, DispatchError>(())
			})?;

			let block_number = frame_system::Pallet::<T>::block_number();

			Self::activate_notary(operator_account, proposal, block_number)?;

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn update(
			origin: OriginFor<T>,
			#[pallet::compact] notary_id: NotaryId,
			meta: NotaryMetaOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let notary = <ActiveNotaries<T>>::get()
				.into_iter()
				.find(|n| n.notary_id == notary_id)
				.ok_or(Error::<T>::NotAnActiveNotary)?;

			ensure!(notary.operator_account_id == who, Error::<T>::InvalidNotaryOperator);

			let meta_change: BlockNumberFor<T> = T::MetaChangesBlockDelay::get().into();

			let effective_block = meta_change + frame_system::Pallet::<T>::block_number();

			<QueuedNotaryMetaChanges<T>>::try_mutate(effective_block.clone(), |changes| {
				changes
					.try_insert(notary_id, meta.clone())
					// shouldn't be possible.
					.map_err(|_| Error::<T>::MaxNotariesExceeded)?;
				Ok::<_, Error<T>>(())
			})?;
			Self::deposit_event(Event::NotaryMetaUpdateQueued { notary_id, meta, effective_block });

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn activate_notary(
			operator_account: T::AccountId,
			meta: NotaryMetaOf<T>,
			block_number: BlockNumberFor<T>,
		) -> DispatchResult {
			let notary_id = Self::next_notary_id()?;

			let public = meta.public.clone();

			let notary = NotaryRecord {
				notary_id,
				operator_account_id: operator_account.clone(),
				activated_block: block_number,
				meta_updated_block: block_number,
				meta,
			};

			<ActiveNotaries<T>>::try_mutate(|active| -> DispatchResult {
				active.try_push(notary.clone()).map_err(|_| Error::<T>::MaxNotariesExceeded)?;
				Ok(())
			})?;
			<NotaryKeyHistory<T>>::try_append(notary_id, (block_number, public))
				.map_err(|_| Error::<T>::MaxNotariesExceeded)?;

			Self::deposit_event(Event::NotaryActivated { notary });

			Ok(())
		}

		fn next_notary_id() -> Result<u32, Error<T>> {
			let notary_id = NextNotaryId::<T>::get()
				.or(Some(1u32.into()))
				.ok_or(Error::<T>::NoMoreNotaryIds)?;
			let next_notary_id = notary_id.checked_add(1).ok_or(Error::<T>::NoMoreNotaryIds)?;
			NextNotaryId::<T>::set(Some(next_notary_id));
			Ok(notary_id)
		}
	}

	impl<T: Config> NotaryProvider<T::Block> for Pallet<T> {
		fn verify_signature(
			notary_id: NotaryId,
			at_block_height: BlockNumberFor<T>,
			message: &H256,
			signature: &NotarySignature,
		) -> bool {
			let key_history = <NotaryKeyHistory<T>>::get(notary_id);

			// find the first key that is valid at the given block height
			let mut public = key_history
				.iter()
				.find(|(block, _)| *block >= at_block_height)
				.map(|(_, public)| public);
			if public.is_none() && key_history.len() > 0 {
				if key_history[0].0 < at_block_height {
					public = key_history.first().map(|(_, public)| public);
				}
			}

			if let Some(public) = public {
				return public.verify(message, &signature)
			}
			false
		}

		fn active_notaries() -> Vec<NotaryId> {
			<ActiveNotaries<T>>::get().into_iter().map(|n| n.notary_id).collect()
		}
	}
}
