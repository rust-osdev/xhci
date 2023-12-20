// The MIT License (MIT)
//
// Copyright (c) 2019 Philipp Oppermann
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use super::task;
use alloc::collections::BTreeMap;
use core::task::{Context, Poll, Waker};
use task::Task;

pub(crate) struct Executor {
    waker_collection: BTreeMap<task::Id, Waker>,
}

impl Executor {
    pub(crate) fn new() -> Self {
        Self {
            waker_collection: BTreeMap::new(),
        }
    }

    pub(crate) fn run(&mut self) -> ! {
        loop {
            self.run_woken_tasks();
        }
    }

    fn run_woken_tasks(&mut self) {
        while let Some(id) = Self::pop_woken_task_id() {
            self.run_task(id);
        }
    }

    fn pop_woken_task_id() -> Option<task::Id> {
        task::COLLECTION.lock().pop_woken_task_id()
    }

    fn run_task(&mut self, id: task::Id) {
        let Self {
            waker_collection: _,
        } = self;

        let mut task = match task::COLLECTION.lock().remove_task(id) {
            Some(task) => task,
            None => return,
        };

        let mut context = self.generate_waker(id);
        match task.poll(&mut context) {
            Poll::Ready(_) => {
                task::COLLECTION.lock().remove_task(id);
                self.waker_collection.remove(&id);
            }
            Poll::Pending => Self::add_task_as_pending(task),
        }
    }

    fn generate_waker(&mut self, id: task::Id) -> Context<'_> {
        let Self { waker_collection } = self;

        let waker = waker_collection
            .entry(id)
            .or_insert_with(|| task::COLLECTION.lock().create_waker(id));
        Context::from_waker(waker)
    }

    fn add_task_as_pending(task: Task) {
        if task.polling() {
            task::COLLECTION.lock().add_task_as_woken(task);
        } else {
            task::COLLECTION.lock().add_task_as_sleep(task);
        }
    }
}
