
use std::time::Duration;
use async_std::sync::Mutex;
use async_std::task;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref MUT_LOCK: Mutex<()> = Mutex::new(());
}

fn main() -> std::io::Result<()> {
    task::block_on(async {
        let ft1 = task::spawn(t1());
        let ft2 = task::spawn(t2());
        futures::join!(ft1, ft2);
    });

    Ok(())
}


async fn t1() {
    println!("t1 start");
    let _guard = MUT_LOCK.lock().await;
    println!("t1 got lock");
    let dur = Duration::from_secs(1);
    for _i in 1..5 {
        println!(" t1: sleeping");
        std::thread::sleep(dur);
    }
    println!("t1 done");
}

async fn t2() {
    println!("t2 start");
    let _guard = MUT_LOCK.lock().await;
    println!("t2 got lock");
    println!("t2 done");
}