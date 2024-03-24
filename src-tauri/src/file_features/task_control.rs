use async_std::sync::{Arc, Mutex};
use async_std::task;

pub struct TaskControl {
    current: usize,
    limit: usize,
}

impl TaskControl {
    pub fn new(limit: usize) -> Self {
        TaskControl { current: 0, limit }
    }

    pub async fn acquire(control: Arc<Mutex<Self>>) {
        let mut ctrl = control.lock().await;
        while ctrl.current >= ctrl.limit {
            drop(ctrl);
            task::sleep(std::time::Duration::from_millis(100)).await;
            ctrl = control.lock().await;
        }
        ctrl.current += 1;
    }

    pub fn release(control: Arc<Mutex<Self>>) {
        task::block_on(async {
            let mut ctrl = control.lock().await;
            ctrl.current -= 1;
        });
    }
}
