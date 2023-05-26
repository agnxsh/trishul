use clap::Parser;
use trishul::{Contract::Contract, queryeth, flag};

#[derive(Parser)]
#[command(author, version, about, longg_about = None)]
#[command(propagate_version = true)]

struct Clitool {

}

fn main (){
    let cli = Clitool::parse();
    let rt = tokio::runtime::Runtime::new().unwrap();
}