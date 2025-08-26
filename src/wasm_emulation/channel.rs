use cosmwasm_std::StdResult;
use cw_orch::daemon::GrpcChannel;
use tokio::runtime::{Handle, Runtime};
use tonic::transport::Channel;

/// Simple helper to get the GRPC transport channel
fn get_channel(
    grpc_urls: &[String],
    chain_id: impl Into<String>,
    rt: &Runtime,
) -> StdResult<tonic::transport::Channel> {
    let channel = rt.block_on(GrpcChannel::connect(grpc_urls, &chain_id.into()))?;
    Ok(channel)
}

#[derive(Clone)]
pub struct RemoteChannel {
    pub rt: Handle,
    pub channel: Channel,
    pub pub_address_prefix: String,
    // For caching
    pub chain_id: String,
}

impl RemoteChannel {
    pub fn new(
        rt: &Runtime,
        grpc_urls: &[&str],
        chain_id: impl Into<String>,
        pub_address_prefix: impl Into<String>,
    ) -> StdResult<Self> {
        let chain_id = chain_id.into();
        Ok(Self {
            rt: rt.handle().clone(),
            channel: get_channel(
                &grpc_urls
                    .iter()
                    .cloned()
                    .map(Into::into)
                    .collect::<Vec<_>>(),
                chain_id.clone(),
                rt,
            )?,
            pub_address_prefix: pub_address_prefix.into(),
            chain_id,
        })
    }
}
