//! Regression test for libzmq #4841: a heartbeat on an engine whose input was stopped
//! (full pipe) when the connection failed must not hit the `!_io_error` assertion.
//!
//! Linux only: the failure is delivered as EPOLLHUP/EPOLLERR, which kqueue does not
//! report once the read filter has been removed.
#![cfg(target_os = "linux")]

use std::{thread::sleep, time::Duration};

const HEARTBEAT_IVL_MS: i32 = 200;

#[test]
fn router_heartbeat_after_peer_hangs_up_with_full_pipe() {
    let ctx = zmq::Context::new();
    let router = ctx.socket(zmq::ROUTER).unwrap();
    router.set_heartbeat_ivl(HEARTBEAT_IVL_MS).unwrap();
    router.set_heartbeat_timeout(60_000).unwrap();
    // Fill the pipe almost immediately so the engine stops reading.
    router.set_rcvhwm(2).unwrap();
    router.bind("tcp://127.0.0.1:*").unwrap();
    let endpoint = router.get_last_endpoint().unwrap().unwrap();

    {
        let peer_ctx = zmq::Context::new();
        let dealer = peer_ctx.socket(zmq::DEALER).unwrap();
        dealer.set_linger(0).unwrap();
        dealer.connect(&endpoint).unwrap();
        sleep(Duration::from_millis(300));
        for _ in 0..100 {
            let _ = dealer.send(&[0u8; 64][..], zmq::DONTWAIT);
        }
        // The ROUTER never reads, so its engine stops input on the full pipe.
        sleep(Duration::from_secs(1));
    }

    // The next heartbeat writes to the closed connection; the resulting RST sets
    // `_io_error`. Unpatched libzmq aborts the process on the heartbeat after that.
    sleep(Duration::from_millis(10 * HEARTBEAT_IVL_MS as u64));
}
