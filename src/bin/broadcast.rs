use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vortex::{Message, Node, NodeServer};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum BroadcastPayload {
    Broadcast {
        #[serde(rename = "message")]
        body_message_id: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        #[serde(rename = "messages")]
        body_messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

struct BroadcastNode {
    id: String,
    body_msg_list: Vec<usize>,
}

impl Node<BroadcastPayload> for BroadcastNode {
    fn init(id: String) -> Self {
        BroadcastNode {
            id,
            body_msg_list: Vec::new(),
        }
    }

    fn handle(
        &mut self,
        req: Message<BroadcastPayload>,
        res_msg_id: usize,
    ) -> Option<Message<BroadcastPayload>> {
        let payload = req.body.payload.clone();

        match payload {
            BroadcastPayload::Broadcast { body_message_id } => {
                self.body_msg_list.push(body_message_id);

                Some(req.build_reply(self.id.clone(), BroadcastPayload::BroadcastOk, res_msg_id))
            }
            BroadcastPayload::Read => Some(req.build_reply(
                self.id.clone(),
                BroadcastPayload::ReadOk {
                    body_messages: self.body_msg_list.clone(),
                },
                res_msg_id,
            )),
            BroadcastPayload::Topology { topology } => {
                Some(req.build_reply(self.id.clone(), BroadcastPayload::TopologyOk, res_msg_id))
            }
            BroadcastPayload::BroadcastOk => None,
            BroadcastPayload::ReadOk { body_messages } => None,
            BroadcastPayload::TopologyOk => None,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut server = NodeServer::new();

    server.listen::<BroadcastNode, BroadcastPayload>(std::io::stdin(), std::io::stdout())
}
