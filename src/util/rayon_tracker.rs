use indicatif::ProgressBar;
use indicatif::ProgressFinish;
use indicatif::ProgressStyle;
use rayon::prelude::IndexedParallelIterator;
use rayon::iter::{
    plumbing::{Consumer, Folder, Producer, ProducerCallback, UnindexedConsumer},
    ParallelIterator,
};
use std::convert::TryFrom;
use std::iter::FusedIterator;

pub trait Tracker: Send + Clone {
    fn inc(&self, v: u64);
}

pub struct TrackerIter<T, F> {
    it: T,
    tracker: F,
}

impl<S, T: Iterator<Item=S>, F: Tracker> Iterator for TrackerIter<T, F> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.it.next();

        if item.is_some() {
            todo!()
        } else {
            todo!()
        }

        item
    }
}

impl<T: ExactSizeIterator, F: Tracker> ExactSizeIterator for TrackerIter<T, F> {
    fn len(&self) -> usize {
        self.it.len()
    }
}

impl<T: DoubleEndedIterator, F: Tracker> DoubleEndedIterator for TrackerIter<T, F> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.it.next_back();

        if item.is_some() {
            todo!()
        }

        item
    }
}

impl<T: FusedIterator, F: Tracker> FusedIterator for TrackerIter<T, F> {}


/// Wraps a Rayon parallel iterator.
///
/// See [`ProgressIterator`](trait.ProgressIterator.html) for method
/// documentation.
pub trait ParallelProgressIterator
    where
        Self: Sized + ParallelIterator,
{
    /// Wrap an iterator with a custom progress bar.
    fn track_with<F>(self, f: F) -> TrackerIter<Self, F>;
}

impl<S: Send, T: ParallelIterator<Item=S>> ParallelProgressIterator for T {
    fn track_with<F>(self, f: F) -> TrackerIter<Self, F> {
        TrackerIter { it: self, tracker: f }
    }
}

impl<S: Send, T: IndexedParallelIterator<Item=S>, F: Tracker> IndexedParallelIterator for TrackerIter<T, F> {
    fn len(&self) -> usize {
        self.it.len()
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> <C as Consumer<Self::Item>>::Result {
        let consumer = ProgressConsumer::new(consumer, self.tracker);
        self.it.drive(consumer)
    }

    fn with_producer<CB: ProducerCallback<Self::Item>>(
        self,
        callback: CB,
    ) -> <CB as ProducerCallback<Self::Item>>::Output {
        return self.it.with_producer(Callback {
            callback,
            tracker: self.tracker,
        });

        struct Callback<CB, F> {
            callback: CB,
            tracker: F,
        }

        impl<T, CB: ProducerCallback<T>, F: Tracker> ProducerCallback<T> for Callback<CB, F> {
            type Output = CB::Output;

            fn callback<P>(self, base: P) -> CB::Output
                where
                    P: Producer<Item=T>,
            {
                let producer = ProgressProducer {
                    base,
                    tracker: self.tracker,
                };
                self.callback.callback(producer)
            }
        }
    }
}

struct ProgressProducer<T, F> {
    base: T,
    tracker: F,
}

impl<T, P: Producer<Item=T>, F: Tracker> Producer for ProgressProducer<P, F> {
    type Item = T;
    type IntoIter = TrackerIter<P::IntoIter, F>;

    fn into_iter(self) -> Self::IntoIter {
        TrackerIter {
            it: self.base.into_iter(),
            tracker: self.tracker,
        }
    }

    fn min_len(&self) -> usize {
        self.base.min_len()
    }

    fn max_len(&self) -> usize {
        self.base.max_len()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.base.split_at(index);
        (
            ProgressProducer {
                base: left,
                tracker: self.tracker.clone(),
            },
            ProgressProducer {
                base: right,
                tracker: self.tracker,
            },
        )
    }
}

struct ProgressConsumer<C, F> {
    base: C,
    tracker: F,
}

impl<C, F: Tracker> ProgressConsumer<C, F> {
    fn new(base: C, tracker: F) -> Self {
        ProgressConsumer { base, tracker }
    }
}

impl<T, C: Consumer<T>, F: Tracker> Consumer<T> for ProgressConsumer<C, F> {
    type Folder = ProgressFolder<C::Folder, F>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            ProgressConsumer::new(left, self.tracker.clone()),
            ProgressConsumer::new(right, self.tracker),
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        ProgressFolder {
            base: self.base.into_folder(),
            tracker: self.tracker,
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<T, C: UnindexedConsumer<T>, F: Tracker> UnindexedConsumer<T> for ProgressConsumer<C, F> {
    fn split_off_left(&self) -> Self {
        ProgressConsumer::new(self.base.split_off_left(), self.tracker.clone())
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct ProgressFolder<C, F: Tracker> {
    base: C,
    tracker: F,
}

impl<T, C: Folder<T>, F: Tracker> Folder<T> for ProgressFolder<C, F> {
    type Result = C::Result;

    fn consume(self, item: T) -> Self {
        self.tracker.inc(1);
        ProgressFolder {
            base: self.base.consume(item),
            tracker: self.tracker,
        }
    }

    fn complete(self) -> C::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<S: Send, T: ParallelIterator<Item=S>, F: Tracker> ParallelIterator for TrackerIter<T, F> {
    type Item = S;

    fn drive_unindexed<C: UnindexedConsumer<Self::Item>>(self, consumer: C) -> C::Result {
        let consumer1 = ProgressConsumer::new(consumer, self.tracker.clone());
        self.it.drive_unindexed(consumer1)
    }
}