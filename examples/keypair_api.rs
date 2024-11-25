use anyhow::{anyhow, Result};
use sui_types::base_types::SuiAddress;
use sui_types::crypto::{get_key_pair, EncodeDecodeBase64, SuiKeyPair};


// 从私钥中导入keypair
fn import_keypair_from_private_key(private_key_str: &str) -> Result<SuiKeyPair> {
    if private_key_str.starts_with("suiprivkey") {
        // Bech32 编码
        SuiKeyPair::decode(private_key_str).map_err(|_| anyhow!("Import Keypair ERROR: private_key_str Invalid Bech32"))
    } else {
        //base64 编码
        SuiKeyPair::decode_base64(private_key_str).map_err(|_| anyhow!("Import Keypair ERROR: private_key_str Invalid base64"))
    }
}

// 生成随机的keypair
fn generate_random_keypair() -> Result<SuiKeyPair> {
    let kp = SuiKeyPair::Ed25519(get_key_pair().1);
    Ok(kp)
}


#[tokio::main]
async fn main() -> Result<()> {
    let private_key = env!("SUI_WALLET_PRIVATE");
    println!("{:?}", private_key);
    let keypair = import_keypair_from_private_key(private_key);
    match keypair {
        Ok(kp) => {
            let address = SuiAddress::from(&kp.public()).to_string();
            println!("SuiAddress: {:?}", address);
        }
        Err(e) => {
            println!("ERROR: {:?}", e.to_string());
        }
    }

    let keypair = generate_random_keypair();
    println!("SuiAddress: {:?}", SuiAddress::from(&keypair?.public()));
    Ok(())
}