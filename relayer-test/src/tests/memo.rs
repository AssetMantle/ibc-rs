use eyre::eyre;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::config::{types::Memo, Config};
use serde_json as json;
use tracing::{debug, info};

use crate::config::TestConfig;
use crate::error::Error;
use crate::framework::binary::channel::{run_binary_channel_test, BinaryChannelTest};
use crate::framework::overrides::{with_overrides, TestOverrides};
use crate::ibc::denom::derive_ibc_denom;
use crate::types::binary::chains::ConnectedChains;
use crate::types::binary::channel::Channel;
use crate::util::random::{random_string, random_u64_range};

#[test]
fn test_memo() -> Result<(), Error> {
    let memo = Memo::new(&random_string());
    let test = MemoTest { memo };
    let overrides = with_overrides(&test);
    run_binary_channel_test(&test, &overrides)
}

struct MemoTest {
    memo: Memo,
}

impl TestOverrides for MemoTest {
    fn modify_relayer_config(&self, config: &mut Config) {
        for mut chain in config.chains.iter_mut() {
            chain.memo_prefix = self.memo.clone();
        }
    }
}

impl BinaryChannelTest for MemoTest {
    fn run<ChainA: ChainHandle, ChainB: ChainHandle>(
        &self,
        _config: &TestConfig,
        chains: &ConnectedChains<ChainA, ChainB>,
        channel: &Channel<ChainA, ChainB>,
    ) -> Result<(), Error> {
        info!(
            "testing IBC transfer with memo configured: \"{}\"",
            self.memo
        );

        let denom_a = chains.side_a.denom();

        let a_to_b_amount = random_u64_range(1000, 5000);

        chains.side_a.chain_driver().transfer_token(
            &channel.port_a,
            &channel.channel_id_a,
            &chains.side_a.wallets().user1().address(),
            &chains.side_b.wallets().user1().address(),
            a_to_b_amount,
            &denom_a,
        )?;

        let denom_b = derive_ibc_denom(
            &channel.port_b.as_ref(),
            &channel.channel_id_b.as_ref(),
            &denom_a,
        )?;

        chains.side_b.chain_driver().assert_eventual_wallet_amount(
            &chains.side_b.wallets().user1(),
            a_to_b_amount,
            &denom_b.as_ref(),
        )?;

        let tx_info = chains
            .side_b
            .chain_driver()
            .query_recipient_transactions(&chains.side_b.wallets().user1().address())?;

        assert_tx_memo_equals(&tx_info, &self.memo.as_str())?;

        Ok(())
    }
}

fn assert_tx_memo_equals(tx_info: &json::Value, expected_memo: &str) -> Result<(), Error> {
    debug!("comparing memo field from json value {}", tx_info);

    let memo_field = &tx_info["txs"][0]["tx"]["body"]["memo"];

    info!("memo field value: {}", memo_field);

    let memo_str = memo_field
        .as_str()
        .ok_or_else(|| eyre!("expect memo string field to be present in JSON"))?;

    assert_eq!(memo_str, expected_memo);

    Ok(())
}
