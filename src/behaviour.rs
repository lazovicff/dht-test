use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    Kademlia, KademliaEvent
};
use libp2p::{
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
};

// We create a custom network behaviour that combines Kademlia and mDNS.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
pub struct MyBehaviour {
	pub(crate) kademlia: Kademlia<MemoryStore>,
	pub(crate) mdns: Mdns,
}

pub enum MyBehaviourEvent {
	Kademlia(KademliaEvent),
	Mdns(MdnsEvent),
}

impl From<KademliaEvent> for MyBehaviourEvent {
	fn from(event: KademliaEvent) -> Self {
		MyBehaviourEvent::Kademlia(event)
	}
}

impl From<MdnsEvent> for MyBehaviourEvent {
	fn from(event: MdnsEvent) -> Self {
		MyBehaviourEvent::Mdns(event)
	}
}