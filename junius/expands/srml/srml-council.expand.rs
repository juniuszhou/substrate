#![feature(prelude_import)]
#![no_std]
// Copyright 2017-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Council system: Handles the voting in and maintenance of council members.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


pub mod voting {


    // These re-exports are here for a reason, edit with care




    // Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.


    //! Council voting system.
    use rstd::prelude::*;
    use rstd::borrow::Borrow;
    use primitives::traits::{Hash, As, Zero};
    use runtime_io::print;
    use srml_support::dispatch::Result;
    use srml_support::{StorageValue, StorageMap, IsSubType, decl_module,
                       decl_storage, decl_event, ensure};
    use {system, democracy};
    use super::{Trait as CouncilTrait, Module as Council};
    use system::ensure_signed;
    pub trait Trait: CouncilTrait {
        type
        Event: From<Event<Self>> +
        Into<<Self as system::Trait>::Event>;
    }
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub struct Module<T: Trait>(::srml_support::rstd::marker::PhantomData<(T)>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::clone::Clone + Trait> ::std::clone::Clone for Module<T> {
        #[inline]
        fn clone(&self) -> Module<T> {
            match *self {
                Module(ref __self_0_0) =>
                Module(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::marker::Copy + Trait> ::std::marker::Copy for Module<T> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::cmp::PartialEq + Trait> ::std::cmp::PartialEq for
     Module<T> {
        #[inline]
        fn eq(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) =>
                match *self {
                    Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) =>
                match *self {
                    Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::cmp::Eq + Trait> ::std::cmp::Eq for Module<T> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _:
                        ::std::cmp::AssertParamIsEq<::srml_support::rstd::marker::PhantomData<(T)>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::fmt::Debug + Trait> ::std::fmt::Debug for Module<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Module(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Module");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl <T: Trait>
     ::srml_support::runtime_primitives::traits::OnInitialize<T::BlockNumber>
     for Module<T> {
    }
    impl <T: Trait>
     ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber>
     for Module<T> {
        fn on_finalize(n: T::BlockNumber) {
            if let Err(e) = Self::end_block(n) {
                print("Guru meditation");
                print(e);
            }
        }
    }
    impl <T: Trait>
     ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
     for Module<T> {
    }
    impl <T: Trait> Module<T> {
        fn deposit_event(event: Event<T>) {
            <system::Module<T>>::deposit_event(<T as
                                                   Trait>::from(event).into());
        }
    }
    /// Can also be called using [`Call`].
    ///
    /// [`Call`]: enum.Call.html
    impl <T: Trait> Module<T> {
        fn propose(origin: T::Origin, proposal: Box<T::Proposal>)
         -> ::srml_support::dispatch::Result {
            {
                let who = ensure_signed(origin)?;
                let expiry =
                    <system::Module<T>>::block_number() +
                        Self::voting_period();
                {
                    if !Self::will_still_be_councillor_at(&who, expiry) {
                        { return Err("proposer would not be on council"); };
                    }
                };
                let proposal_hash = T::Hashing::hash_of(&proposal);
                {
                    if !!<ProposalOf<T>>::exists(proposal_hash) {
                        { return Err("duplicate proposals not allowed"); };
                    }
                };
                {
                    if !!Self::is_vetoed(&proposal_hash) {
                        { return Err("proposal is vetoed"); };
                    }
                };
                let mut proposals = Self::proposals();
                proposals.push((expiry, proposal_hash));
                proposals.sort_by_key(|&(expiry, _)| expiry);
                Self::set_proposals(&proposals);
                <ProposalOf<T>>::insert(proposal_hash, *proposal);
                <ProposalVoters<T>>::insert(proposal_hash,
                                            <[_]>::into_vec(box
                                                                [who.clone()]));
                <CouncilVoteOf<T>>::insert((proposal_hash, who.clone()),
                                           true);
            }
            Ok(())
        }
        fn vote(origin: T::Origin, proposal: T::Hash, approve: bool)
         -> ::srml_support::dispatch::Result {
            {
                let who = ensure_signed(origin)?;
                {
                    if !Self::is_councillor(&who) {
                        {
                            return Err("only councillors may vote on council proposals");
                        };
                    }
                };
                if Self::vote_of((proposal, who.clone())).is_none() {
                    <ProposalVoters<T>>::mutate(proposal,
                                                |voters|
                                                    voters.push(who.clone()));
                }
                <CouncilVoteOf<T>>::insert((proposal, who), approve);
            }
            Ok(())
        }
        fn veto(origin: T::Origin, proposal_hash: T::Hash)
         -> ::srml_support::dispatch::Result {
            {
                let who = ensure_signed(origin)?;
                {
                    if !Self::is_councillor(&who) {
                        {
                            return Err("only councillors may veto council proposals");
                        };
                    }
                };
                {
                    if !<ProposalVoters<T>>::exists(&proposal_hash) {
                        { return Err("proposal must exist to be vetoed"); };
                    }
                };
                let mut existing_vetoers =
                    Self::veto_of(&proposal_hash).map(|pair|
                                                          pair.1).unwrap_or_else(Vec::new);
                let insert_position =
                    existing_vetoers.binary_search(&who).err().ok_or("a councillor may not veto a proposal twice")?;
                existing_vetoers.insert(insert_position, who);
                Self::set_veto_of(&proposal_hash,
                                  <system::Module<T>>::block_number() +
                                      Self::cooloff_period(),
                                  existing_vetoers);
                Self::set_proposals(&Self::proposals().into_iter().filter(|&(_,
                                                                             h)|
                                                                              h
                                                                                  !=
                                                                                  proposal_hash).collect::<Vec<_>>());
                <ProposalVoters<T>>::remove(proposal_hash);
                <ProposalOf<T>>::remove(proposal_hash);
                for (c, _) in <Council<T>>::active_council() {
                    <CouncilVoteOf<T>>::remove((proposal_hash, c));
                }
            }
            Ok(())
        }
        fn set_cooloff_period(blocks: T::BlockNumber)
         -> ::srml_support::dispatch::Result {
            { <CooloffPeriod<T>>::put(blocks); }
            Ok(())
        }
        fn set_voting_period(blocks: T::BlockNumber)
         -> ::srml_support::dispatch::Result {
            { <VotingPeriod<T>>::put(blocks); }
            Ok(())
        }
    }
    pub enum Call<T: Trait> {

        #[doc(hidden)]
        #[codec(skip)]
        __PhantomItem(::srml_support::rstd::marker::PhantomData<(T)>,
                      ::srml_support::dispatch::Never),

        #[allow(non_camel_case_types)]
        propose(Box<T::Proposal>),

        #[allow(non_camel_case_types)]
        vote(T::Hash, bool),

        #[allow(non_camel_case_types)]
        veto(T::Hash),

        #[allow(non_camel_case_types)]
        set_cooloff_period(
                           #[codec(compact)]
                           T::BlockNumber),

        #[allow(non_camel_case_types)]
        set_voting_period(
                          #[codec(compact)]
                          T::BlockNumber),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Call: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <T: Trait> _parity_codec::Encode for Call<T> where
             Box<T::Proposal>: _parity_codec::Encode,
             Box<T::Proposal>: _parity_codec::Encode,
             T::Hash: _parity_codec::Encode, T::Hash: _parity_codec::Encode,
             T::Hash: _parity_codec::Encode, T::Hash: _parity_codec::Encode,
             T::BlockNumber: _parity_codec::HasCompact,
             T::BlockNumber: _parity_codec::HasCompact {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        Call::propose(ref aa) => {
                            dest.push_byte(0usize as u8);
                            dest.push(aa);
                        }
                        Call::vote(ref aa, ref ba) => {
                            dest.push_byte(1usize as u8);
                            dest.push(aa);
                            dest.push(ba);
                        }
                        Call::veto(ref aa) => {
                            dest.push_byte(2usize as u8);
                            dest.push(aa);
                        }
                        Call::set_cooloff_period(ref aa) => {
                            dest.push_byte(3usize as u8);
                            {
                                dest.push(&<<T::BlockNumber as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          T::BlockNumber>>::from(aa));
                            }
                        }
                        Call::set_voting_period(ref aa) => {
                            dest.push_byte(4usize as u8);
                            {
                                dest.push(&<<T::BlockNumber as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          T::BlockNumber>>::from(aa));
                            }
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Call: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <T: Trait> _parity_codec::Decode for Call<T> where
             Box<T::Proposal>: _parity_codec::Decode,
             Box<T::Proposal>: _parity_codec::Decode,
             T::Hash: _parity_codec::Decode, T::Hash: _parity_codec::Decode,
             T::Hash: _parity_codec::Decode, T::Hash: _parity_codec::Decode,
             T::BlockNumber: _parity_codec::HasCompact,
             T::BlockNumber: _parity_codec::HasCompact {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 0usize as u8 => {
                            Some(Call::propose(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 1usize as u8 => {
                            Some(Call::vote(_parity_codec::Decode::decode(input)?,
                                            _parity_codec::Decode::decode(input)?))
                        }
                        x if x == 2usize as u8 => {
                            Some(Call::veto(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 3usize as u8 => {
                            Some(Call::set_cooloff_period(<<T::BlockNumber as
                                                           _parity_codec::HasCompact>::Type
                                                              as
                                                              _parity_codec::Decode>::decode(input)?.into()))
                        }
                        x if x == 4usize as u8 => {
                            Some(Call::set_voting_period(<<T::BlockNumber as
                                                          _parity_codec::HasCompact>::Type
                                                             as
                                                             _parity_codec::Decode>::decode(input)?.into()))
                        }
                        _ => None,
                    }
                }
            }
        };
    impl <T: Trait> ::srml_support::dispatch::Clone for Call<T> {
        fn clone(&self) -> Self {
            match *self {
                Call::propose(ref proposal) =>
                Call::propose((*proposal).clone()),
                Call::vote(ref proposal, ref approve) =>
                Call::vote((*proposal).clone(), (*approve).clone()),
                Call::veto(ref proposal_hash) =>
                Call::veto((*proposal_hash).clone()),
                Call::set_cooloff_period(ref blocks) =>
                Call::set_cooloff_period((*blocks).clone()),
                Call::set_voting_period(ref blocks) =>
                Call::set_voting_period((*blocks).clone()),
                _ => {
                    {
                        ::std::rt::begin_panic("internal error: entered unreachable code",
                                               &("srml/council/src/voting.rs",
                                                 33u32, 1u32))
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
        fn eq(&self, _other: &Self) -> bool {
            match *self {
                Call::propose(ref proposal) => {
                    let self_params = (proposal,);
                    if let Call::propose(ref proposal) = *_other {
                        self_params == (proposal,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/voting.rs",
                                                             33u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::vote(ref proposal, ref approve) => {
                    let self_params = (proposal, approve);
                    if let Call::vote(ref proposal, ref approve) = *_other {
                        self_params == (proposal, approve)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/voting.rs",
                                                             33u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::veto(ref proposal_hash) => {
                    let self_params = (proposal_hash,);
                    if let Call::veto(ref proposal_hash) = *_other {
                        self_params == (proposal_hash,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/voting.rs",
                                                             33u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::set_cooloff_period(ref blocks) => {
                    let self_params = (blocks,);
                    if let Call::set_cooloff_period(ref blocks) = *_other {
                        self_params == (blocks,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/voting.rs",
                                                             33u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::set_voting_period(ref blocks) => {
                    let self_params = (blocks,);
                    if let Call::set_voting_period(ref blocks) = *_other {
                        self_params == (blocks,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/voting.rs",
                                                             33u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                _ => {
                    {
                        ::std::rt::begin_panic("internal error: entered unreachable code",
                                               &("srml/council/src/voting.rs",
                                                 33u32, 1u32))
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::Eq for Call<T> { }
    #[cfg(feature = "std")]
    impl <T: Trait> ::srml_support::dispatch::fmt::Debug for Call<T> {
        fn fmt(&self, _f: &mut ::srml_support::dispatch::fmt::Formatter)
         ->
             ::srml_support::dispatch::result::Result<(),
                                                      ::srml_support::dispatch::fmt::Error> {
            match *self {
                Call::propose(ref proposal) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"propose",
                                                                   &(proposal.clone(),))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::vote(ref proposal, ref approve) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"vote",
                                                                   &(proposal.clone(),
                                                                     approve.clone()))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::veto(ref proposal_hash) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"veto",
                                                                   &(proposal_hash.clone(),))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::set_cooloff_period(ref blocks) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"set_cooloff_period",
                                                                   &(blocks.clone(),))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::set_voting_period(ref blocks) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"set_voting_period",
                                                                   &(blocks.clone(),))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                _ => {
                    {
                        ::std::rt::begin_panic("internal error: entered unreachable code",
                                               &("srml/council/src/voting.rs",
                                                 33u32, 1u32))
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::Dispatchable for Call<T> {
        type
        Trait
        =
        T;
        type
        Origin
        =
        T::Origin;
        fn dispatch(self, _origin: Self::Origin)
         -> ::srml_support::dispatch::Result {
            match self {
                Call::propose(proposal) => {
                    <Module<T>>::propose(_origin, proposal)
                }
                Call::vote(proposal, approve) => {
                    <Module<T>>::vote(_origin, proposal, approve)
                }
                Call::veto(proposal_hash) => {
                    <Module<T>>::veto(_origin, proposal_hash)
                }
                Call::set_cooloff_period(blocks) => {
                    {
                        system::ensure_root(_origin)?;
                        <Module<T>>::set_cooloff_period(blocks)
                    }
                }
                Call::set_voting_period(blocks) => {
                    {
                        system::ensure_root(_origin)?;
                        <Module<T>>::set_voting_period(blocks)
                    }
                }
                Call::__PhantomItem(_, _) => {
                    {
                        {
                            {
                                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                          &match (&"__PhantomItem should never be used.",)
                                                                                               {
                                                                                               (arg0,)
                                                                                               =>
                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::std::fmt::Display::fmt)],
                                                                                           }),
                                                           &("srml/council/src/voting.rs",
                                                             33u32, 1u32))
                            }
                        }
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::Callable for Module<T> {
        type
        Call
        =
        Call<T>;
    }
    impl <T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait =
                        T>>(d: D, origin: D::Origin)
         -> ::srml_support::dispatch::Result {
            d.dispatch(origin)
        }
    }
    impl <T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn call_functions()
         -> &'static [::srml_support::dispatch::FunctionMetadata] {
            &[::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("propose"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("proposal"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Box<T::Proposal>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("vote"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("proposal"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("approve"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("bool"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("veto"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("proposal_hash"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("set_cooloff_period"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("blocks"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::BlockNumber>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("set_voting_period"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("blocks"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::BlockNumber>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[]),}]
        }
    }
    #[doc(hidden)]
    mod sr_api_hidden_includes_decl_storage {
        pub extern crate srml_support as hidden_include;
    }
    pub struct CooloffPeriod<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
     for CooloffPeriod<T> {
        type
        Query
        =
        T::BlockNumber;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "CouncilVoting CooloffPeriod".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                            T::BlockNumber::sa(1000))
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                             T::BlockNumber::sa(1000))
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&val,
                                                                                                                                          storage);
            ret
        }
    }
    pub struct VotingPeriod<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
     for VotingPeriod<T> {
        type
        Query
        =
        T::BlockNumber;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "CouncilVoting VotingPeriod".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                            T::BlockNumber::sa(3))
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                             T::BlockNumber::sa(3))
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&val,
                                                                                                                                          storage);
            ret
        }
    }
    #[doc =
          " Number of blocks by which to delay enactment of successful, non-unanimous-council-instigated referendum proposals."]
    pub struct EnactDelayPeriod<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
     for EnactDelayPeriod<T> {
        type
        Query
        =
        T::BlockNumber;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] {
            "CouncilVoting EnactDelayPeriod".as_bytes()
        }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                            T::BlockNumber::sa(0))
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                             T::BlockNumber::sa(0))
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&val,
                                                                                                                                          storage);
            ret
        }
    }
    pub struct Proposals<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::BlockNumber,
                                                                                                              T::Hash)>>
     for Proposals<T> {
        type
        Query
        =
        Vec<(T::BlockNumber, T::Hash)>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "CouncilVoting Proposals".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::BlockNumber,
                                                                                                                                     T::Hash)>>>::key()).unwrap_or_else(||
                                                                                                                                                                            Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::BlockNumber,
                                                                                                                                      T::Hash)>>>::key()).unwrap_or_else(||
                                                                                                                                                                             Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::BlockNumber,
                                                                                                                             T::Hash)>>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::BlockNumber,
                                                                                                                         T::Hash)>>>::put(&val,
                                                                                                                                          storage);
            ret
        }
    }
    pub struct ProposalOf<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                       T::Proposal>
     for ProposalOf<T> {
        type
        Query
        =
        Option<T::Proposal>;
        type
        Hasher
        =
        self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
        #[doc = r" Get the prefix key in storage."]
        fn prefix() -> &'static [u8] { "CouncilVoting ProposalOf".as_bytes() }
        #[doc =
              r" Get the storage key used to fetch a value corresponding to a specific key."]
        fn key_for(x: &T::Hash)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      T::Proposal>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                                &mut key);
            key
        }
        #[doc =
              r" Load the value associated with the given key from the map."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                       &T::Hash,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      T::Proposal>>::key_for(key);
            storage.get(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Take the value, reading and removing it."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                        &T::Hash,
                                                                                                                                                                    storage:
                                                                                                                                                                        &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      T::Proposal>>::key_for(key);
            storage.take(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Mutate the value under a key"]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                          &T::Hash,
                                                                                                                                                                      f:
                                                                                                                                                                          F,
                                                                                                                                                                      storage:
                                                                                                                                                                          &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      T::Proposal>>::get(key,
                                                                                                                                         storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      T::Proposal>>::insert(key,
                                                                                                                                            &val,
                                                                                                                                            storage),
                None =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      T::Proposal>>::remove(key,
                                                                                                                                            storage),
            };
            ret
        }
    }
    pub struct ProposalVoters<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                       Vec<T::AccountId>>
     for ProposalVoters<T> {
        type
        Query
        =
        Vec<T::AccountId>;
        type
        Hasher
        =
        self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
        #[doc = r" Get the prefix key in storage."]
        fn prefix() -> &'static [u8] {
            "CouncilVoting ProposalVoters".as_bytes()
        }
        #[doc =
              r" Get the storage key used to fetch a value corresponding to a specific key."]
        fn key_for(x: &T::Hash)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      Vec<T::AccountId>>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                                &mut key);
            key
        }
        #[doc =
              r" Load the value associated with the given key from the map."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                       &T::Hash,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      Vec<T::AccountId>>>::key_for(key);
            storage.get(&key[..]).unwrap_or_else(|| Default::default())
        }
        #[doc = r" Take the value, reading and removing it."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                        &T::Hash,
                                                                                                                                                                    storage:
                                                                                                                                                                        &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      Vec<T::AccountId>>>::key_for(key);
            storage.take(&key[..]).unwrap_or_else(|| Default::default())
        }
        #[doc = r" Mutate the value under a key"]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                          &T::Hash,
                                                                                                                                                                      f:
                                                                                                                                                                          F,
                                                                                                                                                                      storage:
                                                                                                                                                                          &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      Vec<T::AccountId>>>::get(key,
                                                                                                                                               storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                  Vec<T::AccountId>>>::insert(key,
                                                                                                                                              &val,
                                                                                                                                              storage);
            ret
        }
    }
    pub struct CouncilVoteOf<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(T::Hash,
                                                                                                        T::AccountId),
                                                                                                       bool>
     for CouncilVoteOf<T> {
        type
        Query
        =
        Option<bool>;
        type
        Hasher
        =
        self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
        #[doc = r" Get the prefix key in storage."]
        fn prefix() -> &'static [u8] {
            "CouncilVoting CouncilVoteOf".as_bytes()
        }
        #[doc =
              r" Get the storage key used to fetch a value corresponding to a specific key."]
        fn key_for(x: &(T::Hash, T::AccountId))
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(T::Hash,
                                                                                                                       T::AccountId),
                                                                                                                      bool>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                                &mut key);
            key
        }
        #[doc =
              r" Load the value associated with the given key from the map."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                       &(T::Hash,
                                                                                                                                                                         T::AccountId),
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(T::Hash,
                                                                                                                       T::AccountId),
                                                                                                                      bool>>::key_for(key);
            storage.get(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Take the value, reading and removing it."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                        &(T::Hash,
                                                                                                                                                                          T::AccountId),
                                                                                                                                                                    storage:
                                                                                                                                                                        &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(T::Hash,
                                                                                                                       T::AccountId),
                                                                                                                      bool>>::key_for(key);
            storage.take(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Mutate the value under a key"]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                          &(T::Hash,
                                                                                                                                                                            T::AccountId),
                                                                                                                                                                      f:
                                                                                                                                                                          F,
                                                                                                                                                                      storage:
                                                                                                                                                                          &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(T::Hash,
                                                                                                                       T::AccountId),
                                                                                                                      bool>>::get(key,
                                                                                                                                  storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(T::Hash,
                                                                                                                       T::AccountId),
                                                                                                                      bool>>::insert(key,
                                                                                                                                     &val,
                                                                                                                                     storage),
                None =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(T::Hash,
                                                                                                                       T::AccountId),
                                                                                                                      bool>>::remove(key,
                                                                                                                                     storage),
            };
            ret
        }
    }
    pub struct VetoedProposal<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                       (T::BlockNumber,
                                                                                                        Vec<T::AccountId>)>
     for VetoedProposal<T> {
        type
        Query
        =
        Option<(T::BlockNumber, Vec<T::AccountId>)>;
        type
        Hasher
        =
        self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
        #[doc = r" Get the prefix key in storage."]
        fn prefix() -> &'static [u8] {
            "CouncilVoting VetoedProposal".as_bytes()
        }
        #[doc =
              r" Get the storage key used to fetch a value corresponding to a specific key."]
        fn key_for(x: &T::Hash)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (T::BlockNumber,
                                                                                                                       Vec<T::AccountId>)>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                                &mut key);
            key
        }
        #[doc =
              r" Load the value associated with the given key from the map."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                       &T::Hash,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (T::BlockNumber,
                                                                                                                       Vec<T::AccountId>)>>::key_for(key);
            storage.get(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Take the value, reading and removing it."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                        &T::Hash,
                                                                                                                                                                    storage:
                                                                                                                                                                        &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (T::BlockNumber,
                                                                                                                       Vec<T::AccountId>)>>::key_for(key);
            storage.take(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Mutate the value under a key"]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                          &T::Hash,
                                                                                                                                                                      f:
                                                                                                                                                                          F,
                                                                                                                                                                      storage:
                                                                                                                                                                          &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (T::BlockNumber,
                                                                                                                       Vec<T::AccountId>)>>::get(key,
                                                                                                                                                 storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (T::BlockNumber,
                                                                                                                       Vec<T::AccountId>)>>::insert(key,
                                                                                                                                                    &val,
                                                                                                                                                    storage),
                None =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (T::BlockNumber,
                                                                                                                       Vec<T::AccountId>)>>::remove(key,
                                                                                                                                                    storage),
            };
            ret
        }
    }
    trait Store {
        type
        CooloffPeriod;
        type
        VotingPeriod;
        type
        EnactDelayPeriod;
        type
        Proposals;
        type
        ProposalOf;
        type
        ProposalVoters;
        type
        CouncilVoteOf;
        type
        VetoedProposal;
    }
    #[doc(hidden)]
    pub struct __GetByteStructCooloffPeriod<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_CooloffPeriod:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructCooloffPeriod<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_CooloffPeriod.get_or_init(||
                                                                  {
                                                                      let def_val:
                                                                              T::BlockNumber =
                                                                          T::BlockNumber::sa(1000);
                                                                      <T::BlockNumber
                                                                          as
                                                                          Encode>::encode(&def_val)
                                                                  }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructVotingPeriod<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_VotingPeriod:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructVotingPeriod<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_VotingPeriod.get_or_init(||
                                                                 {
                                                                     let def_val:
                                                                             T::BlockNumber =
                                                                         T::BlockNumber::sa(3);
                                                                     <T::BlockNumber
                                                                         as
                                                                         Encode>::encode(&def_val)
                                                                 }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructEnactDelayPeriod<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_EnactDelayPeriod:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructEnactDelayPeriod<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_EnactDelayPeriod.get_or_init(||
                                                                     {
                                                                         let def_val:
                                                                                 T::BlockNumber =
                                                                             T::BlockNumber::sa(0);
                                                                         <T::BlockNumber
                                                                             as
                                                                             Encode>::encode(&def_val)
                                                                     }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructProposals<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Proposals:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructProposals<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Proposals.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          Vec<(T::BlockNumber,
                                                                               T::Hash)> =
                                                                      Default::default();
                                                                  <Vec<(T::BlockNumber,
                                                                        T::Hash)>
                                                                      as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructProposalOf<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_ProposalOf:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructProposalOf<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_ProposalOf.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           Option<T::Proposal> =
                                                                       Default::default();
                                                                   <Option<T::Proposal>
                                                                       as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructProposalVoters<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_ProposalVoters:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructProposalVoters<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_ProposalVoters.get_or_init(||
                                                                   {
                                                                       let def_val:
                                                                               Vec<T::AccountId> =
                                                                           Default::default();
                                                                       <Vec<T::AccountId>
                                                                           as
                                                                           Encode>::encode(&def_val)
                                                                   }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructCouncilVoteOf<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_CouncilVoteOf:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructCouncilVoteOf<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_CouncilVoteOf.get_or_init(||
                                                                  {
                                                                      let def_val:
                                                                              Option<bool> =
                                                                          Default::default();
                                                                      <Option<bool>
                                                                          as
                                                                          Encode>::encode(&def_val)
                                                                  }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructVetoedProposal<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_VetoedProposal:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructVetoedProposal<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_VetoedProposal.get_or_init(||
                                                                   {
                                                                       let def_val:
                                                                               Option<(T::BlockNumber,
                                                                                       Vec<T::AccountId>)> =
                                                                           Default::default();
                                                                       <Option<(T::BlockNumber,
                                                                                Vec<T::AccountId>)>
                                                                           as
                                                                           Encode>::encode(&def_val)
                                                                   }).clone()
        }
    }
    impl <T: Trait> Store for Module<T> {
        type
        CooloffPeriod
        =
        CooloffPeriod<T>;
        type
        VotingPeriod
        =
        VotingPeriod<T>;
        type
        EnactDelayPeriod
        =
        EnactDelayPeriod<T>;
        type
        Proposals
        =
        Proposals<T>;
        type
        ProposalOf
        =
        ProposalOf<T>;
        type
        ProposalVoters
        =
        ProposalVoters<T>;
        type
        CouncilVoteOf
        =
        CouncilVoteOf<T>;
        type
        VetoedProposal
        =
        VetoedProposal<T>;
    }
    impl <T: 'static + Trait> Module<T> {
        pub fn cooloff_period() -> T::BlockNumber {
            <CooloffPeriod<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn voting_period() -> T::BlockNumber {
            <VotingPeriod<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " Number of blocks by which to delay enactment of successful, non-unanimous-council-instigated referendum proposals."]
        pub fn enact_delay_period() -> T::BlockNumber {
            <EnactDelayPeriod<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn proposals() -> Vec<(T::BlockNumber, T::Hash)> {
            <Proposals<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::BlockNumber,
                                                                                                                         T::Hash)>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn proposal_of<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::Hash>>(key:
                                                                                                                            K)
         -> Option<T::Proposal> {
            <ProposalOf<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                  T::Proposal>>::get(key.borrow(),
                                                                                                                                     &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn proposal_voters<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::Hash>>(key:
                                                                                                                                K)
         -> Vec<T::AccountId> {
            <ProposalVoters<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                  Vec<T::AccountId>>>::get(key.borrow(),
                                                                                                                                           &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn vote_of<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<(T::Hash,
                                                                                                           T::AccountId)>>(key:
                                                                                                                               K)
         -> Option<bool> {
            <CouncilVoteOf<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(T::Hash,
                                                                                                                   T::AccountId),
                                                                                                                  bool>>::get(key.borrow(),
                                                                                                                              &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn veto_of<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::Hash>>(key:
                                                                                                                        K)
         -> Option<(T::BlockNumber, Vec<T::AccountId>)> {
            <VetoedProposal<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                  (T::BlockNumber,
                                                                                                                   Vec<T::AccountId>)>>::get(key.borrow(),
                                                                                                                                             &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc(hidden)]
        pub fn store_metadata()
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
            self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                      &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CooloffPeriod"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCooloffPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VotingPeriod"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVotingPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("EnactDelayPeriod"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructEnactDelayPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Number of blocks by which to delay enactment of successful, non-unanimous-council-instigated referendum proposals."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Proposals"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(T::BlockNumber, T::Hash)>")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposals::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalOf"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                       key:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                                                                                                                                                                                                       value:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Proposal"),
                                                                                                                                                                                                                                                                                                                                                                                                       is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                           false,},
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalVoters"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                       key:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                                                                                                                                                                                                       value:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>"),
                                                                                                                                                                                                                                                                                                                                                                                                       is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                           false,},
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalVoters::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CouncilVoteOf"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                       key:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(T::Hash, T::AccountId)"),
                                                                                                                                                                                                                                                                                                                                                                                                       value:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("bool"),
                                                                                                                                                                                                                                                                                                                                                                                                       is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                           false,},
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCouncilVoteOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VetoedProposal"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                       key:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                                                                                                                                                                                                       value:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(T::BlockNumber, Vec<T::AccountId>)"),
                                                                                                                                                                                                                                                                                                                                                                                                       is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                           false,},
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVetoedProposal::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),}]
                                                                                                                                                                                                  }),}
        }
        #[doc(hidden)]
        pub fn store_metadata_functions()
         ->
             &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
            {
                &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CooloffPeriod"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCooloffPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VotingPeriod"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVotingPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("EnactDelayPeriod"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructEnactDelayPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Number of blocks by which to delay enactment of successful, non-unanimous-council-instigated referendum proposals."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Proposals"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(T::BlockNumber, T::Hash)>")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposals::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalOf"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                 key:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                 value:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Proposal"),
                                                                                                                                                                                                                 is_linked:
                                                                                                                                                                                                                     false,},
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalVoters"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                 key:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                 value:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>"),
                                                                                                                                                                                                                 is_linked:
                                                                                                                                                                                                                     false,},
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalVoters::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CouncilVoteOf"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                 key:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(T::Hash, T::AccountId)"),
                                                                                                                                                                                                                 value:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("bool"),
                                                                                                                                                                                                                 is_linked:
                                                                                                                                                                                                                     false,},
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCouncilVoteOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VetoedProposal"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                 key:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                 value:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(T::BlockNumber, Vec<T::AccountId>)"),
                                                                                                                                                                                                                 is_linked:
                                                                                                                                                                                                                     false,},
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVetoedProposal::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),}]
            }
        }
        #[doc(hidden)]
        pub fn store_metadata_name() -> &'static str { "CouncilVoting" }
    }
    #[cfg(feature = "std")]
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[serde(bound(serialize =
                      "T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
    #[serde(bound(deserialize =
                      "T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
    pub struct GenesisConfig<T: Trait> {
        pub cooloff_period: T::BlockNumber,
        pub voting_period: T::BlockNumber,
        #[doc =
              " Number of blocks by which to delay enactment of successful, non-unanimous-council-instigated referendum proposals."]
        pub enact_delay_period: T::BlockNumber,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_GenesisConfig: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <T: Trait> _serde::Serialize for GenesisConfig<T> where
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
             {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                                                   "GenesisConfig",
                                                                   false as
                                                                       usize +
                                                                       1 + 1 +
                                                                       1) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "cooloffPeriod",
                                                                        &self.cooloff_period)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "votingPeriod",
                                                                        &self.voting_period)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "enactDelayPeriod",
                                                                        &self.enact_delay_period)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_GenesisConfig: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <'de, T: Trait> _serde::Deserialize<'de> for GenesisConfig<T>
             where
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
             {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, }
                    struct __FieldVisitor;
                    impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type
                        Value
                        =
                        __Field;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                _ =>
                                _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                     &"field index 0 <= i < 3")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                "cooloffPeriod" =>
                                _serde::export::Ok(__Field::__field0),
                                "votingPeriod" =>
                                _serde::export::Ok(__Field::__field1),
                                "enactDelayPeriod" =>
                                _serde::export::Ok(__Field::__field2),
                                _ => {
                                    _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                         FIELDS))
                                }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"cooloffPeriod" =>
                                _serde::export::Ok(__Field::__field0),
                                b"votingPeriod" =>
                                _serde::export::Ok(__Field::__field1),
                                b"enactDelayPeriod" =>
                                _serde::export::Ok(__Field::__field2),
                                _ => {
                                    let __value =
                                        &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                         FIELDS))
                                }
                            }
                        }
                    }
                    impl <'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                         -> _serde::export::Result<Self, __D::Error> where
                         __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                         __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de, T: Trait> where
                           T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                        marker: _serde::export::PhantomData<GenesisConfig<T>>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de, T: Trait> _serde::de::Visitor<'de> for
                     __Visitor<'de, T> where
                     T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
                     {
                        type
                        Value
                        =
                        GenesisConfig<T>;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "struct GenesisConfig")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"struct GenesisConfig with 3 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                     &"struct GenesisConfig with 3 elements"));
                                    }
                                };
                            let __field2 =
                                match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                     &"struct GenesisConfig with 3 elements"));
                                    }
                                };
                            _serde::export::Ok(GenesisConfig{cooloff_period:
                                                                 __field0,
                                                             voting_period:
                                                                 __field1,
                                                             enact_delay_period:
                                                                 __field2,})
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::MapAccess<'de> {
                            let mut __field0:
                                    _serde::export::Option<T::BlockNumber> =
                                _serde::export::None;
                            let mut __field1:
                                    _serde::export::Option<T::BlockNumber> =
                                _serde::export::None;
                            let mut __field2:
                                    _serde::export::Option<T::BlockNumber> =
                                _serde::export::None;
                            while let _serde::export::Some(__key) =
                                      match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("cooloffPeriod"));
                                        }
                                        __field0 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<T::BlockNumber>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("votingPeriod"));
                                        }
                                        __field1 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<T::BlockNumber>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field2 => {
                                        if _serde::export::Option::is_some(&__field2)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("enactDelayPeriod"));
                                        }
                                        __field2 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<T::BlockNumber>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::export::Some(__field0) =>
                                    __field0,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("cooloffPeriod")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::export::Some(__field1) =>
                                    __field1,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("votingPeriod")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field2 =
                                match __field2 {
                                    _serde::export::Some(__field2) =>
                                    __field2,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("enactDelayPeriod")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            _serde::export::Ok(GenesisConfig{cooloff_period:
                                                                 __field0,
                                                             voting_period:
                                                                 __field1,
                                                             enact_delay_period:
                                                                 __field2,})
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["cooloffPeriod", "votingPeriod",
                          "enactDelayPeriod"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                                                             "GenesisConfig",
                                                             FIELDS,
                                                             __Visitor{marker:
                                                                           _serde::export::PhantomData::<GenesisConfig<T>>,
                                                                       lifetime:
                                                                           _serde::export::PhantomData,})
                }
            }
        };
    #[cfg(feature = "std")]
    impl <T: Trait> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig{cooloff_period: T::BlockNumber::sa(1000),
                          voting_period: T::BlockNumber::sa(3),
                          enact_delay_period: T::BlockNumber::sa(0),}
        }
    }
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::BuildStorage
     for GenesisConfig<T> {
        fn assimilate_storage(self,
                              r:
                                  &mut self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::StorageOverlay,
                              c:
                                  &mut self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::ChildrenStorageOverlay)
         -> ::std::result::Result<(), String> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::cell::RefCell;
            let storage = RefCell::new(r);
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.cooloff_period.clone()))(&self);
                <CooloffPeriod<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                              &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.voting_period.clone()))(&self);
                <VotingPeriod<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                              &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.enact_delay_period.clone()))(&self);
                <EnactDelayPeriod<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                              &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v = (|_| <[_]>::into_vec(box []))(&self);
                <Proposals<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::BlockNumber,
                                                                                                                             T::Hash)>>>::put(&v,
                                                                                                                                              &storage);
            }
            let r = storage.into_inner();
            (|_, _, _| { })(r, c, &self);
            Ok(())
        }
    }
    /// [`RawEvent`] specialized for the configuration [`Trait`]
    ///
    /// [`RawEvent`]: enum.RawEvent.html
    /// [`Trait`]: trait.Trait.html
    pub type Event<T> = RawEvent<<T as system::Trait>::Hash>;
    /// Events for this module.
    ///
    #[structural_match]
    pub enum RawEvent<Hash> {

        #[doc =
              r" A voting tally has happened for a referendum cancellation vote."]
        #[doc = r" Last three are yes, no, abstain counts."]
        TallyCancelation(Hash, u32, u32, u32),

        #[doc = r" A voting tally has happened for a referendum vote."]
        #[doc = r" Last three are yes, no, abstain counts."]
        TallyReferendum(Hash, u32, u32, u32),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::clone::Clone> ::std::clone::Clone for RawEvent<Hash> {
        #[inline]
        fn clone(&self) -> RawEvent<Hash> {
            match (&*self,) {
                (&RawEvent::TallyCancelation(ref __self_0, ref __self_1,
                                             ref __self_2, ref __self_3),) =>
                RawEvent::TallyCancelation(::std::clone::Clone::clone(&(*__self_0)),
                                           ::std::clone::Clone::clone(&(*__self_1)),
                                           ::std::clone::Clone::clone(&(*__self_2)),
                                           ::std::clone::Clone::clone(&(*__self_3))),
                (&RawEvent::TallyReferendum(ref __self_0, ref __self_1,
                                            ref __self_2, ref __self_3),) =>
                RawEvent::TallyReferendum(::std::clone::Clone::clone(&(*__self_0)),
                                          ::std::clone::Clone::clone(&(*__self_1)),
                                          ::std::clone::Clone::clone(&(*__self_2)),
                                          ::std::clone::Clone::clone(&(*__self_3))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
     RawEvent<Hash> {
        #[inline]
        fn eq(&self, other: &RawEvent<Hash>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&RawEvent::TallyCancelation(ref __self_0,
                                                     ref __self_1,
                                                     ref __self_2,
                                                     ref __self_3),
                         &RawEvent::TallyCancelation(ref __arg_1_0,
                                                     ref __arg_1_1,
                                                     ref __arg_1_2,
                                                     ref __arg_1_3)) =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1) &&
                            (*__self_2) == (*__arg_1_2) &&
                            (*__self_3) == (*__arg_1_3),
                        (&RawEvent::TallyReferendum(ref __self_0,
                                                    ref __self_1,
                                                    ref __self_2,
                                                    ref __self_3),
                         &RawEvent::TallyReferendum(ref __arg_1_0,
                                                    ref __arg_1_1,
                                                    ref __arg_1_2,
                                                    ref __arg_1_3)) =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1) &&
                            (*__self_2) == (*__arg_1_2) &&
                            (*__self_3) == (*__arg_1_3),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &RawEvent<Hash>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&RawEvent::TallyCancelation(ref __self_0,
                                                     ref __self_1,
                                                     ref __self_2,
                                                     ref __self_3),
                         &RawEvent::TallyCancelation(ref __arg_1_0,
                                                     ref __arg_1_1,
                                                     ref __arg_1_2,
                                                     ref __arg_1_3)) =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1) ||
                            (*__self_2) != (*__arg_1_2) ||
                            (*__self_3) != (*__arg_1_3),
                        (&RawEvent::TallyReferendum(ref __self_0,
                                                    ref __self_1,
                                                    ref __self_2,
                                                    ref __self_3),
                         &RawEvent::TallyReferendum(ref __arg_1_0,
                                                    ref __arg_1_1,
                                                    ref __arg_1_2,
                                                    ref __arg_1_3)) =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1) ||
                            (*__self_2) != (*__arg_1_2) ||
                            (*__self_3) != (*__arg_1_3),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::cmp::Eq> ::std::cmp::Eq for RawEvent<Hash> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_RawEvent: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <Hash> _parity_codec::Encode for RawEvent<Hash> where
             Hash: _parity_codec::Encode, Hash: _parity_codec::Encode,
             Hash: _parity_codec::Encode, Hash: _parity_codec::Encode {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        RawEvent::TallyCancelation(ref aa, ref ba, ref ca,
                                                   ref da) => {
                            dest.push_byte(0usize as u8);
                            dest.push(aa);
                            dest.push(ba);
                            dest.push(ca);
                            dest.push(da);
                        }
                        RawEvent::TallyReferendum(ref aa, ref ba, ref ca,
                                                  ref da) => {
                            dest.push_byte(1usize as u8);
                            dest.push(aa);
                            dest.push(ba);
                            dest.push(ca);
                            dest.push(da);
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_RawEvent: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <Hash> _parity_codec::Decode for RawEvent<Hash> where
             Hash: _parity_codec::Decode, Hash: _parity_codec::Decode,
             Hash: _parity_codec::Decode, Hash: _parity_codec::Decode {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 0usize as u8 => {
                            Some(RawEvent::TallyCancelation(_parity_codec::Decode::decode(input)?,
                                                            _parity_codec::Decode::decode(input)?,
                                                            _parity_codec::Decode::decode(input)?,
                                                            _parity_codec::Decode::decode(input)?))
                        }
                        x if x == 1usize as u8 => {
                            Some(RawEvent::TallyReferendum(_parity_codec::Decode::decode(input)?,
                                                           _parity_codec::Decode::decode(input)?,
                                                           _parity_codec::Decode::decode(input)?,
                                                           _parity_codec::Decode::decode(input)?))
                        }
                        _ => None,
                    }
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::fmt::Debug> ::std::fmt::Debug for RawEvent<Hash> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&RawEvent::TallyCancelation(ref __self_0, ref __self_1,
                                             ref __self_2, ref __self_3),) =>
                {
                    let mut debug_trait_builder =
                        f.debug_tuple("TallyCancelation");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    let _ = debug_trait_builder.field(&&(*__self_3));
                    debug_trait_builder.finish()
                }
                (&RawEvent::TallyReferendum(ref __self_0, ref __self_1,
                                            ref __self_2, ref __self_3),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("TallyReferendum");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    let _ = debug_trait_builder.field(&&(*__self_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl <Hash> From<RawEvent<Hash>> for () {
        fn from(_: RawEvent<Hash>) -> () { () }
    }
    impl <Hash> RawEvent<Hash> {
        #[allow(dead_code)]
        pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
            &[::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("TallyCancelation"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["Hash",
                                                                                                        "u32",
                                                                                                        "u32",
                                                                                                        "u32"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" A voting tally has happened for a referendum cancellation vote.",
                                                                                                        r" Last three are yes, no, abstain counts."]),},
              ::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("TallyReferendum"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["Hash",
                                                                                                        "u32",
                                                                                                        "u32",
                                                                                                        "u32"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" A voting tally has happened for a referendum vote.",
                                                                                                        r" Last three are yes, no, abstain counts."]),}]
        }
    }
    impl <T: Trait> Module<T> {
        pub fn is_vetoed<B: Borrow<T::Hash>>(proposal: B) -> bool {
            Self::veto_of(proposal.borrow()).map(|(expiry, _):
                                                      (T::BlockNumber,
                                                       Vec<T::AccountId>)|
                                                     <system::Module<T>>::block_number()
                                                         <
                                                         expiry).unwrap_or(false)
        }
        pub fn will_still_be_councillor_at(who: &T::AccountId,
                                           n: T::BlockNumber) -> bool {
            <Council<T>>::active_council().iter().find(|&&(ref a, _)|
                                                           a ==
                                                               who).map(|&(_,
                                                                           expires)|
                                                                            expires
                                                                                >
                                                                                n).unwrap_or(false)
        }
        pub fn is_councillor(who: &T::AccountId) -> bool {
            <Council<T>>::active_council().iter().any(|&(ref a, _)| a == who)
        }
        pub fn tally(proposal_hash: &T::Hash) -> (u32, u32, u32) {
            Self::generic_tally(proposal_hash,
                                |w: &T::AccountId, p: &T::Hash|
                                    Self::vote_of((*p, w.clone())))
        }
        fn set_veto_of(proposal: &T::Hash, expiry: T::BlockNumber,
                       vetoers: Vec<T::AccountId>) {
            <VetoedProposal<T>>::insert(proposal, (expiry, vetoers));
        }
        fn kill_veto_of(proposal: &T::Hash) {
            <VetoedProposal<T>>::remove(proposal);
        }
        fn take_tally(proposal_hash: &T::Hash) -> (u32, u32, u32) {
            Self::generic_tally(proposal_hash,
                                |w: &T::AccountId, p: &T::Hash|
                                    <CouncilVoteOf<T>>::take((*p, w.clone())))
        }
        fn generic_tally<F: Fn(&T::AccountId, &T::Hash) ->
                         Option<bool>>(proposal_hash: &T::Hash, vote_of: F)
         -> (u32, u32, u32) {
            let c = <Council<T>>::active_council();
            let (approve, reject) =
                c.iter().filter_map(|&(ref a, _)|
                                        vote_of(a,
                                                proposal_hash)).map(|approve|
                                                                        if approve
                                                                           {
                                                                            (1,
                                                                             0)
                                                                        } else {
                                                                            (0,
                                                                             1)
                                                                        }).fold((0,
                                                                                 0),
                                                                                |(a,
                                                                                  b),
                                                                                 (c,
                                                                                  d)|
                                                                                    (a
                                                                                         +
                                                                                         c,
                                                                                     b
                                                                                         +
                                                                                         d));
            (approve, reject, c.len() as u32 - approve - reject)
        }
        fn set_proposals(p: &Vec<(T::BlockNumber, T::Hash)>) {
            <Proposals<T>>::put(p);
        }
        fn take_proposal_if_expiring_at(n: T::BlockNumber)
         -> Option<(T::Proposal, T::Hash)> {
            let proposals = Self::proposals();
            match proposals.first() {
                Some(&(expiry, hash)) if expiry == n => {
                    Self::set_proposals(&proposals[1..].to_vec());
                    <ProposalOf<T>>::take(hash).map(|p| (p, hash))
                }
                _ => None,
            }
        }
        fn end_block(now: T::BlockNumber) -> Result {
            while let Some((proposal, proposal_hash)) =
                      Self::take_proposal_if_expiring_at(now) {
                let tally = Self::take_tally(&proposal_hash);
                if let Some(&democracy::Call::cancel_referendum(ref_index)) =
                       IsSubType::<democracy::Module<T>>::is_aux_sub_type(&proposal)
                       {
                    Self::deposit_event(RawEvent::TallyCancelation(proposal_hash,
                                                                   tally.0,
                                                                   tally.1,
                                                                   tally.2));
                    if let (_, 0, 0) = tally {
                        <democracy::Module<T>>::internal_cancel_referendum(ref_index.into());
                    }
                } else {
                    Self::deposit_event(RawEvent::TallyReferendum(proposal_hash.clone(),
                                                                  tally.0,
                                                                  tally.1,
                                                                  tally.2));
                    if tally.0 > tally.1 + tally.2 {
                        Self::kill_veto_of(&proposal_hash);
                        let period =
                            match tally.1 {
                                0 => Zero::zero(),
                                _ => Self::enact_delay_period(),
                            };
                        let threshold =
                            match tally {
                                (_, 0, 0) =>
                                democracy::VoteThreshold::SuperMajorityAgainst,
                                _ => democracy::VoteThreshold::SimpleMajority,
                            };
                        <democracy::Module<T>>::internal_start_referendum(proposal,
                                                                          threshold,
                                                                          period).map(|_|
                                                                                          ())?;
                    }
                }
            }
            Ok(())
        }
    }
}
pub mod motions {
    //! Council voting system.
    use rstd::prelude::*;
    use rstd::result;
    use substrate_primitives::u32_trait::Value as U32;
    use primitives::traits::{Hash, EnsureOrigin};
    use srml_support::dispatch::{Dispatchable, Parameter};
    use srml_support::{StorageValue, StorageMap, decl_module, decl_event,
                       decl_storage, ensure};
    use super::{Trait as CouncilTrait, Module as Council};
    use system::{self, ensure_signed};
    /// Simple index type for proposal counting.
    pub type ProposalIndex = u32;
    pub trait Trait: CouncilTrait {
        /// The outer origin type.
        type
        Origin: From<Origin>;
        /// The outer call dispatch type.
        type
        Proposal: Parameter +
        Dispatchable<Origin
        =
        <Self as Trait>::Origin>;
        /// The outer event type.
        type
        Event: From<Event<Self>> +
        Into<<Self as system::Trait>::Event>;
    }
    /// Origin for the council module.
    #[structural_match]
    pub enum Origin {

        /// It has been condoned by a given number of council members.
        Members(u32),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Origin {
        #[inline]
        fn eq(&self, other: &Origin) -> bool {
            match (&*self, &*other) {
                (&Origin::Members(ref __self_0),
                 &Origin::Members(ref __arg_1_0)) =>
                (*__self_0) == (*__arg_1_0),
            }
        }
        #[inline]
        fn ne(&self, other: &Origin) -> bool {
            match (&*self, &*other) {
                (&Origin::Members(ref __self_0),
                 &Origin::Members(ref __arg_1_0)) =>
                (*__self_0) != (*__arg_1_0),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for Origin {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<u32>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Origin {
        #[inline]
        fn clone(&self) -> Origin {
            match (&*self,) {
                (&Origin::Members(ref __self_0),) =>
                Origin::Members(::std::clone::Clone::clone(&(*__self_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Origin {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Origin::Members(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Members");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// [`RawEvent`] specialized for the configuration [`Trait`]
    ///
    /// [`RawEvent`]: enum.RawEvent.html
    /// [`Trait`]: trait.Trait.html
    pub type Event<T>
        =
        RawEvent<<T as system::Trait>::Hash, <T as system::Trait>::AccountId>;
    /// Events for this module.
    ///
    #[structural_match]
    pub enum RawEvent<Hash, AccountId> {

        #[doc =
              r" A motion (given hash) has been proposed (by given account) with a threshold (given u32)."]
        Proposed(AccountId, ProposalIndex, Hash, u32),

        #[doc =
              r" A motion (given hash) has been voted on by given account, leaving"]
        #[doc =
              r" a tally (yes votes and no votes given as u32s respectively)."]
        Voted(AccountId, Hash, bool, u32, u32),

        #[doc = r" A motion was approved by the required threshold."]
        Approved(Hash),

        #[doc = r" A motion was not approved by the required threshold."]
        Disapproved(Hash),

        #[doc =
              r" A motion was executed; `bool` is true if returned without error."]
        Executed(Hash, bool),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::clone::Clone, AccountId: ::std::clone::Clone>
     ::std::clone::Clone for RawEvent<Hash, AccountId> {
        #[inline]
        fn clone(&self) -> RawEvent<Hash, AccountId> {
            match (&*self,) {
                (&RawEvent::Proposed(ref __self_0, ref __self_1, ref __self_2,
                                     ref __self_3),) =>
                RawEvent::Proposed(::std::clone::Clone::clone(&(*__self_0)),
                                   ::std::clone::Clone::clone(&(*__self_1)),
                                   ::std::clone::Clone::clone(&(*__self_2)),
                                   ::std::clone::Clone::clone(&(*__self_3))),
                (&RawEvent::Voted(ref __self_0, ref __self_1, ref __self_2,
                                  ref __self_3, ref __self_4),) =>
                RawEvent::Voted(::std::clone::Clone::clone(&(*__self_0)),
                                ::std::clone::Clone::clone(&(*__self_1)),
                                ::std::clone::Clone::clone(&(*__self_2)),
                                ::std::clone::Clone::clone(&(*__self_3)),
                                ::std::clone::Clone::clone(&(*__self_4))),
                (&RawEvent::Approved(ref __self_0),) =>
                RawEvent::Approved(::std::clone::Clone::clone(&(*__self_0))),
                (&RawEvent::Disapproved(ref __self_0),) =>
                RawEvent::Disapproved(::std::clone::Clone::clone(&(*__self_0))),
                (&RawEvent::Executed(ref __self_0, ref __self_1),) =>
                RawEvent::Executed(::std::clone::Clone::clone(&(*__self_0)),
                                   ::std::clone::Clone::clone(&(*__self_1))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::cmp::PartialEq, AccountId: ::std::cmp::PartialEq>
     ::std::cmp::PartialEq for RawEvent<Hash, AccountId> {
        #[inline]
        fn eq(&self, other: &RawEvent<Hash, AccountId>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&RawEvent::Proposed(ref __self_0, ref __self_1,
                                             ref __self_2, ref __self_3),
                         &RawEvent::Proposed(ref __arg_1_0, ref __arg_1_1,
                                             ref __arg_1_2, ref __arg_1_3)) =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1) &&
                            (*__self_2) == (*__arg_1_2) &&
                            (*__self_3) == (*__arg_1_3),
                        (&RawEvent::Voted(ref __self_0, ref __self_1,
                                          ref __self_2, ref __self_3,
                                          ref __self_4),
                         &RawEvent::Voted(ref __arg_1_0, ref __arg_1_1,
                                          ref __arg_1_2, ref __arg_1_3,
                                          ref __arg_1_4)) =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1) &&
                            (*__self_2) == (*__arg_1_2) &&
                            (*__self_3) == (*__arg_1_3) &&
                            (*__self_4) == (*__arg_1_4),
                        (&RawEvent::Approved(ref __self_0),
                         &RawEvent::Approved(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&RawEvent::Disapproved(ref __self_0),
                         &RawEvent::Disapproved(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&RawEvent::Executed(ref __self_0, ref __self_1),
                         &RawEvent::Executed(ref __arg_1_0, ref __arg_1_1)) =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &RawEvent<Hash, AccountId>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&RawEvent::Proposed(ref __self_0, ref __self_1,
                                             ref __self_2, ref __self_3),
                         &RawEvent::Proposed(ref __arg_1_0, ref __arg_1_1,
                                             ref __arg_1_2, ref __arg_1_3)) =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1) ||
                            (*__self_2) != (*__arg_1_2) ||
                            (*__self_3) != (*__arg_1_3),
                        (&RawEvent::Voted(ref __self_0, ref __self_1,
                                          ref __self_2, ref __self_3,
                                          ref __self_4),
                         &RawEvent::Voted(ref __arg_1_0, ref __arg_1_1,
                                          ref __arg_1_2, ref __arg_1_3,
                                          ref __arg_1_4)) =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1) ||
                            (*__self_2) != (*__arg_1_2) ||
                            (*__self_3) != (*__arg_1_3) ||
                            (*__self_4) != (*__arg_1_4),
                        (&RawEvent::Approved(ref __self_0),
                         &RawEvent::Approved(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&RawEvent::Disapproved(ref __self_0),
                         &RawEvent::Disapproved(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&RawEvent::Executed(ref __self_0, ref __self_1),
                         &RawEvent::Executed(ref __arg_1_0, ref __arg_1_1)) =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::cmp::Eq, AccountId: ::std::cmp::Eq> ::std::cmp::Eq for
     RawEvent<Hash, AccountId> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<ProposalIndex>;
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_RawEvent: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <Hash, AccountId> _parity_codec::Encode for
             RawEvent<Hash, AccountId> where AccountId: _parity_codec::Encode,
             AccountId: _parity_codec::Encode, Hash: _parity_codec::Encode,
             Hash: _parity_codec::Encode, AccountId: _parity_codec::Encode,
             AccountId: _parity_codec::Encode, Hash: _parity_codec::Encode,
             Hash: _parity_codec::Encode, Hash: _parity_codec::Encode,
             Hash: _parity_codec::Encode, Hash: _parity_codec::Encode,
             Hash: _parity_codec::Encode, Hash: _parity_codec::Encode,
             Hash: _parity_codec::Encode {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        RawEvent::Proposed(ref aa, ref ba, ref ca, ref da) =>
                        {
                            dest.push_byte(0usize as u8);
                            dest.push(aa);
                            dest.push(ba);
                            dest.push(ca);
                            dest.push(da);
                        }
                        RawEvent::Voted(ref aa, ref ba, ref ca, ref da,
                                        ref ea) => {
                            dest.push_byte(1usize as u8);
                            dest.push(aa);
                            dest.push(ba);
                            dest.push(ca);
                            dest.push(da);
                            dest.push(ea);
                        }
                        RawEvent::Approved(ref aa) => {
                            dest.push_byte(2usize as u8);
                            dest.push(aa);
                        }
                        RawEvent::Disapproved(ref aa) => {
                            dest.push_byte(3usize as u8);
                            dest.push(aa);
                        }
                        RawEvent::Executed(ref aa, ref ba) => {
                            dest.push_byte(4usize as u8);
                            dest.push(aa);
                            dest.push(ba);
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_RawEvent: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <Hash, AccountId> _parity_codec::Decode for
             RawEvent<Hash, AccountId> where AccountId: _parity_codec::Decode,
             AccountId: _parity_codec::Decode, Hash: _parity_codec::Decode,
             Hash: _parity_codec::Decode, AccountId: _parity_codec::Decode,
             AccountId: _parity_codec::Decode, Hash: _parity_codec::Decode,
             Hash: _parity_codec::Decode, Hash: _parity_codec::Decode,
             Hash: _parity_codec::Decode, Hash: _parity_codec::Decode,
             Hash: _parity_codec::Decode, Hash: _parity_codec::Decode,
             Hash: _parity_codec::Decode {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 0usize as u8 => {
                            Some(RawEvent::Proposed(_parity_codec::Decode::decode(input)?,
                                                    _parity_codec::Decode::decode(input)?,
                                                    _parity_codec::Decode::decode(input)?,
                                                    _parity_codec::Decode::decode(input)?))
                        }
                        x if x == 1usize as u8 => {
                            Some(RawEvent::Voted(_parity_codec::Decode::decode(input)?,
                                                 _parity_codec::Decode::decode(input)?,
                                                 _parity_codec::Decode::decode(input)?,
                                                 _parity_codec::Decode::decode(input)?,
                                                 _parity_codec::Decode::decode(input)?))
                        }
                        x if x == 2usize as u8 => {
                            Some(RawEvent::Approved(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 3usize as u8 => {
                            Some(RawEvent::Disapproved(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 4usize as u8 => {
                            Some(RawEvent::Executed(_parity_codec::Decode::decode(input)?,
                                                    _parity_codec::Decode::decode(input)?))
                        }
                        _ => None,
                    }
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::fmt::Debug, AccountId: ::std::fmt::Debug>
     ::std::fmt::Debug for RawEvent<Hash, AccountId> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&RawEvent::Proposed(ref __self_0, ref __self_1, ref __self_2,
                                     ref __self_3),) => {
                    let mut debug_trait_builder = f.debug_tuple("Proposed");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    let _ = debug_trait_builder.field(&&(*__self_3));
                    debug_trait_builder.finish()
                }
                (&RawEvent::Voted(ref __self_0, ref __self_1, ref __self_2,
                                  ref __self_3, ref __self_4),) => {
                    let mut debug_trait_builder = f.debug_tuple("Voted");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    let _ = debug_trait_builder.field(&&(*__self_3));
                    let _ = debug_trait_builder.field(&&(*__self_4));
                    debug_trait_builder.finish()
                }
                (&RawEvent::Approved(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Approved");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&RawEvent::Disapproved(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("Disapproved");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&RawEvent::Executed(ref __self_0, ref __self_1),) => {
                    let mut debug_trait_builder = f.debug_tuple("Executed");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl <Hash, AccountId> From<RawEvent<Hash, AccountId>> for () {
        fn from(_: RawEvent<Hash, AccountId>) -> () { () }
    }
    impl <Hash, AccountId> RawEvent<Hash, AccountId> {
        #[allow(dead_code)]
        pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
            &[::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("Proposed"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                        "ProposalIndex",
                                                                                                        "Hash",
                                                                                                        "u32"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" A motion (given hash) has been proposed (by given account) with a threshold (given u32)."]),},
              ::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("Voted"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                        "Hash",
                                                                                                        "bool",
                                                                                                        "u32",
                                                                                                        "u32"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" A motion (given hash) has been voted on by given account, leaving",
                                                                                                        r" a tally (yes votes and no votes given as u32s respectively)."]),},
              ::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("Approved"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["Hash"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" A motion was approved by the required threshold."]),},
              ::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("Disapproved"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["Hash"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" A motion was not approved by the required threshold."]),},
              ::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("Executed"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["Hash",
                                                                                                        "bool"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" A motion was executed; `bool` is true if returned without error."]),}]
        }
    }
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub struct Module<T: Trait>(::srml_support::rstd::marker::PhantomData<(T)>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::clone::Clone + Trait> ::std::clone::Clone for Module<T> {
        #[inline]
        fn clone(&self) -> Module<T> {
            match *self {
                Module(ref __self_0_0) =>
                Module(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::marker::Copy + Trait> ::std::marker::Copy for Module<T> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::cmp::PartialEq + Trait> ::std::cmp::PartialEq for
     Module<T> {
        #[inline]
        fn eq(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) =>
                match *self {
                    Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) =>
                match *self {
                    Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::cmp::Eq + Trait> ::std::cmp::Eq for Module<T> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _:
                        ::std::cmp::AssertParamIsEq<::srml_support::rstd::marker::PhantomData<(T)>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::fmt::Debug + Trait> ::std::fmt::Debug for Module<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Module(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Module");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl <T: Trait>
     ::srml_support::runtime_primitives::traits::OnInitialize<T::BlockNumber>
     for Module<T> {
    }
    impl <T: Trait>
     ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber>
     for Module<T> {
    }
    impl <T: Trait>
     ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
     for Module<T> {
    }
    impl <T: Trait> Module<T> {
        fn deposit_event(event: Event<T>) {
            <system::Module<T>>::deposit_event(<T as
                                                   Trait>::from(event).into());
        }
    }
    /// Can also be called using [`Call`].
    ///
    /// [`Call`]: enum.Call.html
    impl <T: Trait> Module<T> {
        fn propose(origin: <T as system::Trait>::Origin, threshold: u32,
                   proposal: Box<<T as Trait>::Proposal>)
         -> ::srml_support::dispatch::Result {
            {
                let who = ensure_signed(origin)?;
                {
                    if !Self::is_councillor(&who) {
                        { return Err("proposer not on council"); };
                    }
                };
                let proposal_hash = T::Hashing::hash_of(&proposal);
                {
                    if !!<ProposalOf<T>>::exists(proposal_hash) {
                        { return Err("duplicate proposals not allowed"); };
                    }
                };
                if threshold < 2 {
                    let ok =
                        proposal.dispatch(Origin::Members(1).into()).is_ok();
                    Self::deposit_event(RawEvent::Executed(proposal_hash,
                                                           ok));
                } else {
                    let index = Self::proposal_count();
                    <ProposalCount<T>>::mutate(|i| *i += 1);
                    <Proposals<T>>::mutate(|proposals|
                                               proposals.push(proposal_hash));
                    <ProposalOf<T>>::insert(proposal_hash, *proposal);
                    <Voting<T>>::insert(proposal_hash,
                                        (index, threshold,
                                         <[_]>::into_vec(box [who.clone()]),
                                         <[_]>::into_vec(box [])));
                    Self::deposit_event(RawEvent::Proposed(who, index,
                                                           proposal_hash,
                                                           threshold));
                }
            }
            Ok(())
        }
        fn vote(origin: <T as system::Trait>::Origin, proposal: T::Hash,
                index: ProposalIndex, approve: bool)
         -> ::srml_support::dispatch::Result {
            {
                let who = ensure_signed(origin)?;
                {
                    if !Self::is_councillor(&who) {
                        { return Err("voter not on council"); };
                    }
                };
                let mut voting =
                    Self::voting(&proposal).ok_or("proposal must exist")?;
                {
                    if !(voting.0 == index) {
                        { return Err("mismatched index"); };
                    }
                };
                let position_yes = voting.2.iter().position(|a| a == &who);
                let position_no = voting.3.iter().position(|a| a == &who);
                if approve {
                    if position_yes.is_none() {
                        voting.2.push(who.clone());
                    } else { return Err("duplicate vote ignored") }
                    if let Some(pos) = position_no {
                        voting.3.swap_remove(pos);
                    }
                } else {
                    if position_no.is_none() {
                        voting.3.push(who.clone());
                    } else { return Err("duplicate vote ignored") }
                    if let Some(pos) = position_yes {
                        voting.2.swap_remove(pos);
                    }
                }
                let yes_votes = voting.2.len() as u32;
                let no_votes = voting.3.len() as u32;
                Self::deposit_event(RawEvent::Voted(who, proposal, approve,
                                                    yes_votes, no_votes));
                let threshold = voting.1;
                let potential_votes =
                    <Council<T>>::active_council().len() as u32;
                let approved = yes_votes >= threshold;
                let disapproved =
                    potential_votes.saturating_sub(no_votes) < threshold;
                if approved || disapproved {
                    if approved {
                        Self::deposit_event(RawEvent::Approved(proposal));
                        if let Some(p) = <ProposalOf<T>>::take(&proposal) {
                            let ok =
                                p.dispatch(Origin::Members(threshold).into()).is_ok();
                            Self::deposit_event(RawEvent::Executed(proposal,
                                                                   ok));
                        }
                    } else {
                        Self::deposit_event(RawEvent::Disapproved(proposal));
                    }
                    <Voting<T>>::remove(&proposal);
                    <Proposals<T>>::mutate(|proposals|
                                               proposals.retain(|h|
                                                                    h !=
                                                                        &proposal));
                } else { <Voting<T>>::insert(&proposal, voting); }
            }
            Ok(())
        }
    }
    pub enum Call<T: Trait> {

        #[doc(hidden)]
        #[codec(skip)]
        __PhantomItem(::srml_support::rstd::marker::PhantomData<(T)>,
                      ::srml_support::dispatch::Never),

        #[allow(non_camel_case_types)]
        propose(
                #[codec(compact)]
                u32, Box<<T as Trait>::Proposal>),

        #[allow(non_camel_case_types)]
        vote(T::Hash,
             #[codec(compact)]
             ProposalIndex, bool),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Call: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <T: Trait> _parity_codec::Encode for Call<T> where
             Box<<T as Trait>::Proposal>: _parity_codec::Encode,
             Box<<T as Trait>::Proposal>: _parity_codec::Encode,
             T::Hash: _parity_codec::Encode, T::Hash: _parity_codec::Encode {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        Call::propose(ref aa, ref ba) => {
                            dest.push_byte(0usize as u8);
                            {
                                dest.push(&<<u32 as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          u32>>::from(aa));
                            }
                            dest.push(ba);
                        }
                        Call::vote(ref aa, ref ba, ref ca) => {
                            dest.push_byte(1usize as u8);
                            dest.push(aa);
                            {
                                dest.push(&<<ProposalIndex as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          ProposalIndex>>::from(ba));
                            }
                            dest.push(ca);
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Call: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <T: Trait> _parity_codec::Decode for Call<T> where
             Box<<T as Trait>::Proposal>: _parity_codec::Decode,
             Box<<T as Trait>::Proposal>: _parity_codec::Decode,
             T::Hash: _parity_codec::Decode, T::Hash: _parity_codec::Decode {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 0usize as u8 => {
                            Some(Call::propose(<<u32 as
                                                _parity_codec::HasCompact>::Type
                                                   as
                                                   _parity_codec::Decode>::decode(input)?.into(),
                                               _parity_codec::Decode::decode(input)?))
                        }
                        x if x == 1usize as u8 => {
                            Some(Call::vote(_parity_codec::Decode::decode(input)?,
                                            <<ProposalIndex as
                                             _parity_codec::HasCompact>::Type
                                                as
                                                _parity_codec::Decode>::decode(input)?.into(),
                                            _parity_codec::Decode::decode(input)?))
                        }
                        _ => None,
                    }
                }
            }
        };
    impl <T: Trait> ::srml_support::dispatch::Clone for Call<T> {
        fn clone(&self) -> Self {
            match *self {
                Call::propose(ref threshold, ref proposal) =>
                Call::propose((*threshold).clone(), (*proposal).clone()),
                Call::vote(ref proposal, ref index, ref approve) =>
                Call::vote((*proposal).clone(), (*index).clone(),
                           (*approve).clone()),
                _ => {
                    {
                        ::std::rt::begin_panic("internal error: entered unreachable code",
                                               &("srml/council/src/motions.rs",
                                                 66u32, 1u32))
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
        fn eq(&self, _other: &Self) -> bool {
            match *self {
                Call::propose(ref threshold, ref proposal) => {
                    let self_params = (threshold, proposal);
                    if let Call::propose(ref threshold, ref proposal) =
                           *_other {
                        self_params == (threshold, proposal)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/motions.rs",
                                                             66u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::vote(ref proposal, ref index, ref approve) => {
                    let self_params = (proposal, index, approve);
                    if let Call::vote(ref proposal, ref index, ref approve) =
                           *_other {
                        self_params == (proposal, index, approve)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/motions.rs",
                                                             66u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                _ => {
                    {
                        ::std::rt::begin_panic("internal error: entered unreachable code",
                                               &("srml/council/src/motions.rs",
                                                 66u32, 1u32))
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::Eq for Call<T> { }
    #[cfg(feature = "std")]
    impl <T: Trait> ::srml_support::dispatch::fmt::Debug for Call<T> {
        fn fmt(&self, _f: &mut ::srml_support::dispatch::fmt::Formatter)
         ->
             ::srml_support::dispatch::result::Result<(),
                                                      ::srml_support::dispatch::fmt::Error> {
            match *self {
                Call::propose(ref threshold, ref proposal) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"propose",
                                                                   &(threshold.clone(),
                                                                     proposal.clone()))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::vote(ref proposal, ref index, ref approve) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"vote",
                                                                   &(proposal.clone(),
                                                                     index.clone(),
                                                                     approve.clone()))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                _ => {
                    {
                        ::std::rt::begin_panic("internal error: entered unreachable code",
                                               &("srml/council/src/motions.rs",
                                                 66u32, 1u32))
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::Dispatchable for Call<T> {
        type
        Trait
        =
        T;
        type
        Origin
        =
        <T as system::Trait>::Origin;
        fn dispatch(self, _origin: Self::Origin)
         -> ::srml_support::dispatch::Result {
            match self {
                Call::propose(threshold, proposal) => {
                    <Module<T>>::propose(_origin, threshold, proposal)
                }
                Call::vote(proposal, index, approve) => {
                    <Module<T>>::vote(_origin, proposal, index, approve)
                }
                Call::__PhantomItem(_, _) => {
                    {
                        {
                            {
                                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                          &match (&"__PhantomItem should never be used.",)
                                                                                               {
                                                                                               (arg0,)
                                                                                               =>
                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::std::fmt::Display::fmt)],
                                                                                           }),
                                                           &("srml/council/src/motions.rs",
                                                             66u32, 1u32))
                            }
                        }
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::Callable for Module<T> {
        type
        Call
        =
        Call<T>;
    }
    impl <T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait =
                        T>>(d: D, origin: D::Origin)
         -> ::srml_support::dispatch::Result {
            d.dispatch(origin)
        }
    }
    impl <T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn call_functions()
         -> &'static [::srml_support::dispatch::FunctionMetadata] {
            &[::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("propose"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("threshold"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<u32>"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("proposal"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Box<<T as Trait>::Proposal>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("vote"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("proposal"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("index"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<ProposalIndex>"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("approve"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("bool"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[]),}]
        }
    }
    #[doc(hidden)]
    mod sr_api_hidden_includes_decl_storage {
        pub extern crate srml_support as hidden_include;
    }
    #[doc = " The (hashes of) the active proposals."]
    pub struct Proposals<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::Hash>>
     for Proposals<T> {
        type
        Query
        =
        Vec<T::Hash>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "CouncilMotions Proposals".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::Hash>>>::key()).unwrap_or_else(||
                                                                                                                                                                          Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::Hash>>>::key()).unwrap_or_else(||
                                                                                                                                                                           Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::Hash>>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::Hash>>>::put(&val,
                                                                                                                                        storage);
            ret
        }
    }
    #[doc = " Actual proposal for a given hash, if it\'s current."]
    pub struct ProposalOf<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                       <T
                                                                                                       as
                                                                                                       Trait>::Proposal>
     for ProposalOf<T> {
        type
        Query
        =
        Option<<T as Trait>::Proposal>;
        type
        Hasher
        =
        self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
        #[doc = r" Get the prefix key in storage."]
        fn prefix() -> &'static [u8] {
            "CouncilMotions ProposalOf".as_bytes()
        }
        #[doc =
              r" Get the storage key used to fetch a value corresponding to a specific key."]
        fn key_for(x: &T::Hash)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      <T
                                                                                                                      as
                                                                                                                      Trait>::Proposal>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                                &mut key);
            key
        }
        #[doc =
              r" Load the value associated with the given key from the map."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                       &T::Hash,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      <T
                                                                                                                      as
                                                                                                                      Trait>::Proposal>>::key_for(key);
            storage.get(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Take the value, reading and removing it."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                        &T::Hash,
                                                                                                                                                                    storage:
                                                                                                                                                                        &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      <T
                                                                                                                      as
                                                                                                                      Trait>::Proposal>>::key_for(key);
            storage.take(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Mutate the value under a key"]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                          &T::Hash,
                                                                                                                                                                      f:
                                                                                                                                                                          F,
                                                                                                                                                                      storage:
                                                                                                                                                                          &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      <T
                                                                                                                      as
                                                                                                                      Trait>::Proposal>>::get(key,
                                                                                                                                              storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      <T
                                                                                                                      as
                                                                                                                      Trait>::Proposal>>::insert(key,
                                                                                                                                                 &val,
                                                                                                                                                 storage),
                None =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      <T
                                                                                                                      as
                                                                                                                      Trait>::Proposal>>::remove(key,
                                                                                                                                                 storage),
            };
            ret
        }
    }
    #[doc =
          " Votes for a given proposal: (required_yes_votes, yes_voters, no_voters)."]
    pub struct Voting<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                       (ProposalIndex,
                                                                                                        u32,
                                                                                                        Vec<T::AccountId>,
                                                                                                        Vec<T::AccountId>)>
     for Voting<T> {
        type
        Query
        =
        Option<(ProposalIndex, u32, Vec<T::AccountId>, Vec<T::AccountId>)>;
        type
        Hasher
        =
        self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
        #[doc = r" Get the prefix key in storage."]
        fn prefix() -> &'static [u8] { "CouncilMotions Voting".as_bytes() }
        #[doc =
              r" Get the storage key used to fetch a value corresponding to a specific key."]
        fn key_for(x: &T::Hash)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (ProposalIndex,
                                                                                                                       u32,
                                                                                                                       Vec<T::AccountId>,
                                                                                                                       Vec<T::AccountId>)>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                                &mut key);
            key
        }
        #[doc =
              r" Load the value associated with the given key from the map."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                       &T::Hash,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (ProposalIndex,
                                                                                                                       u32,
                                                                                                                       Vec<T::AccountId>,
                                                                                                                       Vec<T::AccountId>)>>::key_for(key);
            storage.get(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Take the value, reading and removing it."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                        &T::Hash,
                                                                                                                                                                    storage:
                                                                                                                                                                        &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (ProposalIndex,
                                                                                                                       u32,
                                                                                                                       Vec<T::AccountId>,
                                                                                                                       Vec<T::AccountId>)>>::key_for(key);
            storage.take(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Mutate the value under a key"]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                          &T::Hash,
                                                                                                                                                                      f:
                                                                                                                                                                          F,
                                                                                                                                                                      storage:
                                                                                                                                                                          &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (ProposalIndex,
                                                                                                                       u32,
                                                                                                                       Vec<T::AccountId>,
                                                                                                                       Vec<T::AccountId>)>>::get(key,
                                                                                                                                                 storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (ProposalIndex,
                                                                                                                       u32,
                                                                                                                       Vec<T::AccountId>,
                                                                                                                       Vec<T::AccountId>)>>::insert(key,
                                                                                                                                                    &val,
                                                                                                                                                    storage),
                None =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                      (ProposalIndex,
                                                                                                                       u32,
                                                                                                                       Vec<T::AccountId>,
                                                                                                                       Vec<T::AccountId>)>>::remove(key,
                                                                                                                                                    storage),
            };
            ret
        }
    }
    #[doc = " Proposals so far."]
    pub struct ProposalCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>
     for ProposalCount<T> {
        type
        Query
        =
        u32;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] {
            "CouncilMotions ProposalCount".as_bytes()
        }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                                 Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                                  Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&val,
                                                                                                                               storage);
            ret
        }
    }
    trait Store {
        type
        Proposals;
        type
        ProposalOf;
        type
        Voting;
        type
        ProposalCount;
    }
    #[doc(hidden)]
    pub struct __GetByteStructProposals<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Proposals:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructProposals<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Proposals.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          Vec<T::Hash> =
                                                                      Default::default();
                                                                  <Vec<T::Hash>
                                                                      as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructProposalOf<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_ProposalOf:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructProposalOf<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_ProposalOf.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           Option<<T
                                                                                  as
                                                                                  Trait>::Proposal> =
                                                                       Default::default();
                                                                   <Option<<T
                                                                           as
                                                                           Trait>::Proposal>
                                                                       as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructVoting<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Voting:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructVoting<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Voting.get_or_init(||
                                                           {
                                                               let def_val:
                                                                       Option<(ProposalIndex,
                                                                               u32,
                                                                               Vec<T::AccountId>,
                                                                               Vec<T::AccountId>)> =
                                                                   Default::default();
                                                               <Option<(ProposalIndex,
                                                                        u32,
                                                                        Vec<T::AccountId>,
                                                                        Vec<T::AccountId>)>
                                                                   as
                                                                   Encode>::encode(&def_val)
                                                           }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructProposalCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_ProposalCount:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructProposalCount<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_ProposalCount.get_or_init(||
                                                                  {
                                                                      let def_val:
                                                                              u32 =
                                                                          Default::default();
                                                                      <u32 as
                                                                          Encode>::encode(&def_val)
                                                                  }).clone()
        }
    }
    impl <T: Trait> Store for Module<T> {
        type
        Proposals
        =
        Proposals<T>;
        type
        ProposalOf
        =
        ProposalOf<T>;
        type
        Voting
        =
        Voting<T>;
        type
        ProposalCount
        =
        ProposalCount<T>;
    }
    impl <T: 'static + Trait> Module<T> {
        #[doc = " The (hashes of) the active proposals."]
        pub fn proposals() -> Vec<T::Hash> {
            <Proposals<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::Hash>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc = " Actual proposal for a given hash, if it\'s current."]
        pub fn proposal_of<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::Hash>>(key:
                                                                                                                            K)
         -> Option<<T as Trait>::Proposal> {
            <ProposalOf<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                  <T
                                                                                                                  as
                                                                                                                  Trait>::Proposal>>::get(key.borrow(),
                                                                                                                                          &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " Votes for a given proposal: (required_yes_votes, yes_voters, no_voters)."]
        pub fn voting<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::Hash>>(key:
                                                                                                                       K)
         ->
             Option<(ProposalIndex, u32, Vec<T::AccountId>,
                     Vec<T::AccountId>)> {
            <Voting<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::Hash,
                                                                                                                  (ProposalIndex,
                                                                                                                   u32,
                                                                                                                   Vec<T::AccountId>,
                                                                                                                   Vec<T::AccountId>)>>::get(key.borrow(),
                                                                                                                                             &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc = " Proposals so far."]
        pub fn proposal_count() -> u32 {
            <ProposalCount<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc(hidden)]
        pub fn store_metadata()
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
            self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                      &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Proposals"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::Hash>")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposals::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The (hashes of) the active proposals."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalOf"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                       key:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                                                                                                                                                                                                       value:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("<T as Trait>::Proposal"),
                                                                                                                                                                                                                                                                                                                                                                                                       is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                           false,},
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Actual proposal for a given hash, if it\'s current."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Voting"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                       key:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                                                                                                                                                                                                       value:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(ProposalIndex, u32, Vec<T::AccountId>, Vec<T::AccountId>)"),
                                                                                                                                                                                                                                                                                                                                                                                                       is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                           false,},
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVoting::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Votes for a given proposal: (required_yes_votes, yes_voters, no_voters)."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalCount"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Proposals so far."]),}]
                                                                                                                                                                                                  }),}
        }
        #[doc(hidden)]
        pub fn store_metadata_functions()
         ->
             &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
            {
                &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Proposals"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::Hash>")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposals::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The (hashes of) the active proposals."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalOf"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                 key:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                 value:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("<T as Trait>::Proposal"),
                                                                                                                                                                                                                 is_linked:
                                                                                                                                                                                                                     false,},
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Actual proposal for a given hash, if it\'s current."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Voting"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                 key:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                 value:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(ProposalIndex, u32, Vec<T::AccountId>, Vec<T::AccountId>)"),
                                                                                                                                                                                                                 is_linked:
                                                                                                                                                                                                                     false,},
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVoting::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Votes for a given proposal: (required_yes_votes, yes_voters, no_voters)."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalCount"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Proposals so far."]),}]
            }
        }
        #[doc(hidden)]
        pub fn store_metadata_name() -> &'static str { "CouncilMotions" }
    }
    #[cfg(feature = "std")]
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    pub struct GenesisConfig {
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_GenesisConfig: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl _serde::Serialize for GenesisConfig {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    let __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                                                   "GenesisConfig",
                                                                   false as
                                                                       usize)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_GenesisConfig: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <'de> _serde::Deserialize<'de> for GenesisConfig {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { }
                    struct __FieldVisitor;
                    impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type
                        Value
                        =
                        __Field;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                _ =>
                                _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                     &"field index 0 <= i < 0")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                _ => {
                                    _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                         FIELDS))
                                }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                _ => {
                                    let __value =
                                        &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                         FIELDS))
                                }
                            }
                        }
                    }
                    impl <'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                         -> _serde::export::Result<Self, __D::Error> where
                         __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                         __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<GenesisConfig>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type
                        Value
                        =
                        GenesisConfig;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "struct GenesisConfig")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, _: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::SeqAccess<'de> {
                            _serde::export::Ok(GenesisConfig{})
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::MapAccess<'de> {
                            _serde::export::Option::map(match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                                            {
                                                            _serde::export::Ok(__val)
                                                            => __val,
                                                            _serde::export::Err(__err)
                                                            => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        },
                                                        |__impossible|
                                                            match __impossible
                                                                {
                                                            });
                            _serde::export::Ok(GenesisConfig{})
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                                                             "GenesisConfig",
                                                             FIELDS,
                                                             __Visitor{marker:
                                                                           _serde::export::PhantomData::<GenesisConfig>,
                                                                       lifetime:
                                                                           _serde::export::PhantomData,})
                }
            }
        };
    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self { GenesisConfig{} }
    }
    #[cfg(feature = "std")]
    impl self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::BuildStorage
     for GenesisConfig {
        fn assimilate_storage(self,
                              r:
                                  &mut self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::StorageOverlay,
                              c:
                                  &mut self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::ChildrenStorageOverlay)
         -> ::std::result::Result<(), String> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::cell::RefCell;
            let storage = RefCell::new(r);
            let r = storage.into_inner();
            (|_, _, _| { })(r, c, &self);
            Ok(())
        }
    }
    impl <T: Trait> Module<T> {
        pub fn is_councillor(who: &T::AccountId) -> bool {
            <Council<T>>::active_council().iter().any(|&(ref a, _)| a == who)
        }
    }
    /// Ensure that the origin `o` represents at least `n` council members. Returns
    /// `Ok` or an `Err` otherwise.
    pub fn ensure_council_members<OuterOrigin>(o: OuterOrigin, n: u32)
     -> result::Result<u32, &'static str> where
     OuterOrigin: Into<Option<Origin>> {
        match o.into() {
            Some(Origin::Members(x)) if x >= n => Ok(n),
            _ =>
            Err("bad origin: expected to be a threshold number of council members"),
        }
    }
    pub struct EnsureMembers<N: U32>(::rstd::marker::PhantomData<N>);
    impl <O, N: U32> EnsureOrigin<O> for EnsureMembers<N> where
     O: Into<Option<Origin>> {
        type
        Success
        =
        u32;
        fn ensure_origin(o: O)
         -> result::Result<Self::Success, &'static str> {
            ensure_council_members(o, N::VALUE)
        }
    }
}
pub mod seats {
    //! Council system: Handles the voting in and maintenance of council members.
    use rstd::prelude::*;
    use primitives::traits::{Zero, One, As, StaticLookup};
    use runtime_io::print;
    use srml_support::{StorageValue, StorageMap, dispatch::Result,
                       decl_storage, decl_event, ensure,
                       traits::{Currency, ReservableCurrency, OnUnbalanced}};
    use democracy;
    use system::{self, ensure_signed};
    use srml_support::decl_module;
    pub type VoteIndex = u32;
    type BalanceOf<T>
        =
        <<T as democracy::Trait>::Currency as
        Currency<<T as system::Trait>::AccountId>>::Balance;
    type NegativeImbalanceOf<T>
        =
        <<T as democracy::Trait>::Currency as
        Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;
    pub trait Trait: democracy::Trait {
        type
        Event: From<Event<Self>> +
        Into<<Self as system::Trait>::Event>;
        /// Handler for the unbalanced reduction when slashing a validator.
        type
        BadPresentation: OnUnbalanced<NegativeImbalanceOf<Self>>;
        /// Handler for the unbalanced reduction when slashing an invalid reaping attempt.
        type
        BadReaper: OnUnbalanced<NegativeImbalanceOf<Self>>;
    }
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub struct Module<T: Trait>(::srml_support::rstd::marker::PhantomData<(T)>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::clone::Clone + Trait> ::std::clone::Clone for Module<T> {
        #[inline]
        fn clone(&self) -> Module<T> {
            match *self {
                Module(ref __self_0_0) =>
                Module(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::marker::Copy + Trait> ::std::marker::Copy for Module<T> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::cmp::PartialEq + Trait> ::std::cmp::PartialEq for
     Module<T> {
        #[inline]
        fn eq(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) =>
                match *self {
                    Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) =>
                match *self {
                    Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::cmp::Eq + Trait> ::std::cmp::Eq for Module<T> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _:
                        ::std::cmp::AssertParamIsEq<::srml_support::rstd::marker::PhantomData<(T)>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::fmt::Debug + Trait> ::std::fmt::Debug for Module<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Module(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Module");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl <T: Trait>
     ::srml_support::runtime_primitives::traits::OnInitialize<T::BlockNumber>
     for Module<T> {
    }
    impl <T: Trait>
     ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber>
     for Module<T> {
        fn on_finalize(n: T::BlockNumber) {
            if let Err(e) = Self::end_block(n) {
                print("Guru meditation");
                print(e);
            }
        }
    }
    impl <T: Trait>
     ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
     for Module<T> {
    }
    impl <T: Trait> Module<T> {
        fn deposit_event(event: Event<T>) {
            <system::Module<T>>::deposit_event(<T as
                                                   Trait>::from(event).into());
        }
    }
    /// Can also be called using [`Call`].
    ///
    /// [`Call`]: enum.Call.html
    impl <T: Trait> Module<T> {
        #[doc =
              r" Set candidate approvals. Approval slots stay valid as long as candidates in those slots"]
        #[doc = r" are registered."]
        fn set_approvals(origin: T::Origin, votes: Vec<bool>,
                         index: VoteIndex) -> Result {
            let who = ensure_signed(origin)?;
            Self::do_set_approvals(who, votes, index)
        }
        #[doc =
              r" Set candidate approvals from a proxy. Approval slots stay valid as long as candidates in those slots"]
        #[doc = r" are registered."]
        fn proxy_set_approvals(origin: T::Origin, votes: Vec<bool>,
                               index: VoteIndex) -> Result {
            let who =
                <democracy::Module<T>>::proxy(ensure_signed(origin)?).ok_or("not a proxy")?;
            Self::do_set_approvals(who, votes, index)
        }
        #[doc =
              r" Remove a voter. For it not to be a bond-consuming no-op, all approved candidate indices"]
        #[doc =
              r" must now be either unregistered or registered to a candidate that registered the slot after"]
        #[doc = r" the voter gave their last approval set."]
        #[doc = r""]
        #[doc =
              r" May be called by anyone. Returns the voter deposit to `signed`."]
        fn reap_inactive_voter(origin: T::Origin, reporter_index: u32,
                               who: <T::Lookup as StaticLookup>::Source,
                               who_index: u32, assumed_vote_index: VoteIndex)
         -> ::srml_support::dispatch::Result {
            {
                let reporter = ensure_signed(origin)?;
                let who = T::Lookup::lookup(who)?;
                {
                    if !!Self::presentation_active() {
                        {
                            return Err("cannot reap during presentation period");
                        };
                    }
                };
                {
                    if !Self::voter_last_active(&reporter).is_some() {
                        { return Err("reporter must be a voter"); };
                    }
                };
                let last_active =
                    Self::voter_last_active(&who).ok_or("target for inactivity cleanup must be active")?;
                {
                    if !(assumed_vote_index == Self::vote_index()) {
                        { return Err("vote index not current"); };
                    }
                };
                {
                    if !(assumed_vote_index >
                             last_active + Self::inactivity_grace_period()) {
                        { return Err("cannot reap during grace period"); };
                    }
                };
                let voters = Self::voters();
                let reporter_index = reporter_index as usize;
                let who_index = who_index as usize;
                {
                    if !(reporter_index < voters.len() &&
                             voters[reporter_index] == reporter) {
                        { return Err("bad reporter index"); };
                    }
                };
                {
                    if !(who_index < voters.len() && voters[who_index] == who)
                       {
                        { return Err("bad target index"); };
                    }
                };
                let valid =
                    !Self::approvals_of(&who).iter().zip(Self::candidates().iter()).any(|(&appr,
                                                                                          addr)|
                                                                                            appr
                                                                                                &&
                                                                                                *addr
                                                                                                    !=
                                                                                                    T::AccountId::default()
                                                                                                &&
                                                                                                Self::candidate_reg_info(addr).map_or(false,
                                                                                                                                      |x|
                                                                                                                                          x.0
                                                                                                                                              <=
                                                                                                                                              last_active));
                Self::remove_voter(if valid { &who } else { &reporter },
                                   if valid {
                                       who_index
                                   } else { reporter_index }, voters);
                if valid {
                    T::Currency::repatriate_reserved(&who, &reporter,
                                                     Self::voting_bond())?;
                    Self::deposit_event(RawEvent::VoterReaped(who, reporter));
                } else {
                    let imbalance =
                        T::Currency::slash_reserved(&reporter,
                                                    Self::voting_bond()).0;
                    T::BadReaper::on_unbalanced(imbalance);
                    Self::deposit_event(RawEvent::BadReaperSlashed(reporter));
                }
            }
            Ok(())
        }
        #[doc =
              r" Remove a voter. All votes are cancelled and the voter deposit is returned."]
        fn retract_voter(origin: T::Origin, index: u32)
         -> ::srml_support::dispatch::Result {
            {
                let who = ensure_signed(origin)?;
                {
                    if !!Self::presentation_active() {
                        { return Err("cannot retract when presenting"); };
                    }
                };
                {
                    if !<LastActiveOf<T>>::exists(&who) {
                        { return Err("cannot retract non-voter"); };
                    }
                };
                let voters = Self::voters();
                let index = index as usize;
                {
                    if !(index < voters.len()) {
                        { return Err("retraction index invalid"); };
                    }
                };
                {
                    if !(voters[index] == who) {
                        { return Err("retraction index mismatch"); };
                    }
                };
                Self::remove_voter(&who, index, voters);
                T::Currency::unreserve(&who, Self::voting_bond());
            }
            Ok(())
        }
        #[doc = r" Submit oneself for candidacy."]
        #[doc = r""]
        #[doc =
              r" Account must have enough transferrable funds in it to pay the bond."]
        fn submit_candidacy(origin: T::Origin, slot: u32)
         -> ::srml_support::dispatch::Result {
            {
                let who = ensure_signed(origin)?;
                {
                    if !!Self::is_a_candidate(&who) {
                        { return Err("duplicate candidate submission"); };
                    }
                };
                let slot = slot as usize;
                let count = Self::candidate_count() as usize;
                let candidates = Self::candidates();
                {
                    if !((slot == count && count == candidates.len()) ||
                             (slot < candidates.len() &&
                                  candidates[slot] ==
                                      T::AccountId::default())) {
                        { return Err("invalid candidate slot"); };
                    }
                };
                T::Currency::reserve(&who,
                                     Self::candidacy_bond()).map_err(|_|
                                                                         "candidate has not enough funds")?;
                <RegisterInfoOf<T>>::insert(&who,
                                            (Self::vote_index(),
                                             slot as u32));
                let mut candidates = candidates;
                if slot == candidates.len() {
                    candidates.push(who);
                } else { candidates[slot] = who; }
                <Candidates<T>>::put(candidates);
                <CandidateCount<T>>::put(count as u32 + 1);
            }
            Ok(())
        }
        #[doc =
              r" Claim that `signed` is one of the top Self::carry_count() + current_vote().1 candidates."]
        #[doc =
              r" Only works if the `block_number >= current_vote().0` and `< current_vote().0 + presentation_duration()``"]
        #[doc = r" `signed` should have at least"]
        fn present_winner(origin: T::Origin,
                          candidate: <T::Lookup as StaticLookup>::Source,
                          total: BalanceOf<T>, index: VoteIndex) -> Result {
            let who = ensure_signed(origin)?;
            {
                if !!total.is_zero() {
                    {
                        return Err("stake deposited to present winner and be added to leaderboard should be non-zero");
                    };
                }
            };
            let candidate = T::Lookup::lookup(candidate)?;
            {
                if !(index == Self::vote_index()) {
                    { return Err("index not current"); };
                }
            };
            let (_, _, expiring) =
                Self::next_finalize().ok_or("cannot present outside of presentation period")?;
            let stakes = Self::snapshoted_stakes();
            let voters = Self::voters();
            let bad_presentation_punishment =
                Self::present_slash_per_voter() *
                    BalanceOf::<T>::sa(voters.len() as u64);
            {
                if !T::Currency::can_slash(&who, bad_presentation_punishment)
                   {
                    {
                        return Err("presenter must have sufficient slashable funds");
                    };
                }
            };
            let mut leaderboard =
                Self::leaderboard().ok_or("leaderboard must exist while present phase active")?;
            {
                if !(total > leaderboard[0].0) {
                    { return Err("candidate not worthy of leaderboard"); };
                }
            };
            if let Some(p) =
                   Self::active_council().iter().position(|&(ref c, _)|
                                                              c == &candidate)
                   {
                {
                    if !(p < expiring.len()) {
                        {
                            return Err("candidate must not form a duplicated member if elected");
                        };
                    }
                };
            }
            let (registered_since, candidate_index): (VoteIndex, u32) =
                Self::candidate_reg_info(&candidate).ok_or("presented candidate must be current")?;
            let actual_total =
                voters.iter().zip(stakes.iter()).filter_map(|(voter, stake)|
                                                                match Self::voter_last_active(voter)
                                                                    {
                                                                    Some(b) if
                                                                    b >=
                                                                        registered_since
                                                                    =>
                                                                    Self::approvals_of(voter).get(candidate_index
                                                                                                      as
                                                                                                      usize).and_then(|approved|
                                                                                                                          if *approved
                                                                                                                             {
                                                                                                                              Some(*stake)
                                                                                                                          } else {
                                                                                                                              None
                                                                                                                          }),
                                                                    _ => None,
                                                                }).fold(Zero::zero(),
                                                                        |acc,
                                                                         n|
                                                                            acc
                                                                                +
                                                                                n);
            let dupe =
                leaderboard.iter().find(|&&(_, ref c)|
                                            c == &candidate).is_some();
            if total == actual_total && !dupe {
                leaderboard[0] = (total, candidate);
                leaderboard.sort_by_key(|&(t, _)| t);
                <Leaderboard<T>>::put(leaderboard);
                Ok(())
            } else {
                let imbalance =
                    T::Currency::slash(&who, bad_presentation_punishment).0;
                T::BadPresentation::on_unbalanced(imbalance);
                Err(if dupe {
                        "duplicate presentation"
                    } else { "incorrect total" })
            }
        }
        #[doc =
              r" Set the desired member count; if lower than the current count, then seats will not be up"]
        #[doc =
              r" election when they expire. If more, then a new vote will be started if one is not already"]
        #[doc = r" in progress."]
        fn set_desired_seats(count: u32) -> ::srml_support::dispatch::Result {
            { <DesiredSeats<T>>::put(count); }
            Ok(())
        }
        #[doc =
              r" Remove a particular member. A tally will happen instantly (if not already in a presentation"]
        #[doc =
              r" period) to fill the seat if removal means that the desired members are not met."]
        #[doc = r" This is effective immediately."]
        fn remove_member(who: <T::Lookup as StaticLookup>::Source)
         -> ::srml_support::dispatch::Result {
            {
                let who = T::Lookup::lookup(who)?;
                let new_council: Vec<(T::AccountId, T::BlockNumber)> =
                    Self::active_council().into_iter().filter(|i|
                                                                  i.0 !=
                                                                      who).collect();
                <ActiveCouncil<T>>::put(new_council);
            }
            Ok(())
        }
        #[doc =
              r" Set the presentation duration. If there is currently a vote being presented for, will"]
        #[doc = r" invoke `finalize_vote`."]
        fn set_presentation_duration(count: T::BlockNumber)
         -> ::srml_support::dispatch::Result {
            { <PresentationDuration<T>>::put(count); }
            Ok(())
        }
        #[doc =
              r" Set the presentation duration. If there is current a vote being presented for, will"]
        #[doc = r" invoke `finalize_vote`."]
        fn set_term_duration(count: T::BlockNumber)
         -> ::srml_support::dispatch::Result {
            { <TermDuration<T>>::put(count); }
            Ok(())
        }
    }
    pub enum Call<T: Trait> {

        #[doc(hidden)]
        #[codec(skip)]
        __PhantomItem(::srml_support::rstd::marker::PhantomData<(T)>,
                      ::srml_support::dispatch::Never),

        #[allow(non_camel_case_types)]
        set_approvals(Vec<bool>,
                      #[codec(compact)]
                      VoteIndex),

        #[allow(non_camel_case_types)]
        proxy_set_approvals(Vec<bool>,
                            #[codec(compact)]
                            VoteIndex),

        #[allow(non_camel_case_types)]
        reap_inactive_voter(
                            #[codec(compact)]
                            u32, <T::Lookup as StaticLookup>::Source,
                            #[codec(compact)]
                            u32,
                            #[codec(compact)]
                            VoteIndex),

        #[allow(non_camel_case_types)]
        retract_voter(
                      #[codec(compact)]
                      u32),

        #[allow(non_camel_case_types)]
        submit_candidacy(
                         #[codec(compact)]
                         u32),

        #[allow(non_camel_case_types)]
        present_winner(<T::Lookup as StaticLookup>::Source,
                       #[codec(compact)]
                       BalanceOf<T>,
                       #[codec(compact)]
                       VoteIndex),

        #[allow(non_camel_case_types)]
        set_desired_seats(
                          #[codec(compact)]
                          u32),

        #[allow(non_camel_case_types)]
        remove_member(<T::Lookup as StaticLookup>::Source),

        #[allow(non_camel_case_types)]
        set_presentation_duration(
                                  #[codec(compact)]
                                  T::BlockNumber),

        #[allow(non_camel_case_types)]
        set_term_duration(
                          #[codec(compact)]
                          T::BlockNumber),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Call: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <T: Trait> _parity_codec::Encode for Call<T> where
             <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
             <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
             <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
             <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
             <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
             <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
             BalanceOf<T>: _parity_codec::HasCompact,
             T::BlockNumber: _parity_codec::HasCompact,
             T::BlockNumber: _parity_codec::HasCompact {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        Call::set_approvals(ref aa, ref ba) => {
                            dest.push_byte(0usize as u8);
                            dest.push(aa);
                            {
                                dest.push(&<<VoteIndex as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          VoteIndex>>::from(ba));
                            }
                        }
                        Call::proxy_set_approvals(ref aa, ref ba) => {
                            dest.push_byte(1usize as u8);
                            dest.push(aa);
                            {
                                dest.push(&<<VoteIndex as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          VoteIndex>>::from(ba));
                            }
                        }
                        Call::reap_inactive_voter(ref aa, ref ba, ref ca,
                                                  ref da) => {
                            dest.push_byte(2usize as u8);
                            {
                                dest.push(&<<u32 as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          u32>>::from(aa));
                            }
                            dest.push(ba);
                            {
                                dest.push(&<<u32 as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          u32>>::from(ca));
                            }
                            {
                                dest.push(&<<VoteIndex as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          VoteIndex>>::from(da));
                            }
                        }
                        Call::retract_voter(ref aa) => {
                            dest.push_byte(3usize as u8);
                            {
                                dest.push(&<<u32 as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          u32>>::from(aa));
                            }
                        }
                        Call::submit_candidacy(ref aa) => {
                            dest.push_byte(4usize as u8);
                            {
                                dest.push(&<<u32 as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          u32>>::from(aa));
                            }
                        }
                        Call::present_winner(ref aa, ref ba, ref ca) => {
                            dest.push_byte(5usize as u8);
                            dest.push(aa);
                            {
                                dest.push(&<<BalanceOf<T> as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          BalanceOf<T>>>::from(ba));
                            }
                            {
                                dest.push(&<<VoteIndex as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          VoteIndex>>::from(ca));
                            }
                        }
                        Call::set_desired_seats(ref aa) => {
                            dest.push_byte(6usize as u8);
                            {
                                dest.push(&<<u32 as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          u32>>::from(aa));
                            }
                        }
                        Call::remove_member(ref aa) => {
                            dest.push_byte(7usize as u8);
                            dest.push(aa);
                        }
                        Call::set_presentation_duration(ref aa) => {
                            dest.push_byte(8usize as u8);
                            {
                                dest.push(&<<T::BlockNumber as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          T::BlockNumber>>::from(aa));
                            }
                        }
                        Call::set_term_duration(ref aa) => {
                            dest.push_byte(9usize as u8);
                            {
                                dest.push(&<<T::BlockNumber as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::EncodeAsRef<'_,
                                                                          T::BlockNumber>>::from(aa));
                            }
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Call: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <T: Trait> _parity_codec::Decode for Call<T> where
             <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
             <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
             <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
             <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
             <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
             <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
             BalanceOf<T>: _parity_codec::HasCompact,
             T::BlockNumber: _parity_codec::HasCompact,
             T::BlockNumber: _parity_codec::HasCompact {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 0usize as u8 => {
                            Some(Call::set_approvals(_parity_codec::Decode::decode(input)?,
                                                     <<VoteIndex as
                                                      _parity_codec::HasCompact>::Type
                                                         as
                                                         _parity_codec::Decode>::decode(input)?.into()))
                        }
                        x if x == 1usize as u8 => {
                            Some(Call::proxy_set_approvals(_parity_codec::Decode::decode(input)?,
                                                           <<VoteIndex as
                                                            _parity_codec::HasCompact>::Type
                                                               as
                                                               _parity_codec::Decode>::decode(input)?.into()))
                        }
                        x if x == 2usize as u8 => {
                            Some(Call::reap_inactive_voter(<<u32 as
                                                            _parity_codec::HasCompact>::Type
                                                               as
                                                               _parity_codec::Decode>::decode(input)?.into(),
                                                           _parity_codec::Decode::decode(input)?,
                                                           <<u32 as
                                                            _parity_codec::HasCompact>::Type
                                                               as
                                                               _parity_codec::Decode>::decode(input)?.into(),
                                                           <<VoteIndex as
                                                            _parity_codec::HasCompact>::Type
                                                               as
                                                               _parity_codec::Decode>::decode(input)?.into()))
                        }
                        x if x == 3usize as u8 => {
                            Some(Call::retract_voter(<<u32 as
                                                      _parity_codec::HasCompact>::Type
                                                         as
                                                         _parity_codec::Decode>::decode(input)?.into()))
                        }
                        x if x == 4usize as u8 => {
                            Some(Call::submit_candidacy(<<u32 as
                                                         _parity_codec::HasCompact>::Type
                                                            as
                                                            _parity_codec::Decode>::decode(input)?.into()))
                        }
                        x if x == 5usize as u8 => {
                            Some(Call::present_winner(_parity_codec::Decode::decode(input)?,
                                                      <<BalanceOf<T> as
                                                       _parity_codec::HasCompact>::Type
                                                          as
                                                          _parity_codec::Decode>::decode(input)?.into(),
                                                      <<VoteIndex as
                                                       _parity_codec::HasCompact>::Type
                                                          as
                                                          _parity_codec::Decode>::decode(input)?.into()))
                        }
                        x if x == 6usize as u8 => {
                            Some(Call::set_desired_seats(<<u32 as
                                                          _parity_codec::HasCompact>::Type
                                                             as
                                                             _parity_codec::Decode>::decode(input)?.into()))
                        }
                        x if x == 7usize as u8 => {
                            Some(Call::remove_member(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 8usize as u8 => {
                            Some(Call::set_presentation_duration(<<T::BlockNumber
                                                                  as
                                                                  _parity_codec::HasCompact>::Type
                                                                     as
                                                                     _parity_codec::Decode>::decode(input)?.into()))
                        }
                        x if x == 9usize as u8 => {
                            Some(Call::set_term_duration(<<T::BlockNumber as
                                                          _parity_codec::HasCompact>::Type
                                                             as
                                                             _parity_codec::Decode>::decode(input)?.into()))
                        }
                        _ => None,
                    }
                }
            }
        };
    impl <T: Trait> ::srml_support::dispatch::Clone for Call<T> {
        fn clone(&self) -> Self {
            match *self {
                Call::set_approvals(ref votes, ref index) =>
                Call::set_approvals((*votes).clone(), (*index).clone()),
                Call::proxy_set_approvals(ref votes, ref index) =>
                Call::proxy_set_approvals((*votes).clone(), (*index).clone()),
                Call::reap_inactive_voter(ref reporter_index, ref who,
                                          ref who_index,
                                          ref assumed_vote_index) =>
                Call::reap_inactive_voter((*reporter_index).clone(),
                                          (*who).clone(),
                                          (*who_index).clone(),
                                          (*assumed_vote_index).clone()),
                Call::retract_voter(ref index) =>
                Call::retract_voter((*index).clone()),
                Call::submit_candidacy(ref slot) =>
                Call::submit_candidacy((*slot).clone()),
                Call::present_winner(ref candidate, ref total, ref index) =>
                Call::present_winner((*candidate).clone(), (*total).clone(),
                                     (*index).clone()),
                Call::set_desired_seats(ref count) =>
                Call::set_desired_seats((*count).clone()),
                Call::remove_member(ref who) =>
                Call::remove_member((*who).clone()),
                Call::set_presentation_duration(ref count) =>
                Call::set_presentation_duration((*count).clone()),
                Call::set_term_duration(ref count) =>
                Call::set_term_duration((*count).clone()),
                _ => {
                    {
                        ::std::rt::begin_panic("internal error: entered unreachable code",
                                               &("srml/council/src/seats.rs",
                                                 100u32, 1u32))
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
        fn eq(&self, _other: &Self) -> bool {
            match *self {
                Call::set_approvals(ref votes, ref index) => {
                    let self_params = (votes, index);
                    if let Call::set_approvals(ref votes, ref index) = *_other
                           {
                        self_params == (votes, index)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::proxy_set_approvals(ref votes, ref index) => {
                    let self_params = (votes, index);
                    if let Call::proxy_set_approvals(ref votes, ref index) =
                           *_other {
                        self_params == (votes, index)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::reap_inactive_voter(ref reporter_index, ref who,
                                          ref who_index,
                                          ref assumed_vote_index) => {
                    let self_params =
                        (reporter_index, who, who_index, assumed_vote_index);
                    if let Call::reap_inactive_voter(ref reporter_index,
                                                     ref who, ref who_index,
                                                     ref assumed_vote_index) =
                           *_other {
                        self_params ==
                            (reporter_index, who, who_index,
                             assumed_vote_index)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::retract_voter(ref index) => {
                    let self_params = (index,);
                    if let Call::retract_voter(ref index) = *_other {
                        self_params == (index,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::submit_candidacy(ref slot) => {
                    let self_params = (slot,);
                    if let Call::submit_candidacy(ref slot) = *_other {
                        self_params == (slot,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::present_winner(ref candidate, ref total, ref index) => {
                    let self_params = (candidate, total, index);
                    if let Call::present_winner(ref candidate, ref total,
                                                ref index) = *_other {
                        self_params == (candidate, total, index)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::set_desired_seats(ref count) => {
                    let self_params = (count,);
                    if let Call::set_desired_seats(ref count) = *_other {
                        self_params == (count,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::remove_member(ref who) => {
                    let self_params = (who,);
                    if let Call::remove_member(ref who) = *_other {
                        self_params == (who,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::set_presentation_duration(ref count) => {
                    let self_params = (count,);
                    if let Call::set_presentation_duration(ref count) =
                           *_other {
                        self_params == (count,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                Call::set_term_duration(ref count) => {
                    let self_params = (count,);
                    if let Call::set_term_duration(ref count) = *_other {
                        self_params == (count,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => {
                                {
                                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                                }
                            }
                            _ => false,
                        }
                    }
                }
                _ => {
                    {
                        ::std::rt::begin_panic("internal error: entered unreachable code",
                                               &("srml/council/src/seats.rs",
                                                 100u32, 1u32))
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::Eq for Call<T> { }
    #[cfg(feature = "std")]
    impl <T: Trait> ::srml_support::dispatch::fmt::Debug for Call<T> {
        fn fmt(&self, _f: &mut ::srml_support::dispatch::fmt::Formatter)
         ->
             ::srml_support::dispatch::result::Result<(),
                                                      ::srml_support::dispatch::fmt::Error> {
            match *self {
                Call::set_approvals(ref votes, ref index) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"set_approvals",
                                                                   &(votes.clone(),
                                                                     index.clone()))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::proxy_set_approvals(ref votes, ref index) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"proxy_set_approvals",
                                                                   &(votes.clone(),
                                                                     index.clone()))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::reap_inactive_voter(ref reporter_index, ref who,
                                          ref who_index,
                                          ref assumed_vote_index) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"reap_inactive_voter",
                                                                   &(reporter_index.clone(),
                                                                     who.clone(),
                                                                     who_index.clone(),
                                                                     assumed_vote_index.clone()))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::retract_voter(ref index) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"retract_voter",
                                                                   &(index.clone(),))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::submit_candidacy(ref slot) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"submit_candidacy",
                                                                   &(slot.clone(),))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::present_winner(ref candidate, ref total, ref index) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"present_winner",
                                                                   &(candidate.clone(),
                                                                     total.clone(),
                                                                     index.clone()))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::set_desired_seats(ref count) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"set_desired_seats",
                                                                   &(count.clone(),))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::remove_member(ref who) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"remove_member",
                                                                   &(who.clone(),))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::set_presentation_duration(ref count) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"set_presentation_duration",
                                                                   &(count.clone(),))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                Call::set_term_duration(ref count) =>
                _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                           &match (&"set_term_duration",
                                                                   &(count.clone(),))
                                                                {
                                                                (arg0, arg1)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Debug::fmt)],
                                                            })),
                _ => {
                    {
                        ::std::rt::begin_panic("internal error: entered unreachable code",
                                               &("srml/council/src/seats.rs",
                                                 100u32, 1u32))
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::Dispatchable for Call<T> {
        type
        Trait
        =
        T;
        type
        Origin
        =
        T::Origin;
        fn dispatch(self, _origin: Self::Origin)
         -> ::srml_support::dispatch::Result {
            match self {
                Call::set_approvals(votes, index) => {
                    <Module<T>>::set_approvals(_origin, votes, index)
                }
                Call::proxy_set_approvals(votes, index) => {
                    <Module<T>>::proxy_set_approvals(_origin, votes, index)
                }
                Call::reap_inactive_voter(reporter_index, who, who_index,
                                          assumed_vote_index) => {
                    <Module<T>>::reap_inactive_voter(_origin, reporter_index,
                                                     who, who_index,
                                                     assumed_vote_index)
                }
                Call::retract_voter(index) => {
                    <Module<T>>::retract_voter(_origin, index)
                }
                Call::submit_candidacy(slot) => {
                    <Module<T>>::submit_candidacy(_origin, slot)
                }
                Call::present_winner(candidate, total, index) => {
                    <Module<T>>::present_winner(_origin, candidate, total,
                                                index)
                }
                Call::set_desired_seats(count) => {
                    {
                        system::ensure_root(_origin)?;
                        <Module<T>>::set_desired_seats(count)
                    }
                }
                Call::remove_member(who) => {
                    {
                        system::ensure_root(_origin)?;
                        <Module<T>>::remove_member(who)
                    }
                }
                Call::set_presentation_duration(count) => {
                    {
                        system::ensure_root(_origin)?;
                        <Module<T>>::set_presentation_duration(count)
                    }
                }
                Call::set_term_duration(count) => {
                    {
                        system::ensure_root(_origin)?;
                        <Module<T>>::set_term_duration(count)
                    }
                }
                Call::__PhantomItem(_, _) => {
                    {
                        {
                            {
                                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                          &match (&"__PhantomItem should never be used.",)
                                                                                               {
                                                                                               (arg0,)
                                                                                               =>
                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::std::fmt::Display::fmt)],
                                                                                           }),
                                                           &("srml/council/src/seats.rs",
                                                             100u32, 1u32))
                            }
                        }
                    }
                }
            }
        }
    }
    impl <T: Trait> ::srml_support::dispatch::Callable for Module<T> {
        type
        Call
        =
        Call<T>;
    }
    impl <T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait =
                        T>>(d: D, origin: D::Origin)
         -> ::srml_support::dispatch::Result {
            d.dispatch(origin)
        }
    }
    impl <T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn call_functions()
         -> &'static [::srml_support::dispatch::FunctionMetadata] {
            &[::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("set_approvals"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("votes"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Vec<bool>"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("index"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<VoteIndex>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set candidate approvals. Approval slots stay valid as long as candidates in those slots",
                                                                                                                 r" are registered."]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("proxy_set_approvals"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("votes"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Vec<bool>"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("index"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<VoteIndex>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set candidate approvals from a proxy. Approval slots stay valid as long as candidates in those slots",
                                                                                                                 r" are registered."]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("reap_inactive_voter"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("reporter_index"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<u32>"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("who"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("<T::Lookup as StaticLookup>::Source"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("who_index"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<u32>"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("assumed_vote_index"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<VoteIndex>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Remove a voter. For it not to be a bond-consuming no-op, all approved candidate indices",
                                                                                                                 r" must now be either unregistered or registered to a candidate that registered the slot after",
                                                                                                                 r" the voter gave their last approval set.",
                                                                                                                 r"",
                                                                                                                 r" May be called by anyone. Returns the voter deposit to `signed`."]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("retract_voter"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("index"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<u32>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Remove a voter. All votes are cancelled and the voter deposit is returned."]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("submit_candidacy"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("slot"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<u32>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Submit oneself for candidacy.",
                                                                                                                 r"",
                                                                                                                 r" Account must have enough transferrable funds in it to pay the bond."]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("present_winner"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("candidate"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("<T::Lookup as StaticLookup>::Source"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("total"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<BalanceOf<T>>"),},
                                                                                                                 ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("index"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<VoteIndex>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Claim that `signed` is one of the top Self::carry_count() + current_vote().1 candidates.",
                                                                                                                 r" Only works if the `block_number >= current_vote().0` and `< current_vote().0 + presentation_duration()``",
                                                                                                                 r" `signed` should have at least"]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("set_desired_seats"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("count"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<u32>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the desired member count; if lower than the current count, then seats will not be up",
                                                                                                                 r" election when they expire. If more, then a new vote will be started if one is not already",
                                                                                                                 r" in progress."]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("remove_member"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("who"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("<T::Lookup as StaticLookup>::Source"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Remove a particular member. A tally will happen instantly (if not already in a presentation",
                                                                                                                 r" period) to fill the seat if removal means that the desired members are not met.",
                                                                                                                 r" This is effective immediately."]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("set_presentation_duration"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("count"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::BlockNumber>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the presentation duration. If there is currently a vote being presented for, will",
                                                                                                                 r" invoke `finalize_vote`."]),},
              ::srml_support::dispatch::FunctionMetadata{name:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode("set_term_duration"),
                                                         arguments:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("count"),
                                                                                                                                                                    ty:
                                                                                                                                                                        ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::BlockNumber>"),}]),
                                                         documentation:
                                                             ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the presentation duration. If there is current a vote being presented for, will",
                                                                                                                 r" invoke `finalize_vote`."]),}]
        }
    }
    #[doc(hidden)]
    mod sr_api_hidden_includes_decl_storage {
        pub extern crate srml_support as hidden_include;
    }
    #[doc =
          " How much should be locked up in order to submit one\'s candidacy."]
    pub struct CandidacyBond<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
     for CandidacyBond<T> {
        type
        Query
        =
        BalanceOf<T>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council CandidacyBond".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                          BalanceOf::<T>::sa(9))
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                           BalanceOf::<T>::sa(9))
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::put(&val,
                                                                                                                                        storage);
            ret
        }
    }
    #[doc =
          " How much should be locked up in order to be able to submit votes."]
    pub struct VotingBond<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
     for VotingBond<T> {
        type
        Query
        =
        BalanceOf<T>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council VotingBond".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                          Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                           Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::put(&val,
                                                                                                                                        storage);
            ret
        }
    }
    #[doc =
          " The punishment, per voter, if you provide an invalid presentation."]
    pub struct PresentSlashPerVoter<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
     for PresentSlashPerVoter<T> {
        type
        Query
        =
        BalanceOf<T>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] {
            "Council PresentSlashPerVoter".as_bytes()
        }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                          BalanceOf::<T>::sa(1))
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                           BalanceOf::<T>::sa(1))
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::put(&val,
                                                                                                                                        storage);
            ret
        }
    }
    #[doc =
          " How many runners-up should have their approvals persist until the next vote."]
    pub struct CarryCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>
     for CarryCount<T> {
        type
        Query
        =
        u32;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council CarryCount".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                                 2)
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                                  2)
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&val,
                                                                                                                               storage);
            ret
        }
    }
    #[doc =
          " How long to give each top candidate to present themselves after the vote ends."]
    pub struct PresentationDuration<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
     for PresentationDuration<T> {
        type
        Query
        =
        T::BlockNumber;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] {
            "Council PresentationDuration".as_bytes()
        }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                            T::BlockNumber::sa(1000))
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                             T::BlockNumber::sa(1000))
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&val,
                                                                                                                                          storage);
            ret
        }
    }
    #[doc =
          " How many vote indexes need to go by after a target voter\'s last vote before they can be reaped if their"]
    #[doc = " approvals are moot."]
    pub struct InactiveGracePeriod<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>
     for InactiveGracePeriod<T> {
        type
        Query
        =
        VoteIndex;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council InactiveGracePeriod".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                       1)
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                        1)
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::put(&val,
                                                                                                                                     storage);
            ret
        }
    }
    #[doc = " How often (in blocks) to check for new votes."]
    pub struct VotingPeriod<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
     for VotingPeriod<T> {
        type
        Query
        =
        T::BlockNumber;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council VotingPeriod".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                            T::BlockNumber::sa(1000))
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                             T::BlockNumber::sa(1000))
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&val,
                                                                                                                                          storage);
            ret
        }
    }
    #[doc = " How long each position is active for."]
    pub struct TermDuration<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
     for TermDuration<T> {
        type
        Query
        =
        T::BlockNumber;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council TermDuration".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                            T::BlockNumber::sa(5))
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                             T::BlockNumber::sa(5))
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&val,
                                                                                                                                          storage);
            ret
        }
    }
    #[doc = " Number of accounts that should be sitting on the council."]
    pub struct DesiredSeats<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>
     for DesiredSeats<T> {
        type
        Query
        =
        u32;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council DesiredSeats".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                                 Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                                  Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&val,
                                                                                                                               storage);
            ret
        }
    }
    #[doc =
          " The current council. When there\'s a vote going on, this should still be used for executive"]
    #[doc =
          " matters. The block number (second element in the tuple) is the block that their position is"]
    #[doc =
          " active until (calculated by the sum of the block number when the council member was elected"]
    #[doc = " and their term duration)."]
    pub struct ActiveCouncil<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                              T::BlockNumber)>>
     for ActiveCouncil<T> {
        type
        Query
        =
        Vec<(T::AccountId, T::BlockNumber)>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council ActiveCouncil".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                                     T::BlockNumber)>>>::key()).unwrap_or_else(||
                                                                                                                                                                                   Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                                      T::BlockNumber)>>>::key()).unwrap_or_else(||
                                                                                                                                                                                    Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                             T::BlockNumber)>>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                         T::BlockNumber)>>>::put(&val,
                                                                                                                                                 storage);
            ret
        }
    }
    #[doc =
          " The total number of votes that have happened or are in progress."]
    pub struct VoteCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>
     for VoteCount<T> {
        type
        Query
        =
        VoteIndex;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council VoteCount".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                       Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                        Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::put(&val,
                                                                                                                                     storage);
            ret
        }
    }
    #[doc =
          " A list of votes for each voter, respecting the last cleared vote index that this voter was"]
    #[doc = " last active at."]
    pub struct ApprovalsOf<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                       Vec<bool>>
     for ApprovalsOf<T> {
        type
        Query
        =
        Vec<bool>;
        type
        Hasher
        =
        self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
        #[doc = r" Get the prefix key in storage."]
        fn prefix() -> &'static [u8] { "Council ApprovalsOf".as_bytes() }
        #[doc =
              r" Get the storage key used to fetch a value corresponding to a specific key."]
        fn key_for(x: &T::AccountId)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      Vec<bool>>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                                &mut key);
            key
        }
        #[doc =
              r" Load the value associated with the given key from the map."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                       &T::AccountId,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      Vec<bool>>>::key_for(key);
            storage.get(&key[..]).unwrap_or_else(|| Default::default())
        }
        #[doc = r" Take the value, reading and removing it."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                        &T::AccountId,
                                                                                                                                                                    storage:
                                                                                                                                                                        &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      Vec<bool>>>::key_for(key);
            storage.take(&key[..]).unwrap_or_else(|| Default::default())
        }
        #[doc = r" Mutate the value under a key"]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                          &T::AccountId,
                                                                                                                                                                      f:
                                                                                                                                                                          F,
                                                                                                                                                                      storage:
                                                                                                                                                                          &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      Vec<bool>>>::get(key,
                                                                                                                                       storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  Vec<bool>>>::insert(key,
                                                                                                                                      &val,
                                                                                                                                      storage);
            ret
        }
    }
    #[doc =
          " The vote index and list slot that the candidate `who` was registered or `None` if they are not"]
    #[doc = " currently registered."]
    pub struct RegisterInfoOf<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                       (VoteIndex,
                                                                                                        u32)>
     for RegisterInfoOf<T> {
        type
        Query
        =
        Option<(VoteIndex, u32)>;
        type
        Hasher
        =
        self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
        #[doc = r" Get the prefix key in storage."]
        fn prefix() -> &'static [u8] { "Council RegisterInfoOf".as_bytes() }
        #[doc =
              r" Get the storage key used to fetch a value corresponding to a specific key."]
        fn key_for(x: &T::AccountId)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      (VoteIndex,
                                                                                                                       u32)>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                                &mut key);
            key
        }
        #[doc =
              r" Load the value associated with the given key from the map."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                       &T::AccountId,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      (VoteIndex,
                                                                                                                       u32)>>::key_for(key);
            storage.get(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Take the value, reading and removing it."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                        &T::AccountId,
                                                                                                                                                                    storage:
                                                                                                                                                                        &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      (VoteIndex,
                                                                                                                       u32)>>::key_for(key);
            storage.take(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Mutate the value under a key"]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                          &T::AccountId,
                                                                                                                                                                      f:
                                                                                                                                                                          F,
                                                                                                                                                                      storage:
                                                                                                                                                                          &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      (VoteIndex,
                                                                                                                       u32)>>::get(key,
                                                                                                                                   storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      (VoteIndex,
                                                                                                                       u32)>>::insert(key,
                                                                                                                                      &val,
                                                                                                                                      storage),
                None =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      (VoteIndex,
                                                                                                                       u32)>>::remove(key,
                                                                                                                                      storage),
            };
            ret
        }
    }
    #[doc =
          " The last cleared vote index that this voter was last active at."]
    pub struct LastActiveOf<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                       VoteIndex>
     for LastActiveOf<T> {
        type
        Query
        =
        Option<VoteIndex>;
        type
        Hasher
        =
        self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
        #[doc = r" Get the prefix key in storage."]
        fn prefix() -> &'static [u8] { "Council LastActiveOf".as_bytes() }
        #[doc =
              r" Get the storage key used to fetch a value corresponding to a specific key."]
        fn key_for(x: &T::AccountId)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      VoteIndex>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                                &mut key);
            key
        }
        #[doc =
              r" Load the value associated with the given key from the map."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                       &T::AccountId,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      VoteIndex>>::key_for(key);
            storage.get(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Take the value, reading and removing it."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                        &T::AccountId,
                                                                                                                                                                    storage:
                                                                                                                                                                        &S)
         -> Self::Query {
            let key =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      VoteIndex>>::key_for(key);
            storage.take(&key[..]).or_else(|| Default::default())
        }
        #[doc = r" Mutate the value under a key"]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                          &T::AccountId,
                                                                                                                                                                      f:
                                                                                                                                                                          F,
                                                                                                                                                                      storage:
                                                                                                                                                                          &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      VoteIndex>>::get(key,
                                                                                                                                       storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      VoteIndex>>::insert(key,
                                                                                                                                          &val,
                                                                                                                                          storage),
                None =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      VoteIndex>>::remove(key,
                                                                                                                                          storage),
            };
            ret
        }
    }
    #[doc = " The present voter list."]
    pub struct Voters<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>
     for Voters<T> {
        type
        Query
        =
        Vec<T::AccountId>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council Voters".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::key()).unwrap_or_else(||
                                                                                                                                                                               Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::key()).unwrap_or_else(||
                                                                                                                                                                                Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::put(&val,
                                                                                                                                             storage);
            ret
        }
    }
    #[doc = " The present candidate list."]
    pub struct Candidates<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>
     for Candidates<T> {
        type
        Query
        =
        Vec<T::AccountId>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council Candidates".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::key()).unwrap_or_else(||
                                                                                                                                                                               Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::key()).unwrap_or_else(||
                                                                                                                                                                                Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::put(&val,
                                                                                                                                             storage);
            ret
        }
    }
    pub struct CandidateCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>
     for CandidateCount<T> {
        type
        Query
        =
        u32;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council CandidateCount".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                                 Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                                  Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&val,
                                                                                                                               storage);
            ret
        }
    }
    #[doc =
          " The accounts holding the seats that will become free on the next tally."]
    pub struct NextFinalize<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(T::BlockNumber,
                                                                                                          u32,
                                                                                                          Vec<T::AccountId>)>
     for NextFinalize<T> {
        type
        Query
        =
        Option<(T::BlockNumber, u32, Vec<T::AccountId>)>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council NextFinalize".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(T::BlockNumber,
                                                                                                                                 u32,
                                                                                                                                 Vec<T::AccountId>)>>::key()).or_else(||
                                                                                                                                                                          Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(T::BlockNumber,
                                                                                                                                  u32,
                                                                                                                                  Vec<T::AccountId>)>>::key()).or_else(||
                                                                                                                                                                           Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(T::BlockNumber,
                                                                                                                         u32,
                                                                                                                         Vec<T::AccountId>)>>::get(storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(T::BlockNumber,
                                                                                                                         u32,
                                                                                                                         Vec<T::AccountId>)>>::put(&val,
                                                                                                                                                   storage),
                None =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(T::BlockNumber,
                                                                                                                         u32,
                                                                                                                         Vec<T::AccountId>)>>::kill(storage),
            };
            ret
        }
    }
    #[doc = " The stakes as they were at the point that the vote ended."]
    pub struct SnapshotedStakes<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<BalanceOf<T>>>
     for SnapshotedStakes<T> {
        type
        Query
        =
        Vec<BalanceOf<T>>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council SnapshotedStakes".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<BalanceOf<T>>>>::key()).unwrap_or_else(||
                                                                                                                                                                               Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<BalanceOf<T>>>>::key()).unwrap_or_else(||
                                                                                                                                                                                Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<BalanceOf<T>>>>::get(storage);
            let ret = f(&mut val);
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<BalanceOf<T>>>>::put(&val,
                                                                                                                                             storage);
            ret
        }
    }
    #[doc = " Get the leaderboard if we;re in the presentation phase."]
    pub struct Leaderboard<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(BalanceOf<T>,
                                                                                                              T::AccountId)>>
     for Leaderboard<T> {
        type
        Query
        =
        Option<Vec<(BalanceOf<T>, T::AccountId)>>;
        #[doc = r" Get the storage key."]
        fn key() -> &'static [u8] { "Council Leaderboard".as_bytes() }
        #[doc = r" Load the value from the provided storage instance."]
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                    &S)
         -> Self::Query {
            storage.get(<Self as
                            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(BalanceOf<T>,
                                                                                                                                     T::AccountId)>>>::key()).or_else(||
                                                                                                                                                                          Default::default())
        }
        #[doc = r" Take a value from storage, removing it afterwards."]
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                     &S)
         -> Self::Query {
            storage.take(<Self as
                             self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(BalanceOf<T>,
                                                                                                                                      T::AccountId)>>>::key()).or_else(||
                                                                                                                                                                           Default::default())
        }
        #[doc = r" Mutate the value under a key."]
        fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
                  S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                       F,
                                                                                                                                                                   storage:
                                                                                                                                                                       &S)
         -> R {
            let mut val =
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(BalanceOf<T>,
                                                                                                                             T::AccountId)>>>::get(storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(BalanceOf<T>,
                                                                                                                             T::AccountId)>>>::put(&val,
                                                                                                                                                   storage),
                None =>
                <Self as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(BalanceOf<T>,
                                                                                                                             T::AccountId)>>>::kill(storage),
            };
            ret
        }
    }
    trait Store {
        type
        CandidacyBond;
        type
        VotingBond;
        type
        PresentSlashPerVoter;
        type
        CarryCount;
        type
        PresentationDuration;
        type
        InactiveGracePeriod;
        type
        VotingPeriod;
        type
        TermDuration;
        type
        DesiredSeats;
        type
        ActiveCouncil;
        type
        VoteCount;
        type
        ApprovalsOf;
        type
        RegisterInfoOf;
        type
        LastActiveOf;
        type
        Voters;
        type
        Candidates;
        type
        CandidateCount;
        type
        NextFinalize;
        type
        SnapshotedStakes;
        type
        Leaderboard;
    }
    #[doc(hidden)]
    pub struct __GetByteStructCandidacyBond<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_CandidacyBond:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructCandidacyBond<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_CandidacyBond.get_or_init(||
                                                                  {
                                                                      let def_val:
                                                                              BalanceOf<T> =
                                                                          BalanceOf::<T>::sa(9);
                                                                      <BalanceOf<T>
                                                                          as
                                                                          Encode>::encode(&def_val)
                                                                  }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructVotingBond<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_VotingBond:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructVotingBond<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_VotingBond.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           BalanceOf<T> =
                                                                       Default::default();
                                                                   <BalanceOf<T>
                                                                       as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructPresentSlashPerVoter<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_PresentSlashPerVoter:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructPresentSlashPerVoter<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_PresentSlashPerVoter.get_or_init(||
                                                                         {
                                                                             let def_val:
                                                                                     BalanceOf<T> =
                                                                                 BalanceOf::<T>::sa(1);
                                                                             <BalanceOf<T>
                                                                                 as
                                                                                 Encode>::encode(&def_val)
                                                                         }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructCarryCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_CarryCount:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructCarryCount<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_CarryCount.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           u32 =
                                                                       2;
                                                                   <u32 as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructPresentationDuration<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_PresentationDuration:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructPresentationDuration<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_PresentationDuration.get_or_init(||
                                                                         {
                                                                             let def_val:
                                                                                     T::BlockNumber =
                                                                                 T::BlockNumber::sa(1000);
                                                                             <T::BlockNumber
                                                                                 as
                                                                                 Encode>::encode(&def_val)
                                                                         }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructInactiveGracePeriod<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_InactiveGracePeriod:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructInactiveGracePeriod<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_InactiveGracePeriod.get_or_init(||
                                                                        {
                                                                            let def_val:
                                                                                    VoteIndex =
                                                                                1;
                                                                            <VoteIndex
                                                                                as
                                                                                Encode>::encode(&def_val)
                                                                        }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructVotingPeriod<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_VotingPeriod:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructVotingPeriod<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_VotingPeriod.get_or_init(||
                                                                 {
                                                                     let def_val:
                                                                             T::BlockNumber =
                                                                         T::BlockNumber::sa(1000);
                                                                     <T::BlockNumber
                                                                         as
                                                                         Encode>::encode(&def_val)
                                                                 }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructTermDuration<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_TermDuration:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructTermDuration<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_TermDuration.get_or_init(||
                                                                 {
                                                                     let def_val:
                                                                             T::BlockNumber =
                                                                         T::BlockNumber::sa(5);
                                                                     <T::BlockNumber
                                                                         as
                                                                         Encode>::encode(&def_val)
                                                                 }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructDesiredSeats<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_DesiredSeats:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructDesiredSeats<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_DesiredSeats.get_or_init(||
                                                                 {
                                                                     let def_val:
                                                                             u32 =
                                                                         Default::default();
                                                                     <u32 as
                                                                         Encode>::encode(&def_val)
                                                                 }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructActiveCouncil<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_ActiveCouncil:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructActiveCouncil<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_ActiveCouncil.get_or_init(||
                                                                  {
                                                                      let def_val:
                                                                              Vec<(T::AccountId,
                                                                                   T::BlockNumber)> =
                                                                          Default::default();
                                                                      <Vec<(T::AccountId,
                                                                            T::BlockNumber)>
                                                                          as
                                                                          Encode>::encode(&def_val)
                                                                  }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructVoteCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_VoteCount:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructVoteCount<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_VoteCount.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          VoteIndex =
                                                                      Default::default();
                                                                  <VoteIndex
                                                                      as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructApprovalsOf<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_ApprovalsOf:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructApprovalsOf<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_ApprovalsOf.get_or_init(||
                                                                {
                                                                    let def_val:
                                                                            Vec<bool> =
                                                                        Default::default();
                                                                    <Vec<bool>
                                                                        as
                                                                        Encode>::encode(&def_val)
                                                                }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructRegisterInfoOf<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_RegisterInfoOf:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructRegisterInfoOf<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_RegisterInfoOf.get_or_init(||
                                                                   {
                                                                       let def_val:
                                                                               Option<(VoteIndex,
                                                                                       u32)> =
                                                                           Default::default();
                                                                       <Option<(VoteIndex,
                                                                                u32)>
                                                                           as
                                                                           Encode>::encode(&def_val)
                                                                   }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructLastActiveOf<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_LastActiveOf:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructLastActiveOf<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_LastActiveOf.get_or_init(||
                                                                 {
                                                                     let def_val:
                                                                             Option<VoteIndex> =
                                                                         Default::default();
                                                                     <Option<VoteIndex>
                                                                         as
                                                                         Encode>::encode(&def_val)
                                                                 }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructVoters<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Voters:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructVoters<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Voters.get_or_init(||
                                                           {
                                                               let def_val:
                                                                       Vec<T::AccountId> =
                                                                   Default::default();
                                                               <Vec<T::AccountId>
                                                                   as
                                                                   Encode>::encode(&def_val)
                                                           }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructCandidates<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Candidates:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructCandidates<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Candidates.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           Vec<T::AccountId> =
                                                                       Default::default();
                                                                   <Vec<T::AccountId>
                                                                       as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructCandidateCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_CandidateCount:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructCandidateCount<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_CandidateCount.get_or_init(||
                                                                   {
                                                                       let def_val:
                                                                               u32 =
                                                                           Default::default();
                                                                       <u32 as
                                                                           Encode>::encode(&def_val)
                                                                   }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructNextFinalize<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_NextFinalize:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructNextFinalize<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_NextFinalize.get_or_init(||
                                                                 {
                                                                     let def_val:
                                                                             Option<(T::BlockNumber,
                                                                                     u32,
                                                                                     Vec<T::AccountId>)> =
                                                                         Default::default();
                                                                     <Option<(T::BlockNumber,
                                                                              u32,
                                                                              Vec<T::AccountId>)>
                                                                         as
                                                                         Encode>::encode(&def_val)
                                                                 }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructSnapshotedStakes<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_SnapshotedStakes:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructSnapshotedStakes<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_SnapshotedStakes.get_or_init(||
                                                                     {
                                                                         let def_val:
                                                                                 Vec<BalanceOf<T>> =
                                                                             Default::default();
                                                                         <Vec<BalanceOf<T>>
                                                                             as
                                                                             Encode>::encode(&def_val)
                                                                     }).clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructLeaderboard<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Leaderboard:
           self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
           =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
     for __GetByteStructLeaderboard<T> {
        fn default_byte(&self)
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Leaderboard.get_or_init(||
                                                                {
                                                                    let def_val:
                                                                            Option<Vec<(BalanceOf<T>,
                                                                                        T::AccountId)>> =
                                                                        Default::default();
                                                                    <Option<Vec<(BalanceOf<T>,
                                                                                 T::AccountId)>>
                                                                        as
                                                                        Encode>::encode(&def_val)
                                                                }).clone()
        }
    }
    impl <T: Trait> Store for Module<T> {
        type
        CandidacyBond
        =
        CandidacyBond<T>;
        type
        VotingBond
        =
        VotingBond<T>;
        type
        PresentSlashPerVoter
        =
        PresentSlashPerVoter<T>;
        type
        CarryCount
        =
        CarryCount<T>;
        type
        PresentationDuration
        =
        PresentationDuration<T>;
        type
        InactiveGracePeriod
        =
        InactiveGracePeriod<T>;
        type
        VotingPeriod
        =
        VotingPeriod<T>;
        type
        TermDuration
        =
        TermDuration<T>;
        type
        DesiredSeats
        =
        DesiredSeats<T>;
        type
        ActiveCouncil
        =
        ActiveCouncil<T>;
        type
        VoteCount
        =
        VoteCount<T>;
        type
        ApprovalsOf
        =
        ApprovalsOf<T>;
        type
        RegisterInfoOf
        =
        RegisterInfoOf<T>;
        type
        LastActiveOf
        =
        LastActiveOf<T>;
        type
        Voters
        =
        Voters<T>;
        type
        Candidates
        =
        Candidates<T>;
        type
        CandidateCount
        =
        CandidateCount<T>;
        type
        NextFinalize
        =
        NextFinalize<T>;
        type
        SnapshotedStakes
        =
        SnapshotedStakes<T>;
        type
        Leaderboard
        =
        Leaderboard<T>;
    }
    impl <T: 'static + Trait> Module<T> {
        #[doc =
              " How much should be locked up in order to submit one\'s candidacy."]
        pub fn candidacy_bond() -> BalanceOf<T> {
            <CandidacyBond<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " How much should be locked up in order to be able to submit votes."]
        pub fn voting_bond() -> BalanceOf<T> {
            <VotingBond<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " The punishment, per voter, if you provide an invalid presentation."]
        pub fn present_slash_per_voter() -> BalanceOf<T> {
            <PresentSlashPerVoter<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " How many runners-up should have their approvals persist until the next vote."]
        pub fn carry_count() -> u32 {
            <CarryCount<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " How long to give each top candidate to present themselves after the vote ends."]
        pub fn presentation_duration() -> T::BlockNumber {
            <PresentationDuration<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " How many vote indexes need to go by after a target voter\'s last vote before they can be reaped if their"]
        #[doc = " approvals are moot."]
        pub fn inactivity_grace_period() -> VoteIndex {
            <InactiveGracePeriod<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc = " How often (in blocks) to check for new votes."]
        pub fn voting_period() -> T::BlockNumber {
            <VotingPeriod<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc = " How long each position is active for."]
        pub fn term_duration() -> T::BlockNumber {
            <TermDuration<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc = " Number of accounts that should be sitting on the council."]
        pub fn desired_seats() -> u32 {
            <DesiredSeats<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " The current council. When there\'s a vote going on, this should still be used for executive"]
        #[doc =
              " matters. The block number (second element in the tuple) is the block that their position is"]
        #[doc =
              " active until (calculated by the sum of the block number when the council member was elected"]
        #[doc = " and their term duration)."]
        pub fn active_council() -> Vec<(T::AccountId, T::BlockNumber)> {
            <ActiveCouncil<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                         T::BlockNumber)>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " The total number of votes that have happened or are in progress."]
        pub fn vote_index() -> VoteIndex {
            <VoteCount<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " A list of votes for each voter, respecting the last cleared vote index that this voter was"]
        #[doc = " last active at."]
        pub fn approvals_of<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                                  K)
         -> Vec<bool> {
            <ApprovalsOf<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  Vec<bool>>>::get(key.borrow(),
                                                                                                                                   &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " The vote index and list slot that the candidate `who` was registered or `None` if they are not"]
        #[doc = " currently registered."]
        pub fn candidate_reg_info<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                                        K)
         -> Option<(VoteIndex, u32)> {
            <RegisterInfoOf<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  (VoteIndex,
                                                                                                                   u32)>>::get(key.borrow(),
                                                                                                                               &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " The last cleared vote index that this voter was last active at."]
        pub fn voter_last_active<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                                       K)
         -> Option<VoteIndex> {
            <LastActiveOf<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  VoteIndex>>::get(key.borrow(),
                                                                                                                                   &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc = " The present voter list."]
        pub fn voters() -> Vec<T::AccountId> {
            <Voters<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc = " The present candidate list."]
        pub fn candidates() -> Vec<T::AccountId> {
            <Candidates<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn candidate_count() -> u32 {
            <CandidateCount<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc =
              " The accounts holding the seats that will become free on the next tally."]
        pub fn next_finalize()
         -> Option<(T::BlockNumber, u32, Vec<T::AccountId>)> {
            <NextFinalize<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(T::BlockNumber,
                                                                                                                     u32,
                                                                                                                     Vec<T::AccountId>)>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc = " The stakes as they were at the point that the vote ended."]
        pub fn snapshoted_stakes() -> Vec<BalanceOf<T>> {
            <SnapshotedStakes<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<BalanceOf<T>>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc = " Get the leaderboard if we;re in the presentation phase."]
        pub fn leaderboard() -> Option<Vec<(BalanceOf<T>, T::AccountId)>> {
            <Leaderboard<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(BalanceOf<T>,
                                                                                                                         T::AccountId)>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc(hidden)]
        pub fn store_metadata()
         ->
             self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
            self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                      &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CandidacyBond"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCandidacyBond::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How much should be locked up in order to submit one\'s candidacy."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VotingBond"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVotingBond::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How much should be locked up in order to be able to submit votes."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PresentSlashPerVoter"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPresentSlashPerVoter::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The punishment, per voter, if you provide an invalid presentation."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CarryCount"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCarryCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How many runners-up should have their approvals persist until the next vote."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PresentationDuration"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPresentationDuration::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How long to give each top candidate to present themselves after the vote ends."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("InactiveGracePeriod"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VoteIndex")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructInactiveGracePeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How many vote indexes need to go by after a target voter\'s last vote before they can be reaped if their",
                                                                                                                                                                                                                                                                                                                                                                                                        " approvals are moot."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VotingPeriod"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVotingPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How often (in blocks) to check for new votes."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TermDuration"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTermDuration::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How long each position is active for."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("DesiredSeats"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDesiredSeats::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Number of accounts that should be sitting on the council."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ActiveCouncil"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(T::AccountId, T::BlockNumber)>")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructActiveCouncil::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The current council. When there\'s a vote going on, this should still be used for executive",
                                                                                                                                                                                                                                                                                                                                                                                                        " matters. The block number (second element in the tuple) is the block that their position is",
                                                                                                                                                                                                                                                                                                                                                                                                        " active until (calculated by the sum of the block number when the council member was elected",
                                                                                                                                                                                                                                                                                                                                                                                                        " and their term duration)."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VoteCount"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VoteIndex")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVoteCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The total number of votes that have happened or are in progress."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ApprovalsOf"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                       key:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                       value:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<bool>"),
                                                                                                                                                                                                                                                                                                                                                                                                       is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                           false,},
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructApprovalsOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" A list of votes for each voter, respecting the last cleared vote index that this voter was",
                                                                                                                                                                                                                                                                                                                                                                                                        " last active at."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RegisterInfoOf"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                       key:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                       value:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(VoteIndex, u32)"),
                                                                                                                                                                                                                                                                                                                                                                                                       is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                           false,},
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRegisterInfoOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The vote index and list slot that the candidate `who` was registered or `None` if they are not",
                                                                                                                                                                                                                                                                                                                                                                                                        " currently registered."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("LastActiveOf"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                       key:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                       value:
                                                                                                                                                                                                                                                                                                                                                                                                           self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VoteIndex"),
                                                                                                                                                                                                                                                                                                                                                                                                       is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                           false,},
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLastActiveOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The last cleared vote index that this voter was last active at."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Voters"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVoters::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The present voter list."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Candidates"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCandidates::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The present candidate list."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CandidateCount"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCandidateCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextFinalize"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(T::BlockNumber, u32, Vec<T::AccountId>)")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextFinalize::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The accounts holding the seats that will become free on the next tally."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SnapshotedStakes"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<BalanceOf<T>>")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSnapshotedStakes::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The stakes as they were at the point that the vote ended."]),},
                                                                                                                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Leaderboard"),
                                                                                                                                                                                                                                                                                                     modifier:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                     ty:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(BalanceOf<T>, T::AccountId)>")),
                                                                                                                                                                                                                                                                                                     default:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLeaderboard::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                     documentation:
                                                                                                                                                                                                                                                                                                         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Get the leaderboard if we;re in the presentation phase."]),}]
                                                                                                                                                                                                  }),}
        }
        #[doc(hidden)]
        pub fn store_metadata_functions()
         ->
             &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
            {
                &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CandidacyBond"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCandidacyBond::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How much should be locked up in order to submit one\'s candidacy."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VotingBond"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVotingBond::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How much should be locked up in order to be able to submit votes."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PresentSlashPerVoter"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPresentSlashPerVoter::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The punishment, per voter, if you provide an invalid presentation."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CarryCount"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCarryCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How many runners-up should have their approvals persist until the next vote."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PresentationDuration"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPresentationDuration::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How long to give each top candidate to present themselves after the vote ends."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("InactiveGracePeriod"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VoteIndex")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructInactiveGracePeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How many vote indexes need to go by after a target voter\'s last vote before they can be reaped if their",
                                                                                                                                                                                                                  " approvals are moot."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VotingPeriod"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVotingPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How often (in blocks) to check for new votes."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TermDuration"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTermDuration::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How long each position is active for."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("DesiredSeats"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDesiredSeats::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Number of accounts that should be sitting on the council."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ActiveCouncil"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(T::AccountId, T::BlockNumber)>")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructActiveCouncil::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The current council. When there\'s a vote going on, this should still be used for executive",
                                                                                                                                                                                                                  " matters. The block number (second element in the tuple) is the block that their position is",
                                                                                                                                                                                                                  " active until (calculated by the sum of the block number when the council member was elected",
                                                                                                                                                                                                                  " and their term duration)."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VoteCount"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VoteIndex")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVoteCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The total number of votes that have happened or are in progress."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ApprovalsOf"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                 key:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                 value:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<bool>"),
                                                                                                                                                                                                                 is_linked:
                                                                                                                                                                                                                     false,},
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructApprovalsOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" A list of votes for each voter, respecting the last cleared vote index that this voter was",
                                                                                                                                                                                                                  " last active at."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RegisterInfoOf"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                 key:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                 value:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(VoteIndex, u32)"),
                                                                                                                                                                                                                 is_linked:
                                                                                                                                                                                                                     false,},
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRegisterInfoOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The vote index and list slot that the candidate `who` was registered or `None` if they are not",
                                                                                                                                                                                                                  " currently registered."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("LastActiveOf"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                 key:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                 value:
                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VoteIndex"),
                                                                                                                                                                                                                 is_linked:
                                                                                                                                                                                                                     false,},
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLastActiveOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The last cleared vote index that this voter was last active at."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Voters"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVoters::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The present voter list."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Candidates"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCandidates::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The present candidate list."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CandidateCount"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCandidateCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextFinalize"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(T::BlockNumber, u32, Vec<T::AccountId>)")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextFinalize::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The accounts holding the seats that will become free on the next tally."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SnapshotedStakes"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<BalanceOf<T>>")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSnapshotedStakes::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The stakes as they were at the point that the vote ended."]),},
                  self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Leaderboard"),
                                                                                                               modifier:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                               ty:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(BalanceOf<T>, T::AccountId)>")),
                                                                                                               default:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLeaderboard::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                               documentation:
                                                                                                                   self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Get the leaderboard if we;re in the presentation phase."]),}]
            }
        }
        #[doc(hidden)]
        pub fn store_metadata_name() -> &'static str { "Council" }
    }
    #[cfg(feature = "std")]
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[serde(bound(serialize =
                      "BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, VoteIndex : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Vec < ( T :: AccountId , T :: BlockNumber ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
    #[serde(bound(deserialize =
                      "BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, VoteIndex : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Vec < ( T :: AccountId , T :: BlockNumber ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
    pub struct GenesisConfig<T: Trait> {
        #[doc =
              " How much should be locked up in order to submit one\'s candidacy."]
        pub candidacy_bond: BalanceOf<T>,
        #[doc =
              " How much should be locked up in order to be able to submit votes."]
        pub voter_bond: BalanceOf<T>,
        #[doc =
              " The punishment, per voter, if you provide an invalid presentation."]
        pub present_slash_per_voter: BalanceOf<T>,
        #[doc =
              " How many runners-up should have their approvals persist until the next vote."]
        pub carry_count: u32,
        #[doc =
              " How long to give each top candidate to present themselves after the vote ends."]
        pub presentation_duration: T::BlockNumber,
        #[doc =
              " How many vote indexes need to go by after a target voter\'s last vote before they can be reaped if their"]
        #[doc = " approvals are moot."]
        pub inactive_grace_period: VoteIndex,
        #[doc = " How often (in blocks) to check for new votes."]
        pub approval_voting_period: T::BlockNumber,
        #[doc = " How long each position is active for."]
        pub term_duration: T::BlockNumber,
        #[doc = " Number of accounts that should be sitting on the council."]
        pub desired_seats: u32,
        #[doc =
              " The current council. When there\'s a vote going on, this should still be used for executive"]
        #[doc =
              " matters. The block number (second element in the tuple) is the block that their position is"]
        #[doc =
              " active until (calculated by the sum of the block number when the council member was elected"]
        #[doc = " and their term duration)."]
        pub active_council: Vec<(T::AccountId, T::BlockNumber)>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_GenesisConfig: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <T: Trait> _serde::Serialize for GenesisConfig<T> where
             BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             VoteIndex: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
             Vec<(T::AccountId,
                  T::BlockNumber)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
             {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                                                   "GenesisConfig",
                                                                   false as
                                                                       usize +
                                                                       1 + 1 +
                                                                       1 + 1 +
                                                                       1 + 1 +
                                                                       1 + 1 +
                                                                       1 + 1)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "candidacyBond",
                                                                        &self.candidacy_bond)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "voterBond",
                                                                        &self.voter_bond)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "presentSlashPerVoter",
                                                                        &self.present_slash_per_voter)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "carryCount",
                                                                        &self.carry_count)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "presentationDuration",
                                                                        &self.presentation_duration)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "inactiveGracePeriod",
                                                                        &self.inactive_grace_period)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "approvalVotingPeriod",
                                                                        &self.approval_voting_period)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "termDuration",
                                                                        &self.term_duration)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "desiredSeats",
                                                                        &self.desired_seats)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "activeCouncil",
                                                                        &self.active_council)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_GenesisConfig: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <'de, T: Trait> _serde::Deserialize<'de> for GenesisConfig<T>
             where
             BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             VoteIndex: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
             Vec<(T::AccountId,
                  T::BlockNumber)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
             {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                    }
                    struct __FieldVisitor;
                    impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type
                        Value
                        =
                        __Field;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                4u64 => _serde::export::Ok(__Field::__field4),
                                5u64 => _serde::export::Ok(__Field::__field5),
                                6u64 => _serde::export::Ok(__Field::__field6),
                                7u64 => _serde::export::Ok(__Field::__field7),
                                8u64 => _serde::export::Ok(__Field::__field8),
                                9u64 => _serde::export::Ok(__Field::__field9),
                                _ =>
                                _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                     &"field index 0 <= i < 10")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                "candidacyBond" =>
                                _serde::export::Ok(__Field::__field0),
                                "voterBond" =>
                                _serde::export::Ok(__Field::__field1),
                                "presentSlashPerVoter" =>
                                _serde::export::Ok(__Field::__field2),
                                "carryCount" =>
                                _serde::export::Ok(__Field::__field3),
                                "presentationDuration" =>
                                _serde::export::Ok(__Field::__field4),
                                "inactiveGracePeriod" =>
                                _serde::export::Ok(__Field::__field5),
                                "approvalVotingPeriod" =>
                                _serde::export::Ok(__Field::__field6),
                                "termDuration" =>
                                _serde::export::Ok(__Field::__field7),
                                "desiredSeats" =>
                                _serde::export::Ok(__Field::__field8),
                                "activeCouncil" =>
                                _serde::export::Ok(__Field::__field9),
                                _ => {
                                    _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                         FIELDS))
                                }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"candidacyBond" =>
                                _serde::export::Ok(__Field::__field0),
                                b"voterBond" =>
                                _serde::export::Ok(__Field::__field1),
                                b"presentSlashPerVoter" =>
                                _serde::export::Ok(__Field::__field2),
                                b"carryCount" =>
                                _serde::export::Ok(__Field::__field3),
                                b"presentationDuration" =>
                                _serde::export::Ok(__Field::__field4),
                                b"inactiveGracePeriod" =>
                                _serde::export::Ok(__Field::__field5),
                                b"approvalVotingPeriod" =>
                                _serde::export::Ok(__Field::__field6),
                                b"termDuration" =>
                                _serde::export::Ok(__Field::__field7),
                                b"desiredSeats" =>
                                _serde::export::Ok(__Field::__field8),
                                b"activeCouncil" =>
                                _serde::export::Ok(__Field::__field9),
                                _ => {
                                    let __value =
                                        &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                         FIELDS))
                                }
                            }
                        }
                    }
                    impl <'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                         -> _serde::export::Result<Self, __D::Error> where
                         __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                         __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de, T: Trait> where
                           BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           VoteIndex: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                           Vec<(T::AccountId,
                                T::BlockNumber)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                        marker: _serde::export::PhantomData<GenesisConfig<T>>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de, T: Trait> _serde::de::Visitor<'de> for
                     __Visitor<'de, T> where
                     BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     VoteIndex: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                     Vec<(T::AccountId,
                          T::BlockNumber)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
                     {
                        type
                        Value
                        =
                        GenesisConfig<T>;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "struct GenesisConfig")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"struct GenesisConfig with 10 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                     &"struct GenesisConfig with 10 elements"));
                                    }
                                };
                            let __field2 =
                                match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                     &"struct GenesisConfig with 10 elements"));
                                    }
                                };
                            let __field3 =
                                match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                     &"struct GenesisConfig with 10 elements"));
                                    }
                                };
                            let __field4 =
                                match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                     &"struct GenesisConfig with 10 elements"));
                                    }
                                };
                            let __field5 =
                                match match _serde::de::SeqAccess::next_element::<VoteIndex>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(5usize,
                                                                                                     &"struct GenesisConfig with 10 elements"));
                                    }
                                };
                            let __field6 =
                                match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(6usize,
                                                                                                     &"struct GenesisConfig with 10 elements"));
                                    }
                                };
                            let __field7 =
                                match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(7usize,
                                                                                                     &"struct GenesisConfig with 10 elements"));
                                    }
                                };
                            let __field8 =
                                match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(8usize,
                                                                                                     &"struct GenesisConfig with 10 elements"));
                                    }
                                };
                            let __field9 =
                                match match _serde::de::SeqAccess::next_element::<Vec<(T::AccountId,
                                                                                       T::BlockNumber)>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(9usize,
                                                                                                     &"struct GenesisConfig with 10 elements"));
                                    }
                                };
                            _serde::export::Ok(GenesisConfig{candidacy_bond:
                                                                 __field0,
                                                             voter_bond:
                                                                 __field1,
                                                             present_slash_per_voter:
                                                                 __field2,
                                                             carry_count:
                                                                 __field3,
                                                             presentation_duration:
                                                                 __field4,
                                                             inactive_grace_period:
                                                                 __field5,
                                                             approval_voting_period:
                                                                 __field6,
                                                             term_duration:
                                                                 __field7,
                                                             desired_seats:
                                                                 __field8,
                                                             active_council:
                                                                 __field9,})
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::MapAccess<'de> {
                            let mut __field0:
                                    _serde::export::Option<BalanceOf<T>> =
                                _serde::export::None;
                            let mut __field1:
                                    _serde::export::Option<BalanceOf<T>> =
                                _serde::export::None;
                            let mut __field2:
                                    _serde::export::Option<BalanceOf<T>> =
                                _serde::export::None;
                            let mut __field3: _serde::export::Option<u32> =
                                _serde::export::None;
                            let mut __field4:
                                    _serde::export::Option<T::BlockNumber> =
                                _serde::export::None;
                            let mut __field5:
                                    _serde::export::Option<VoteIndex> =
                                _serde::export::None;
                            let mut __field6:
                                    _serde::export::Option<T::BlockNumber> =
                                _serde::export::None;
                            let mut __field7:
                                    _serde::export::Option<T::BlockNumber> =
                                _serde::export::None;
                            let mut __field8: _serde::export::Option<u32> =
                                _serde::export::None;
                            let mut __field9:
                                    _serde::export::Option<Vec<(T::AccountId,
                                                                T::BlockNumber)>> =
                                _serde::export::None;
                            while let _serde::export::Some(__key) =
                                      match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("candidacyBond"));
                                        }
                                        __field0 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<BalanceOf<T>>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("voterBond"));
                                        }
                                        __field1 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<BalanceOf<T>>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field2 => {
                                        if _serde::export::Option::is_some(&__field2)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("presentSlashPerVoter"));
                                        }
                                        __field2 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<BalanceOf<T>>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field3 => {
                                        if _serde::export::Option::is_some(&__field3)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("carryCount"));
                                        }
                                        __field3 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<u32>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field4 => {
                                        if _serde::export::Option::is_some(&__field4)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("presentationDuration"));
                                        }
                                        __field4 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<T::BlockNumber>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field5 => {
                                        if _serde::export::Option::is_some(&__field5)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("inactiveGracePeriod"));
                                        }
                                        __field5 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<VoteIndex>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field6 => {
                                        if _serde::export::Option::is_some(&__field6)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("approvalVotingPeriod"));
                                        }
                                        __field6 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<T::BlockNumber>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field7 => {
                                        if _serde::export::Option::is_some(&__field7)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("termDuration"));
                                        }
                                        __field7 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<T::BlockNumber>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field8 => {
                                        if _serde::export::Option::is_some(&__field8)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("desiredSeats"));
                                        }
                                        __field8 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<u32>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    __Field::__field9 => {
                                        if _serde::export::Option::is_some(&__field9)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("activeCouncil"));
                                        }
                                        __field9 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<(T::AccountId,
                                                                                                                T::BlockNumber)>>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::export::Some(__field0) =>
                                    __field0,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("candidacyBond")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::export::Some(__field1) =>
                                    __field1,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("voterBond")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field2 =
                                match __field2 {
                                    _serde::export::Some(__field2) =>
                                    __field2,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("presentSlashPerVoter")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field3 =
                                match __field3 {
                                    _serde::export::Some(__field3) =>
                                    __field3,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("carryCount")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field4 =
                                match __field4 {
                                    _serde::export::Some(__field4) =>
                                    __field4,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("presentationDuration")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field5 =
                                match __field5 {
                                    _serde::export::Some(__field5) =>
                                    __field5,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("inactiveGracePeriod")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field6 =
                                match __field6 {
                                    _serde::export::Some(__field6) =>
                                    __field6,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("approvalVotingPeriod")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field7 =
                                match __field7 {
                                    _serde::export::Some(__field7) =>
                                    __field7,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("termDuration")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field8 =
                                match __field8 {
                                    _serde::export::Some(__field8) =>
                                    __field8,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("desiredSeats")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field9 =
                                match __field9 {
                                    _serde::export::Some(__field9) =>
                                    __field9,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("activeCouncil")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            _serde::export::Ok(GenesisConfig{candidacy_bond:
                                                                 __field0,
                                                             voter_bond:
                                                                 __field1,
                                                             present_slash_per_voter:
                                                                 __field2,
                                                             carry_count:
                                                                 __field3,
                                                             presentation_duration:
                                                                 __field4,
                                                             inactive_grace_period:
                                                                 __field5,
                                                             approval_voting_period:
                                                                 __field6,
                                                             term_duration:
                                                                 __field7,
                                                             desired_seats:
                                                                 __field8,
                                                             active_council:
                                                                 __field9,})
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["candidacyBond", "voterBond",
                          "presentSlashPerVoter", "carryCount",
                          "presentationDuration", "inactiveGracePeriod",
                          "approvalVotingPeriod", "termDuration",
                          "desiredSeats", "activeCouncil"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                                                             "GenesisConfig",
                                                             FIELDS,
                                                             __Visitor{marker:
                                                                           _serde::export::PhantomData::<GenesisConfig<T>>,
                                                                       lifetime:
                                                                           _serde::export::PhantomData,})
                }
            }
        };
    #[cfg(feature = "std")]
    impl <T: Trait> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig{candidacy_bond: BalanceOf::<T>::sa(9),
                          voter_bond: Default::default(),
                          present_slash_per_voter: BalanceOf::<T>::sa(1),
                          carry_count: 2,
                          presentation_duration: T::BlockNumber::sa(1000),
                          inactive_grace_period: 1,
                          approval_voting_period: T::BlockNumber::sa(1000),
                          term_duration: T::BlockNumber::sa(5),
                          desired_seats: Default::default(),
                          active_council: Default::default(),}
        }
    }
    #[cfg(feature = "std")]
    impl <T: Trait>
     self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::BuildStorage
     for GenesisConfig<T> {
        fn assimilate_storage(self,
                              r:
                                  &mut self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::StorageOverlay,
                              c:
                                  &mut self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::ChildrenStorageOverlay)
         -> ::std::result::Result<(), String> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::cell::RefCell;
            let storage = RefCell::new(r);
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.candidacy_bond.clone()))(&self);
                <CandidacyBond<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::put(&v,
                                                                                                                                            &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.voter_bond.clone()))(&self);
                <VotingBond<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::put(&v,
                                                                                                                                            &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.present_slash_per_voter.clone()))(&self);
                <PresentSlashPerVoter<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::put(&v,
                                                                                                                                            &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.carry_count.clone()))(&self);
                <CarryCount<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&v,
                                                                                                                                   &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.presentation_duration.clone()))(&self);
                <PresentationDuration<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                              &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.inactive_grace_period.clone()))(&self);
                <InactiveGracePeriod<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<VoteIndex>>::put(&v,
                                                                                                                                         &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.approval_voting_period.clone()))(&self);
                <VotingPeriod<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                              &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.term_duration.clone()))(&self);
                <TermDuration<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                              &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.desired_seats.clone()))(&self);
                <DesiredSeats<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&v,
                                                                                                                                   &storage);
            }
            {
                use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                      marker::PhantomData};
                use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                       Decode};
                let v =
                    ((|config: &GenesisConfig<T>|
                          config.active_council.clone()))(&self);
                <ActiveCouncil<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                             T::BlockNumber)>>>::put(&v,
                                                                                                                                                     &storage);
            }
            let r = storage.into_inner();
            (|_, _, _| { })(r, c, &self);
            Ok(())
        }
    }
    /// [`RawEvent`] specialized for the configuration [`Trait`]
    ///
    /// [`RawEvent`]: enum.RawEvent.html
    /// [`Trait`]: trait.Trait.html
    pub type Event<T> = RawEvent<<T as system::Trait>::AccountId>;
    /// Events for this module.
    ///
    #[structural_match]
    pub enum RawEvent<AccountId> {

        #[doc = r" reaped voter, reaper"]
        VoterReaped(AccountId, AccountId),

        #[doc = r" slashed reaper"]
        BadReaperSlashed(AccountId),

        #[doc =
              r" A tally (for approval votes of council seat(s)) has started."]
        TallyStarted(u32),

        #[doc =
              r" A tally (for approval votes of council seat(s)) has ended (with one or more new members)."]
        TallyFinalized(Vec<AccountId>, Vec<AccountId>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::clone::Clone> ::std::clone::Clone for
     RawEvent<AccountId> {
        #[inline]
        fn clone(&self) -> RawEvent<AccountId> {
            match (&*self,) {
                (&RawEvent::VoterReaped(ref __self_0, ref __self_1),) =>
                RawEvent::VoterReaped(::std::clone::Clone::clone(&(*__self_0)),
                                      ::std::clone::Clone::clone(&(*__self_1))),
                (&RawEvent::BadReaperSlashed(ref __self_0),) =>
                RawEvent::BadReaperSlashed(::std::clone::Clone::clone(&(*__self_0))),
                (&RawEvent::TallyStarted(ref __self_0),) =>
                RawEvent::TallyStarted(::std::clone::Clone::clone(&(*__self_0))),
                (&RawEvent::TallyFinalized(ref __self_0, ref __self_1),) =>
                RawEvent::TallyFinalized(::std::clone::Clone::clone(&(*__self_0)),
                                         ::std::clone::Clone::clone(&(*__self_1))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
     RawEvent<AccountId> {
        #[inline]
        fn eq(&self, other: &RawEvent<AccountId>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&RawEvent::VoterReaped(ref __self_0, ref __self_1),
                         &RawEvent::VoterReaped(ref __arg_1_0, ref __arg_1_1))
                        =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1),
                        (&RawEvent::BadReaperSlashed(ref __self_0),
                         &RawEvent::BadReaperSlashed(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&RawEvent::TallyStarted(ref __self_0),
                         &RawEvent::TallyStarted(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&RawEvent::TallyFinalized(ref __self_0,
                                                   ref __self_1),
                         &RawEvent::TallyFinalized(ref __arg_1_0,
                                                   ref __arg_1_1)) =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &RawEvent<AccountId>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&RawEvent::VoterReaped(ref __self_0, ref __self_1),
                         &RawEvent::VoterReaped(ref __arg_1_0, ref __arg_1_1))
                        =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1),
                        (&RawEvent::BadReaperSlashed(ref __self_0),
                         &RawEvent::BadReaperSlashed(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&RawEvent::TallyStarted(ref __self_0),
                         &RawEvent::TallyStarted(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&RawEvent::TallyFinalized(ref __self_0,
                                                   ref __self_1),
                         &RawEvent::TallyFinalized(ref __arg_1_0,
                                                   ref __arg_1_1)) =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::cmp::Eq> ::std::cmp::Eq for RawEvent<AccountId> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<Vec<AccountId>>;
                let _: ::std::cmp::AssertParamIsEq<Vec<AccountId>>;
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_RawEvent: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <AccountId> _parity_codec::Encode for RawEvent<AccountId>
             where AccountId: _parity_codec::Encode,
             AccountId: _parity_codec::Encode,
             AccountId: _parity_codec::Encode,
             AccountId: _parity_codec::Encode,
             AccountId: _parity_codec::Encode,
             AccountId: _parity_codec::Encode,
             Vec<AccountId>: _parity_codec::Encode,
             Vec<AccountId>: _parity_codec::Encode,
             Vec<AccountId>: _parity_codec::Encode,
             Vec<AccountId>: _parity_codec::Encode {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        RawEvent::VoterReaped(ref aa, ref ba) => {
                            dest.push_byte(0usize as u8);
                            dest.push(aa);
                            dest.push(ba);
                        }
                        RawEvent::BadReaperSlashed(ref aa) => {
                            dest.push_byte(1usize as u8);
                            dest.push(aa);
                        }
                        RawEvent::TallyStarted(ref aa) => {
                            dest.push_byte(2usize as u8);
                            dest.push(aa);
                        }
                        RawEvent::TallyFinalized(ref aa, ref ba) => {
                            dest.push_byte(3usize as u8);
                            dest.push(aa);
                            dest.push(ba);
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_RawEvent: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <AccountId> _parity_codec::Decode for RawEvent<AccountId>
             where AccountId: _parity_codec::Decode,
             AccountId: _parity_codec::Decode,
             AccountId: _parity_codec::Decode,
             AccountId: _parity_codec::Decode,
             AccountId: _parity_codec::Decode,
             AccountId: _parity_codec::Decode,
             Vec<AccountId>: _parity_codec::Decode,
             Vec<AccountId>: _parity_codec::Decode,
             Vec<AccountId>: _parity_codec::Decode,
             Vec<AccountId>: _parity_codec::Decode {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 0usize as u8 => {
                            Some(RawEvent::VoterReaped(_parity_codec::Decode::decode(input)?,
                                                       _parity_codec::Decode::decode(input)?))
                        }
                        x if x == 1usize as u8 => {
                            Some(RawEvent::BadReaperSlashed(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 2usize as u8 => {
                            Some(RawEvent::TallyStarted(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 3usize as u8 => {
                            Some(RawEvent::TallyFinalized(_parity_codec::Decode::decode(input)?,
                                                          _parity_codec::Decode::decode(input)?))
                        }
                        _ => None,
                    }
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::fmt::Debug> ::std::fmt::Debug for
     RawEvent<AccountId> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&RawEvent::VoterReaped(ref __self_0, ref __self_1),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("VoterReaped");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&RawEvent::BadReaperSlashed(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("BadReaperSlashed");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&RawEvent::TallyStarted(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("TallyStarted");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&RawEvent::TallyFinalized(ref __self_0, ref __self_1),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("TallyFinalized");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl <AccountId> From<RawEvent<AccountId>> for () {
        fn from(_: RawEvent<AccountId>) -> () { () }
    }
    impl <AccountId> RawEvent<AccountId> {
        #[allow(dead_code)]
        pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
            &[::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("VoterReaped"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                        "AccountId"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" reaped voter, reaper"]),},
              ::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("BadReaperSlashed"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["AccountId"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" slashed reaper"]),},
              ::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("TallyStarted"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["u32"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" A tally (for approval votes of council seat(s)) has started."]),},
              ::srml_support::event::EventMetadata{name:
                                                       ::srml_support::event::DecodeDifferent::Encode("TallyFinalized"),
                                                   arguments:
                                                       ::srml_support::event::DecodeDifferent::Encode(&["Vec<AccountId>",
                                                                                                        "Vec<AccountId>"]),
                                                   documentation:
                                                       ::srml_support::event::DecodeDifferent::Encode(&[r" A tally (for approval votes of council seat(s)) has ended (with one or more new members)."]),}]
        }
    }
    impl <T: Trait> Module<T> {
        /// True if we're currently in a presentation period.
        pub fn presentation_active() -> bool { <NextFinalize<T>>::exists() }
        /// If `who` a candidate at the moment?
        pub fn is_a_candidate(who: &T::AccountId) -> bool {
            <RegisterInfoOf<T>>::exists(who)
        }
        /// Determine the block that a vote can happen on which is no less than `n`.
        pub fn next_vote_from(n: T::BlockNumber) -> T::BlockNumber {
            let voting_period = Self::voting_period();
            (n + voting_period - One::one()) / voting_period * voting_period
        }
        /// The block number on which the tally for the next election will happen. `None` only if the
        /// desired seats of the council is zero.
        pub fn next_tally() -> Option<T::BlockNumber> {
            let desired_seats = Self::desired_seats();
            if desired_seats == 0 {
                None
            } else {
                let c = Self::active_council();
                let (next_possible, count, coming) =
                    if let Some((tally_end, comers, leavers)) =
                           Self::next_finalize() {
                        (tally_end, c.len() - leavers.len() + comers as usize,
                         comers)
                    } else {
                        (<system::Module<T>>::block_number(), c.len(), 0)
                    };
                if count < desired_seats as usize {
                    Some(next_possible)
                } else {
                    if desired_seats <= coming {
                        Some(next_possible + Self::term_duration())
                    } else {
                        Some(c[c.len() - (desired_seats - coming) as usize].1)
                    }
                }.map(Self::next_vote_from)
            }
        }
        /// Check there's nothing to do this block
        fn end_block(block_number: T::BlockNumber) -> Result {
            if (block_number % Self::voting_period()).is_zero() {
                if let Some(number) = Self::next_tally() {
                    if block_number == number { Self::start_tally(); }
                }
            }
            if let Some((number, _, _)) = Self::next_finalize() {
                if block_number == number { Self::finalize_tally()? }
            }
            Ok(())
        }
        /// Remove a voter from the system. Trusts that Self::voters()[index] != voter.
        fn remove_voter(voter: &T::AccountId, index: usize,
                        mut voters: Vec<T::AccountId>) {
            <Voters<T>>::put({ voters.swap_remove(index); voters });
            <ApprovalsOf<T>>::remove(voter);
            <LastActiveOf<T>>::remove(voter);
        }
        fn do_set_approvals(who: T::AccountId, votes: Vec<bool>,
                            index: VoteIndex) -> Result {
            let candidates = Self::candidates();
            {
                if !!Self::presentation_active() {
                    {
                        return Err("no approval changes during presentation period");
                    };
                }
            };
            {
                if !(index == Self::vote_index()) {
                    { return Err("incorrect vote index"); };
                }
            };
            {
                if !!candidates.is_empty() {
                    {
                        return Err("amount of candidates to receive approval votes should be non-zero");
                    };
                }
            };
            {
                if !(candidates.len() >= votes.len()) {
                    {
                        return Err("amount of candidate approval votes cannot exceed amount of candidates");
                    };
                }
            };
            if !<LastActiveOf<T>>::exists(&who) {
                T::Currency::reserve(&who, Self::voting_bond())?;
                <Voters<T>>::mutate(|v| v.push(who.clone()));
            }
            <LastActiveOf<T>>::insert(&who, index);
            <ApprovalsOf<T>>::insert(&who, votes);
            Ok(())
        }
        /// Close the voting, snapshot the staking and the number of seats that are actually up for grabs.
        fn start_tally() {
            let active_council = Self::active_council();
            let desired_seats = Self::desired_seats() as usize;
            let number = <system::Module<T>>::block_number();
            let expiring =
                active_council.iter().take_while(|i|
                                                     i.1 ==
                                                         number).map(|i|
                                                                         i.0.clone()).collect::<Vec<_>>();
            let retaining_seats = active_council.len() - expiring.len();
            if retaining_seats < desired_seats {
                let empty_seats = desired_seats - retaining_seats;
                <NextFinalize<T>>::put((number +
                                            Self::presentation_duration(),
                                        empty_seats as u32, expiring));
                let voters = Self::voters();
                let votes =
                    voters.iter().map(T::Currency::total_balance).collect::<Vec<_>>();
                <SnapshotedStakes<T>>::put(votes);
                let leaderboard_size =
                    empty_seats + Self::carry_count() as usize;
                <Leaderboard<T>>::put(::alloc::vec::from_elem((BalanceOf::<T>::zero(),
                                                               T::AccountId::default()),
                                                              leaderboard_size));
                Self::deposit_event(RawEvent::TallyStarted(empty_seats as
                                                               u32));
            }
        }
        /// Finalize the vote, removing each of the `removals` and inserting `seats` of the most approved
        /// candidates in their place. If the total council members is less than the desired membership
        /// a new vote is started.
        /// Clears all presented candidates, returning the bond of the elected ones.
        fn finalize_tally() -> Result {
            <SnapshotedStakes<T>>::kill();
            let (_, coming, expiring):
                    (T::BlockNumber, u32, Vec<T::AccountId>) =
                <NextFinalize<T>>::take().ok_or("finalize can only be called after a tally is started.")?;
            let leaderboard: Vec<(BalanceOf<T>, T::AccountId)> =
                <Leaderboard<T>>::take().unwrap_or_default();
            let new_expiry =
                <system::Module<T>>::block_number() + Self::term_duration();
            let candidacy_bond = Self::candidacy_bond();
            let incoming: Vec<T::AccountId> =
                leaderboard.iter().rev().take_while(|&&(b, _)|
                                                        !b.is_zero()).take(coming
                                                                               as
                                                                               usize).map(|(_,
                                                                                            a)|
                                                                                              a).cloned().inspect(|a|
                                                                                                                      {
                                                                                                                          T::Currency::unreserve(a,
                                                                                                                                                 candidacy_bond);
                                                                                                                      }).collect();
            let active_council = Self::active_council();
            let outgoing =
                active_council.iter().take(expiring.len()).map(|a|
                                                                   a.0.clone()).collect();
            let mut new_council: Vec<_> =
                active_council.into_iter().skip(expiring.len()).chain(incoming.iter().cloned().map(|a|
                                                                                                       (a,
                                                                                                        new_expiry))).collect();
            new_council.sort_by_key(|&(_, expiry)| expiry);
            <ActiveCouncil<T>>::put(new_council);
            let candidates = Self::candidates();
            let mut new_candidates =
                ::alloc::vec::from_elem(T::AccountId::default(),
                                        candidates.len());
            let runners_up =
                leaderboard.into_iter().rev().take_while(|&(b, _)|
                                                             !b.is_zero()).skip(coming
                                                                                    as
                                                                                    usize).filter_map(|(_,
                                                                                                        a)|
                                                                                                          Self::candidate_reg_info(&a).map(|i|
                                                                                                                                               (a,
                                                                                                                                                i.1)));
            let mut count = 0u32;
            for (address, slot) in runners_up {
                new_candidates[slot as usize] = address;
                count += 1;
            }
            for (old, new) in candidates.iter().zip(new_candidates.iter()) {
                if old != new { <RegisterInfoOf<T>>::remove(old); }
            }
            if let Some(last_index) =
                   new_candidates.iter().rposition(|c|
                                                       *c !=
                                                           T::AccountId::default())
                   {
                new_candidates.truncate(last_index + 1);
            }
            Self::deposit_event(RawEvent::TallyFinalized(incoming, outgoing));
            <Candidates<T>>::put(new_candidates);
            <CandidateCount<T>>::put(count);
            <VoteCount<T>>::put(Self::vote_index() + 1);
            Ok(())
        }
    }
}
pub use crate::seats::{Trait, Module, RawEvent, Event, VoteIndex};
