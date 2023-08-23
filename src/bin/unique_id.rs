use serde::{Deserialize, Serialize};
use ulid::Ulid;
use vortex::{Message, Node, NodeServer};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum UniqueIdPayload {
    Generate,
    GenerateOk { id: Ulid },
}

struct UniqueIdNode {
    id: String,
}

impl Node<UniqueIdPayload> for UniqueIdNode {
    fn init(id: String) -> Self {
        UniqueIdNode { id }
    }

    fn handle(
        &mut self,
        message: Message<UniqueIdPayload>,
        msg_id: usize,
    ) -> Option<Message<UniqueIdPayload>> {
        let payload = message.body.payload.clone();

        match payload {
            UniqueIdPayload::Generate => Some(message.build_reply(
                self.id.clone(),
                UniqueIdPayload::GenerateOk { id: Ulid::new() },
                msg_id,
            )),
            UniqueIdPayload::GenerateOk { id } => None,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut server = NodeServer::new();

    server.listen::<UniqueIdNode, UniqueIdPayload>(std::io::stdin(), std::io::stdout())
}
