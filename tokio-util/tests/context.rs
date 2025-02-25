#![warn(rust_2018_idioms)]
#![cfg(feature = "rt")]

use tokio::runtime::Builder;
use tokio::time::*;
use tokio_util::context::HandleExt;

#[test]
fn tokio_context_with_another_runtime() {
    let mut rt1 = Builder::new()
        .threaded_scheduler()
        .core_threads(1)
        // no timer!
        .build()
        .unwrap();
    let rt2 = Builder::new()
        .threaded_scheduler()
        .core_threads(1)
        .enable_all()
        .build()
        .unwrap();

    // Without the `HandleExt.wrap()` there would be a panic because there is
    // no timer running, since it would be referencing runtime r1.
    let _ = rt1.block_on(
        rt2.handle()
            .wrap(async move { delay_for(Duration::from_millis(2)).await }),
    );
}
