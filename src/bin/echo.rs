use serde::{Deserialize, Serialize};
use vortex::{Message, Node, NodeServer};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum EchoPayload {
    Echo { echo: String },
    EchoOk { echo: String },
}

struct EchoNode {
    id: String,
}

impl Node<EchoPayload> for EchoNode {
    fn init(id: String) -> Self {
        EchoNode { id }
    }

    fn handle(
        &mut self,
        req: Message<EchoPayload>,
        res_msg_id: usize,
    ) -> Option<Message<EchoPayload>> {
        let payload = req.body.payload.clone();

        match payload {
            EchoPayload::Echo { echo } => {
                Some(req.build_reply(self.id.clone(), EchoPayload::EchoOk { echo }, res_msg_id))
            }
            EchoPayload::EchoOk { echo } => None,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut server = NodeServer::new();

    server.listen::<EchoNode, EchoPayload>(std::io::stdin(), std::io::stdout())
}
