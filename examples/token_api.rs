// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
/// 演示如何获取Coin的信息，以及
///
///
///
///
use futures::{future, stream::StreamExt};
use std::str::FromStr;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::SuiClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // rpc节点URL
    let rpc_url = env!("SUI_RPC_URL").to_string();
    let wallet_address = env!("SUI_WALLET_ADDRESS").to_string();
    println!("{:?}", rpc_url);
    println!("{:?}", wallet_address);
    // 构建sui client
    let sui_client = SuiClientBuilder::default().build(rpc_url).await?;
    // 指定代币类型，例如sui
    let coin_type = "0x2::sui::SUI".to_string();
    // 获取代币元数据
    let coin_metadata = sui_client
        .coin_read_api()
        .get_coin_metadata(coin_type.clone())
        .await?
        .unwrap();
    println!("id {:?}", coin_metadata.id);
    println!("name {:?}", coin_metadata.name);
    println!("symbol {:?}", coin_metadata.symbol);
    println!("decimals {:?}", coin_metadata.decimals);
    println!("icon_url {:?}", coin_metadata.icon_url);
    println!("description {:?}", coin_metadata.description);
    // 获取代币的总供应量
    let supply = sui_client
        .coin_read_api()
        .get_total_supply(coin_type.clone())
        .await?;
    println!("coin type= {} total_supply = {:?}", coin_type, supply.value);

    // 分页的方式，获取用户的代币持有量
    let addr = SuiAddress::from_str(wallet_address.as_str())?;
    let coins_stream = sui_client.coin_read_api().get_coins_stream(addr, None);

    println!(" *** Coins Stream ***");
    coins_stream
        .for_each(|coin| {
            println!("{:?}", coin);
            future::ready(())
        })
        .await;
    println!(" *** Coins Stream ***\n");

    // 获取指定用户的指定Coin的余额统计信息
    let coin_type = "0x2::sui::SUI".to_string();
    let balance = sui_client.coin_read_api().get_balance(addr, Some(coin_type)).await?;
    println!("balance: {:?}", balance);
    // 获取用户的全部Coin信息
    let all_balances = sui_client.coin_read_api().get_all_balances(addr).await?;
    print!("all_balances: {:?}", all_balances);

    Ok(())
}
