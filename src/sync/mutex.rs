use alloc::collections::VecDeque;
use core::ops::{Deref, DerefMut};

use crate::sched;
use crate::sched::TaskId;

use super::spin_lock::{SpinLock, SpinLockGuard};

#[derive(Debug)]
pub(crate) struct Mutex<T> {
    inner: SpinLock<T>,
    waiting_tasks: SpinLock<VecDeque<TaskId>>,
}

impl<T> Mutex<T> {
    pub(crate) const fn new(data: T) -> Self {
        Self {
            inner: SpinLock::new(data),
            waiting_tasks: SpinLock::new(VecDeque::new()),
        }
    }

    fn wake_tasks(&self) {
        let mut waiting_tasks = self.waiting_tasks.lock();
        while let Some(task_id) = waiting_tasks.pop_front() {
            sched::awaken_task(task_id);
        }
    }

    pub(crate) fn lock(&self) -> MutexGuard<'_, T> {
        loop {
            let task_id = sched::prepare_to_sleep();

            self.waiting_tasks
                .lock_disable_interrupts()
                .push_back(task_id);
            
            let guard = self.inner.try_lock_allow_preempt();
            if let Some(guard) = guard {
                sched::awaken_task(task_id);
                return MutexGuard {
                    inner_guard: guard,
                    parent: self,
                };
            }

            sched::run_scheduler();
        }
    }
}

pub(crate) struct MutexGuard<'a, T> {
    inner_guard: SpinLockGuard<'a, T>,
    parent: &'a Mutex<T>,
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    #[allow(clippy::explicit_deref_methods)]
    fn deref(&self) -> &T {
        self.inner_guard.deref();
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    #[allow(clippy::explicit_deref_methods)]
    fn deref_mut(&mut self) -> &mut T {
        self.inner_guard.deref_mut();
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.parent.wake_tasks();
    }
}
