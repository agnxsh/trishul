use clap::Parser;
use trishul::{contract_handler::Contract, queryeth, flag};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

struct Clitool {
    #[command(subcommand)]
    dispatcher: flag :: Commands
}

fn main (){
    let cli = Clitool::parse();
    let rt = tokio::runtime::Runtime::new().unwrap();

    match &cli.dispatcher {
        flag::Commands::Search {
            rpc_url,
            contract_address,
        } => {
            println! (
                "rpc_url provided {}, Contract Address {}",
                rpc_url, contract_address
            );

            let bytecode = rt
                .block_on(queryeth::get_opcode(rpc_url, contract_address))
                .unwrap();

            let contract = Contract{bytecode};
            let pattern: Vec<u8> = vec![0x80,0x63,0x14,0x61,0x57];
            contract.get_smart_contract_dispatcher(pattern);
        }
    }
}