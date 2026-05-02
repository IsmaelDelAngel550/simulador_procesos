/// FCFS (First-Come, First-Served) scheduling algorithm.
///
/// Selects the first process in the ready queue (FIFO order).
/// Non-preemptive: once a process starts, it runs until completion or I/O.

use std::collections::VecDeque;

use crate::process::PCB;
use super::SchedulingAlgorithm;

pub struct Fcfs;

impl SchedulingAlgorithm for Fcfs {
    fn select_next(&self, ready_queue: &VecDeque<PCB>) -> Option<usize> {
        if ready_queue.is_empty() {
            None
        } else {
            // Skip kernel daemon if there are other processes
            let non_kernel = ready_queue
                .iter()
                .position(|p| !p.is_kernel_daemon());
            non_kernel.or(Some(0))
        }
    }

    fn name(&self) -> &'static str {
        "FCFS"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::ProcessState;

    fn make_pcb(pid: u32, burst: u32) -> PCB {
        PCB {
            pid,
            name: format!("P{}", pid),
            state: ProcessState::Ready,
            burst_time: burst,
            remaining_time: burst,
            arrival_time: 0,
            priority: 5,
            memory_mb: 64.0,
            io_burst: None,
            finish_time: None,
            turnaround_time: None,
            waiting_time: None,
        }
    }

    #[test]
    fn selects_first_in_queue() {
        let algo = Fcfs;
        let mut queue = VecDeque::new();
        queue.push_back(make_pcb(1, 10));
        queue.push_back(make_pcb(2, 5));
        queue.push_back(make_pcb(3, 20));
        assert_eq!(algo.select_next(&queue), Some(0));
    }

    #[test]
    fn empty_queue_returns_none() {
        let algo = Fcfs;
        let queue = VecDeque::new();
        assert_eq!(algo.select_next(&queue), None);
    }
}
