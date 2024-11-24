//这个例子展示了几种连接到Sui网络的基本方法。
//有几个内置的方法连接到
//支持devnet、tesnet和localnet（本地运行）；
//以及连接到自定义url的自定义方式。
//打印出不同网络的API版本，
//最后，输出可用RPC方法的列表
//订阅列表。
//注意，如果没有Sui网络，运行此代码将失败
//本地运行，默认地址：127.0.0.1:9000
/// 本地：http://127.0.0.1:9000
/// 开发网：https://fullnode.devnet.sui.io:443
/// 测试网：https://fullnode.testnet.sui.io:443
/// 主网：https://fullnode.mainnet.sui.io:443
use sui_sdk::SuiClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 连接主网
    let rpc_uir = env!("SUI_RPC_URL");
    let sui_mainnet = SuiClientBuilder::default().build(rpc_uir).await?;
    println!("Sui mainnet version: {}", sui_mainnet.api_version());

    // 连接测试网
    let test_client = SuiClientBuilder::default().build_testnet().await?;
    println!("Sui TestNet version:{}", test_client.api_version());

    // 连接开发网
    let dev_client = SuiClientBuilder::default().build_devnet().await?;
    println!("Sui dev Net version:{}", dev_client.api_version());

    Ok(())
}
