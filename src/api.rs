use anyhow::{anyhow, Result};
use shared_crypto::intent::{Intent, IntentMessage};
use sui_sdk::rpc_types::{Balance, SuiTransactionBlockResponseOptions};
use sui_sdk::{SuiClient, SuiClientBuilder};
use sui_types::base_types::SuiAddress;
use sui_types::crypto::{Signature, SuiKeyPair};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::transaction::{Transaction, TransactionData};
/// @Name api
///
/// @Date 2024/11/25 下午2:54
///
/// @Author Matrix.Ye
///
/// @Description:

pub struct ApiEngine {
    rpc_url: String,
    sender: SuiAddress,
    keypair: SuiKeyPair,
    client: SuiClient,
}


impl ApiEngine {
    /// 构造RPC连接引擎
    ///
    /// # Arguments
    ///
    /// * `rpc_url`: 连接url，必须携带443端口号
    /// * `wallet_private`: 钱包私钥
    ///
    /// returns: Result<ApiEngine, Error>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub async fn new(
        rpc_url: String,
        wallet_private: String,
    ) -> Result<Self> {
        // 构造 Sui 客户端
        let client = SuiClientBuilder::default()
            .build(rpc_url.clone())
            .await?;


        // 转换 Wallet 私钥
        let keypair = SuiKeyPair::decode(&wallet_private).map_err(|_| anyhow!("Invalid wallet private key"))?;
        // 转换 Wallet 地址
        let sender: SuiAddress = SuiAddress::from(&keypair.public());

        Ok(Self { rpc_url, sender, keypair, client })
    }
    // 获取Sender
    pub fn get_sender(&self) -> SuiAddress {
        self.sender
    }

    // 获取钱包余额
    pub async fn get_balance(&self, coin_type: Option<String>) -> Result<Balance> {
        self.client.coin_read_api().get_balance(self.sender, coin_type).await.map_err(|_| anyhow!("get balance of sender failed!"))
    }


    ///
    /// 进行SUI代币的转账
    /// # Arguments
    ///
    /// * `recipient_address`: 接收地址，字符串
    /// * `amount`: 转账数量,小数，精确到9位，如1表示1个sui，实际数量为1_000_000_000
    /// * `gas_budget`:gas 消耗，默认5_000_000
    ///
    /// returns: Result<(), Error>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub async fn trans_sui(&self, recipient_address: String, amount: f64, gas_budget: Option<u64>, gas_price: Option<u64>) -> Result<()> {
        // 转换目标地址
        let recipient: SuiAddress = recipient_address
            .parse()
            .map_err(|_| anyhow!("Invalid recipient address"))?;

        // 获取 Gas 对象
        let gas_coin = self
            .client
            .coin_read_api()
            .get_coins(self.sender, None, None, None)
            .await?
            .data
            .into_iter()
            .next()
            .ok_or(anyhow!("No coins available for gas payment"))?;

        // 获取 Gas 价格
        let gas_budget = gas_budget.unwrap_or(5_000_000);
        let gas_price = if gas_price.is_none() { self.client.read_api().get_reference_gas_price().await? } else { 9999999 };
        let scaled_amount = (amount * 1e9).round() as u64;

        // 构造交易数据
        let tx_data = TransactionData::new_transfer_sui(
            recipient,
            self.sender,
            Some(scaled_amount),
            gas_coin.object_ref(),
            gas_budget,
            gas_price,
        );

        // 构造 IntentMessage 并签名
        let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
        let signature = Signature::new_secure(&intent_msg, &self.keypair);

        // 构造交易
        let tx = Transaction::from_data(tx_data, vec![signature]);

        // 发送交易
        let response = self
            .client
            .quorum_driver_api()
            .execute_transaction_block(
                tx,
                SuiTransactionBlockResponseOptions {
                    show_effects: true,
                    ..Default::default()
                },
                Some(ExecuteTransactionRequestType::WaitForEffectsCert),
            )
            .await?;

        // 打印交易执行结果
        println!("Transaction executed successfully!");
        println!("Transaction Digest: {:?}", response.digest);
        println!("Effects: {:?}", response.effects);

        Ok(())
    }
}