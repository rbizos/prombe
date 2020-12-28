use tokio::sync::watch::{channel, Receiver, Sender};
use tokio::time::{delay_for, Duration};
//TODO port to  tokio 3 when hyper and hyper-tls are fully compatible
pub struct Clock {
    period: u64,
    tx: Sender<()>,
    rx: Receiver<()>,
}

impl Clock {
    pub fn new(period: u64) -> Clock {
        let (tx, rx) = channel(());
        Clock { period, tx, rx }
    }

    pub async fn run(&mut self) {
        loop {
            self.tx.broadcast(()).unwrap(); // no need for error handling here
                                            // TODO if exit uglily, add some select close à la golang
            delay_for(Duration::from_secs(self.period)).await;
        }
    }

    pub fn receiver(&self) -> Receiver<()> {
        self.rx.clone()
    }
}
