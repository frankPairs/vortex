use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::{Stdin, Stdout, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message<P: Sized + Serialize> {
    pub src: String,
    pub dest: String,
    pub body: Body<P>,
}

impl<P> Message<P>
where
    P: Sized + Serialize,
{
    pub fn build_reply(&self, node_id: String, payload: P, msg_id: usize) -> Message<P> {
        Message {
            src: node_id,
            dest: self.src.clone(),
            body: Body {
                id: Some(msg_id),
                in_reply_to: self.body.id,
                payload,
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Body<P>
where
    P: Sized + Serialize,
{
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: P,
}

pub trait Node<P>
where
    P: Sized + Serialize + DeserializeOwned,
    Self: Sized,
{
    fn init(id: String) -> Self;
    fn handle(&mut self, req: Message<P>, res_msg_id: usize) -> Option<Message<P>>;
}

pub struct NodeServer {
    msg_id: usize,
}

impl NodeServer {
    pub fn new() -> Self {
        NodeServer { msg_id: 1 }
    }

    pub fn listen<N: Node<P>, P: Sized + Serialize + DeserializeOwned>(
        &mut self,
        stdin: Stdin,
        stdout: Stdout,
    ) -> anyhow::Result<()> {
        let mut stdout = stdout.lock();
        let mut inputs = stdin.lines();

        let decoded_msg = inputs.next().expect("Missing init message")?;
        let init_msg: Message<InitPayload> =
            serde_json::from_str(&decoded_msg).context("Invalid init message")?;

        let InitPayload::Init { node_id, node_ids } = &init_msg.body.payload else {
            return Err(anyhow::anyhow!("Unable to initialize a node"));
        };

        // Initialize the node
        let mut node = N::init(node_id.clone());

        // Write init ok response
        self.write(
            init_msg.build_reply(node_id.clone(), InitPayload::InitOk, self.msg_id),
            &mut stdout,
        )?;

        for input in inputs {
            let input = input.context("Error reading from stdin")?;
            let msg: Message<P> = serde_json::from_str(&input).context("Invalid init message")?;

            let reply = node.handle(msg, self.msg_id);

            if let Some(reply) = reply {
                // Write node response
                self.write(reply, &mut stdout)?;
            }
        }

        stdout.flush()?;

        Ok(())
    }

    fn write<P: Sized + Serialize + DeserializeOwned>(
        &mut self,
        msg: Message<P>,
        mut output: impl Write,
    ) -> anyhow::Result<()> {
        serde_json::to_writer(&mut output, &msg).context("Unable to encode message")?;

        output.write_all(b"\n")?;

        self.msg_id += 1;

        Ok(())
    }
}
