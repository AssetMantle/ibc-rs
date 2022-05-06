/*!
   Methods for performing IBC token transfer on a chain.
*/

use ibc::core::ics24_host::identifier::{ChannelId, PortId};

use crate::error::Error;
use crate::ibc::denom::Denom;
use crate::types::wallet::{Wallet, WalletAddress};

use super::ChainDriver;

/**
    Submits an IBC token transfer transaction.

    We use gaiad instead of the internal raw tx transfer to transfer tokens,
    as the current chain implementation cannot dynamically switch the sender,
    and instead always use the configured relayer wallet for sending tokens.
*/
pub fn transfer_token(
    driver: &ChainDriver,
    port_id: &PortId,
    channel_id: &ChannelId,
    sender: &Wallet,
    recipient: &WalletAddress,
    amount: u64,
    denom: &Denom,
) -> Result<(), Error> {
    // let message = build_transfer_message(
    //     port_id,
    //     channel_id,
    //     amount.into(),
    //     denom.as_str().to_string(),
    //     Signer::new(sender.address.0),
    //     Signer::new(recipient.0),
    // )

    driver.exec(&[
        "--node",
        &driver.rpc_listen_address(),
        "tx",
        "ibc-transfer",
        "transfer",
        port_id.as_str(),
        &channel_id.to_string(),
        &recipient.0,
        &format!("{}{}", amount, denom),
        "--from",
        &sender.address.0,
        "--chain-id",
        driver.chain_id.as_str(),
        "--home",
        &driver.home_path,
        "--keyring-backend",
        "test",
        "--yes",
    ])?;

    Ok(())
}

pub fn local_transfer_token(
    driver: &ChainDriver,
    sender: &Wallet,
    recipient: &WalletAddress,
    amount: u64,
    denom: &Denom,
) -> Result<(), Error> {
    driver.exec(&[
        "--node",
        &driver.rpc_listen_address(),
        "tx",
        "bank",
        "send",
        &sender.address.0,
        &recipient.0,
        &format!("{}{}", amount, denom),
        "--chain-id",
        driver.chain_id.as_str(),
        "--home",
        &driver.home_path,
        "--keyring-backend",
        "test",
        "--yes",
    ])?;

    Ok(())
}
