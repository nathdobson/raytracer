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
use std::{mem, thread};
use std::sync::Arc;
use std::time::Duration;
use rayon::iter::Map;
use crate::util::counter::Counter;

type ProgressAs<I: IndexedParallelIterator> = impl IndexedParallelIterator<Item=I::Item>;

pub trait IndexedParallelIteratorExt: IndexedParallelIterator {
    fn progress_as(self, message: &'static str) -> ProgressAs<Self>;
}

impl<T: IndexedParallelIterator> IndexedParallelIteratorExt for T {
    fn progress_as(self, message: &'static str) -> ProgressAs<Self> {
        let mut pb = ProgressBar::new(self.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .on_finish(ProgressFinish::AndLeave)
                .progress_chars("▓▓#")//▒░
                .template("[{elapsed_precise}] {wide_bar:.green/yellow} {pos:>10}/{len:10} {msg}")
        );
        pb.set_message(message);
        let counter = Arc::new(Counter::new());
        thread::spawn({
            let pb = pb.downgrade();
            let counter = counter.clone();
            move || {
                while let Some(pb) = pb.upgrade() {
                    pb.set_position(counter.total());
                    mem::drop(pb);
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });
        self.map(move |x| {
            mem::drop(&pb);
            counter.inc();
            x
        })
    }
}
