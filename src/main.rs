mod key_value;
mod behaviour;

use async_std::task;
use futures::{prelude::*, select};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::store::RecordStore;
use libp2p::kad::record::Key;
use libp2p::kad::Quorum;
use libp2p::kad::{
    AddProviderOk, Kademlia, KademliaEvent, PeerRecord, PutRecordOk, QueryResult,
    Record,
};
use libp2p::{
    development_transport, identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::SwarmEvent, PeerId, Swarm,
};
use std::error::Error;
use behaviour::{MyBehaviour, MyBehaviourEvent};
use key_value::{DHTKey, DHTValue};

pub fn handle_query_result(result: QueryResult) {
	match result {
		QueryResult::GetProviders(Ok(ok)) => {
			for peer in ok.providers {
				println!(
					"Peer {:?} provides key {:?}",
					peer,
					ok.key.as_ref()
				);
			}
		}
		QueryResult::GetRecord(Ok(ok)) => {
			for PeerRecord {
				record: Record { key, value, .. },
				..
			} in ok.records
			{
				println!(
					"Got record {:?} {:?}",
					key.as_ref(),
					&value,
				);
			}
		}
		QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
			println!(
				"Successfully put record {:?}",
				key.as_ref()
			);
		}
		QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
			println!(
				"Successfully put provider record {:?}",
				key.as_ref()
			);
		}
		e => {
			eprintln!("other event: {:?}", e);
		}
	}
}

pub fn handle_kademlia_events(event: KademliaEvent) {
	match event {
		KademliaEvent::OutboundQueryCompleted { result, ..} => {
			handle_query_result(result);
		},
		ev => {
			// println!("Other event: {:?}", ev);
		}
	}
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a random key for ourselves.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol.
    let transport = development_transport(local_key).await?;

    // Create a swarm to manage peers and events.
    let mut swarm = {
        // Create a Kademlia behaviour.
        let store = MemoryStore::new(local_peer_id);
        let kademlia = Kademlia::new(local_peer_id, store);
        let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
        let behaviour = MyBehaviour { kademlia, mdns };
        Swarm::new(transport, behaviour, local_peer_id)
    };

    // Listen on all interfaces and whatever port the OS assigns.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

	let mut rng = rand::thread_rng();
	let key = DHTKey::random(&mut rng);
	let value = DHTValue::random(&mut rng);
	let record = Record {
		key: Key::new(&key.to_bytes()),
		value: value.to_bytes(),
		publisher: None,
		expires: None,
	};

    // Kick it off.
    loop {
        select! {
        event = swarm.select_next_some() => match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening in {:?}", address);
            },
            SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Discovered(list))) => {
                for (peer_id, multiaddr) in list {
                    swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr);
                }
				swarm.behaviour_mut().kademlia.put_record(record.clone(), Quorum::One)?;
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Kademlia(event)) => {
                handle_kademlia_events(event);
				let store = swarm.behaviour_mut().kademlia.store_mut();
				let records = store.records();
				println!("num local records {:?}", records.len());
				for r in records {
					println!("local record: {:?}", r.into_owned());
				}
            }
            _ => {}
        }
        }
    }
}
