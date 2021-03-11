#![feature(result_into_ok_or_err, default_free_fn)]

mod counter;
mod map;

pub use crate::counter::SmallCounter;
pub use crate::map::{
    entry::{Entry, OccupiedEntry, VacantEntry},
    iter::ValuesIter,
    SmallSortedMap,
};
