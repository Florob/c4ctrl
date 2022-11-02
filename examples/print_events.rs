use std::time::Duration;

use c4ctrl::C4Ctrl;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (mut ctrl, mut eventloop) = C4Ctrl::new("localhost", 1883);
    ctrl.subscribe().await.unwrap();

    tokio::spawn(async move {
        while let Ok(event) = eventloop.poll().await {
            println!("{:?}", event);
        }
    });

    let mut plenarsaal = ctrl.plenarsaal();
    let mut light = plenarsaal.light_back_window();
    let mut on = true;
    loop {
        sleep(Duration::from_secs(5)).await;
        light.set(on).await.unwrap();
        on = !on;
    }
}
