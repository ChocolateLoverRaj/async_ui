use std::future::Future;
use std::mem;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Mutex;

use futures::future::Shared;
use futures::FutureExt;
use futures_signals::signal::{Mutable, MutableSignalCloned, SignalExt};

struct SetAsyncState<T: Clone, R: Clone + 'static> {
    future: Mutable<Option<Shared<Box<dyn Future<Output=R> + Unpin>>>>,
    queued: Mutable<Option<T>>,
}

impl<T: Clone, R: Clone + 'static> Default for SetAsyncState<T, R> {
    fn default() -> Self {
        Self {
            queued: Mutable::new(None),
            future: Mutable::new(None),
        }
    }
}

#[derive(Clone)]
pub struct SetAsync<T: Clone, R: Clone + 'static> {
    set_fn: Rc<Box<dyn Fn(T) -> Box<dyn Future<Output=R> + Unpin>>>,
    state: Rc<SetAsyncState<T, R>>,
    check_double_run: Rc<Mutex<()>>,
}

impl<T: Clone + 'static, R: Clone + 'static> SetAsync<T, R> {
    pub fn new(set_fn: Box<dyn Fn(T) -> Box<dyn Future<Output=R> + Unpin>>) -> Self {
        Self {
            set_fn: set_fn.into(),
            state: Default::default(),
            check_double_run: Default::default(),
        }
    }

    pub async fn set(&self, value: T) {
        // let future = self.state.future.lock_ref().deref();
        // let queued = self.state.queued.lock_ref();
        let mut future = self.state.future.lock_mut();
        match future.deref_mut() {
            Some(future) => {
                self.state.queued.set(Some(value));
            }
            None => {
                *future = Some((self.set_fn)(value).shared());
            }
        }
    }

    pub fn get_future_signal(&self) -> MutableSignalCloned<Option<Shared<Box<dyn Future<Output=R> + Unpin>>>> {
        self.state.future.signal_cloned()
    }

    pub fn get_queued_signal(&self) -> MutableSignalCloned<Option<T>> {
        self.state.queued.signal_cloned()
    }

    pub async fn run(&self) {
        match self.check_double_run.try_lock() {
            Ok(lock) => {
                self.state.future.signal_cloned().for_each(|future| async move {
                    if let Some(future) = future {
                        future.await;
                        let mut future = self.state.future.lock_mut();
                        let mut queued = self.state.queued.lock_mut();
                        if queued.is_some() {
                            // let mut new_queued: Option<T>;
                            let queued = mem::replace(queued.deref_mut(), None).unwrap();
                            *future = Some((self.set_fn)(queued).shared())
                        }
                    }
                }).await;
                drop(lock);
            }
            Err(_e) => {
                panic!("The run method is already running. Calling run more than once at a time is wasteful and unnecessary.")
            }
        }
    }
}