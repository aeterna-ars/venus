use rtnetlink::new_connection;

pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub use rtnetlink::packet_route::*;
pub use rtnetlink::packet_route::rule::*;
pub use rtnetlink::packet_route::route::*;

pub use rtnetlink::{Handle, RouteMessageBuilder};

use crate::error::IoError;

pub struct RoutingPolicy {
    pub handle: Handle,
    pub table_id: u32,
    pub rule_priority: u32,
    pub iface_id: Option<u32>,
}

impl RoutingPolicy {
    pub fn new(
        iface_id: Option<u32>,
        rule_priority: u32,
        table_id: u32,
    ) -> Result<Self, IoError> {
        let (connection, handle, _) = new_connection()
            .map_err(|e| IoError(format!("new_connection error: {e}")))?;
        tokio::spawn(connection);

        Ok(Self {
            handle,
            table_id,
            rule_priority,
            iface_id,
        })
    }
}