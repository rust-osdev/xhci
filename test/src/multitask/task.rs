// The MIT License (MIT)
//
// Copyright (c) 2019 Philipp Oppermann
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, task::Wake};
use conquer_once::spin::Lazy;
use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll, Waker},
};
use crossbeam_queue::ArrayQueue;
use spinning_top::Spinlock;

pub(super) static COLLECTION: Lazy<Spinlock<Collection>> =
    Lazy::new(|| Spinlock::new(Collection::new()));

pub(crate) struct Collection {
    tasks: BTreeMap<Id, Task>,
    woken_task_ids: Arc<ArrayQueue<Id>>,
}
impl Collection {
    pub(crate) fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            woken_task_ids: Arc::new(ArrayQueue::new(100)),
        }
    }

    pub(crate) fn add_task_as_woken(&mut self, task: Task) {
        let id = task.id();
        self.push_task(task);
        self.push_woken_task_id(id);
    }

    pub(crate) fn add_task_as_sleep(&mut self, task: Task) {
        self.push_task(task);
    }

    fn push_task(&mut self, task: Task) {
        let id = task.id();
        if self.tasks.insert(id, task).is_some() {
            panic!("Task ID confliction.");
        }
    }

    fn push_woken_task_id(&mut self, id: Id) {
        self.woken_task_ids
            .push(id)
            .expect("Woken task id queue is full.");
    }

    pub(crate) fn pop_woken_task_id(&mut self) -> Option<Id> {
        self.woken_task_ids.pop()
    }

    pub(crate) fn remove_task(&mut self, id: Id) -> Option<Task> {
        self.tasks.remove(&id)
    }

    pub(crate) fn create_waker(&mut self, id: Id) -> Waker {
        Waker::from(Arc::new(TaskWaker::new(id, self.woken_task_ids.clone())))
    }
}

// task::Waker conflicts with alloc::task::Waker.
#[allow(clippy::module_name_repetitions)]
pub(crate) struct TaskWaker {
    id: Id,
    woken_task_ids: Arc<ArrayQueue<Id>>,
}

impl TaskWaker {
    pub(crate) fn new(id: Id, woken_task_ids: Arc<ArrayQueue<Id>>) -> Self {
        Self { id, woken_task_ids }
    }

    fn wake_task(&self) {
        self.woken_task_ids
            .push(self.id)
            .expect("task_queue is full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task()
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Debug)]
pub(crate) struct Id(u64);

impl Id {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Id(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub(crate) struct Task {
    id: Id,
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    polling: bool,
}

impl Task {
    pub(crate) fn new(future: impl Future<Output = ()> + 'static + Send) -> Self {
        Self {
            id: Id::new(),
            future: Box::pin(future),
            polling: false,
        }
    }

    pub(crate) fn new_poll(future: impl Future<Output = ()> + 'static + Send) -> Self {
        Self {
            id: Id::new(),
            future: Box::pin(future),
            polling: true,
        }
    }

    pub(crate) fn poll(&mut self, context: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(context)
    }

    pub(crate) fn polling(&self) -> bool {
        self.polling
    }

    pub(super) fn id(&self) -> Id {
        self.id
    }
}
