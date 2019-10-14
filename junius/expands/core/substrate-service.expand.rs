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

//! Substrate service. Starts a thread that spins up the network, client, and extrinsic pool.
//! Manages communication between them.

#![warn(missing_docs)]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

mod components {












    // Create client


    // This is meant to be for testing only
    // FIXME #1063 remove this
    // Keep the public key for telemetry









    // block notifications






    // finality notifications

    // A utility stream that drops all ready items and only returns the last one.
    // This is used to only keep the last finality notification and avoid
    // overloading the sync module with notifications.






    // extrinsic notifications



    // RPC


    // Telemetry

































    //! Substrate service components.
    use std::{sync::Arc, net::SocketAddr, ops::Deref, ops::DerefMut};
    use serde::{Serialize, de::DeserializeOwned};
    use tokio::runtime::TaskExecutor;
    use crate::chain_spec::ChainSpec;
    use client_db;
    use client::{self, Client, runtime_api};
    use crate::{error, Service, maybe_start_server};
    use consensus_common::{import_queue::ImportQueue, SelectChain};
    use network::{self, OnDemand, FinalityProofProvider};
    use substrate_executor::{NativeExecutor, NativeExecutionDispatch};
    use transaction_pool::txpool::{self, Options as TransactionPoolOptions,
                                   Pool as TransactionPool};
    use runtime_primitives::{BuildStorage,
                             traits::{Block as BlockT, Header as HeaderT,
                                      ProvideRuntimeApi}, generic::BlockId};
    use crate::config::Configuration;
    use primitives::{Blake2Hasher, H256};
    use rpc::{self, apis::system::SystemInfo};
    use parking_lot::Mutex;
    /// Network service type for a factory.
    pub type NetworkService<F>
        =
        network::Service<<F as ServiceFactory>::Block,
                         <F as ServiceFactory>::NetworkProtocol>;
    /// Code executor type for a factory.
    pub type CodeExecutor<F>
        =
        NativeExecutor<<F as ServiceFactory>::RuntimeDispatch>;
    /// Full client backend type for a factory.
    pub type FullBackend<F>
        =
        client_db::Backend<<F as ServiceFactory>::Block>;
    /// Full client executor type for a factory.
    pub type FullExecutor<F>
        =
        client::LocalCallExecutor<client_db::Backend<<F as
                                                     ServiceFactory>::Block>,
                                  CodeExecutor<F>>;
    /// Light client backend type for a factory.
    pub type LightBackend<F>
        =
        client::light::backend::Backend<client_db::light::LightStorage<<F as
                                                                       ServiceFactory>::Block>,
                                        network::OnDemand<<F as
                                                          ServiceFactory>::Block>,
                                        Blake2Hasher>;
    /// Light client executor type for a factory.
    pub type LightExecutor<F>
        =
        client::light::call_executor::RemoteOrLocalCallExecutor<<F as
                                                                ServiceFactory>::Block,
                                                                client::light::backend::Backend<client_db::light::LightStorage<<F
                                                                                                                               as
                                                                                                                               ServiceFactory>::Block>,
                                                                                                network::OnDemand<<F
                                                                                                                  as
                                                                                                                  ServiceFactory>::Block>,
                                                                                                Blake2Hasher>,
                                                                client::light::call_executor::RemoteCallExecutor<client::light::blockchain::Blockchain<client_db::light::LightStorage<<F
                                                                                                                                                                                      as
                                                                                                                                                                                      ServiceFactory>::Block>,
                                                                                                                                                       network::OnDemand<<F
                                                                                                                                                                         as
                                                                                                                                                                         ServiceFactory>::Block>>,
                                                                                                                 network::OnDemand<<F
                                                                                                                                   as
                                                                                                                                   ServiceFactory>::Block>>,
                                                                client::LocalCallExecutor<client::light::backend::Backend<client_db::light::LightStorage<<F
                                                                                                                                                         as
                                                                                                                                                         ServiceFactory>::Block>,
                                                                                                                          network::OnDemand<<F
                                                                                                                                            as
                                                                                                                                            ServiceFactory>::Block>,
                                                                                                                          Blake2Hasher>,
                                                                                          CodeExecutor<F>>>;
    /// Full client type for a factory.
    pub type FullClient<F>
        =
        Client<FullBackend<F>, FullExecutor<F>, <F as ServiceFactory>::Block,
               <F as ServiceFactory>::RuntimeApi>;
    /// Light client type for a factory.
    pub type LightClient<F>
        =
        Client<LightBackend<F>, LightExecutor<F>,
               <F as ServiceFactory>::Block,
               <F as ServiceFactory>::RuntimeApi>;
    /// `ChainSpec` specialization for a factory.
    pub type FactoryChainSpec<F> = ChainSpec<<F as ServiceFactory>::Genesis>;
    /// `Genesis` specialization for a factory.
    pub type FactoryGenesis<F> = <F as ServiceFactory>::Genesis;
    /// `Block` type for a factory.
    pub type FactoryBlock<F> = <F as ServiceFactory>::Block;
    /// `Extrinsic` type for a factory.
    pub type FactoryExtrinsic<F>
        =
        <<F as ServiceFactory>::Block as BlockT>::Extrinsic;
    /// `Number` type for a factory.
    pub type FactoryBlockNumber<F>
        =
        <<FactoryBlock<F> as BlockT>::Header as HeaderT>::Number;
    /// Full `Configuration` type for a factory.
    pub type FactoryFullConfiguration<F>
        =
        Configuration<<F as ServiceFactory>::Configuration,
                      FactoryGenesis<F>>;
    /// Client type for `Components`.
    pub type ComponentClient<C>
        =
        Client<<C as Components>::Backend, <C as Components>::Executor,
               FactoryBlock<<C as Components>::Factory>,
               <C as Components>::RuntimeApi>;
    /// Block type for `Components`
    pub type ComponentBlock<C>
        =
        <<C as Components>::Factory as ServiceFactory>::Block;
    /// Extrinsic hash type for `Components`
    pub type ComponentExHash<C>
        =
        <<C as Components>::TransactionPoolApi as txpool::ChainApi>::Hash;
    /// Extrinsic type.
    pub type ComponentExtrinsic<C> = <ComponentBlock<C> as BlockT>::Extrinsic;
    /// Extrinsic pool API type for `Components`.
    pub type PoolApi<C> = <C as Components>::TransactionPoolApi;
    /// A set of traits for the runtime genesis config.
    pub trait RuntimeGenesis: Serialize + DeserializeOwned + BuildStorage { }
    impl <T: Serialize + DeserializeOwned + BuildStorage> RuntimeGenesis for T
     {
    }
    /// Something that can start the RPC service.
    pub trait StartRPC<C: Components> {
        type
        ServersHandle: Send +
        Sync;
        fn start_rpc(client: Arc<ComponentClient<C>>,
                     network: Arc<network::SyncProvider<ComponentBlock<C>>>,
                     should_have_peers: bool, system_info: SystemInfo,
                     rpc_http: Option<SocketAddr>, rpc_ws: Option<SocketAddr>,
                     rpc_cors: Option<Vec<String>>,
                     task_executor: TaskExecutor,
                     transaction_pool:
                         Arc<TransactionPool<C::TransactionPoolApi>>)
        -> error::Result<Self::ServersHandle>;
    }
    impl <C: Components> StartRPC<Self> for C where
     ComponentClient<C>: ProvideRuntimeApi,
     <ComponentClient<C> as
     ProvideRuntimeApi>::Api: runtime_api::Metadata<ComponentBlock<C>> {
        type
        ServersHandle
        =
        (Option<rpc::HttpServer>, Option<Mutex<rpc::WsServer>>);
        fn start_rpc(client: Arc<ComponentClient<C>>,
                     network: Arc<network::SyncProvider<ComponentBlock<C>>>,
                     should_have_peers: bool, rpc_system_info: SystemInfo,
                     rpc_http: Option<SocketAddr>, rpc_ws: Option<SocketAddr>,
                     rpc_cors: Option<Vec<String>>,
                     task_executor: TaskExecutor,
                     transaction_pool:
                         Arc<TransactionPool<C::TransactionPoolApi>>)
         -> error::Result<Self::ServersHandle> {
            let handler =
                ||
                    {
                        let client = client.clone();
                        let subscriptions =
                            rpc::apis::Subscriptions::new(task_executor.clone());
                        let chain =
                            rpc::apis::chain::Chain::new(client.clone(),
                                                         subscriptions.clone());
                        let state =
                            rpc::apis::state::State::new(client.clone(),
                                                         subscriptions.clone());
                        let author =
                            rpc::apis::author::Author::new(client.clone(),
                                                           transaction_pool.clone(),
                                                           subscriptions);
                        let system =
                            rpc::apis::system::System::new(rpc_system_info.clone(),
                                                           network.clone(),
                                                           should_have_peers);
                        rpc::rpc_handler::<ComponentBlock<C>,
                                           ComponentExHash<C>, _, _, _,
                                           _>(state, chain, author, system)
                    };
            Ok((maybe_start_server(rpc_http,
                                   |address|
                                       rpc::start_http(address,
                                                       rpc_cors.as_ref(),
                                                       handler()))?,
                maybe_start_server(rpc_ws,
                                   |address|
                                       rpc::start_ws(address,
                                                     rpc_cors.as_ref(),
                                                     handler()))?.map(Mutex::new)))
        }
    }
    /// Something that can maintain transaction pool on every imported block.
    pub trait MaintainTransactionPool<C: Components> {
        fn maintain_transaction_pool(id: &BlockId<ComponentBlock<C>>,
                                     client: &ComponentClient<C>,
                                     transaction_pool:
                                         &TransactionPool<C::TransactionPoolApi>)
        -> error::Result<()>;
    }
    fn maintain_transaction_pool<Api, Backend, Block, Executor,
                                 PoolApi>(id: &BlockId<Block>,
                                          client:
                                              &Client<Backend, Executor,
                                                      Block, Api>,
                                          transaction_pool:
                                              &TransactionPool<PoolApi>)
     -> error::Result<()> where Block: BlockT<Hash =
     <Blake2Hasher as ::primitives::Hasher>::Out>,
     Backend: client::backend::Backend<Block, Blake2Hasher>,
     Client<Backend, Executor, Block, Api>: ProvideRuntimeApi,
     <Client<Backend, Executor, Block, Api> as
     ProvideRuntimeApi>::Api: runtime_api::TaggedTransactionQueue<Block>,
     Executor: client::CallExecutor<Block, Blake2Hasher>,
     PoolApi: txpool::ChainApi<Hash = Block::Hash, Block = Block> {
        if transaction_pool.status().is_empty() { return Ok(()) }
        if let Some(block) = client.block(id)? {
            let parent_id =
                BlockId::hash(*block.block.header().parent_hash());
            let extrinsics = block.block.extrinsics();
            transaction_pool.prune(id, &parent_id,
                                   extrinsics).map_err(|e|
                                                           ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                              &match (&e,)
                                                                                                                   {
                                                                                                                   (arg0,)
                                                                                                                   =>
                                                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                ::std::fmt::Debug::fmt)],
                                                                                                               })))?;
        }
        Ok(())
    }
    impl <C: Components> MaintainTransactionPool<Self> for C where
     ComponentClient<C>: ProvideRuntimeApi,
     <ComponentClient<C> as
     ProvideRuntimeApi>::Api: runtime_api::TaggedTransactionQueue<ComponentBlock<C>>
     {
        fn maintain_transaction_pool(id: &BlockId<ComponentBlock<C>>,
                                     client: &ComponentClient<C>,
                                     transaction_pool:
                                         &TransactionPool<C::TransactionPoolApi>)
         -> error::Result<()> {
            maintain_transaction_pool(id, client, transaction_pool)
        }
    }
    pub trait OffchainWorker<C: Components> {
        fn offchain_workers(number: &FactoryBlockNumber<C::Factory>,
                            offchain:
                                &offchain::OffchainWorkers<ComponentClient<C>,
                                                           ComponentBlock<C>>,
                            pool:
                                &Arc<TransactionPool<C::TransactionPoolApi>>)
        -> error::Result<()>;
    }
    impl <C: Components> OffchainWorker<Self> for C where
     ComponentClient<C>: ProvideRuntimeApi,
     <ComponentClient<C> as
     ProvideRuntimeApi>::Api: offchain::OffchainWorkerApi<ComponentBlock<C>> {
        fn offchain_workers(number: &FactoryBlockNumber<C::Factory>,
                            offchain:
                                &offchain::OffchainWorkers<ComponentClient<C>,
                                                           ComponentBlock<C>>,
                            pool:
                                &Arc<TransactionPool<C::TransactionPoolApi>>)
         -> error::Result<()> {
            Ok(offchain.on_block_imported(number, pool))
        }
    }
    /// The super trait that combines all required traits a `Service` needs to implement.
    pub trait ServiceTrait<C: Components>: Deref<Target = Service<C>> + Send +
     Sync + 'static + StartRPC<C> + MaintainTransactionPool<C> +
     OffchainWorker<C> {
    }
    impl <C: Components, T> ServiceTrait<C> for T where T: Deref<Target =
     Service<C>> + Send + Sync + 'static + StartRPC<C> +
     MaintainTransactionPool<C> + OffchainWorker<C> {
    }
    /// A collection of types and methods to build a service on top of the substrate service.
    pub trait ServiceFactory: 'static + Sized {
        /// Block type.
        type
        Block: BlockT<Hash
        =
        H256>;
        /// The type that implements the runtime API.
        type
        RuntimeApi: Send +
        Sync;
        /// Network protocol extensions.
        type
        NetworkProtocol: network::specialization::NetworkSpecialization<Self::Block>;
        /// Chain runtime.
        type
        RuntimeDispatch: NativeExecutionDispatch +
        Send +
        Sync +
        'static;
        /// Extrinsic pool backend type for the full client.
        type
        FullTransactionPoolApi: txpool::ChainApi<Hash
        =
        <Self::Block as BlockT>::Hash,
        Block
        =
        Self::Block> +
        Send +
        'static;
        /// Extrinsic pool backend type for the light client.
        type
        LightTransactionPoolApi: txpool::ChainApi<Hash
        =
        <Self::Block as BlockT>::Hash,
        Block
        =
        Self::Block> +
        'static;
        /// Genesis configuration for the runtime.
        type
        Genesis: RuntimeGenesis;
        /// Other configuration for service members.
        type
        Configuration: Default;
        /// Extended full service type.
        type
        FullService: ServiceTrait<FullComponents<Self>>;
        /// Extended light service type.
        type
        LightService: ServiceTrait<LightComponents<Self>>;
        /// ImportQueue for full client
        type
        FullImportQueue: ImportQueue<Self::Block> +
        'static;
        /// ImportQueue for light clients
        type
        LightImportQueue: ImportQueue<Self::Block> +
        'static;
        /// The Fork Choice Strategy for the chain
        type
        SelectChain: SelectChain<Self::Block> +
        'static;
        /// Extrinsic pool constructor for the full client.
        fn build_full_transaction_pool(config: TransactionPoolOptions,
                                       client: Arc<FullClient<Self>>)
        ->
            Result<TransactionPool<Self::FullTransactionPoolApi>,
                   error::Error>;
        /// Extrinsic pool constructor for the light client.
        fn build_light_transaction_pool(config: TransactionPoolOptions,
                                        client: Arc<LightClient<Self>>)
        ->
            Result<TransactionPool<Self::LightTransactionPoolApi>,
                   error::Error>;
        /// Build network protocol.
        fn build_network_protocol(config: &FactoryFullConfiguration<Self>)
        -> Result<Self::NetworkProtocol, error::Error>;
        /// Build finality proof provider for serving network requests on full node.
        fn build_finality_proof_provider(client: Arc<FullClient<Self>>)
        ->
            Result<Option<Arc<FinalityProofProvider<Self::Block>>>,
                   error::Error>;
        /// Build the Fork Choice algorithm for full client
        fn build_select_chain(config: &mut FactoryFullConfiguration<Self>,
                              client: Arc<FullClient<Self>>)
        -> Result<Self::SelectChain, error::Error>;
        /// Build full service.
        fn new_full(config: FactoryFullConfiguration<Self>,
                    executor: TaskExecutor)
        -> Result<Self::FullService, error::Error>;
        /// Build light service.
        fn new_light(config: FactoryFullConfiguration<Self>,
                     executor: TaskExecutor)
        -> Result<Self::LightService, error::Error>;
        /// ImportQueue for a full client
        fn build_full_import_queue(config:
                                       &mut FactoryFullConfiguration<Self>,
                                   _client: Arc<FullClient<Self>>,
                                   _select_chain: Self::SelectChain)
         -> Result<Self::FullImportQueue, error::Error> {
            if let Some(name) = config.chain_spec.consensus_engine() {
                match name {
                    _ =>
                    Err(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Chain Specification defines unknown consensus engine \'",
                                                                             "\'"],
                                                                           &match (&name,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            })).into()),
                }
            } else {
                Err("Chain Specification doesn't contain any consensus_engine name".into())
            }
        }
        /// ImportQueue for a light client
        fn build_light_import_queue(config:
                                        &mut FactoryFullConfiguration<Self>,
                                    _client: Arc<LightClient<Self>>)
         -> Result<Self::LightImportQueue, error::Error> {
            if let Some(name) = config.chain_spec.consensus_engine() {
                match name {
                    _ =>
                    Err(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Chain Specification defines unknown consensus engine \'",
                                                                             "\'"],
                                                                           &match (&name,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            })).into()),
                }
            } else {
                Err("Chain Specification doesn't contain any consensus_engine name".into())
            }
        }
    }
    /// A collection of types and function to generalize over full / light client type.
    pub trait Components: Sized + 'static {
        /// Associated service factory.
        type
        Factory: ServiceFactory;
        /// Client backend.
        type
        Backend: 'static +
        client::backend::Backend<FactoryBlock<Self::Factory>, Blake2Hasher>;
        /// Client executor.
        type
        Executor: 'static +
        client::CallExecutor<FactoryBlock<Self::Factory>, Blake2Hasher> +
        Send +
        Sync +
        Clone;
        /// The type that implements the runtime API.
        type
        RuntimeApi: Send +
        Sync;
        /// A type that can start all runtime-dependent services.
        type
        RuntimeServices: ServiceTrait<Self>;
        /// Extrinsic pool type.
        type
        TransactionPoolApi: 'static +
        txpool::ChainApi<Hash
        =
        <FactoryBlock<Self::Factory> as BlockT>::Hash,
        Block
        =
        FactoryBlock<Self::Factory>>;
        /// Our Import Queue
        type
        ImportQueue: ImportQueue<FactoryBlock<Self::Factory>> +
        'static;
        /// The Fork Choice Strategy for the chain
        type
        SelectChain: SelectChain<FactoryBlock<Self::Factory>>;
        /// Create client.
        fn build_client(config: &FactoryFullConfiguration<Self::Factory>,
                        executor: CodeExecutor<Self::Factory>)
        ->
            Result<(Arc<ComponentClient<Self>>,
                    Option<Arc<OnDemand<FactoryBlock<Self::Factory>>>>),
                   error::Error>;
        /// Create extrinsic pool.
        fn build_transaction_pool(config: TransactionPoolOptions,
                                  client: Arc<ComponentClient<Self>>)
        -> Result<TransactionPool<Self::TransactionPoolApi>, error::Error>;
        /// instance of import queue for clients
        fn build_import_queue(config:
                                  &mut FactoryFullConfiguration<Self::Factory>,
                              client: Arc<ComponentClient<Self>>,
                              select_chain: Option<Self::SelectChain>)
        -> Result<Self::ImportQueue, error::Error>;
        /// Finality proof provider for serving network requests.
        fn build_finality_proof_provider(client: Arc<ComponentClient<Self>>)
        ->
            Result<Option<Arc<FinalityProofProvider<<Self::Factory as
                                                    ServiceFactory>::Block>>>,
                   error::Error>;
        /// Build fork choice selector
        fn build_select_chain(config:
                                  &mut FactoryFullConfiguration<Self::Factory>,
                              client: Arc<ComponentClient<Self>>)
        -> Result<Option<Self::SelectChain>, error::Error>;
    }
    /// A struct that implement `Components` for the full client.
    pub struct FullComponents<Factory: ServiceFactory> {
        service: Service<FullComponents<Factory>>,
    }
    impl <Factory: ServiceFactory> FullComponents<Factory> {
        /// Create new `FullComponents`
        pub fn new(config: FactoryFullConfiguration<Factory>,
                   task_executor: TaskExecutor)
         -> Result<Self, error::Error> {
            Ok(Self{service: Service::new(config, task_executor)?,})
        }
    }
    impl <Factory: ServiceFactory> Deref for FullComponents<Factory> {
        type
        Target
        =
        Service<Self>;
        fn deref(&self) -> &Self::Target { &self.service }
    }
    impl <Factory: ServiceFactory> DerefMut for FullComponents<Factory> {
        fn deref_mut(&mut self) -> &mut Service<Self> { &mut self.service }
    }
    impl <Factory: ServiceFactory> Components for FullComponents<Factory> {
        type
        Factory
        =
        Factory;
        type
        Executor
        =
        FullExecutor<Factory>;
        type
        Backend
        =
        FullBackend<Factory>;
        type
        TransactionPoolApi
        =
        <Factory as ServiceFactory>::FullTransactionPoolApi;
        type
        ImportQueue
        =
        Factory::FullImportQueue;
        type
        RuntimeApi
        =
        Factory::RuntimeApi;
        type
        RuntimeServices
        =
        Factory::FullService;
        type
        SelectChain
        =
        Factory::SelectChain;
        fn build_client(config: &FactoryFullConfiguration<Factory>,
                        executor: CodeExecutor<Self::Factory>)
         ->
             Result<(Arc<ComponentClient<Self>>,
                     Option<Arc<OnDemand<FactoryBlock<Self::Factory>>>>),
                    error::Error> {
            let db_settings =
                client_db::DatabaseSettings{cache_size:
                                                config.database_cache_size.map(|u|
                                                                                   u
                                                                                       as
                                                                                       usize),
                                            state_cache_size:
                                                config.state_cache_size,
                                            path:
                                                config.database_path.as_str().into(),
                                            pruning: config.pruning.clone(),};
            Ok((Arc::new(client_db::new_client(db_settings, executor,
                                               &config.chain_spec,
                                               config.execution_strategies.clone())?),
                None))
        }
        fn build_transaction_pool(config: TransactionPoolOptions,
                                  client: Arc<ComponentClient<Self>>)
         -> Result<TransactionPool<Self::TransactionPoolApi>, error::Error> {
            Factory::build_full_transaction_pool(config, client)
        }
        fn build_import_queue(config:
                                  &mut FactoryFullConfiguration<Self::Factory>,
                              client: Arc<ComponentClient<Self>>,
                              select_chain: Option<Self::SelectChain>)
         -> Result<Self::ImportQueue, error::Error> {
            let select_chain =
                select_chain.ok_or_else(||
                                            error::Error::from(error::ErrorKind::SelectChainRequired))?;
            Factory::build_full_import_queue(config, client, select_chain)
        }
        fn build_select_chain(config:
                                  &mut FactoryFullConfiguration<Self::Factory>,
                              client: Arc<ComponentClient<Self>>)
         -> Result<Option<Self::SelectChain>, error::Error> {
            Self::Factory::build_select_chain(config, client).map(Some)
        }
        fn build_finality_proof_provider(client: Arc<ComponentClient<Self>>)
         ->
             Result<Option<Arc<FinalityProofProvider<<Self::Factory as
                                                     ServiceFactory>::Block>>>,
                    error::Error> {
            Factory::build_finality_proof_provider(client)
        }
    }
    /// A struct that implement `Components` for the light client.
    pub struct LightComponents<Factory: ServiceFactory> {
        service: Service<LightComponents<Factory>>,
    }
    impl <Factory: ServiceFactory> LightComponents<Factory> {
        /// Create new `LightComponents`
        pub fn new(config: FactoryFullConfiguration<Factory>,
                   task_executor: TaskExecutor)
         -> Result<Self, error::Error> {
            Ok(Self{service: Service::new(config, task_executor)?,})
        }
    }
    impl <Factory: ServiceFactory> Deref for LightComponents<Factory> {
        type
        Target
        =
        Service<Self>;
        fn deref(&self) -> &Self::Target { &self.service }
    }
    impl <Factory: ServiceFactory> Components for LightComponents<Factory> {
        type
        Factory
        =
        Factory;
        type
        Executor
        =
        LightExecutor<Factory>;
        type
        Backend
        =
        LightBackend<Factory>;
        type
        TransactionPoolApi
        =
        <Factory as ServiceFactory>::LightTransactionPoolApi;
        type
        ImportQueue
        =
        <Factory as ServiceFactory>::LightImportQueue;
        type
        RuntimeApi
        =
        Factory::RuntimeApi;
        type
        RuntimeServices
        =
        Factory::LightService;
        type
        SelectChain
        =
        Factory::SelectChain;
        fn build_client(config: &FactoryFullConfiguration<Factory>,
                        executor: CodeExecutor<Self::Factory>)
         ->
             Result<(Arc<ComponentClient<Self>>,
                     Option<Arc<OnDemand<FactoryBlock<Self::Factory>>>>),
                    error::Error> {
            let db_settings =
                client_db::DatabaseSettings{cache_size: None,
                                            state_cache_size:
                                                config.state_cache_size,
                                            path:
                                                config.database_path.as_str().into(),
                                            pruning: config.pruning.clone(),};
            let db_storage =
                client_db::light::LightStorage::new(db_settings)?;
            let light_blockchain =
                client::light::new_light_blockchain(db_storage);
            let fetch_checker =
                Arc::new(client::light::new_fetch_checker(light_blockchain.clone(),
                                                          executor.clone()));
            let fetcher = Arc::new(network::OnDemand::new(fetch_checker));
            let client_backend =
                client::light::new_light_backend(light_blockchain,
                                                 fetcher.clone());
            let client =
                client::light::new_light(client_backend, fetcher.clone(),
                                         &config.chain_spec, executor)?;
            Ok((Arc::new(client), Some(fetcher)))
        }
        fn build_transaction_pool(config: TransactionPoolOptions,
                                  client: Arc<ComponentClient<Self>>)
         -> Result<TransactionPool<Self::TransactionPoolApi>, error::Error> {
            Factory::build_light_transaction_pool(config, client)
        }
        fn build_import_queue(config:
                                  &mut FactoryFullConfiguration<Self::Factory>,
                              client: Arc<ComponentClient<Self>>,
                              _select_chain: Option<Self::SelectChain>)
         -> Result<Self::ImportQueue, error::Error> {
            Factory::build_light_import_queue(config, client)
        }
        fn build_finality_proof_provider(_client: Arc<ComponentClient<Self>>)
         ->
             Result<Option<Arc<FinalityProofProvider<<Self::Factory as
                                                     ServiceFactory>::Block>>>,
                    error::Error> {
            Ok(None)
        }
        fn build_select_chain(_config:
                                  &mut FactoryFullConfiguration<Self::Factory>,
                              _client: Arc<ComponentClient<Self>>)
         -> Result<Option<Self::SelectChain>, error::Error> {
            Ok(None)
        }
    }
}
mod chain_spec {
    //! Substrate chain configurations.
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::PathBuf;
    use serde::{Serialize, Deserialize};
    use primitives::storage::{StorageKey, StorageData};
    use runtime_primitives::{BuildStorage, StorageOverlay,
                             ChildrenStorageOverlay};
    use serde_json as json;
    use crate::components::RuntimeGenesis;
    use network::Multiaddr;
    use tel::TelemetryEndpoints;
    enum GenesisSource<G> {
        File(PathBuf),
        Embedded(&'static [u8]),
        Factory(fn() -> G),
    }
    impl <G: RuntimeGenesis> Clone for GenesisSource<G> {
        fn clone(&self) -> Self {
            match *self {
                GenesisSource::File(ref path) =>
                GenesisSource::File(path.clone()),
                GenesisSource::Embedded(d) => GenesisSource::Embedded(d),
                GenesisSource::Factory(f) => GenesisSource::Factory(f),
            }
        }
    }
    impl <G: RuntimeGenesis> GenesisSource<G> {
        fn resolve(&self) -> Result<Genesis<G>, String> {
            struct GenesisContainer<G> {
                genesis: Genesis<G>,
            }
            #[allow(non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications)]
            const _IMPL_SERIALIZE_FOR_GenesisContainer: () =
                {
                    #[allow(unknown_lints)]
                    #[allow(rust_2018_idioms)]
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                    #[automatically_derived]
                    impl <G> _serde::Serialize for GenesisContainer<G> where
                     G: _serde::Serialize {
                        fn serialize<__S>(&self, __serializer: __S)
                         -> _serde::export::Result<__S::Ok, __S::Error> where
                         __S: _serde::Serializer {
                            let mut __serde_state =
                                match _serde::Serializer::serialize_struct(__serializer,
                                                                           "GenesisContainer",
                                                                           false
                                                                               as
                                                                               usize
                                                                               +
                                                                               1)
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                                "genesis",
                                                                                &self.genesis)
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
            #[allow(non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_GenesisContainer: () =
                {
                    #[allow(unknown_lints)]
                    #[allow(rust_2018_idioms)]
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                    #[automatically_derived]
                    impl <'de, G> _serde::Deserialize<'de> for
                     GenesisContainer<G> where G: _serde::Deserialize<'de> {
                        fn deserialize<__D>(__deserializer: __D)
                         -> _serde::export::Result<Self, __D::Error> where
                         __D: _serde::Deserializer<'de> {
                            #[allow(non_camel_case_types)]
                            enum __Field { __field0, __ignore, }
                            struct __FieldVisitor;
                            impl <'de> _serde::de::Visitor<'de> for
                             __FieldVisitor {
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
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    match __value {
                                        0u64 =>
                                        _serde::export::Ok(__Field::__field0),
                                        _ =>
                                        _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                             &"field index 0 <= i < 1")),
                                    }
                                }
                                fn visit_str<__E>(self, __value: &str)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    match __value {
                                        "genesis" =>
                                        _serde::export::Ok(__Field::__field0),
                                        _ => {
                                            _serde::export::Ok(__Field::__ignore)
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(self, __value: &[u8])
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    match __value {
                                        b"genesis" =>
                                        _serde::export::Ok(__Field::__field0),
                                        _ => {
                                            _serde::export::Ok(__Field::__ignore)
                                        }
                                    }
                                }
                            }
                            impl <'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(__deserializer: __D)
                                 -> _serde::export::Result<Self, __D::Error>
                                 where __D: _serde::Deserializer<'de> {
                                    _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                                 __FieldVisitor)
                                }
                            }
                            struct __Visitor<'de, G> where
                                   G: _serde::Deserialize<'de> {
                                marker: _serde::export::PhantomData<GenesisContainer<G>>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl <'de, G> _serde::de::Visitor<'de> for
                             __Visitor<'de, G> where
                             G: _serde::Deserialize<'de> {
                                type
                                Value
                                =
                                GenesisContainer<G>;
                                fn expecting(&self,
                                             __formatter:
                                                 &mut _serde::export::Formatter)
                                 -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(__formatter,
                                                                         "struct GenesisContainer")
                                }
                                #[inline]
                                fn visit_seq<__A>(self, mut __seq: __A)
                                 ->
                                     _serde::export::Result<Self::Value,
                                                            __A::Error> where
                                 __A: _serde::de::SeqAccess<'de> {
                                    let __field0 =
                                        match match _serde::de::SeqAccess::next_element::<Genesis<G>>(&mut __seq)
                                                  {
                                                  _serde::export::Ok(__val) =>
                                                  __val,
                                                  _serde::export::Err(__err)
                                                  => {
                                                      return _serde::export::Err(__err);
                                                  }
                                              } {
                                            _serde::export::Some(__value) =>
                                            __value,
                                            _serde::export::None => {
                                                return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                             &"struct GenesisContainer with 1 element"));
                                            }
                                        };
                                    _serde::export::Ok(GenesisContainer{genesis:
                                                                            __field0,})
                                }
                                #[inline]
                                fn visit_map<__A>(self, mut __map: __A)
                                 ->
                                     _serde::export::Result<Self::Value,
                                                            __A::Error> where
                                 __A: _serde::de::MapAccess<'de> {
                                    let mut __field0:
                                            _serde::export::Option<Genesis<G>> =
                                        _serde::export::None;
                                    while let _serde::export::Some(__key) =
                                              match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                                  {
                                                  _serde::export::Ok(__val) =>
                                                  __val,
                                                  _serde::export::Err(__err)
                                                  => {
                                                      return _serde::export::Err(__err);
                                                  }
                                              } {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::export::Option::is_some(&__field0)
                                                   {
                                                    return _serde::export::Err(<__A::Error
                                                                                   as
                                                                                   _serde::de::Error>::duplicate_field("genesis"));
                                                }
                                                __field0 =
                                                    _serde::export::Some(match _serde::de::MapAccess::next_value::<Genesis<G>>(&mut __map)
                                                                             {
                                                                             _serde::export::Ok(__val)
                                                                             =>
                                                                             __val,
                                                                             _serde::export::Err(__err)
                                                                             =>
                                                                             {
                                                                                 return _serde::export::Err(__err);
                                                                             }
                                                                         });
                                            }
                                            _ => {
                                                let _ =
                                                    match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                        {
                                                        _serde::export::Ok(__val)
                                                        => __val,
                                                        _serde::export::Err(__err)
                                                        => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    };
                                            }
                                        }
                                    }
                                    let __field0 =
                                        match __field0 {
                                            _serde::export::Some(__field0) =>
                                            __field0,
                                            _serde::export::None =>
                                            match _serde::private::de::missing_field("genesis")
                                                {
                                                _serde::export::Ok(__val) =>
                                                __val,
                                                _serde::export::Err(__err) =>
                                                {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        };
                                    _serde::export::Ok(GenesisContainer{genesis:
                                                                            __field0,})
                                }
                            }
                            const FIELDS: &'static [&'static str] =
                                &["genesis"];
                            _serde::Deserializer::deserialize_struct(__deserializer,
                                                                     "GenesisContainer",
                                                                     FIELDS,
                                                                     __Visitor{marker:
                                                                                   _serde::export::PhantomData::<GenesisContainer<G>>,
                                                                               lifetime:
                                                                                   _serde::export::PhantomData,})
                        }
                    }
                };
            match *self {
                GenesisSource::File(ref path) => {
                    let file =
                        File::open(path).map_err(|e|
                                                     ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Error opening spec file: "],
                                                                                                        &match (&e,)
                                                                                                             {
                                                                                                             (arg0,)
                                                                                                             =>
                                                                                                             [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                          ::std::fmt::Display::fmt)],
                                                                                                         })))?;
                    let genesis: GenesisContainer<G> =
                        json::from_reader(file).map_err(|e|
                                                            ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Error parsing spec file: "],
                                                                                                               &match (&e,)
                                                                                                                    {
                                                                                                                    (arg0,)
                                                                                                                    =>
                                                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                                                })))?;
                    Ok(genesis.genesis)
                }
                GenesisSource::Embedded(buf) => {
                    let genesis: GenesisContainer<G> =
                        json::from_reader(buf).map_err(|e|
                                                           ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Error parsing embedded file: "],
                                                                                                              &match (&e,)
                                                                                                                   {
                                                                                                                   (arg0,)
                                                                                                                   =>
                                                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                ::std::fmt::Display::fmt)],
                                                                                                               })))?;
                    Ok(genesis.genesis)
                }
                GenesisSource::Factory(f) => Ok(Genesis::Runtime(f())),
            }
        }
    }
    impl <'a, G: RuntimeGenesis> BuildStorage for &'a ChainSpec<G> {
        fn build_storage(self)
         -> Result<(StorageOverlay, ChildrenStorageOverlay), String> {
            match self.genesis.resolve()? {
                Genesis::Runtime(gc) => gc.build_storage(),
                Genesis::Raw(map) =>
                Ok((map.into_iter().map(|(k, v)| (k.0, v.0)).collect(),
                    Default::default())),
            }
        }
        fn assimilate_storage(self, _: &mut StorageOverlay,
                              _: &mut ChildrenStorageOverlay)
         -> Result<(), String> {
            Err("`assimilate_storage` not implemented for `ChainSpec`.".into())
        }
    }
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    enum Genesis<G> { Runtime(G), Raw(HashMap<StorageKey, StorageData>), }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Genesis: () =
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
            impl <G> _serde::Serialize for Genesis<G> where
             G: _serde::Serialize {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    match *self {
                        Genesis::Runtime(ref __field0) =>
                        _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                      "Genesis",
                                                                      0u32,
                                                                      "runtime",
                                                                      __field0),
                        Genesis::Raw(ref __field0) =>
                        _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                      "Genesis",
                                                                      1u32,
                                                                      "raw",
                                                                      __field0),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Genesis: () =
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
            impl <'de, G> _serde::Deserialize<'de> for Genesis<G> where
             G: _serde::Deserialize<'de> {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, }
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
                                                                 "variant identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                _ =>
                                _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                     &"variant index 0 <= i < 2")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                "runtime" =>
                                _serde::export::Ok(__Field::__field0),
                                "raw" =>
                                _serde::export::Ok(__Field::__field1),
                                _ => {
                                    _serde::export::Err(_serde::de::Error::unknown_variant(__value,
                                                                                           VARIANTS))
                                }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"runtime" =>
                                _serde::export::Ok(__Field::__field0),
                                b"raw" =>
                                _serde::export::Ok(__Field::__field1),
                                _ => {
                                    let __value =
                                        &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_variant(__value,
                                                                                           VARIANTS))
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
                    struct __Visitor<'de, G> where
                           G: _serde::Deserialize<'de> {
                        marker: _serde::export::PhantomData<Genesis<G>>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de, G> _serde::de::Visitor<'de> for
                     __Visitor<'de, G> where G: _serde::Deserialize<'de> {
                        type
                        Value
                        =
                        Genesis<G>;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "enum Genesis")
                        }
                        fn visit_enum<__A>(self, __data: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::EnumAccess<'de> {
                            match match _serde::de::EnumAccess::variant(__data)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                (__Field::__field0, __variant) =>
                                _serde::export::Result::map(_serde::de::VariantAccess::newtype_variant::<G>(__variant),
                                                            Genesis::Runtime),
                                (__Field::__field1, __variant) =>
                                _serde::export::Result::map(_serde::de::VariantAccess::newtype_variant::<HashMap<StorageKey,
                                                                                                                 StorageData>>(__variant),
                                                            Genesis::Raw),
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["runtime", "raw"];
                    _serde::Deserializer::deserialize_enum(__deserializer,
                                                           "Genesis",
                                                           VARIANTS,
                                                           __Visitor{marker:
                                                                         _serde::export::PhantomData::<Genesis<G>>,
                                                                     lifetime:
                                                                         _serde::export::PhantomData,})
                }
            }
        };
    #[serde(rename_all = "camelCase")]
    struct ChainSpecFile {
        pub name: String,
        pub id: String,
        pub boot_nodes: Vec<String>,
        pub telemetry_endpoints: Option<TelemetryEndpoints>,
        pub protocol_id: Option<String>,
        pub consensus_engine: Option<String>,
        pub properties: Option<Properties>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ChainSpecFile: () =
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
            impl _serde::Serialize for ChainSpecFile {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                                                   "ChainSpecFile",
                                                                   false as
                                                                       usize +
                                                                       1 + 1 +
                                                                       1 + 1 +
                                                                       1 + 1 +
                                                                       1) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "name",
                                                                        &self.name)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "id",
                                                                        &self.id)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "bootNodes",
                                                                        &self.boot_nodes)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "telemetryEndpoints",
                                                                        &self.telemetry_endpoints)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "protocolId",
                                                                        &self.protocol_id)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "consensusEngine",
                                                                        &self.consensus_engine)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "properties",
                                                                        &self.properties)
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
    const _IMPL_DESERIALIZE_FOR_ChainSpecFile: () =
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
            impl <'de> _serde::Deserialize<'de> for ChainSpecFile {
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
                        __ignore,
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
                                _ =>
                                _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                     &"field index 0 <= i < 7")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                "name" =>
                                _serde::export::Ok(__Field::__field0),
                                "id" => _serde::export::Ok(__Field::__field1),
                                "bootNodes" =>
                                _serde::export::Ok(__Field::__field2),
                                "telemetryEndpoints" =>
                                _serde::export::Ok(__Field::__field3),
                                "protocolId" =>
                                _serde::export::Ok(__Field::__field4),
                                "consensusEngine" =>
                                _serde::export::Ok(__Field::__field5),
                                "properties" =>
                                _serde::export::Ok(__Field::__field6),
                                _ => { _serde::export::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"name" =>
                                _serde::export::Ok(__Field::__field0),
                                b"id" =>
                                _serde::export::Ok(__Field::__field1),
                                b"bootNodes" =>
                                _serde::export::Ok(__Field::__field2),
                                b"telemetryEndpoints" =>
                                _serde::export::Ok(__Field::__field3),
                                b"protocolId" =>
                                _serde::export::Ok(__Field::__field4),
                                b"consensusEngine" =>
                                _serde::export::Ok(__Field::__field5),
                                b"properties" =>
                                _serde::export::Ok(__Field::__field6),
                                _ => { _serde::export::Ok(__Field::__ignore) }
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
                        marker: _serde::export::PhantomData<ChainSpecFile>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type
                        Value
                        =
                        ChainSpecFile;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "struct ChainSpecFile")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"struct ChainSpecFile with 7 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                     &"struct ChainSpecFile with 7 elements"));
                                    }
                                };
                            let __field2 =
                                match match _serde::de::SeqAccess::next_element::<Vec<String>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                     &"struct ChainSpecFile with 7 elements"));
                                    }
                                };
                            let __field3 =
                                match match _serde::de::SeqAccess::next_element::<Option<TelemetryEndpoints>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                     &"struct ChainSpecFile with 7 elements"));
                                    }
                                };
                            let __field4 =
                                match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                     &"struct ChainSpecFile with 7 elements"));
                                    }
                                };
                            let __field5 =
                                match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(5usize,
                                                                                                     &"struct ChainSpecFile with 7 elements"));
                                    }
                                };
                            let __field6 =
                                match match _serde::de::SeqAccess::next_element::<Option<Properties>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(6usize,
                                                                                                     &"struct ChainSpecFile with 7 elements"));
                                    }
                                };
                            _serde::export::Ok(ChainSpecFile{name: __field0,
                                                             id: __field1,
                                                             boot_nodes:
                                                                 __field2,
                                                             telemetry_endpoints:
                                                                 __field3,
                                                             protocol_id:
                                                                 __field4,
                                                             consensus_engine:
                                                                 __field5,
                                                             properties:
                                                                 __field6,})
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::MapAccess<'de> {
                            let mut __field0: _serde::export::Option<String> =
                                _serde::export::None;
                            let mut __field1: _serde::export::Option<String> =
                                _serde::export::None;
                            let mut __field2:
                                    _serde::export::Option<Vec<String>> =
                                _serde::export::None;
                            let mut __field3:
                                    _serde::export::Option<Option<TelemetryEndpoints>> =
                                _serde::export::None;
                            let mut __field4:
                                    _serde::export::Option<Option<String>> =
                                _serde::export::None;
                            let mut __field5:
                                    _serde::export::Option<Option<String>> =
                                _serde::export::None;
                            let mut __field6:
                                    _serde::export::Option<Option<Properties>> =
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
                                                                           _serde::de::Error>::duplicate_field("name"));
                                        }
                                        __field0 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
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
                                                                           _serde::de::Error>::duplicate_field("id"));
                                        }
                                        __field1 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
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
                                                                           _serde::de::Error>::duplicate_field("bootNodes"));
                                        }
                                        __field2 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<String>>(&mut __map)
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
                                                                           _serde::de::Error>::duplicate_field("telemetryEndpoints"));
                                        }
                                        __field3 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<TelemetryEndpoints>>(&mut __map)
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
                                                                           _serde::de::Error>::duplicate_field("protocolId"));
                                        }
                                        __field4 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
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
                                                                           _serde::de::Error>::duplicate_field("consensusEngine"));
                                        }
                                        __field5 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
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
                                                                           _serde::de::Error>::duplicate_field("properties"));
                                        }
                                        __field6 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<Properties>>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__val)
                                                                     => __val,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
                                                                 });
                                    }
                                    _ => {
                                        let _ =
                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                {
                                                _serde::export::Ok(__val) =>
                                                __val,
                                                _serde::export::Err(__err) =>
                                                {
                                                    return _serde::export::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::export::Some(__field0) =>
                                    __field0,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("name")
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
                                    match _serde::private::de::missing_field("id")
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
                                    match _serde::private::de::missing_field("bootNodes")
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
                                    match _serde::private::de::missing_field("telemetryEndpoints")
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
                                    match _serde::private::de::missing_field("protocolId")
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
                                    match _serde::private::de::missing_field("consensusEngine")
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
                                    match _serde::private::de::missing_field("properties")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            _serde::export::Ok(ChainSpecFile{name: __field0,
                                                             id: __field1,
                                                             boot_nodes:
                                                                 __field2,
                                                             telemetry_endpoints:
                                                                 __field3,
                                                             protocol_id:
                                                                 __field4,
                                                             consensus_engine:
                                                                 __field5,
                                                             properties:
                                                                 __field6,})
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["name", "id", "bootNodes", "telemetryEndpoints",
                          "protocolId", "consensusEngine", "properties"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                                                             "ChainSpecFile",
                                                             FIELDS,
                                                             __Visitor{marker:
                                                                           _serde::export::PhantomData::<ChainSpecFile>,
                                                                       lifetime:
                                                                           _serde::export::PhantomData,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ChainSpecFile {
        #[inline]
        fn clone(&self) -> ChainSpecFile {
            match *self {
                ChainSpecFile {
                name: ref __self_0_0,
                id: ref __self_0_1,
                boot_nodes: ref __self_0_2,
                telemetry_endpoints: ref __self_0_3,
                protocol_id: ref __self_0_4,
                consensus_engine: ref __self_0_5,
                properties: ref __self_0_6 } =>
                ChainSpecFile{name:
                                  ::std::clone::Clone::clone(&(*__self_0_0)),
                              id: ::std::clone::Clone::clone(&(*__self_0_1)),
                              boot_nodes:
                                  ::std::clone::Clone::clone(&(*__self_0_2)),
                              telemetry_endpoints:
                                  ::std::clone::Clone::clone(&(*__self_0_3)),
                              protocol_id:
                                  ::std::clone::Clone::clone(&(*__self_0_4)),
                              consensus_engine:
                                  ::std::clone::Clone::clone(&(*__self_0_5)),
                              properties:
                                  ::std::clone::Clone::clone(&(*__self_0_6)),},
            }
        }
    }
    /// Arbitrary properties defined in chain spec as a JSON object
    pub type Properties = json::map::Map<String, json::Value>;
    /// A configuration of a chain. Can be used to build a genesis block.
    pub struct ChainSpec<G: RuntimeGenesis> {
        spec: ChainSpecFile,
        genesis: GenesisSource<G>,
    }
    impl <G: RuntimeGenesis> Clone for ChainSpec<G> {
        fn clone(&self) -> Self {
            ChainSpec{spec: self.spec.clone(), genesis: self.genesis.clone(),}
        }
    }
    impl <G: RuntimeGenesis> ChainSpec<G> {
        /// A list of bootnode addresses.
        pub fn boot_nodes(&self) -> &[String] { &self.spec.boot_nodes }
        /// Spec name.
        pub fn name(&self) -> &str { &self.spec.name }
        /// Spec id.
        pub fn id(&self) -> &str { &self.spec.id }
        /// Telemetry endpoints (if any)
        pub fn telemetry_endpoints(&self) -> &Option<TelemetryEndpoints> {
            &self.spec.telemetry_endpoints
        }
        /// Network protocol id.
        pub fn protocol_id(&self) -> Option<&str> {
            self.spec.protocol_id.as_ref().map(String::as_str)
        }
        /// Name of the consensus engine.
        pub fn consensus_engine(&self) -> Option<&str> {
            self.spec.consensus_engine.as_ref().map(String::as_str)
        }
        /// Additional loosly-typed properties of the chain.
        pub fn properties(&self) -> Properties {
            self.spec.properties.as_ref().unwrap_or(&json::map::Map::new()).clone()
        }
        /// Add a bootnode to the list.
        pub fn add_boot_node(&mut self, addr: Multiaddr) {
            self.spec.boot_nodes.push(addr.to_string())
        }
        /// Parse json content into a `ChainSpec`
        pub fn from_embedded(json: &'static [u8]) -> Result<Self, String> {
            let spec =
                json::from_slice(json).map_err(|e|
                                                   ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Error parsing spec file: "],
                                                                                                      &match (&e,)
                                                                                                           {
                                                                                                           (arg0,)
                                                                                                           =>
                                                                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                        ::std::fmt::Display::fmt)],
                                                                                                       })))?;
            Ok(ChainSpec{spec, genesis: GenesisSource::Embedded(json),})
        }
        /// Parse json file into a `ChainSpec`
        pub fn from_json_file(path: PathBuf) -> Result<Self, String> {
            let file =
                File::open(&path).map_err(|e|
                                              ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Error opening spec file: "],
                                                                                                 &match (&e,)
                                                                                                      {
                                                                                                      (arg0,)
                                                                                                      =>
                                                                                                      [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                   ::std::fmt::Display::fmt)],
                                                                                                  })))?;
            let spec =
                json::from_reader(file).map_err(|e|
                                                    ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Error parsing spec file: "],
                                                                                                       &match (&e,)
                                                                                                            {
                                                                                                            (arg0,)
                                                                                                            =>
                                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                                        })))?;
            Ok(ChainSpec{spec, genesis: GenesisSource::File(path),})
        }
        /// Create hardcoded spec.
        pub fn from_genesis(name: &str, id: &str, constructor: fn() -> G,
                            boot_nodes: Vec<String>,
                            telemetry_endpoints: Option<TelemetryEndpoints>,
                            protocol_id: Option<&str>,
                            consensus_engine: Option<&str>,
                            properties: Option<Properties>) -> Self {
            let spec =
                ChainSpecFile{name: name.to_owned(),
                              id: id.to_owned(),
                              boot_nodes: boot_nodes,
                              telemetry_endpoints,
                              protocol_id: protocol_id.map(str::to_owned),
                              consensus_engine:
                                  consensus_engine.map(str::to_owned),
                              properties,};
            ChainSpec{spec, genesis: GenesisSource::Factory(constructor),}
        }
        /// Dump to json string.
        pub fn to_json(self, raw: bool) -> Result<String, String> {
            struct Container<G> {
                #[serde(flatten)]
                spec: ChainSpecFile,
                genesis: Genesis<G>,
            }
            #[allow(non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications)]
            const _IMPL_SERIALIZE_FOR_Container: () =
                {
                    #[allow(unknown_lints)]
                    #[allow(rust_2018_idioms)]
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                    #[automatically_derived]
                    impl <G> _serde::Serialize for Container<G> where
                     G: _serde::Serialize {
                        fn serialize<__S>(&self, __serializer: __S)
                         -> _serde::export::Result<__S::Ok, __S::Error> where
                         __S: _serde::Serializer {
                            let mut __serde_state =
                                match _serde::Serializer::serialize_map(__serializer,
                                                                        _serde::export::None)
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            match _serde::Serialize::serialize(&&self.spec,
                                                               _serde::private::ser::FlatMapSerializer(&mut __serde_state))
                                {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeMap::serialize_entry(&mut __serde_state,
                                                                             "genesis",
                                                                             &self.genesis)
                                {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::ser::SerializeMap::end(__serde_state)
                        }
                    }
                };
            #[allow(non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_Container: () =
                {
                    #[allow(unknown_lints)]
                    #[allow(rust_2018_idioms)]
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                    #[automatically_derived]
                    impl <'de, G> _serde::Deserialize<'de> for Container<G>
                     where G: _serde::Deserialize<'de> {
                        fn deserialize<__D>(__deserializer: __D)
                         -> _serde::export::Result<Self, __D::Error> where
                         __D: _serde::Deserializer<'de> {
                            #[allow(non_camel_case_types)]
                            enum __Field<'de> {
                                __field1,
                                __other(_serde::private::de::Content<'de>),
                            }
                            struct __FieldVisitor;
                            impl <'de> _serde::de::Visitor<'de> for
                             __FieldVisitor {
                                type
                                Value
                                =
                                __Field<'de>;
                                fn expecting(&self,
                                             __formatter:
                                                 &mut _serde::export::Formatter)
                                 -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(__formatter,
                                                                         "field identifier")
                                }
                                fn visit_bool<__E>(self, __value: bool)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::Bool(__value)))
                                }
                                fn visit_i8<__E>(self, __value: i8)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::I8(__value)))
                                }
                                fn visit_i16<__E>(self, __value: i16)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::I16(__value)))
                                }
                                fn visit_i32<__E>(self, __value: i32)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::I32(__value)))
                                }
                                fn visit_i64<__E>(self, __value: i64)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::I64(__value)))
                                }
                                fn visit_u8<__E>(self, __value: u8)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::U8(__value)))
                                }
                                fn visit_u16<__E>(self, __value: u16)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::U16(__value)))
                                }
                                fn visit_u32<__E>(self, __value: u32)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::U32(__value)))
                                }
                                fn visit_u64<__E>(self, __value: u64)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::U64(__value)))
                                }
                                fn visit_f32<__E>(self, __value: f32)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::F32(__value)))
                                }
                                fn visit_f64<__E>(self, __value: f64)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::F64(__value)))
                                }
                                fn visit_char<__E>(self, __value: char)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::Char(__value)))
                                }
                                fn visit_unit<__E>(self)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    _serde::export::Ok(__Field::__other(_serde::private::de::Content::Unit))
                                }
                                fn visit_borrowed_str<__E>(self,
                                                           __value: &'de str)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    match __value {
                                        "genesis" =>
                                        _serde::export::Ok(__Field::__field1),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Str(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_borrowed_bytes<__E>(self,
                                                             __value:
                                                                 &'de [u8])
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    match __value {
                                        b"genesis" =>
                                        _serde::export::Ok(__Field::__field1),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Bytes(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_str<__E>(self, __value: &str)
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    match __value {
                                        "genesis" =>
                                        _serde::export::Ok(__Field::__field1),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::String(__value.to_string());
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(self, __value: &[u8])
                                 -> _serde::export::Result<Self::Value, __E>
                                 where __E: _serde::de::Error {
                                    match __value {
                                        b"genesis" =>
                                        _serde::export::Ok(__Field::__field1),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::ByteBuf(__value.to_vec());
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                            }
                            impl <'de> _serde::Deserialize<'de> for
                             __Field<'de> {
                                #[inline]
                                fn deserialize<__D>(__deserializer: __D)
                                 -> _serde::export::Result<Self, __D::Error>
                                 where __D: _serde::Deserializer<'de> {
                                    _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                                 __FieldVisitor)
                                }
                            }
                            struct __Visitor<'de, G> where
                                   G: _serde::Deserialize<'de> {
                                marker: _serde::export::PhantomData<Container<G>>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl <'de, G> _serde::de::Visitor<'de> for
                             __Visitor<'de, G> where
                             G: _serde::Deserialize<'de> {
                                type
                                Value
                                =
                                Container<G>;
                                fn expecting(&self,
                                             __formatter:
                                                 &mut _serde::export::Formatter)
                                 -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(__formatter,
                                                                         "struct Container")
                                }
                                #[inline]
                                fn visit_map<__A>(self, mut __map: __A)
                                 ->
                                     _serde::export::Result<Self::Value,
                                                            __A::Error> where
                                 __A: _serde::de::MapAccess<'de> {
                                    let mut __field1:
                                            _serde::export::Option<Genesis<G>> =
                                        _serde::export::None;
                                    let mut __collect =
                                        _serde::export::Vec::<_serde::export::Option<(_serde::private::de::Content,
                                                                                      _serde::private::de::Content)>>::new();
                                    while let _serde::export::Some(__key) =
                                              match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                                  {
                                                  _serde::export::Ok(__val) =>
                                                  __val,
                                                  _serde::export::Err(__err)
                                                  => {
                                                      return _serde::export::Err(__err);
                                                  }
                                              } {
                                        match __key {
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1)
                                                   {
                                                    return _serde::export::Err(<__A::Error
                                                                                   as
                                                                                   _serde::de::Error>::duplicate_field("genesis"));
                                                }
                                                __field1 =
                                                    _serde::export::Some(match _serde::de::MapAccess::next_value::<Genesis<G>>(&mut __map)
                                                                             {
                                                                             _serde::export::Ok(__val)
                                                                             =>
                                                                             __val,
                                                                             _serde::export::Err(__err)
                                                                             =>
                                                                             {
                                                                                 return _serde::export::Err(__err);
                                                                             }
                                                                         });
                                            }
                                            __Field::__other(__name) => {
                                                __collect.push(_serde::export::Some((__name,
                                                                                     match _serde::de::MapAccess::next_value(&mut __map)
                                                                                         {
                                                                                         _serde::export::Ok(__val)
                                                                                         =>
                                                                                         __val,
                                                                                         _serde::export::Err(__err)
                                                                                         =>
                                                                                         {
                                                                                             return _serde::export::Err(__err);
                                                                                         }
                                                                                     })));
                                            }
                                        }
                                    }
                                    let __field1 =
                                        match __field1 {
                                            _serde::export::Some(__field1) =>
                                            __field1,
                                            _serde::export::None =>
                                            match _serde::private::de::missing_field("genesis")
                                                {
                                                _serde::export::Ok(__val) =>
                                                __val,
                                                _serde::export::Err(__err) =>
                                                {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        };
                                    let __field0: ChainSpecFile =
                                        match _serde::de::Deserialize::deserialize(_serde::private::de::FlatMapDeserializer(&mut __collect,
                                                                                                                            _serde::export::PhantomData))
                                            {
                                            _serde::export::Ok(__val) =>
                                            __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    _serde::export::Ok(Container{spec:
                                                                     __field0,
                                                                 genesis:
                                                                     __field1,})
                                }
                            }
                            _serde::Deserializer::deserialize_map(__deserializer,
                                                                  __Visitor{marker:
                                                                                _serde::export::PhantomData::<Container<G>>,
                                                                            lifetime:
                                                                                _serde::export::PhantomData,})
                        }
                    }
                };
            let genesis =
                match (raw, self.genesis.resolve()?) {
                    (true, Genesis::Runtime(g)) => {
                        let storage =
                            g.build_storage()?.0.into_iter().map(|(k, v)|
                                                                     (StorageKey(k),
                                                                      StorageData(v))).collect();
                        Genesis::Raw(storage)
                    }
                    (_, genesis) => genesis,
                };
            let spec = Container{spec: self.spec, genesis,};
            json::to_string_pretty(&spec).map_err(|e|
                                                      ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Error generating spec json: "],
                                                                                                         &match (&e,)
                                                                                                              {
                                                                                                              (arg0,)
                                                                                                              =>
                                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                                          })))
        }
    }
}
pub mod config {
    //! Service configuration.
    use std::net::SocketAddr;
    use transaction_pool;
    use crate::chain_spec::ChainSpec;
    pub use client::ExecutionStrategies;
    pub use client_db::PruningMode;
    pub use network::config::{NetworkConfiguration, Roles};
    use runtime_primitives::BuildStorage;
    use serde::{Serialize, de::DeserializeOwned};
    use target_info::Target;
    use tel::TelemetryEndpoints;
    /// Service configuration.
    pub struct Configuration<C, G: Serialize + DeserializeOwned +
                             BuildStorage> {
        /// Implementation name
        pub impl_name: &'static str,
        /// Implementation version
        pub impl_version: &'static str,
        /// Git commit if any.
        pub impl_commit: &'static str,
        /// Node roles.
        pub roles: Roles,
        /// Extrinsic pool configuration.
        pub transaction_pool: transaction_pool::txpool::Options,
        /// Network configuration.
        pub network: NetworkConfiguration,
        /// Path to key files.
        pub keystore_path: String,
        /// Path to the database.
        pub database_path: String,
        /// Cache Size for internal database in MiB
        pub database_cache_size: Option<u32>,
        /// Size of internal state cache in Bytes
        pub state_cache_size: usize,
        /// Pruning settings.
        pub pruning: PruningMode,
        /// Additional key seeds.
        pub keys: Vec<String>,
        /// Chain configuration.
        pub chain_spec: ChainSpec<G>,
        /// Custom configuration.
        pub custom: C,
        /// Node name.
        pub name: String,
        /// Execution strategies.
        pub execution_strategies: ExecutionStrategies,
        /// RPC over HTTP binding address. `None` if disabled.
        pub rpc_http: Option<SocketAddr>,
        /// RPC over Websockets binding address. `None` if disabled.
        pub rpc_ws: Option<SocketAddr>,
        /// CORS settings for HTTP & WS servers. `None` if all origins are allowed.
        pub rpc_cors: Option<Vec<String>>,
        /// Telemetry service URL. `None` if disabled.
        pub telemetry_endpoints: Option<TelemetryEndpoints>,
        /// The default number of 64KB pages to allocate for Wasm execution
        pub default_heap_pages: Option<u64>,
        /// Should offchain workers be executed.
        pub offchain_worker: bool,
        /// Enable authoring even when offline.
        pub force_authoring: bool,
        /// Disable GRANDPA when running in validator mode
        pub disable_grandpa: bool,
        /// Node keystore's password
        pub password: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <C: ::std::clone::Clone, G: ::std::clone::Clone + Serialize +
          DeserializeOwned + BuildStorage> ::std::clone::Clone for
     Configuration<C, G> {
        #[inline]
        fn clone(&self) -> Configuration<C, G> {
            match *self {
                Configuration {
                impl_name: ref __self_0_0,
                impl_version: ref __self_0_1,
                impl_commit: ref __self_0_2,
                roles: ref __self_0_3,
                transaction_pool: ref __self_0_4,
                network: ref __self_0_5,
                keystore_path: ref __self_0_6,
                database_path: ref __self_0_7,
                database_cache_size: ref __self_0_8,
                state_cache_size: ref __self_0_9,
                pruning: ref __self_0_10,
                keys: ref __self_0_11,
                chain_spec: ref __self_0_12,
                custom: ref __self_0_13,
                name: ref __self_0_14,
                execution_strategies: ref __self_0_15,
                rpc_http: ref __self_0_16,
                rpc_ws: ref __self_0_17,
                rpc_cors: ref __self_0_18,
                telemetry_endpoints: ref __self_0_19,
                default_heap_pages: ref __self_0_20,
                offchain_worker: ref __self_0_21,
                force_authoring: ref __self_0_22,
                disable_grandpa: ref __self_0_23,
                password: ref __self_0_24 } =>
                Configuration{impl_name:
                                  ::std::clone::Clone::clone(&(*__self_0_0)),
                              impl_version:
                                  ::std::clone::Clone::clone(&(*__self_0_1)),
                              impl_commit:
                                  ::std::clone::Clone::clone(&(*__self_0_2)),
                              roles:
                                  ::std::clone::Clone::clone(&(*__self_0_3)),
                              transaction_pool:
                                  ::std::clone::Clone::clone(&(*__self_0_4)),
                              network:
                                  ::std::clone::Clone::clone(&(*__self_0_5)),
                              keystore_path:
                                  ::std::clone::Clone::clone(&(*__self_0_6)),
                              database_path:
                                  ::std::clone::Clone::clone(&(*__self_0_7)),
                              database_cache_size:
                                  ::std::clone::Clone::clone(&(*__self_0_8)),
                              state_cache_size:
                                  ::std::clone::Clone::clone(&(*__self_0_9)),
                              pruning:
                                  ::std::clone::Clone::clone(&(*__self_0_10)),
                              keys:
                                  ::std::clone::Clone::clone(&(*__self_0_11)),
                              chain_spec:
                                  ::std::clone::Clone::clone(&(*__self_0_12)),
                              custom:
                                  ::std::clone::Clone::clone(&(*__self_0_13)),
                              name:
                                  ::std::clone::Clone::clone(&(*__self_0_14)),
                              execution_strategies:
                                  ::std::clone::Clone::clone(&(*__self_0_15)),
                              rpc_http:
                                  ::std::clone::Clone::clone(&(*__self_0_16)),
                              rpc_ws:
                                  ::std::clone::Clone::clone(&(*__self_0_17)),
                              rpc_cors:
                                  ::std::clone::Clone::clone(&(*__self_0_18)),
                              telemetry_endpoints:
                                  ::std::clone::Clone::clone(&(*__self_0_19)),
                              default_heap_pages:
                                  ::std::clone::Clone::clone(&(*__self_0_20)),
                              offchain_worker:
                                  ::std::clone::Clone::clone(&(*__self_0_21)),
                              force_authoring:
                                  ::std::clone::Clone::clone(&(*__self_0_22)),
                              disable_grandpa:
                                  ::std::clone::Clone::clone(&(*__self_0_23)),
                              password:
                                  ::std::clone::Clone::clone(&(*__self_0_24)),},
            }
        }
    }
    impl <C: Default, G: Serialize + DeserializeOwned + BuildStorage>
     Configuration<C, G> {
        /// Create default config for given chain spec.
        pub fn default_with_spec(chain_spec: ChainSpec<G>) -> Self {
            let mut configuration =
                Configuration{impl_name: "parity-substrate",
                              impl_version: "0.0.0",
                              impl_commit: "",
                              chain_spec,
                              name: Default::default(),
                              roles: Roles::FULL,
                              transaction_pool: Default::default(),
                              network: Default::default(),
                              keystore_path: Default::default(),
                              database_path: Default::default(),
                              database_cache_size: Default::default(),
                              state_cache_size: Default::default(),
                              keys: Default::default(),
                              custom: Default::default(),
                              pruning: PruningMode::default(),
                              execution_strategies: Default::default(),
                              rpc_http: None,
                              rpc_ws: None,
                              rpc_cors: Some(<[_]>::into_vec(box [])),
                              telemetry_endpoints: None,
                              default_heap_pages: None,
                              offchain_worker: Default::default(),
                              force_authoring: false,
                              disable_grandpa: false,
                              password: "".to_string(),};
            configuration.network.boot_nodes =
                configuration.chain_spec.boot_nodes().to_vec();
            configuration.telemetry_endpoints =
                configuration.chain_spec.telemetry_endpoints().clone();
            configuration
        }
        /// Returns full version string of this configuration.
        pub fn full_version(&self) -> String {
            full_version_from_strs(self.impl_version, self.impl_commit)
        }
        /// Implementation id and version.
        pub fn client_id(&self) -> String {
            ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["", "/v"],
                                                               &match (&self.impl_name,
                                                                       &self.full_version())
                                                                    {
                                                                    (arg0,
                                                                     arg1) =>
                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                 ::std::fmt::Display::fmt),
                                                                     ::std::fmt::ArgumentV1::new(arg1,
                                                                                                 ::std::fmt::Display::fmt)],
                                                                }))
        }
    }
    /// Returns platform info
    pub fn platform() -> String {
        let env = Target::env();
        let env_dash = if env.is_empty() { "" } else { "-" };
        ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["", "-", "", ""],
                                                           &match (&Target::arch(),
                                                                   &Target::os(),
                                                                   &env_dash,
                                                                   &env) {
                                                                (arg0, arg1,
                                                                 arg2, arg3)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg2,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg3,
                                                                                             ::std::fmt::Display::fmt)],
                                                            }))
    }
    /// Returns full version string, using supplied version and commit.
    pub fn full_version_from_strs(impl_version: &str, impl_commit: &str)
     -> String {
        let commit_dash = if impl_commit.is_empty() { "" } else { "-" };
        ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["", "", "", "-"],
                                                           &match (&impl_version,
                                                                   &commit_dash,
                                                                   &impl_commit,
                                                                   &platform())
                                                                {
                                                                (arg0, arg1,
                                                                 arg2, arg3)
                                                                =>
                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg2,
                                                                                             ::std::fmt::Display::fmt),
                                                                 ::std::fmt::ArgumentV1::new(arg3,
                                                                                             ::std::fmt::Display::fmt)],
                                                            }))
    }
}
pub mod chain_ops {
    //! Chain utilities.
    use std::{self, io::{Read, Write}};
    use futures::Future;
    use log::{info, warn};
    use runtime_primitives::generic::{SignedBlock, BlockId};
    use runtime_primitives::traits::{As, Block, Header, NumberFor};
    use consensus_common::import_queue::{ImportQueue, IncomingBlock, Link};
    use network::message;
    use consensus_common::BlockOrigin;
    use crate::components::{self, Components, ServiceFactory,
                            FactoryFullConfiguration, FactoryBlockNumber,
                            RuntimeGenesis};
    use crate::new_client;
    use parity_codec::{Decode, Encode};
    use crate::error;
    use crate::chain_spec::ChainSpec;
    /// Export a range of blocks to a binary stream.
    pub fn export_blocks<F, E,
                         W>(config: FactoryFullConfiguration<F>, exit: E,
                            mut output: W, from: FactoryBlockNumber<F>,
                            to: Option<FactoryBlockNumber<F>>, json: bool)
     -> error::Result<()> where F: ServiceFactory, E: Future<Item = (), Error
     = ()> + Send + 'static, W: Write {
        let client = new_client::<F>(&config)?;
        let mut block = from;
        let last =
            match to {
                Some(v) if v == As::sa(0) => As::sa(1),
                Some(v) => v,
                None => client.info()?.chain.best_number,
            };
        if last < block {
            return Err("Invalid block range specified".into());
        }
        let (exit_send, exit_recv) = std::sync::mpsc::channel();
        ::std::thread::spawn(move ||
                                 {
                                     let _ = exit.wait();
                                     let _ = exit_send.send(());
                                 });
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Exporting blocks from #",
                                                                         " to #"],
                                                                       &match (&block,
                                                                               &last)
                                                                            {
                                                                            (arg0,
                                                                             arg1)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                         ::std::fmt::Display::fmt),
                                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                                         ::std::fmt::Display::fmt)],
                                                                        }),
                                         lvl,
                                         &("substrate_service::chain_ops",
                                           "substrate_service::chain_ops",
                                           "core/service/src/chain_ops.rs",
                                           67u32));
            }
        };
        if !json {
            let last_: u64 = last.as_();
            let block_: u64 = block.as_();
            let len: u64 = last_ - block_ + 1;
            output.write(&len.encode())?;
        }
        loop  {
            if exit_recv.try_recv().is_ok() { break ; }
            match client.block(&BlockId::number(block))? {
                Some(block) => {
                    if json {
                        serde_json::to_writer(&mut output,
                                              &block).map_err(|e|
                                                                  ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Error writing JSON: "],
                                                                                                                     &match (&e,)
                                                                                                                          {
                                                                                                                          (arg0,)
                                                                                                                          =>
                                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                       ::std::fmt::Display::fmt)],
                                                                                                                      })))?;
                    } else { output.write(&block.encode())?; }
                }
                None => break ,
            }
            if block.as_() % 10000 == 0 {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["#"],
                                                                               &match (&block,)
                                                                                    {
                                                                                    (arg0,)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                }),
                                                 lvl,
                                                 &("substrate_service::chain_ops",
                                                   "substrate_service::chain_ops",
                                                   "core/service/src/chain_ops.rs",
                                                   91u32));
                    }
                };
            }
            if block == last { break ; }
            block += As::sa(1);
        }
        Ok(())
    }
    struct WaitLink {
        wait_send: std::sync::mpsc::Sender<()>,
    }
    impl WaitLink {
        fn new(wait_send: std::sync::mpsc::Sender<()>) -> WaitLink {
            WaitLink{wait_send,}
        }
    }
    impl <B: Block> Link<B> for WaitLink {
        fn block_imported(&self, _hash: &B::Hash, _number: NumberFor<B>) {
            self.wait_send.send(()).expect("Unable to notify main process; if the main process panicked then this thread would already be dead as well. qed.");
        }
    }
    /// Import blocks from a binary stream.
    pub fn import_blocks<F, E,
                         R>(mut config: FactoryFullConfiguration<F>, exit: E,
                            mut input: R) -> error::Result<()> where
     F: ServiceFactory, E: Future<Item = (), Error = ()> + Send + 'static,
     R: Read {
        let client = new_client::<F>(&config)?;
        let select_chain =
            components::FullComponents::<F>::build_select_chain(&mut config,
                                                                client.clone())?;
        let queue =
            components::FullComponents::<F>::build_import_queue(&mut config,
                                                                client.clone(),
                                                                select_chain)?;
        let (wait_send, wait_recv) = std::sync::mpsc::channel();
        let wait_link = WaitLink::new(wait_send);
        queue.start(Box::new(wait_link))?;
        let (exit_send, exit_recv) = std::sync::mpsc::channel();
        ::std::thread::spawn(move ||
                                 {
                                     let _ = exit.wait();
                                     let _ = exit_send.send(());
                                 });
        let count: u64 =
            Decode::decode(&mut input).ok_or("Error reading file")?;
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Importing ",
                                                                         " blocks"],
                                                                       &match (&count,)
                                                                            {
                                                                            (arg0,)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                         ::std::fmt::Display::fmt)],
                                                                        }),
                                         lvl,
                                         &("substrate_service::chain_ops",
                                           "substrate_service::chain_ops",
                                           "core/service/src/chain_ops.rs",
                                           144u32));
            }
        };
        let mut block_count = 0;
        for b in 0..count {
            if exit_recv.try_recv().is_ok() { break ; }
            if let Some(signed) = SignedBlock::<F::Block>::decode(&mut input)
                   {
                let (header, extrinsics) = signed.block.deconstruct();
                let hash = header.hash();
                let block =
                    message::BlockData::<F::Block>{hash: hash,
                                                   justification:
                                                       signed.justification,
                                                   header: Some(header),
                                                   body: Some(extrinsics),
                                                   receipt: None,
                                                   message_queue: None,};
                queue.import_blocks(BlockOrigin::File,
                                    <[_]>::into_vec(box
                                                        [IncomingBlock::<F::Block>{hash:
                                                                                       block.hash,
                                                                                   header:
                                                                                       block.header,
                                                                                   body:
                                                                                       block.body,
                                                                                   justification:
                                                                                       block.justification,
                                                                                   origin:
                                                                                       None,}]));
            } else {
                {
                    let lvl = ::log::Level::Warn;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Error reading block data at ",
                                                                                 "."],
                                                                               &match (&b,)
                                                                                    {
                                                                                    (arg0,)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                }),
                                                 lvl,
                                                 &("substrate_service::chain_ops",
                                                   "substrate_service::chain_ops",
                                                   "core/service/src/chain_ops.rs",
                                                   172u32));
                    }
                };
                break ;
            }
            block_count = b;
            if b % 1000 == 0 {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["#"],
                                                                               &match (&b,)
                                                                                    {
                                                                                    (arg0,)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                }),
                                                 lvl,
                                                 &("substrate_service::chain_ops",
                                                   "substrate_service::chain_ops",
                                                   "core/service/src/chain_ops.rs",
                                                   178u32));
                    }
                };
            }
        }
        let mut blocks_imported = 0;
        while blocks_imported < count {
            wait_recv.recv().expect("Importing thread has panicked. Then the main process will die before this can be reached. qed.");
            blocks_imported += 1;
        }
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Imported ",
                                                                         " blocks. Best: #"],
                                                                       &match (&block_count,
                                                                               &client.info()?.chain.best_number)
                                                                            {
                                                                            (arg0,
                                                                             arg1)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                         ::std::fmt::Display::fmt),
                                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                                         ::std::fmt::Display::fmt)],
                                                                        }),
                                         lvl,
                                         &("substrate_service::chain_ops",
                                           "substrate_service::chain_ops",
                                           "core/service/src/chain_ops.rs",
                                           189u32));
            }
        };
        Ok(())
    }
    /// Revert the chain.
    pub fn revert_chain<F>(config: FactoryFullConfiguration<F>,
                           blocks: FactoryBlockNumber<F>) -> error::Result<()>
     where F: ServiceFactory {
        let client = new_client::<F>(&config)?;
        let reverted = client.revert(blocks)?;
        let info = client.info()?.chain;
        if reverted.as_() == 0 {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["There aren\'t any non-finalized blocks to revert."],
                                                                           &match ()
                                                                                {
                                                                                ()
                                                                                =>
                                                                                [],
                                                                            }),
                                             lvl,
                                             &("substrate_service::chain_ops",
                                               "substrate_service::chain_ops",
                                               "core/service/src/chain_ops.rs",
                                               206u32));
                }
            };
        } else {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Reverted ",
                                                                             " blocks. Best: #",
                                                                             " (",
                                                                             ")"],
                                                                           &match (&reverted,
                                                                                   &info.best_number,
                                                                                   &info.best_hash)
                                                                                {
                                                                                (arg0,
                                                                                 arg1,
                                                                                 arg2)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                             ::std::fmt::Display::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg2,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }),
                                             lvl,
                                             &("substrate_service::chain_ops",
                                               "substrate_service::chain_ops",
                                               "core/service/src/chain_ops.rs",
                                               208u32));
                }
            };
        }
        Ok(())
    }
    /// Build a chain spec json
    pub fn build_spec<G>(spec: ChainSpec<G>, raw: bool)
     -> error::Result<String> where G: RuntimeGenesis {
        Ok(spec.to_json(raw)?)
    }
}
pub mod error {
    //! Errors that can occur during the service operation.
    #![allow(deprecated)]
    use client;
    use network;
    use keystore;
    use consensus_common;
    use error_chain::*;
    /// The Error type.
    ///
    /// This tuple struct is made of two elements:
    ///
    /// - an `ErrorKind` which is used to determine the type of the error.
    /// - An internal `State`, not meant for direct use outside of `error_chain`
    ///   internals, containing:
    ///   - a backtrace, generated when the error is created.
    ///   - an error chain, used for the implementation of `Error::cause()`.
    pub struct Error(
                     /// The kind of the error.
                     pub ErrorKind,
                     /// Contains the error chain and the backtrace.
                     #[doc(hidden)]
                     pub ::error_chain::State);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Error(ref __self_0_0, ref __self_0_1) => {
                    let mut debug_trait_builder = f.debug_tuple("Error");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    let _ = debug_trait_builder.field(&&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl ::error_chain::ChainedError for Error {
        type
        ErrorKind
        =
        ErrorKind;
        fn new(kind: ErrorKind, state: ::error_chain::State) -> Error {
            Error(kind, state)
        }
        fn from_kind(kind: Self::ErrorKind) -> Self { Self::from_kind(kind) }
        fn with_chain<E, K>(error: E, kind: K) -> Self where
         E: ::std::error::Error + Send + 'static, K: Into<Self::ErrorKind> {
            Self::with_chain(error, kind)
        }
        fn kind(&self) -> &Self::ErrorKind { self.kind() }
        fn iter(&self) -> ::error_chain::Iter {
            ::error_chain::Iter::new(Some(self))
        }
        fn chain_err<F, EK>(self, error: F) -> Self where F: FnOnce() -> EK,
         EK: Into<ErrorKind> {
            self.chain_err(error)
        }
        fn backtrace(&self) -> Option<&::error_chain::Backtrace> {
            self.backtrace()
        }
        #[allow(unknown_lints,
                renamed_and_removed_lints,
                unused_doc_comment,
                unused_doc_comments)]
        fn extract_backtrace(e: &(::std::error::Error + Send + 'static))
         -> Option<::error_chain::InternalBacktrace> {
            if let Some(e) = e.downcast_ref::<Error>() {
                return Some(e.1.backtrace.clone());
            }

            #[doc = "Consensus error"]
            {
                if let Some(e) = e.downcast_ref::<consensus_common::Error>() {
                    return Some(e.1.backtrace.clone());
                }
            }

            #[doc = "Network error"]
            {
                if let Some(e) = e.downcast_ref::<network::error::Error>() {
                    return Some(e.1.backtrace.clone());
                }
            }

            #[doc = "Keystore error"]
            {
                if let Some(e) = e.downcast_ref::<keystore::Error>() {
                    return Some(e.1.backtrace.clone());
                }
            }
            None
        }
    }
    #[allow(dead_code)]
    impl Error {
        /// Constructs an error from a kind, and generates a backtrace.
        pub fn from_kind(kind: ErrorKind) -> Error {
            Error(kind, ::error_chain::State::default())
        }
        /// Constructs a chained error from another error and a kind, and generates a backtrace.
        pub fn with_chain<E, K>(error: E, kind: K) -> Error where
         E: ::std::error::Error + Send + 'static, K: Into<ErrorKind> {
            Error::with_boxed_chain(Box::new(error), kind)
        }
        /// Construct a chained error from another boxed error and a kind, and generates a backtrace
        pub fn with_boxed_chain<K>(error: Box<::std::error::Error + Send>,
                                   kind: K) -> Error where
         K: Into<ErrorKind> {
            Error(kind.into(), ::error_chain::State::new::<Error>(error))
        }
        /// Returns the kind of the error.
        pub fn kind(&self) -> &ErrorKind { &self.0 }
        /// Iterates over the error chain.
        pub fn iter(&self) -> ::error_chain::Iter {
            ::error_chain::ChainedError::iter(self)
        }
        /// Returns the backtrace associated with this error.
        pub fn backtrace(&self) -> Option<&::error_chain::Backtrace> {
            self.1.backtrace()
        }
        /// Extends the error chain with a new entry.
        pub fn chain_err<F, EK>(self, error: F) -> Error where F: FnOnce() ->
         EK, EK: Into<ErrorKind> {
            Error::with_chain(self, Self::from_kind(error().into()))
        }
        /// A short description of the error.
        /// This method is identical to [`Error::description()`](https://doc.rust-lang.org/nightly/std/error/trait.Error.html#tymethod.description)
        pub fn description(&self) -> &str { self.0.description() }
    }
    impl ::std::error::Error for Error {
        fn description(&self) -> &str { self.description() }
        #[allow(unknown_lints,
                renamed_and_removed_lints,
                unused_doc_comment,
                unused_doc_comments)]
        fn cause(&self) -> Option<&::std::error::Error> {
            match self.1.next_error {
                Some(ref c) => Some(&**c),
                None => {
                    match self.0
                        {
                         #[doc = "Client error"]
                         ErrorKind::Client(ref foreign_err) => {
                             foreign_err.cause()
                         }
                          #[doc = "IO error"]
                          ErrorKind::Io(ref foreign_err) => {
                              foreign_err.cause()
                          }
                        _ => None,
                    }
                }
            }
        }
    }
    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    #[doc = "Consensus error"]
    impl From<consensus_common::Error> for Error {
        fn from(e: consensus_common::Error) -> Self {
            Error(ErrorKind::Consensus(e.0), e.1)
        }
    }
    #[doc = "Network error"]
    impl From<network::error::Error> for Error {
        fn from(e: network::error::Error) -> Self {
            Error(ErrorKind::Network(e.0), e.1)
        }
    }
    #[doc = "Keystore error"]
    impl From<keystore::Error> for Error {
        fn from(e: keystore::Error) -> Self {
            Error(ErrorKind::Keystore(e.0), e.1)
        }
    }
    #[doc = "Client error"]
    impl From<client::error::Error> for Error {
        fn from(e: client::error::Error) -> Self {
            Error::from_kind(ErrorKind::Client(e))
        }
    }
    #[doc = "IO error"]
    impl From<::std::io::Error> for Error {
        fn from(e: ::std::io::Error) -> Self {
            Error::from_kind(ErrorKind::Io(e))
        }
    }
    impl From<ErrorKind> for Error {
        fn from(e: ErrorKind) -> Self { Error::from_kind(e) }
    }
    impl <'a> From<&'a str> for Error {
        fn from(s: &'a str) -> Self { Error::from_kind(s.into()) }
    }
    impl From<String> for Error {
        fn from(s: String) -> Self { Error::from_kind(s.into()) }
    }
    #[doc = r" The kind of an error."]
    pub enum ErrorKind {

        #[doc = r" A convenient variant for String."]
        Msg(String),

        #[doc = "Consensus error"]
        Consensus(consensus_common::ErrorKind),

        #[doc = "Network error"]
        Network(network::error::ErrorKind),

        #[doc = "Keystore error"]
        Keystore(keystore::ErrorKind),

        #[doc = "Client error"]
        Client(client::error::Error),

        #[doc = "IO error"]
        Io(::std::io::Error),
        SelectChainRequired,

        #[doc(hidden)]
        __Nonexhaustive {
        },
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ErrorKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&ErrorKind::Msg(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Msg");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::Consensus(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Consensus");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::Network(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Network");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::Keystore(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Keystore");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::Client(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Client");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::Io(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Io");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::SelectChainRequired,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("SelectChainRequired");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::__Nonexhaustive {  },) => {
                    let mut debug_trait_builder =
                        f.debug_struct("__Nonexhaustive");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(unknown_lints,
            unused,
            renamed_and_removed_lints,
            unused_doc_comment,
            unused_doc_comments)]
    impl ::std::fmt::Display for ErrorKind {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self
                {
                 #[doc = r" A convenient variant for String."]
                 ErrorKind::Msg(ref s) => {
                     let display_fn =
                         |_, f: &mut ::std::fmt::Formatter|
                             {
                                 f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                           &match (&s,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }))
                             };
                     display_fn(self, fmt)
                 }
                  #[doc = "Consensus error"]
                  ErrorKind::Consensus(ref e) => {
                      let display_fn =
                          |_, f: &mut ::std::fmt::Formatter|
                              {
                                  f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                            &match (&e,)
                                                                                 {
                                                                                 (arg0,)
                                                                                 =>
                                                                                 [::std::fmt::ArgumentV1::new(arg0,
                                                                                                              ::std::fmt::Display::fmt)],
                                                                             }))
                              };
                      display_fn(self, fmt)
                  }
                   #[doc = "Network error"]
                   ErrorKind::Network(ref e) => {
                       let display_fn =
                           |_, f: &mut ::std::fmt::Formatter|
                               {
                                   f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                             &match (&e,)
                                                                                  {
                                                                                  (arg0,)
                                                                                  =>
                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                               ::std::fmt::Display::fmt)],
                                                                              }))
                               };
                       display_fn(self, fmt)
                   }
                    #[doc = "Keystore error"]
                    ErrorKind::Keystore(ref e) => {
                        let display_fn =
                            |_, f: &mut ::std::fmt::Formatter|
                                {
                                    f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                              &match (&e,)
                                                                                   {
                                                                                   (arg0,)
                                                                                   =>
                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                ::std::fmt::Display::fmt)],
                                                                               }))
                                };
                        display_fn(self, fmt)
                    }
                     #[doc = "Client error"]
                     ErrorKind::Client(ref err) => {
                         let display_fn =
                             |_, f: &mut ::std::fmt::Formatter|
                                 {
                                     f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                               &match (&err,)
                                                                                    {
                                                                                    (arg0,)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                }))
                                 };
                         display_fn(self, fmt)
                     }
                      #[doc = "IO error"]
                      ErrorKind::Io(ref err) => {
                          let display_fn =
                              |_, f: &mut ::std::fmt::Formatter|
                                  {
                                      f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                                &match (&err,)
                                                                                     {
                                                                                     (arg0,)
                                                                                     =>
                                                                                     [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                  ::std::fmt::Display::fmt)],
                                                                                 }))
                                  };
                          display_fn(self, fmt)
                      }
                ErrorKind::SelectChainRequired => {
                    let display_fn =
                        |_, f: &mut ::std::fmt::Formatter|
                            {
                                f.write_fmt(::std::fmt::Arguments::new_v1(&["Best chain selection strategy (SelectChain) is not provided."],
                                                                          &match ()
                                                                               {
                                                                               ()
                                                                               =>
                                                                               [],
                                                                           }))
                            };
                    display_fn(self, fmt)
                }
                _ => Ok(()),
            }
        }
    }
    #[allow(unknown_lints,
            unused,
            renamed_and_removed_lints,
            unused_doc_comment,
            unused_doc_comments)]
    impl ErrorKind {
        /// A string describing the error kind.
        pub fn description(&self) -> &str {
            match *self
                {
                 #[doc = r" A convenient variant for String."]
                 ErrorKind::Msg(ref s) => {
                     &s
                 }
                  #[doc = "Consensus error"]
                  ErrorKind::Consensus(ref e) => {
                      e.description()
                  }
                   #[doc = "Network error"]
                   ErrorKind::Network(ref e) => {
                       e.description()
                   }
                    #[doc = "Keystore error"]
                    ErrorKind::Keystore(ref e) => {
                        e.description()
                    }
                     #[doc = "Client error"]
                     ErrorKind::Client(ref err) => {
                         ::std::error::Error::description(err)
                     }
                      #[doc = "IO error"]
                      ErrorKind::Io(ref err) => {
                          ::std::error::Error::description(err)
                      }
                ErrorKind::SelectChainRequired => {
                    "Best chain selection strategy (SelectChain) must be provided when starting full node or authority."
                }
                _ => "",
            }
        }
    }
    #[doc = "Consensus error"]
    impl From<consensus_common::ErrorKind> for ErrorKind {
        fn from(e: consensus_common::ErrorKind) -> Self {
            ErrorKind::Consensus(e)
        }
    }
    #[doc = "Network error"]
    impl From<network::error::ErrorKind> for ErrorKind {
        fn from(e: network::error::ErrorKind) -> Self {
            ErrorKind::Network(e)
        }
    }
    #[doc = "Keystore error"]
    impl From<keystore::ErrorKind> for ErrorKind {
        fn from(e: keystore::ErrorKind) -> Self { ErrorKind::Keystore(e) }
    }
    impl <'a> From<&'a str> for ErrorKind {
        fn from(s: &'a str) -> Self { ErrorKind::Msg(s.to_string()) }
    }
    impl From<String> for ErrorKind {
        fn from(s: String) -> Self { ErrorKind::Msg(s) }
    }
    impl From<Error> for ErrorKind {
        fn from(e: Error) -> Self { e.0 }
    }
    /// Additional methods for `Result`, for easy interaction with this crate.
    pub trait ResultExt<T> {
        /// If the `Result` is an `Err` then `chain_err` evaluates the closure,
        /// which returns *some type that can be converted to `ErrorKind`*, boxes
        /// the original error to store as the cause, then returns a new error
        /// containing the original error.
        fn chain_err<F, EK>(self, callback: F)
        -> ::std::result::Result<T, Error>
        where
        F: FnOnce()
        ->
        EK,
        EK: Into<ErrorKind>;
    }
    impl <T, E> ResultExt<T> for ::std::result::Result<T, E> where
     E: ::std::error::Error + Send + 'static {
        fn chain_err<F, EK>(self, callback: F)
         -> ::std::result::Result<T, Error> where F: FnOnce() -> EK,
         EK: Into<ErrorKind> {
            self.map_err(move |e|
                             {
                                 let state =
                                     ::error_chain::State::new::<Error>(Box::new(e));
                                 ::error_chain::ChainedError::new(callback().into(),
                                                                  state)
                             })
        }
    }
    impl <T> ResultExt<T> for ::std::option::Option<T> {
        fn chain_err<F, EK>(self, callback: F)
         -> ::std::result::Result<T, Error> where F: FnOnce() -> EK,
         EK: Into<ErrorKind> {
            self.ok_or_else(move ||
                                {
                                    ::error_chain::ChainedError::from_kind(callback().into())
                                })
        }
    }
    /// Convenient wrapper around `std::Result`.
    #[allow(unused)]
    pub type Result<T> = ::std::result::Result<T, Error>;
}
use std::io;
use std::net::SocketAddr;
use std::collections::HashMap;
use futures::sync::mpsc;
use parking_lot::Mutex;
use client::BlockchainEvents;
use exit_future::Signal;
use futures::prelude::*;
use inherents::pool::InherentsPool;
use keystore::Store as Keystore;
use log::{info, warn, debug};
use parity_codec::{Encode, Decode};
use primitives::Pair;
use runtime_primitives::generic::BlockId;
use runtime_primitives::traits::{Header, As};
use substrate_executor::NativeExecutor;
use tel::{telemetry, SUBSTRATE_INFO};
pub use self::error::{ErrorKind, Error};
pub use config::{Configuration, Roles, PruningMode};
pub use chain_spec::{ChainSpec, Properties};
pub use transaction_pool::txpool::{self, Pool as TransactionPool, Options as
                                   TransactionPoolOptions, ChainApi,
                                   IntoPoolError};
use client::runtime_api::BlockT;
pub use client::FinalityNotifications;
pub use components::{ServiceFactory, FullBackend, FullExecutor, LightBackend,
                     LightExecutor, Components, PoolApi, ComponentClient,
                     ComponentBlock, FullClient, LightClient, FullComponents,
                     LightComponents, CodeExecutor, NetworkService,
                     FactoryChainSpec, FactoryBlock, FactoryFullConfiguration,
                     RuntimeGenesis, FactoryGenesis, ComponentExHash,
                     ComponentExtrinsic, FactoryExtrinsic};
use components::{StartRPC, MaintainTransactionPool, OffchainWorker};
#[doc(hidden)]
pub use std::{ops::Deref, result::Result, sync::Arc};
#[doc(hidden)]
pub use network::{FinalityProofProvider, OnDemand};
#[doc(hidden)]
pub use tokio::runtime::TaskExecutor;
const DEFAULT_PROTOCOL_ID: &str = "sup";
/// Substrate service.
pub struct Service<Components: components::Components> {
    client: Arc<ComponentClient<Components>>,
    select_chain: Option<<Components as components::Components>::SelectChain>,
    network: Option<Arc<components::NetworkService<Components::Factory>>>,
    transaction_pool: Arc<TransactionPool<Components::TransactionPoolApi>>,
    inherents_pool: Arc<InherentsPool<ComponentExtrinsic<Components>>>,
    keystore: Keystore,
    exit: ::exit_future::Exit,
    signal: Option<Signal>,
    /// Configuration of this Service
    pub config: FactoryFullConfiguration<Components::Factory>,
    _rpc: Box<::std::any::Any + Send + Sync>,
    _telemetry: Option<Arc<tel::Telemetry>>,
    _offchain_workers: Option<Arc<offchain::OffchainWorkers<ComponentClient<Components>,
                                                            ComponentBlock<Components>>>>,
    _telemetry_on_connect_sinks: Arc<Mutex<Vec<mpsc::UnboundedSender<()>>>>,
}
/// Creates bare client without any networking.
pub fn new_client<Factory: components::ServiceFactory>(config:
                                                           &FactoryFullConfiguration<Factory>)
 ->
     Result<Arc<ComponentClient<components::FullComponents<Factory>>>,
            error::Error> {
    let executor = NativeExecutor::new(config.default_heap_pages);
    let (client, _) =
        components::FullComponents::<Factory>::build_client(config,
                                                            executor)?;
    Ok(client)
}
/// Stream of events for connection established to a telemetry server.
pub type TelemetryOnConnectNotifications = mpsc::UnboundedReceiver<()>;
/// Used to hook on telemetry connection established events.
pub struct TelemetryOnConnect<'a> {
    /// Handle to a future that will resolve on exit.
    pub on_exit: Box<Future<Item = (), Error = ()> + Send + 'static>,
    /// Event stream.
    pub telemetry_connection_sinks: TelemetryOnConnectNotifications,
    /// Executor to which the hook is spawned.
    pub executor: &'a TaskExecutor,
}
impl <Components: components::Components> Service<Components> {
    /// Get event stream for telemetry connection established events.
    pub fn telemetry_on_connect_stream(&self)
     -> TelemetryOnConnectNotifications {
        let (sink, stream) = mpsc::unbounded();
        self._telemetry_on_connect_sinks.lock().push(sink);
        stream
    }
    /// Creates a new service.
    pub fn new(mut config: FactoryFullConfiguration<Components::Factory>,
               task_executor: TaskExecutor) -> Result<Self, error::Error> {
        let (signal, exit) = ::exit_future::signal();
        let executor = NativeExecutor::new(config.default_heap_pages);
        let mut keystore =
            Keystore::open(config.keystore_path.as_str().into())?;
        for seed in &config.keys { keystore.generate_from_seed(seed)?; }
        let public_key =
            match keystore.contents()?.get(0) {
                Some(public_key) => public_key.clone(),
                None => {
                    let key = keystore.generate(&config.password)?;
                    let public_key = key.public();
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                               lvl <= ::log::max_level() {
                            ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Generated a new keypair: "],
                                                                                   &match (&public_key,)
                                                                                        {
                                                                                        (arg0,)
                                                                                        =>
                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                     ::std::fmt::Debug::fmt)],
                                                                                    }),
                                                     lvl,
                                                     &("substrate_service",
                                                       "substrate_service",
                                                       "core/service/src/lib.rs",
                                                       147u32));
                        }
                    };
                    public_key
                }
            };
        let (client, on_demand) =
            Components::build_client(&config, executor)?;
        let select_chain =
            Components::build_select_chain(&mut config, client.clone())?;
        let import_queue =
            Box::new(Components::build_import_queue(&mut config,
                                                    client.clone(),
                                                    select_chain.clone())?);
        let finality_proof_provider =
            Components::build_finality_proof_provider(client.clone())?;
        let chain_info = client.info()?.chain;
        let version = config.full_version();
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Highest known block at #"],
                                                                       &match (&chain_info.best_number,)
                                                                            {
                                                                            (arg0,)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                         ::std::fmt::Display::fmt)],
                                                                        }),
                                         lvl,
                                         &("substrate_service",
                                           "substrate_service",
                                           "core/service/src/lib.rs",
                                           164u32));
            }
        };
        ::substrate_telemetry::with_logger(|l|
                                               {
                                                   if ::slog::Level::Info.as_usize()
                                                          <=
                                                          ::slog::__slog_static_max_level().as_usize()
                                                      {
                                                       l.log(&{
                                                                  static RS:
                                                                         ::slog::RecordStatic<'static>
                                                                         =
                                                                      {
                                                                          static LOC:
                                                                                 ::slog::RecordLocation
                                                                                 =
                                                                              ::slog::RecordLocation{file:
                                                                                                         "core/service/src/lib.rs",
                                                                                                     line:
                                                                                                         165u32,
                                                                                                     column:
                                                                                                         3u32,
                                                                                                     function:
                                                                                                         "",
                                                                                                     module:
                                                                                                         "substrate_service",};
                                                                          ::slog::RecordStatic{location:
                                                                                                   &LOC,
                                                                                               level:
                                                                                                   ::slog::Level::Info,
                                                                                               tag:
                                                                                                   SUBSTRATE_INFO,}
                                                                      };
                                                                  ::slog::Record::new(&RS,
                                                                                      &::std::fmt::Arguments::new_v1(&["node.start"],
                                                                                                                     &match ()
                                                                                                                          {
                                                                                                                          ()
                                                                                                                          =>
                                                                                                                          [],
                                                                                                                      }),
                                                                                      ::slog::BorrowedKV(&(::slog::SingleKV::from(("best",
                                                                                                                                   ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                 &match (&chain_info.best_hash,)
                                                                                                                                                                      {
                                                                                                                                                                      (arg0,)
                                                                                                                                                                      =>
                                                                                                                                                                      [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                   ::std::fmt::Debug::fmt)],
                                                                                                                                                                  }))),
                                                                                                           (::slog::SingleKV::from(("height",
                                                                                                                                    chain_info.best_number.as_())),
                                                                                                            ()))))
                                                              })
                                                   }
                                               });
        let network_protocol =
            <Components::Factory>::build_network_protocol(&config)?;
        let transaction_pool =
            Arc::new(Components::build_transaction_pool(config.transaction_pool.clone(),
                                                        client.clone())?);
        let transaction_pool_adapter =
            Arc::new(TransactionPoolAdapter::<Components>{imports_external_transactions:
                                                              !config.roles.is_light(),
                                                          pool:
                                                              transaction_pool.clone(),
                                                          client:
                                                              client.clone(),});
        let network_params =
            network::config::Params{config:
                                        network::config::ProtocolConfig{roles:
                                                                            config.roles,},
                                    network_config: config.network.clone(),
                                    chain: client.clone(),
                                    finality_proof_provider,
                                    on_demand:
                                        on_demand.as_ref().map(|d|
                                                                   d.clone()
                                                                       as _),
                                    transaction_pool:
                                        transaction_pool_adapter.clone() as _,
                                    specialization: network_protocol,};
        let protocol_id =
            {
                let protocol_id_full =
                    match config.chain_spec.protocol_id() {
                        Some(pid) => pid,
                        None => {
                            {
                                let lvl = ::log::Level::Warn;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Using default protocol ID ",
                                                                                             " because none is configured in the chain specs"],
                                                                                           &match (&DEFAULT_PROTOCOL_ID,)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                            }),
                                                             lvl,
                                                             &("substrate_service",
                                                               "substrate_service",
                                                               "core/service/src/lib.rs",
                                                               191u32));
                                }
                            };
                            DEFAULT_PROTOCOL_ID
                        }
                    }.as_bytes();
                network::ProtocolId::from(protocol_id_full)
            };
        let has_bootnodes =
            !network_params.network_config.boot_nodes.is_empty();
        let network =
            network::Service::new(network_params, protocol_id, import_queue)?;
        if let Some(on_demand) = on_demand.as_ref() {
            on_demand.set_network_interface(Box::new(Arc::downgrade(&network)));
        }
        let inherents_pool = Arc::new(InherentsPool::default());
        let offchain_workers =
            if config.offchain_worker {
                Some(Arc::new(offchain::OffchainWorkers::new(client.clone(),
                                                             inherents_pool.clone(),
                                                             task_executor.clone())))
            } else { None };
        {
            let network = Arc::downgrade(&network);
            let txpool = Arc::downgrade(&transaction_pool);
            let wclient = Arc::downgrade(&client);
            let offchain = offchain_workers.as_ref().map(Arc::downgrade);
            let events =
                client.import_notification_stream().for_each(move
                                                                 |notification|
                                                                 {
                                                                     let number =
                                                                         *notification.header.number();
                                                                     if let Some(network)
                                                                            =
                                                                            network.upgrade()
                                                                            {
                                                                         network.on_block_imported(notification.hash,
                                                                                                   notification.header);
                                                                     }
                                                                     if let (Some(txpool),
                                                                             Some(client))
                                                                            =
                                                                            (txpool.upgrade(),
                                                                             wclient.upgrade())
                                                                            {
                                                                         Components::RuntimeServices::maintain_transaction_pool(&BlockId::hash(notification.hash),
                                                                                                                                &*client,
                                                                                                                                &*txpool).map_err(|e|
                                                                                                                                                      {
                                                                                                                                                          let lvl =
                                                                                                                                                              ::log::Level::Warn;
                                                                                                                                                          if lvl
                                                                                                                                                                 <=
                                                                                                                                                                 ::log::STATIC_MAX_LEVEL
                                                                                                                                                                 &&
                                                                                                                                                                 lvl
                                                                                                                                                                     <=
                                                                                                                                                                     ::log::max_level()
                                                                                                                                                             {
                                                                                                                                                              ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Pool error processing new block: "],
                                                                                                                                                                                                                     &match (&e,)
                                                                                                                                                                                                                          {
                                                                                                                                                                                                                          (arg0,)
                                                                                                                                                                                                                          =>
                                                                                                                                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                                       ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                                      }),
                                                                                                                                                                                       lvl,
                                                                                                                                                                                       &("substrate_service",
                                                                                                                                                                                         "substrate_service",
                                                                                                                                                                                         "core/service/src/lib.rs",
                                                                                                                                                                                         237u32));
                                                                                                                                                          }
                                                                                                                                                      })?;
                                                                     }
                                                                     if let (Some(txpool),
                                                                             Some(offchain))
                                                                            =
                                                                            (txpool.upgrade(),
                                                                             offchain.as_ref().and_then(|o|
                                                                                                            o.upgrade()))
                                                                            {
                                                                         Components::RuntimeServices::offchain_workers(&number,
                                                                                                                       &offchain,
                                                                                                                       &txpool).map_err(|e|
                                                                                                                                            {
                                                                                                                                                let lvl =
                                                                                                                                                    ::log::Level::Warn;
                                                                                                                                                if lvl
                                                                                                                                                       <=
                                                                                                                                                       ::log::STATIC_MAX_LEVEL
                                                                                                                                                       &&
                                                                                                                                                       lvl
                                                                                                                                                           <=
                                                                                                                                                           ::log::max_level()
                                                                                                                                                   {
                                                                                                                                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Offchain workers error processing new block: "],
                                                                                                                                                                                                           &match (&e,)
                                                                                                                                                                                                                {
                                                                                                                                                                                                                (arg0,)
                                                                                                                                                                                                                =>
                                                                                                                                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                            }),
                                                                                                                                                                             lvl,
                                                                                                                                                                             &("substrate_service",
                                                                                                                                                                               "substrate_service",
                                                                                                                                                                               "core/service/src/lib.rs",
                                                                                                                                                                               245u32));
                                                                                                                                                }
                                                                                                                                            })?;
                                                                     }
                                                                     Ok(())
                                                                 }).select(exit.clone()).then(|_|
                                                                                                  Ok(()));
            task_executor.spawn(events);
        }
        {
            let network = Arc::downgrade(&network);
            struct MostRecentNotification<B: BlockT>(futures::stream::Fuse<FinalityNotifications<B>>);
            impl <B: BlockT> Stream for MostRecentNotification<B> {
                type
                Item
                =
                <FinalityNotifications<B> as Stream>::Item;
                type
                Error
                =
                <FinalityNotifications<B> as Stream>::Error;
                fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
                    let mut last = None;
                    let last =
                        loop  {
                            match self.0.poll()? {
                                Async::Ready(Some(item)) => {
                                    last = Some(item)
                                }
                                Async::Ready(None) =>
                                match last {
                                    None => return Ok(Async::Ready(None)),
                                    Some(last) => break last ,
                                },
                                Async::NotReady =>
                                match last {
                                    None => return Ok(Async::NotReady),
                                    Some(last) => break last ,
                                },
                            }
                        };
                    Ok(Async::Ready(Some(last)))
                }
            }
            let events =
                MostRecentNotification(client.finality_notification_stream().fuse()).for_each(move
                                                                                                  |notification|
                                                                                                  {
                                                                                                      if let Some(network)
                                                                                                             =
                                                                                                             network.upgrade()
                                                                                                             {
                                                                                                          network.on_block_finalized(notification.hash,
                                                                                                                                     notification.header);
                                                                                                      }
                                                                                                      Ok(())
                                                                                                  }).select(exit.clone()).then(|_|
                                                                                                                                   Ok(()));
            task_executor.spawn(events);
        }
        {
            let network = Arc::downgrade(&network);
            let events =
                transaction_pool.import_notification_stream().for_each(move
                                                                           |_|
                                                                           {
                                                                               if let Some(network)
                                                                                      =
                                                                                      network.upgrade()
                                                                                      {
                                                                                   network.trigger_repropagate();
                                                                               }
                                                                               Ok(())
                                                                           }).select(exit.clone()).then(|_|
                                                                                                            Ok(()));
            task_executor.spawn(events);
        }
        let system_info =
            rpc::apis::system::SystemInfo{chain_name:
                                              config.chain_spec.name().into(),
                                          impl_name: config.impl_name.into(),
                                          impl_version:
                                              config.impl_version.into(),
                                          properties:
                                              config.chain_spec.properties(),};
        let rpc =
            Components::RuntimeServices::start_rpc(client.clone(),
                                                   network.clone(),
                                                   has_bootnodes, system_info,
                                                   config.rpc_http,
                                                   config.rpc_ws,
                                                   config.rpc_cors.clone(),
                                                   task_executor.clone(),
                                                   transaction_pool.clone())?;
        let telemetry_connection_sinks:
                Arc<Mutex<Vec<mpsc::UnboundedSender<()>>>> =
            Default::default();
        let telemetry =
            config.telemetry_endpoints.clone().map(|endpoints|
                                                       {
                                                           let is_authority =
                                                               config.roles ==
                                                                   Roles::AUTHORITY;
                                                           let network_id =
                                                               network.local_peer_id().to_base58();
                                                           let pubkey =
                                                               ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                  &match (&public_key,)
                                                                                                                       {
                                                                                                                       (arg0,)
                                                                                                                       =>
                                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                    ::std::fmt::Display::fmt)],
                                                                                                                   }));
                                                           let name =
                                                               config.name.clone();
                                                           let impl_name =
                                                               config.impl_name.to_owned();
                                                           let version =
                                                               version.clone();
                                                           let chain_name =
                                                               config.chain_spec.name().to_owned();
                                                           let telemetry_connection_sinks_ =
                                                               telemetry_connection_sinks.clone();
                                                           Arc::new(tel::init_telemetry(tel::TelemetryConfig{endpoints,
                                                                                                             on_connect:
                                                                                                                 Box::new(move
                                                                                                                              ||
                                                                                                                              {
                                                                                                                                  ::substrate_telemetry::with_logger(|l|
                                                                                                                                                                         {
                                                                                                                                                                             if ::slog::Level::Info.as_usize()
                                                                                                                                                                                    <=
                                                                                                                                                                                    ::slog::__slog_static_max_level().as_usize()
                                                                                                                                                                                {
                                                                                                                                                                                 l.log(&{
                                                                                                                                                                                            static RS:
                                                                                                                                                                                                   ::slog::RecordStatic<'static>
                                                                                                                                                                                                   =
                                                                                                                                                                                                {
                                                                                                                                                                                                    static LOC:
                                                                                                                                                                                                           ::slog::RecordLocation
                                                                                                                                                                                                           =
                                                                                                                                                                                                        ::slog::RecordLocation{file:
                                                                                                                                                                                                                                   "core/service/src/lib.rs",
                                                                                                                                                                                                                               line:
                                                                                                                                                                                                                                   345u32,
                                                                                                                                                                                                                               column:
                                                                                                                                                                                                                                   6u32,
                                                                                                                                                                                                                               function:
                                                                                                                                                                                                                                   "",
                                                                                                                                                                                                                               module:
                                                                                                                                                                                                                                   "substrate_service",};
                                                                                                                                                                                                    ::slog::RecordStatic{location:
                                                                                                                                                                                                                             &LOC,
                                                                                                                                                                                                                         level:
                                                                                                                                                                                                                             ::slog::Level::Info,
                                                                                                                                                                                                                         tag:
                                                                                                                                                                                                                             SUBSTRATE_INFO,}
                                                                                                                                                                                                };
                                                                                                                                                                                            ::slog::Record::new(&RS,
                                                                                                                                                                                                                &::std::fmt::Arguments::new_v1(&["system.connected"],
                                                                                                                                                                                                                                               &match ()
                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                    ()
                                                                                                                                                                                                                                                    =>
                                                                                                                                                                                                                                                    [],
                                                                                                                                                                                                                                                }),
                                                                                                                                                                                                                ::slog::BorrowedKV(&(::slog::SingleKV::from(("network_id",
                                                                                                                                                                                                                                                             network_id.clone())),
                                                                                                                                                                                                                                     (::slog::SingleKV::from(("authority",
                                                                                                                                                                                                                                                              is_authority)),
                                                                                                                                                                                                                                      (::slog::SingleKV::from(("pubkey",
                                                                                                                                                                                                                                                               &pubkey)),
                                                                                                                                                                                                                                       (::slog::SingleKV::from(("chain",
                                                                                                                                                                                                                                                                chain_name.clone())),
                                                                                                                                                                                                                                        (::slog::SingleKV::from(("config",
                                                                                                                                                                                                                                                                 "")),
                                                                                                                                                                                                                                         (::slog::SingleKV::from(("version",
                                                                                                                                                                                                                                                                  version.clone())),
                                                                                                                                                                                                                                          (::slog::SingleKV::from(("implementation",
                                                                                                                                                                                                                                                                   impl_name.clone())),
                                                                                                                                                                                                                                           (::slog::SingleKV::from(("name",
                                                                                                                                                                                                                                                                    name.clone())),
                                                                                                                                                                                                                                            ()))))))))))
                                                                                                                                                                                        })
                                                                                                                                                                             }
                                                                                                                                                                         });
                                                                                                                                  telemetry_connection_sinks_.lock().retain(|sink|
                                                                                                                                                                                {
                                                                                                                                                                                    sink.unbounded_send(()).is_ok()
                                                                                                                                                                                });
                                                                                                                              }),}))
                                                       });
        Ok(Service{client,
                   network: Some(network),
                   select_chain,
                   transaction_pool,
                   inherents_pool,
                   signal: Some(signal),
                   keystore,
                   config,
                   exit,
                   _rpc: Box::new(rpc),
                   _telemetry: telemetry,
                   _offchain_workers: offchain_workers,
                   _telemetry_on_connect_sinks:
                       telemetry_connection_sinks.clone(),})
    }
    /// give the authority key, if we are an authority and have a key
    pub fn authority_key(&self) -> Option<primitives::ed25519::Pair> {
        if self.config.roles != Roles::AUTHORITY { return None }
        let keystore = &self.keystore;
        if let Ok(Some(Ok(key))) =
               keystore.contents().map(|keys|
                                           keys.get(0).map(|k|
                                                               keystore.load(k,
                                                                             &self.config.password)))
               {
            Some(key)
        } else { None }
    }
    /// return a shared instance of Telemetry (if enabled)
    pub fn telemetry(&self) -> Option<Arc<tel::Telemetry>> {
        self._telemetry.as_ref().map(|t| t.clone())
    }
}
impl <Components> Service<Components> where Components: components::Components
 {
    /// Get shared client instance.
    pub fn client(&self) -> Arc<ComponentClient<Components>> {
        self.client.clone()
    }
    /// Get clone of select chain.
    pub fn select_chain(&self)
     -> Option<<Components as components::Components>::SelectChain> {
        self.select_chain.clone()
    }
    /// Get shared network instance.
    pub fn network(&self)
     -> Arc<components::NetworkService<Components::Factory>> {
        self.network.as_ref().expect("self.network always Some").clone()
    }
    /// Get shared transaction pool instance.
    pub fn transaction_pool(&self)
     -> Arc<TransactionPool<Components::TransactionPoolApi>> {
        self.transaction_pool.clone()
    }
    /// Get shared inherents pool instance.
    pub fn inherents_pool(&self)
     -> Arc<InherentsPool<ComponentExtrinsic<Components>>> {
        self.inherents_pool.clone()
    }
    /// Get shared keystore.
    pub fn keystore(&self) -> &Keystore { &self.keystore }
    /// Get a handle to a future that will resolve on exit.
    pub fn on_exit(&self) -> ::exit_future::Exit { self.exit.clone() }
}
impl <Components> Drop for Service<Components> where
 Components: components::Components {
    fn drop(&mut self) {
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Substrate service shutdown"],
                                                                       &match ()
                                                                            {
                                                                            ()
                                                                            =>
                                                                            [],
                                                                        }),
                                         lvl,
                                         &("service", "substrate_service",
                                           "core/service/src/lib.rs",
                                           439u32));
            }
        };
        drop(self.network.take());
        if let Some(signal) = self.signal.take() { signal.fire(); }
    }
}
fn maybe_start_server<T, F>(address: Option<SocketAddr>, start: F)
 -> Result<Option<T>, io::Error> where F: Fn(&SocketAddr) ->
 Result<T, io::Error> {
    Ok(match address {
           Some(mut address) =>
           Some(start(&address).or_else(|e|
                                            match e.kind() {
                                                io::ErrorKind::AddrInUse |
                                                io::ErrorKind::PermissionDenied
                                                => {
                                                    {
                                                        let lvl =
                                                            ::log::Level::Warn;
                                                        if lvl <=
                                                               ::log::STATIC_MAX_LEVEL
                                                               &&
                                                               lvl <=
                                                                   ::log::max_level()
                                                           {
                                                            ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Unable to bind server to ",
                                                                                                                     ". Trying random port."],
                                                                                                                   &match (&address,)
                                                                                                                        {
                                                                                                                        (arg0,)
                                                                                                                        =>
                                                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                     ::std::fmt::Display::fmt)],
                                                                                                                    }),
                                                                                     lvl,
                                                                                     &("substrate_service",
                                                                                       "substrate_service",
                                                                                       "core/service/src/lib.rs",
                                                                                       457u32));
                                                        }
                                                    };
                                                    address.set_port(0);
                                                    start(&address)
                                                }
                                                _ => Err(e),
                                            })?),
           None => None,
       })
}
/// Transaction pool adapter.
pub struct TransactionPoolAdapter<C: Components> {
    imports_external_transactions: bool,
    pool: Arc<TransactionPool<C::TransactionPoolApi>>,
    client: Arc<ComponentClient<C>>,
}
impl <C: Components> TransactionPoolAdapter<C> {
    fn best_block_id(&self) -> Option<BlockId<ComponentBlock<C>>> {
        self.client.info().map(|info|
                                   BlockId::hash(info.chain.best_hash)).map_err(|e|
                                                                                    {
                                                                                        {
                                                                                            let lvl =
                                                                                                ::log::Level::Debug;
                                                                                            if lvl
                                                                                                   <=
                                                                                                   ::log::STATIC_MAX_LEVEL
                                                                                                   &&
                                                                                                   lvl
                                                                                                       <=
                                                                                                       ::log::max_level()
                                                                                               {
                                                                                                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Error getting best block: "],
                                                                                                                                                       &match (&e,)
                                                                                                                                                            {
                                                                                                                                                            (arg0,)
                                                                                                                                                            =>
                                                                                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                         ::std::fmt::Debug::fmt)],
                                                                                                                                                        }),
                                                                                                                         lvl,
                                                                                                                         &("substrate_service",
                                                                                                                           "substrate_service",
                                                                                                                           "core/service/src/lib.rs",
                                                                                                                           479u32));
                                                                                            }
                                                                                        };
                                                                                    }).ok()
    }
}
impl <C: Components>
 network::TransactionPool<ComponentExHash<C>, ComponentBlock<C>> for
 TransactionPoolAdapter<C> where
 <C as components::Components>::RuntimeApi: Send + Sync {
    fn transactions(&self)
     -> Vec<(ComponentExHash<C>, ComponentExtrinsic<C>)> {
        self.pool.ready().map(|t|
                                  {
                                      let hash = t.hash.clone();
                                      let ex: ComponentExtrinsic<C> =
                                          t.data.clone();
                                      (hash, ex)
                                  }).collect()
    }
    fn import(&self, transaction: &ComponentExtrinsic<C>)
     -> Option<ComponentExHash<C>> {
        if !self.imports_external_transactions {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Transaction rejected"],
                                                                           &match ()
                                                                                {
                                                                                ()
                                                                                =>
                                                                                [],
                                                                            }),
                                             lvl,
                                             &("substrate_service",
                                               "substrate_service",
                                               "core/service/src/lib.rs",
                                               500u32));
                }
            };
            return None;
        }
        let encoded = transaction.encode();
        if let Some(uxt) = Decode::decode(&mut &encoded[..]) {
            let best_block_id = self.best_block_id()?;
            match self.pool.submit_one(&best_block_id, uxt) {
                Ok(hash) => Some(hash),
                Err(e) =>
                match e.into_pool_error() {
                    Ok(txpool::error::Error(txpool::error::ErrorKind::AlreadyImported(hash),
                                            _)) => {
                        hash.downcast::<ComponentExHash<C>>().ok().map(|x|
                                                                           x.as_ref().clone())
                    }
                    Ok(e) => {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL &&
                                   lvl <= ::log::max_level() {
                                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Error adding transaction to the pool: "],
                                                                                       &match (&e,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Debug::fmt)],
                                                                                        }),
                                                         lvl,
                                                         &("substrate_service",
                                                           "substrate_service",
                                                           "core/service/src/lib.rs",
                                                           515u32));
                            }
                        };
                        None
                    }
                    Err(e) => {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL &&
                                   lvl <= ::log::max_level() {
                                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Error converting pool error: "],
                                                                                       &match (&e,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Debug::fmt)],
                                                                                        }),
                                                         lvl,
                                                         &("substrate_service",
                                                           "substrate_service",
                                                           "core/service/src/lib.rs",
                                                           519u32));
                            }
                        };
                        None
                    }
                },
            }
        } else {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Error decoding transaction"],
                                                                           &match ()
                                                                                {
                                                                                ()
                                                                                =>
                                                                                [],
                                                                            }),
                                             lvl,
                                             &("substrate_service",
                                               "substrate_service",
                                               "core/service/src/lib.rs",
                                               525u32));
                }
            };
            None
        }
    }
    fn on_broadcasted(&self,
                      propagations:
                          HashMap<ComponentExHash<C>, Vec<String>>) {
        self.pool.on_broadcasted(propagations)
    }
}
/// Constructs a service factory with the given name that implements the `ServiceFactory` trait.
/// The required parameters are required to be given in the exact order. Some parameters are followed
/// by `{}` blocks. These blocks are required and used to initialize the given parameter.
/// In these block it is required to write a closure that takes the same number of arguments,
/// the corresponding function in the `ServiceFactory` trait provides.
///
/// # Example
///
/// ```
/// # use substrate_service::{
/// # 	construct_service_factory, Service, FullBackend, FullExecutor, LightBackend, LightExecutor,
/// # 	FullComponents, LightComponents, FactoryFullConfiguration, FullClient, TaskExecutor
/// # };
/// # use transaction_pool::{self, txpool::{Pool as TransactionPool}};
/// # use network::construct_simple_protocol;
/// # use client::{self, LongestChain};
/// # use primitives::{Pair as PairT, ed25519};
/// # use consensus_common::import_queue::{BasicQueue, Verifier};
/// # use consensus_common::{BlockOrigin, ImportBlock};
/// # use node_runtime::{GenesisConfig, RuntimeApi};
/// # use std::sync::Arc;
/// # use node_primitives::Block;
/// # use runtime_primitives::Justification;
/// # use runtime_primitives::traits::{AuthorityIdFor, Block as BlockT};
/// # use grandpa;
/// # construct_simple_protocol! {
/// # 	pub struct NodeProtocol where Block = Block { }
/// # }
/// # struct MyVerifier;
/// # impl<B: BlockT> Verifier<B> for MyVerifier {
/// # 	fn verify(
/// # 		&self,
/// # 		origin: BlockOrigin,
/// # 		header: B::Header,
/// # 		justification: Option<Justification>,
/// # 		body: Option<Vec<B::Extrinsic>>,
/// # 	) -> Result<(ImportBlock<B>, Option<Vec<AuthorityIdFor<B>>>), String> {
/// # 		unimplemented!();
/// # 	}
/// # }
/// type FullChainApi<T> = transaction_pool::ChainApi<
/// 	client::Client<FullBackend<T>, FullExecutor<T>, Block, RuntimeApi>, Block>;
/// type LightChainApi<T> = transaction_pool::ChainApi<
/// 	client::Client<LightBackend<T>, LightExecutor<T>, Block, RuntimeApi>, Block>;
///
/// construct_service_factory! {
/// 	struct Factory {
/// 		// Declare the block type
/// 		Block = Block,
/// 		RuntimeApi = RuntimeApi,
/// 		// Declare the network protocol and give an initializer.
/// 		NetworkProtocol = NodeProtocol { |config| Ok(NodeProtocol::new()) },
/// 		RuntimeDispatch = node_executor::Executor,
/// 		FullTransactionPoolApi = FullChainApi<Self>
/// 			{ |config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client))) },
/// 		LightTransactionPoolApi = LightChainApi<Self>
/// 			{ |config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client))) },
/// 		Genesis = GenesisConfig,
/// 		Configuration = (),
/// 		FullService = FullComponents<Self>
/// 			{ |config, executor| <FullComponents<Factory>>::new(config, executor) },
/// 		// Setup as Consensus Authority (if the role and key are given)
/// 		AuthoritySetup = {
/// 			|service: Self::FullService, executor: TaskExecutor, key: Option<Arc<ed25519::Pair>>| {
/// 				Ok(service)
/// 			}},
/// 		LightService = LightComponents<Self>
/// 			{ |config, executor| <LightComponents<Factory>>::new(config, executor) },
/// 		FullImportQueue = BasicQueue<Block>
/// 			{ |_, client, _| Ok(BasicQueue::new(Arc::new(MyVerifier), client, None, None, None)) },
/// 		LightImportQueue = BasicQueue<Block>
/// 			{ |_, client| Ok(BasicQueue::new(Arc::new(MyVerifier), client, None, None, None)) },
/// 		SelectChain = LongestChain<FullBackend<Self>, Self::Block>
/// 			{ |config: &FactoryFullConfiguration<Self>, client: Arc<FullClient<Self>>| {
/// 				Ok(LongestChain::new(client.backend().clone(), client.import_lock()))
/// 			}},
/// 		FinalityProofProvider = { |client: Arc<FullClient<Self>>| {
/// 				Ok(Some(Arc::new(grandpa::FinalityProofProvider::new(client.clone(), client)) as _))
/// 			}},
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! construct_service_factory((
                                       $ ( # [ $ attr : meta ] ) * struct $
                                       name : ident {
                                       Block = $ block : ty , RuntimeApi = $
                                       runtime_api : ty , NetworkProtocol = $
                                       protocol : ty {
                                       $ ( $ protocol_init : tt ) * } ,
                                       RuntimeDispatch = $ dispatch : ty ,
                                       FullTransactionPoolApi = $
                                       full_transaction : ty {
                                       $ ( $ full_transaction_init : tt ) * }
                                       , LightTransactionPoolApi = $
                                       light_transaction : ty {
                                       $ ( $ light_transaction_init : tt ) * }
                                       , Genesis = $ genesis : ty ,
                                       Configuration = $ config : ty ,
                                       FullService = $ full_service : ty {
                                       $ ( $ full_service_init : tt ) * } ,
                                       AuthoritySetup = {
                                       $ ( $ authority_setup : tt ) * } ,
                                       LightService = $ light_service : ty {
                                       $ ( $ light_service_init : tt ) * } ,
                                       FullImportQueue = $ full_import_queue :
                                       ty {
                                       $ ( $ full_import_queue_init : tt ) * }
                                       , LightImportQueue = $
                                       light_import_queue : ty {
                                       $ ( $ light_import_queue_init : tt ) *
                                       } , SelectChain = $ select_chain : ty {
                                       $ ( $ select_chain_init : tt ) * } ,
                                       FinalityProofProvider = {
                                       $ ( $ finality_proof_provider_init : tt
                                       ) * } , } ) => {
                                       $ ( # [ $ attr ] ) * pub struct $ name
                                       {  } # [ allow ( unused_variables ) ]
                                       impl $ crate :: ServiceFactory for $
                                       name {
                                       type Block = $ block ; type RuntimeApi
                                       = $ runtime_api ; type NetworkProtocol
                                       = $ protocol ; type RuntimeDispatch = $
                                       dispatch ; type FullTransactionPoolApi
                                       = $ full_transaction ; type
                                       LightTransactionPoolApi = $
                                       light_transaction ; type Genesis = $
                                       genesis ; type Configuration = $ config
                                       ; type FullService = $ full_service ;
                                       type LightService = $ light_service ;
                                       type FullImportQueue = $
                                       full_import_queue ; type
                                       LightImportQueue = $ light_import_queue
                                       ; type SelectChain = $ select_chain ;
                                       fn build_full_transaction_pool (
                                       config : $ crate ::
                                       TransactionPoolOptions , client : $
                                       crate :: Arc < $ crate :: FullClient <
                                       Self >> ) -> $ crate :: Result < $
                                       crate :: TransactionPool < Self ::
                                       FullTransactionPoolApi > , $ crate ::
                                       Error > {
                                       ( $ ( $ full_transaction_init ) * ) (
                                       config , client ) } fn
                                       build_light_transaction_pool (
                                       config : $ crate ::
                                       TransactionPoolOptions , client : $
                                       crate :: Arc < $ crate :: LightClient <
                                       Self >> ) -> $ crate :: Result < $
                                       crate :: TransactionPool < Self ::
                                       LightTransactionPoolApi > , $ crate ::
                                       Error > {
                                       ( $ ( $ light_transaction_init ) * ) (
                                       config , client ) } fn
                                       build_network_protocol (
                                       config : & $ crate ::
                                       FactoryFullConfiguration < Self > ) ->
                                       $ crate :: Result < Self ::
                                       NetworkProtocol , $ crate :: Error > {
                                       ( $ ( $ protocol_init ) * ) ( config )
                                       } fn build_select_chain (
                                       config : & mut $ crate ::
                                       FactoryFullConfiguration < Self > ,
                                       client : Arc < $ crate :: FullClient <
                                       Self >> ) -> $ crate :: Result < Self
                                       :: SelectChain , $ crate :: Error > {
                                       ( $ ( $ select_chain_init ) * ) (
                                       config , client ) } fn
                                       build_full_import_queue (
                                       config : & mut $ crate ::
                                       FactoryFullConfiguration < Self > ,
                                       client : $ crate :: Arc < $ crate ::
                                       FullClient < Self >> , select_chain :
                                       Self :: SelectChain ) -> $ crate ::
                                       Result < Self :: FullImportQueue , $
                                       crate :: Error > {
                                       ( $ ( $ full_import_queue_init ) * ) (
                                       config , client , select_chain ) } fn
                                       build_light_import_queue (
                                       config : & mut FactoryFullConfiguration
                                       < Self > , client : Arc < $ crate ::
                                       LightClient < Self >> , ) -> Result <
                                       Self :: LightImportQueue , $ crate ::
                                       Error > {
                                       ( $ ( $ light_import_queue_init ) * ) (
                                       config , client ) } fn
                                       build_finality_proof_provider (
                                       client : Arc < $ crate :: FullClient <
                                       Self >> ) -> Result < Option < Arc < $
                                       crate :: FinalityProofProvider < Self
                                       :: Block >> > , $ crate :: Error > {
                                       (
                                       $ ( $ finality_proof_provider_init ) *
                                       ) ( client ) } fn new_light (
                                       config : $ crate ::
                                       FactoryFullConfiguration < Self > ,
                                       executor : $ crate :: TaskExecutor ) ->
                                       $ crate :: Result < Self ::
                                       LightService , $ crate :: Error > {
                                       ( $ ( $ light_service_init ) * ) (
                                       config , executor ) } fn new_full (
                                       config : $ crate ::
                                       FactoryFullConfiguration < Self > ,
                                       executor : $ crate :: TaskExecutor , )
                                       -> Result < Self :: FullService , $
                                       crate :: Error > {
                                       ( $ ( $ full_service_init ) * ) (
                                       config , executor . clone (  ) ) .
                                       and_then (
                                       | service | {
                                       let key = ( & service ) . authority_key
                                       (  ) . map ( Arc :: new ) ; (
                                       $ ( $ authority_setup ) * ) (
                                       service , executor , key ) } ) } } });
