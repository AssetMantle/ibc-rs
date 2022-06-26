use async_trait::async_trait;

use crate::traits::relay_types::RelayTypes;
use crate::types::aliases::{ChannelId, Height, IbcMessage, PortId, Sequence};

#[async_trait]
pub trait TimeoutUnorderedPacketMessageBuilder<Relay: RelayTypes> {
    async fn build_timeout_unordered_packet_message(
        &self,
        height: Height<Relay::DstChain>,
        port_id: PortId<Relay::DstChain, Relay::SrcChain>,
        channel_id: ChannelId<Relay::DstChain, Relay::SrcChain>,
        sequence: Sequence<Relay::SrcChain, Relay::DstChain>,
    ) -> Result<IbcMessage<Relay::SrcChain, Relay::DstChain>, Relay::Error>;
}

#[async_trait]
pub trait TimeoutOrderedPacketMessageBuilder<Relay: RelayTypes> {
    async fn build_timeout_ordered_packet_message(
        &self,
        height: Height<Relay::DstChain>,
        port_id: PortId<Relay::DstChain, Relay::SrcChain>,
        channel_id: ChannelId<Relay::DstChain, Relay::SrcChain>,
    ) -> Result<IbcMessage<Relay::SrcChain, Relay::DstChain>, Relay::Error>;
}

#[async_trait]
pub trait TimeoutChannelClosedMessageBuilder<Relay: RelayTypes> {
    async fn build_timeout_channel_closed_message(
        &self,
        height: Height<Relay::DstChain>,
        port_id: PortId<Relay::DstChain, Relay::SrcChain>,
        channel_id: ChannelId<Relay::DstChain, Relay::SrcChain>,
    ) -> Result<IbcMessage<Relay::SrcChain, Relay::DstChain>, Relay::Error>;
}
