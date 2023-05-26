use std::str::FromStr;

use ethers:: {
    providers::{Http, Middleware, Provider, ProviderError},
    types::{Address, BlockId, BlockId, BlockNumber, Bytes},
}

pub async fn get_opcode(rpc_url; &str. contract_address: &str)-> Result<Bytes, ProviderError>{
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();

    let bytecode = match provider 
        .get_opcode(
            Address::from_str(contract_address).unwrap(),
            Some(BlockId::Number(BlockNumber::Latest)),

        )
        .await
    {
        Ok(data) => {
            data
        }
        Err(err) => return Err(err),

    };
    Ok(bytecode)
}
