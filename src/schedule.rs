use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

/* ----------------------------------- Job ---------------------------------- */
pub struct Job {
    pub timestamp: Instant,
    pub module_index: usize,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp.eq(&other.timestamp)
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.timestamp.cmp(&other.timestamp) {
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => Ordering::Greater,
        }
    }
}

/* -------------------------------- Schedule -------------------------------- */
pub struct Schedule {
    queue: BinaryHeap<Job>,
    count: u64,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            count: 0,
        }
    }

    pub fn push_job(&mut self, timestamp: Instant, module_index: usize) {
        self.count += 1;
        self.queue.push(Job {
            timestamp,
            module_index,
        })
    }

    pub fn pop(&mut self) -> Option<Job> {
        self.queue.pop()
    }

    pub fn next_timestamp(&self) -> Option<Instant> {
        self.queue.peek().map(|j| j.timestamp)
    }
}
