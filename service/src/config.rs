// Copyright 2023-, GraphOps and Semiotic Labs.
// SPDX-License-Identifier: Apache-2.0

use clap::{command, Args, Parser, ValueEnum};

use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

use crate::{query_processor::QueryError, util::init_tracing};

#[derive(Clone, Debug, Parser, Serialize, Deserialize, Default)]
#[clap(
    name = "indexer-service",
    about = "Indexer service on top of graph node",
    author = "hopeyen"
)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(flatten)]
    pub ethereum: Ethereum,
    #[command(flatten)]
    pub indexer_infrastructure: IndexerInfrastructure,
    #[command(flatten)]
    pub postgres: Postgres,
    #[command(flatten)]
    pub network_subgraph: NetworkSubgraph,
    #[command(flatten)]
    pub escrow_subgraph: EscrowSubgraph,

    #[arg(
        short,
        value_name = "config",
        env = "CONFIG",
        help = "Indexer service configuration file (YAML format)"
    )]
    config: Option<String>,
}

#[derive(Clone, Debug, Args, Serialize, Deserialize, Default)]
#[group(required = true, multiple = true)]
pub struct Ethereum {
    #[clap(
        long,
        value_name = "ethereum-node-provider",
        env = "ETH_NODE",
        help = "Ethereum node or provider URL"
    )]
    pub ethereum: String,
    #[clap(
        long,
        value_name = "ethereum-polling-interval",
        env = "ETHEREUM_POLLING_INTERVAL",
        default_value_t = 4000,
        help = "Polling interval for the Ethereum provider (ms)"
    )]
    pub ethereum_polling_interval: usize,
    #[clap(
        long,
        value_name = "mnemonic",
        env = "MNEMONIC",
        help = "Mnemonic for the operator wallet"
    )]
    pub mnemonic: String,
    #[clap(
        long,
        value_name = "indexer-address",
        env = "INDEXER_ADDRESS",
        help = "Ethereum address of the indexer"
    )]
    pub indexer_address: Address,
}

#[derive(Clone, Debug, Args, Serialize, Deserialize, Default)]
#[group(required = true, multiple = true)]
pub struct IndexerInfrastructure {
    #[clap(
        long,
        value_name = "port",
        env = "PORT",
        default_value_t = 7600,
        help = "Port to serve queries at"
    )]
    pub port: u32,
    #[clap(
        long,
        value_name = "metrics-port",
        env = "METRICS_PORT",
        default_value_t = 7300,
        help = "Port to serve Prometheus metrics at"
    )]
    pub metrics_port: u16,
    #[clap(
        long,
        value_name = "graph-node-query-endpoint",
        env = "GRAPH_NODE_QUERY_ENDPOINT",
        default_value_t = String::from("http://0.0.0.0:8000"),
        help = "Graph node GraphQL HTTP service endpoint"
    )]
    pub graph_node_query_endpoint: String,
    #[clap(
        long,
        value_name = "graph-node-status-endpoint",
        env = "GRAPH_NODE_STATUS_ENDPOINT",
        default_value_t = String::from("http://0.0.0.0:8030"),
        help = "Graph node endpoint for the index node server"
    )]
    pub graph_node_status_endpoint: String,
    #[clap(
        long,
        value_name = "log-level",
        env = "LOG_LEVEL",
        value_enum,
        help = "Log level in RUST_LOG format"
    )]
    pub log_level: Option<String>,
    #[clap(
        long,
        value_name = "gcloud-profiling",
        env = "GCLOUD_PROFILING",
        default_value_t = false,
        help = "Whether to enable Google Cloud profiling"
    )]
    pub gcloud_profiling: bool,
    #[clap(
        long,
        value_name = "free-query-auth-token",
        env = "FREE_QUERY_AUTH_TOKEN",
        help = "Auth token that clients can use to query for free"
    )]
    pub free_query_auth_token: Option<String>,
}

#[derive(Clone, Debug, Args, Serialize, Deserialize, Default)]
#[group(required = true, multiple = true)]
pub struct Postgres {
    #[clap(
        long,
        value_name = "postgres-host",
        env = "POSTGRES_HOST",
        default_value_t = String::from("http://0.0.0.0/"),
        help = "Postgres host"
    )]
    pub postgres_host: String,
    #[clap(
        long,
        value_name = "postgres-port",
        env = "POSTGRES_PORT",
        default_value_t = 5432,
        help = "Postgres port"
    )]
    pub postgres_port: usize,
    #[clap(
        long,
        value_name = "postgres-database",
        env = "POSTGRES_DATABASE",
        help = "Postgres database name"
    )]
    pub postgres_database: String,
    #[clap(
        long,
        value_name = "postgres-username",
        env = "POSTGRES_USERNAME",
        default_value_t = String::from("postgres"),
        help = "Postgres username"
    )]
    pub postgres_username: String,
    #[clap(
        long,
        value_name = "postgres-password",
        env = "POSTGRES_PASSWORD",
        default_value_t = String::from(""),
        help = "Postgres password"
    )]
    pub postgres_password: String,
}

#[derive(Clone, Debug, Args, Serialize, Deserialize, Default)]
#[group(required = true, multiple = true)]
pub struct NetworkSubgraph {
    #[clap(
        long,
        value_name = "network-subgraph-deployment",
        env = "NETWORK_SUBGRAPH_DEPLOYMENT",
        help = "Network subgraph deployment"
    )]
    pub network_subgraph_deployment: Option<String>,
    #[clap(
        long,
        value_name = "network-subgraph-endpoint",
        env = "NETWORK_SUBGRAPH_ENDPOINT",
        default_value_t = String::from("https://api.thegraph.com/subgraphs/name/graphprotocol/graph-network-goerli"),
        help = "Endpoint to query the network subgraph from"
    )]
    pub network_subgraph_endpoint: String,
    #[clap(
        long,
        value_name = "network-subgraph-auth-token",
        env = "NETWORK_SUBGRAPH_AUTH_TOKEN",
        help = "Bearer token to require for /network queries"
    )]
    pub network_subgraph_auth_token: Option<String>,
    #[clap(
        long,
        value_name = "serve-network-subgraph",
        env = "SERVE_NETWORK_SUBGRAPH",
        default_value_t = false,
        help = "Whether to serve the network subgraph at /network"
    )]
    pub serve_network_subgraph: bool,
    #[clap(
        long,
        value_name = "allocation-syncing-interval",
        env = "ALLOCATION_SYNCING_INTERVAL",
        default_value_t = 120_000,
        help = "Interval (in ms) for syncing indexer allocations from the network"
    )]
    pub allocation_syncing_interval: u64,
    #[clap(
        long,
        value_name = "client-signer-address",
        env = "CLIENT_SIGNER_ADDRESS",
        help = "Address that signs query fee receipts from a known client"
    )]
    pub client_signer_address: Option<String>,
}

#[derive(Clone, Debug, Args, Serialize, Deserialize, Default)]
#[group(required = true, multiple = true)]
pub struct EscrowSubgraph {
    #[clap(
        long,
        value_name = "escrow-subgraph-deployment",
        env = "ESCROW_SUBGRAPH_DEPLOYMENT",
        help = "Escrow subgraph deployment"
    )]
    pub escrow_subgraph_deployment: String,
    // TODO:
    //
    // #[clap(
    //     long,
    //     value_name = "escrow-subgraph-endpoint",
    //     env = "ESCROW_SUBGRAPH_ENDPOINT",
    //     // TODO:
    //     // default_value_t = String::from("https://api.thegraph.com/subgraphs/name/?????????????"),
    //     help = "Endpoint to query the network subgraph from"
    // )]
    // pub escrow_subgraph_endpoint: Option<String>,
    // #[clap(
    //     long,
    //     value_name = "escrow-subgraph-auth-token",
    //     env = "ESCROW_SUBGRAPH_AUTH_TOKEN",
    //     help = "Bearer token to require for /network queries"
    // )]
    // pub escrow_subgraph_auth_token: Option<String>,
    // #[clap(
    //     long,
    //     value_name = "serve-escrow-subgraph",
    //     env = "SERVE_ESCROW_SUBGRAPH",
    //     default_value_t = false,
    //     help = "Whether to serve the escrow subgraph at /escrow"
    // )]
    // pub serve_escrow_subgraph: bool,
    // #[clap(
    //     long,
    //     value_name = "escrow-syncing-interval",
    //     env = "ESCROW_SYNCING_INTERVAL",
    //     default_value_t = 120_000,
    //     help = "Interval (in ms) for syncing indexer escrow accounts from the escrow subgraph"
    // )]
    pub escrow_syncing_interval: u64,
}

impl Cli {
    /// Parse config arguments
    /// If environmental variable for config is set to a valid config file path, then parse from config
    /// Otherwise parse from command line arguments
    pub fn args() -> Self {
        let cli = if let Ok(file_path) = std::env::var("config") {
            confy::load_path::<Cli>(file_path.clone())
                .unwrap_or_else(|_| panic!("Parse config file at {}", file_path.clone()))
        } else {
            Cli::parse()
            // Potentially store it for the user
            // let _ = confy::store_path("./args.toml", cli.clone());
        };

        // Enables tracing under RUST_LOG variable
        if let Some(log_setting) = &cli.indexer_infrastructure.log_level {
            std::env::set_var("RUST_LOG", log_setting);
        };
        // add a LogFormat to config
        init_tracing("pretty".to_string()).expect("Could not set up global default subscriber for logger, check environmental variable `RUST_LOG` or the CLI input `log-level`");
        cli
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Validate the input: {0}")]
    ValidateInput(String),
    #[error("Generate JSON representation of the config file: {0}")]
    GenerateJson(serde_json::Error),
    #[error("QueryError: {0}")]
    QueryError(QueryError),
    #[error("Toml file error: {0}")]
    ReadStr(std::io::Error),
    #[error("Unknown error: {0}")]
    Other(anyhow::Error),
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize, Default,
)]
pub enum LogLevel {
    Trace,
    #[default]
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
