use core::ops::{Deref, DerefMut};
use spin::mutex::{SpinMutex, SpinMutexGuard};

use crate::sched::PreemptGuard;

#[derive(Debug)]
pub(crate) struct SpinLock<T> {
    mutex: SpinMutex<T>,
}

impl<T> SpinLock<T> {
    pub(crate) const fn new(data: T) -> Self {
        Self {
            mutex: SpinMutex::new(data),
        }
    }

    pub(crate) fn lock(&self) -> SpinLockGuard<'_, T> {
        let preempt_guard = PreemptGuard::new(());

        SpinLockGuard {
            guard: self.mutex.lock(),
            _interrupt_guard: InterruptGuard {
                needs_enabling: false,
            },
            _preempt_guard: Some(preempt_guard),
        }
    }

    pub(crate) fn lock_disable_interrupts(&self) -> SpinLockGuard<'_, T> {
        let preempt_guard = PreemptGuard::new(());

        let saved_intpt_flag = x86_64::instuctions::interrupts::are_enabled();

        if saved_intpt_flag {
            x86_64::instructions::interrupts::disable();
        }

        SpinLockGuard {
            guard: self.mutex.lock(),
            _interrupt_guard: InterruptGuard {
                needs_enabling: saved_intpt_flag,
            },
            _preempt_guard: Some(preempt_guard),
        }
    }

    pub(crate) unsafe fn force_unlock(&self) {
        self.mutex.force_unlock();
    }
}

pub(crate) struct SpinLockGuard<'a, T: ?Sized + 'a> {
    guard: SpinMutexGuard<'a, T>,
    _interrupt_guard: InterruptGuard,
    _preempt_guard: Option<PreemptGuard<()>>,
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    #[allow(clippy::eplicit_deref_methods)]
    fn deref(&self) -> &T {
        self.guard.deref()
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    #[allow(clippy::explicit_deref_methods)]
    fn deref_mut(&mut self) -> &mut T {
        self.guard.deref_mut()
    }
}
