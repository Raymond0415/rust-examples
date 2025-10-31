use futures::future::join_all;
use futures::{Future, try_join};
use std::error::Error;
use std::pin::Pin;
use futures::FutureExt;

type BoxError = Box<dyn Error + Send + Sync>;

fn make_future<T>(ms: u64, ret: T) -> impl std::future::Future<Output = T> {
    async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
        ret
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Single await
    let await_result = async { 0 }.await;
    println!("Single await: {await_result}");

    // join_all: waits for all futures, returns all results
    let slow = make_future(100, "slow");
    let fast = make_future(10, "fast");
    let futures: Vec<Pin<Box<dyn Future<Output = &str>>>> = vec![Box::pin(slow), Box::pin(fast)];
    let join_all_results = join_all(futures).await;
    println!("Join all: {:?}", join_all_results);

    // try_join: short-circuits on first error
    let fut_ok = make_future(100, Ok::<(), BoxError>(()));
    let fut_err1 = make_future(10, Err::<(), BoxError>("1st err".into()));
    let fut_err2 = make_future(100, Err::<(), BoxError>("2nd err".into()));
    let try_join_results = try_join!(fut_ok, fut_err1, fut_err2);
    println!("Try join (ok + err): {:?}", try_join_results);

    // try_join: both succeed
    let fut_ok1 = make_future(100, Ok::<i32, BoxError>(1));
    let fut_ok2 = make_future(10, Ok::<i32, BoxError>(2));
    let try_join_results = try_join!(fut_ok1, fut_ok2);
    println!("Try join (ok + ok): {:?}", try_join_results);

    // select: returns first completed future
    let mut slow = Box::pin(make_future(100, "slow").fuse());
    let mut fast = Box::pin(make_future(10, "fast").fuse());
    futures::select! {
        s = slow => println!("Futures select: {}", s),
        f = fast => println!("Futures select: {}", f),
    }

    let slow = make_future(100, "slow");
    let fast1 = make_future(10, "fast1");
    let fast2 = make_future(10, "fast2");
    tokio::select! {
        /*fair or*/ biased;
        s = slow => println!("Tokio select: {}", s),
        f2 = fast2 => println!("Tokio select: {}", f2),
        f1 = fast1 => println!("Tokio select: {}", f1),
        else => println!("Tokio select: what?"),
    }

    // let task_ok = tokio::spawn()
}
