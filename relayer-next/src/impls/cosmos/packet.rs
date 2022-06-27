use ibc::core::ics04_channel::packet::Packet;
use ibc::core::ics04_channel::packet::Sequence;
use ibc::core::ics24_host::identifier::{ChannelId, PortId};
use ibc::timestamp::Timestamp;
use ibc::Height;

use crate::impls::cosmos::handler::CosmosChainHandler;
use crate::traits::core::Async;
use crate::traits::packet::IbcPacket;

impl<SrcChain, DstChain> IbcPacket<CosmosChainHandler<SrcChain>, CosmosChainHandler<DstChain>>
    for Packet
where
    SrcChain: Async,
    DstChain: Async,
{
    fn source_port(&self) -> &PortId {
        &self.source_port
    }

    fn source_channel_id(&self) -> &ChannelId {
        &self.source_channel
    }

    fn destination_port(&self) -> &PortId {
        &self.destination_port
    }

    fn destination_channel_id(&self) -> &ChannelId {
        &self.destination_channel
    }

    fn sequence(&self) -> &Sequence {
        &self.sequence
    }

    fn timeout_height(&self) -> &Height {
        &self.timeout_height
    }

    fn timeout_timestamp(&self) -> &Timestamp {
        &self.timeout_timestamp
    }
}
