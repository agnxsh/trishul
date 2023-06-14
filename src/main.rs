use clap::Parser;
use trishul::{Contract::Contract, queryeth, flag};

#[derive(Parser)]
#[command(author, version, about, longg_about = None)]
#[command(propagate_version = true)]

struct Clitool {
    #[command(subcommand)]
    dispatcher: sub :: Commands
}

fn main (){
    let cli = Clitool::parse();
    let rt = tokio::runtime::Runtime::new().unwrap();

    match &cli.dispatcher {
        sub::Commands::Find {
            rpc_url,
            contract_address,
        } => {
            println! (
                "rpc_url provided {}, Contract Address {}",
                rpc_url, contract_address
            );

            let bytecode = rt
                .block_on(queryeth::get_opcode(rpc_url, contract_address))
                .unwrap()
        }
    }
}