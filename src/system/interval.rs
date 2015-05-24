
use DataHelper;
use EntityData;
use {Process, System};

/// System which operates every certain number of updates.
pub struct IntervalSystem<T: Process>
{
    interval: u8,
    ticker: u8,
    inner: T,
}

impl<T: Process> IntervalSystem<T>
{
    /// Create a new interval system with the specified number of updates between processes.
    pub fn new(system: T, interval: u8) -> IntervalSystem<T>
    {
        IntervalSystem
        {
            interval: interval,
            ticker: 0,
            inner: system,
        }
    }
}

impl<T: Process> Process for IntervalSystem<T>
{
    fn process(&mut self, c: &mut DataHelper<T::Components, T::Services>)
    {
        self.ticker += 1;
        if self.ticker == self.interval
        {
            self.ticker = 0;
            self.inner.process(c);
        }
    }
}

impl<T: Process> System for IntervalSystem<T>
{
    type Components = T::Components;
    type Services = T::Services;
    fn activated(&mut self, e: &EntityData<T::Components>, w: &T::Components)
    {
        self.inner.activated(e, w);
    }

    fn reactivated(&mut self, e: &EntityData<T::Components>, w: &T::Components)
    {
        self.inner.reactivated(e, w);
    }

    fn deactivated(&mut self, e: &EntityData<T::Components>, w: &T::Components)
    {
        self.inner.deactivated(e, w);
    }

    fn is_active(&self) -> bool
    {
        self.inner.is_active()
    }
}
