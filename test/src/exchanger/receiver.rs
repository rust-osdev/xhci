// SPDX-License-Identifier: GPL-3.0-or-later

use alloc::{collections::BTreeMap, sync::Arc};
use conquer_once::spin::Lazy;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::task::AtomicWaker;
use spinning_top::{Spinlock, SpinlockGuard};
use x86_64::PhysAddr;
use xhci::ring::trb::event;

static RECEIVER: Lazy<Spinlock<Receiver>> = Lazy::new(|| Spinlock::new(Receiver::new()));

pub(crate) fn add_entry(trb_a: PhysAddr, waker: Arc<Spinlock<AtomicWaker>>) -> Result<(), Error> {
    lock().add_entry(trb_a, waker)
}

pub(crate) fn receive(t: event::Allowed) {
    lock().receive(t)
}

fn lock() -> SpinlockGuard<'static, Receiver> {
    RECEIVER
        .try_lock()
        .expect("Failed to acquire the lock of `RECEIVER`.")
}

struct Receiver {
    trbs: BTreeMap<PhysAddr, Option<event::Allowed>>,
    wakers: BTreeMap<PhysAddr, Arc<Spinlock<AtomicWaker>>>,
}
impl Receiver {
    fn new() -> Self {
        Self {
            trbs: BTreeMap::new(),
            wakers: BTreeMap::new(),
        }
    }

    fn add_entry(
        &mut self,
        addr_to_trb: PhysAddr,
        waker: Arc<Spinlock<AtomicWaker>>,
    ) -> Result<(), Error> {
        if self.trbs.insert(addr_to_trb, None).is_some() {
            return Err(Error::AddrAlreadyRegistered);
        }

        if self.wakers.insert(addr_to_trb, waker).is_some() {
            return Err(Error::AddrAlreadyRegistered);
        }
        Ok(())
    }

    fn receive(&mut self, trb: event::Allowed) {
        if let Err(e) = self.insert_trb_and_wake_runner(trb) {
            panic!("Failed to receive a command completion trb: {:?}", e);
        }
    }

    fn insert_trb_and_wake_runner(&mut self, trb: event::Allowed) -> Result<(), Error> {
        let addr_to_trb = Self::trb_addr(trb);
        self.insert_trb(trb)?;
        self.wake_runner(addr_to_trb)?;
        Ok(())
    }

    fn insert_trb(&mut self, trb: event::Allowed) -> Result<(), Error> {
        let addr_to_trb = Self::trb_addr(trb);
        *self
            .trbs
            .get_mut(&addr_to_trb)
            .ok_or(Error::NoSuchAddress)? = Some(trb);
        Ok(())
    }

    fn wake_runner(&mut self, addr_to_trb: PhysAddr) -> Result<(), Error> {
        self.wakers
            .remove(&addr_to_trb)
            .ok_or(Error::NoSuchAddress)?
            .lock()
            .wake();
        Ok(())
    }

    fn trb_arrives(&self, addr_to_trb: PhysAddr) -> bool {
        match self.trbs.get(&addr_to_trb) {
            Some(trb) => trb.is_some(),
            None => panic!("No such TRB with the address {:?}", addr_to_trb),
        }
    }

    fn remove_entry(&mut self, addr_to_trb: PhysAddr) -> Option<event::Allowed> {
        match self.trbs.remove(&addr_to_trb) {
            Some(trb) => trb,
            None => panic!("No such receiver with TRB address: {:?}", addr_to_trb),
        }
    }

    fn trb_addr(t: event::Allowed) -> PhysAddr {
        PhysAddr::new(match t {
            event::Allowed::TransferEvent(e) => e.trb_pointer(),
            event::Allowed::CommandCompletion(c) => c.command_trb_pointer(),
            _ => todo!(),
        })
    }
}

#[derive(Debug)]
pub(crate) enum Error {
    AddrAlreadyRegistered,
    NoSuchAddress,
}

pub(crate) struct ReceiveFuture {
    addr_to_trb: PhysAddr,
    waker: Arc<Spinlock<AtomicWaker>>,
}
impl ReceiveFuture {
    pub(crate) fn new(addr_to_trb: PhysAddr, waker: Arc<Spinlock<AtomicWaker>>) -> Self {
        Self { addr_to_trb, waker }
    }
}
impl Future for ReceiveFuture {
    type Output = event::Allowed;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = self.waker.clone();
        let addr = self.addr_to_trb;
        let mut r = lock();

        waker.lock().register(cx.waker());
        if r.trb_arrives(addr) {
            waker.lock().take();
            let trb = r.remove_entry(addr).unwrap();
            Poll::Ready(trb)
        } else {
            Poll::Pending
        }
    }
}
