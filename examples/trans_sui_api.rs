use anyhow::{anyhow, Result};
use shared_crypto::intent::{Intent, IntentMessage};
use std::str::FromStr;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::SuiClientBuilder;

use sui_types::base_types::SuiAddress;
use sui_types::crypto::{Signature, SuiKeyPair};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::transaction::{Transaction, TransactionData};

/// @Name sign_api
///
/// @Date 2024/11/25 上午10:50
///
/// @Author Matrix.Ye
///
/// @Description: 用于演示，如何将Sui代币从A地址转移到B地址

#[tokio::main]
async fn main() -> Result<()> {
    println!("{:?}", "this is test api");
    let rpc_url = env!("SUI_RPC_URL");
    let wallet_address = env!("SUI_WALLET_ADDRESS");
    let wallet_private = env!("SUI_WALLET_PRIVATE");
    let recipient_address_str = env!("SUI_RECIPIENT_ADDRESS"); // Wallet B 的地址

    let sui_client = SuiClientBuilder::default().build(rpc_url).await?;
    let keypair = SuiKeyPair::decode(wallet_private).unwrap();
    // 发送人地址
    let sender = SuiAddress::from_str(wallet_address)?;
    // 接受人地址
    let recipient: SuiAddress = recipient_address_str.parse()?;

    println!("Sender: {:?}", sender);
    println!("Recipient: {:?}", recipient);

    // 获取 Gas 支付对象
    let gas_coin = sui_client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?
        .data
        .into_iter()
        .next()
        .ok_or(anyhow!("No coins available for gas payment"))?;

    println!("GasCoin{:?}", gas_coin);

    // 构造转账交易
    let amount: u64 = 1_000_000_000; // 转移 0.001 SUI
    let gas_budget = 5_000_000; // 预估 Gas
    let gas_price = sui_client.read_api().get_reference_gas_price().await?;

    //构造交易数据
    let tx_data = TransactionData::new_transfer_sui(
        recipient,
        sender,
        Some(amount),
        gas_coin.object_ref(),
        gas_budget,
        gas_price,
    );
    println!("TxData: {:?}", tx_data);

    // 构造Intent
    let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
    // 生成签名
    let signature = Signature::new_secure(&intent_msg, &keypair);
    println!("signature: {:?}", signature);
    // 生成签名交易
    let tx = Transaction::from_data(tx_data, vec![signature]);
    println!("Tx: {:?}", tx);

    //发送交易
    let response = sui_client
        .quorum_driver_api()
        .execute_transaction_block(
            tx,
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForEffectsCert),
        )
        .await?;
    // 打印交易执行结果
    println!("Transaction executed successfully!");
    println!("Transaction Digest: {:?}", response.digest);
    println!("effect: {:?}", response.effects);

    Ok(())
}