use std::{
    future::Future,
    pin::Pin,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Waker, Poll},
    thread,
    time::Duration,
};
use futures:: {
    future::{BoxFuture, FutureExt},
    task::{ waker_ref, ArcWake},
};


pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    // associated type
    type Output = ();
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
                println!("wwweqweqwe");
            }
        });
        TimerFuture { shared_state }
    }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued")
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            println!("recv task");
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if future.as_mut().poll(context).is_pending() {
                    println!("pendding");
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[derive(Debug)]
struct RawPt {
    data: String,
    rData: *const String,
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run() {
        let (executor, spawner) = new_executor_and_spawner();
        spawner.spawn(async {
            println!("hody!");
            TimerFuture::new(Duration::new(2, 0)).await;
            println!("done!");
        });

        drop(spawner);
        executor.run();
    }

    #[test]
    fn raw_pt() {
        let str = String::from("data");
        let mut r1 = RawPt {
            data: str,
            rData: std::ptr::null(),
        };
        r1.rData = &r1.data;
        println!("{:?}", r1.rData);
        println!("{:?}", unsafe { &*(r1.rData) });
        r1.data = String::from("newData");
        println!("{:?}", r1.rData);
        println!("{:?}", unsafe { &*(r1.rData) });
    }
}