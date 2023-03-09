use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::pubsub::{DynSubscriber, PubSubChannel};

use crate::AtatUrc;

pub type UrcSubscription<'sub, Urc> = DynSubscriber<'sub, <Urc as AtatUrc>::Response>;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    MaximumSubscribersReached,
}

pub trait AtatUrcChannel<Urc: AtatUrc> {
    fn subscribe<'sub>(&'sub self) -> Result<UrcSubscription<'sub, Urc>, Error>;

    fn max_urc_len() -> usize;
}

pub struct UrcChannel<
    Urc: AtatUrc,
    const INGRESS_BUF_SIZE: usize,
    const CAPACITY: usize,
    const SUBSCRIBERS: usize,
>(pub(crate) PubSubChannel<CriticalSectionRawMutex, Urc::Response, CAPACITY, SUBSCRIBERS, 1>);

impl<
        Urc: AtatUrc,
        const INGRESS_BUF_SIZE: usize,
        const CAPACITY: usize,
        const SUBSCRIBERS: usize,
    > UrcChannel<Urc, INGRESS_BUF_SIZE, CAPACITY, SUBSCRIBERS>
{
    pub(crate) const fn new() -> Self {
        Self(PubSubChannel::new())
    }
}

impl<
        Urc: AtatUrc,
        const INGRESS_BUF_SIZE: usize,
        const CAPACITY: usize,
        const SUBSCRIBERS: usize,
    > AtatUrcChannel<Urc> for UrcChannel<Urc, INGRESS_BUF_SIZE, CAPACITY, SUBSCRIBERS>
{
    fn subscribe<'sub>(&'sub self) -> Result<UrcSubscription<'sub, Urc>, Error> {
        self.0
            .dyn_subscriber()
            .map_err(|_| Error::MaximumSubscribersReached)
    }

    fn max_urc_len() -> usize {
        INGRESS_BUF_SIZE
    }
}
