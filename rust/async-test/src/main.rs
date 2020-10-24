use tokio::runtime;
use std::error::Error;
use tokio::sync::mpsc;
use std::thread::spawn;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let runtime = runtime::Builder::new_multi_thread().enable_all().build()?;
    let (sender, receiver) = mpsc::channel::<Message>(32);
    let mut server = Server::new(receiver);
    let handle = runtime.spawn(async move {
        server.run().await;
    });

    runtime.block_on(async move {
        for _ in 0..5i32 {
            sender.send(Message::PlusOne).await;
        }
        sender.send(Message::Shutdown).await;

        handle.await;

        Ok(())
    })
}

enum Message {
    PlusOne,
    Shutdown,
}

struct Server {
    channel: mpsc::Receiver<Message>,
    res: i32,
}

impl Server {
    fn new(channel: mpsc::Receiver<Message>) -> Self {
        Server {
            channel,
            res: 0,
        }
    }

    async fn run(&mut self) {
        loop {
            if let Some(msg) = self.channel.recv().await {
                match msg {
                    Message::PlusOne => {
                        self.res += 1;
                        println!("Value: {}", self.res)
                    }
                    Message::Shutdown => {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }
}
