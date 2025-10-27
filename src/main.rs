use tokio::time;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();

    let (tx1, mut rx1) = tokio::sync::mpsc::channel(100);

    tokio::spawn(async move {
        let mut n = 1;
        loop {
            let _ = tx1.send(n).await;
            println!("Sent {}", n);
            n += 1;

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    let mut ticker = time::interval(time::Duration::from_secs(1));
    ticker.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
    let mut idx = 0;

    loop {
        idx += 1;
        tokio::select! {
            // random
            Some(str) = rx1.recv() => {
                println!("[{:?}] Received: {:?}, idx {:?}", start.elapsed(), str, idx);
            }
            _ = ticker.tick() => {
                println!("[{:?}] Starting sleep, idx {:?}", start.elapsed(), idx);
                tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
                println!("[{:?}] Finished sleeping, idx {:?}", start.elapsed(), idx);
            }
        }
    }
}

//     0 ------------------------ 1 ------------------------ 2 ------------------------ 
//     |                                       |*|
//     | --------- Flush Task Awaits --------- |*| --------- Flush Task Awaits ---------
//     |                                       |*|
//                                              ⬆
//                                      Small time interval
//                               Might allow a few point to be pushed
//