use clap::Subcommand
/// Clap is a cli-handling tool for rust, you can find the crate
/// @ https://crates.io/crates/clap

#[derive(Subcommand)]
pub enum Commands {
    /// Search subcommand returns all the public/external functions of the given contract
    Search {
        /// Url to connect with an rpc
        #[clap(short = 'r', long = "rpc-url")]
        rpc_url: Box<str>,

        #[clap(short = 'c', long = "contract-address")]
        contract_address: Box<str>,
    },

}