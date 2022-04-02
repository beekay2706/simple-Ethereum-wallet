use anyhow::Result;
mod wallet;

Const URL: &str = "https://eth-ropsten.alchemyapi.io/v2/VHf-pRM84TyOtKCJL7ey2Yq5lyhUN_lN"
fn main()->  Result<()>{
    
  let keypair: Result<(Publickey, Secretkey)> = wallet::create_keypair();
  let web3: Result<Web3<Http>, Error> = wallet::establish_web3_connection(url: URL)?;
  Ok(());
}
