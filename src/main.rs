#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    tokio::spawn(async move {
        let mut n = 1;
        loop {
            match tx.try_send(n) {
                Ok(()) => {
                    // println!("[{:?}] Sent {}", start.elapsed(), n);
                }
                Err(tokio::sync::mpsc::error::TrySendError::Full(value)) => {
                    println!("Channel full, couldn't send {}", value);
                }
                Err(tokio::sync::mpsc::error::TrySendError::Closed(value)) => {
                    println!("Channel closed, couldn't send {}", value);
                }
            }
            // println!("Sent {}", n);
            n += 1;

            tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
        }
    });

    let mut ticker = tokio::time::interval(tokio::time::Duration::from_secs(1));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let mut buffer = Vec::new();

    loop {
        tokio_select(&mut buffer, &mut rx, &mut ticker, start).await;
    }
}

async fn tokio_select(
    buffer: &mut Vec<i32>,
    rx: &mut tokio::sync::mpsc::Receiver<i32>,
    ticker: &mut tokio::time::Interval,
    start: std::time::Instant
) {
    tokio::select! {
        biased;
        _ = ticker.tick() => {
            println!("[{:?}] time trigger, size = {}", start.elapsed(), buffer.len());
            if buffer.len() != 0 {
                sleep_2_sec(start).await;
                buffer.clear();
            }
        }
        Some(str) = rx.recv() => {
            println!("[{:?}] Received: {:?}", start.elapsed(), str);
            buffer.push(str);
            if buffer.len() > 200 {
                println!("[{:?}] size trigger", start.elapsed());
                sleep_2_sec(start).await;
                buffer.clear();
            }
        }
    }
}

async fn sleep_2_sec(start: std::time::Instant) {
    println!("[{:?}] Starting flush", start.elapsed());
    tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
    println!("[{:?}] Finished flushing", start.elapsed());
}
