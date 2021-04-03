use anyhow::{Context, Result};
use std::sync::{Arc, atomic::AtomicBool, atomic::Ordering};
use std::thread;
use std::time::{Duration, Instant};

pub struct Timer {
    delay: Arc<Duration>,
    handle: Option<thread::JoinHandle<()>>,
    active: Arc<AtomicBool>
}

impl Timer {
    pub fn new(delay: Duration) -> Timer {
        Timer {
            delay: Arc::new(delay),
            handle: None,
            active: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn start<F>(&mut self, func: F)
        where
            F: Fn(),
            F: Send + 'static {
        self.active.store(true, Ordering::Relaxed);

        let delay = self.delay.clone();
        let active = Arc::clone(&self.active);

        self.handle = Some(
            thread::spawn(move || {
                loop {
                    func();

                    let start_time = Instant::now();

                    loop {
                        thread::sleep(Duration::from_millis(500));
                        if !active.load(Ordering::Relaxed) {
                            return;
                        } else if *delay <= Instant::now().duration_since(start_time) {
                            break;
                        }
                    }
                }
            })
        );
    }

    pub fn stop(&mut self) -> Result<()> {
        self.active.store(false, Ordering::Relaxed);
        Ok(
            self.handle
                .take().context("Trying to stop uninitialized thread")?
                .join().unwrap()
        )
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.stop().ok();
    }
}
