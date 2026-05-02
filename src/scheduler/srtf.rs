/// SRTF (Shortest Remaining Time First) scheduling algorithm.
///
/// Preemptive version of SJF. At each tick, compares the remaining time
/// of the current process with all processes in the ready queue. If a
/// process with shorter remaining time exists, preempts the current one.

use std::collections::VecDeque;

use crate::process::PCB;
use super::SchedulingAlgorithm;

pub struct Srtf;

impl SchedulingAlgorithm for Srtf {
    fn select_next(&self, ready_queue: &VecDeque<PCB>) -> Option<usize> {
        if ready_queue.is_empty() {
            return None;
        }

        let mut best_idx: Option<usize> = None;
        let mut best_remaining = u32::MAX;

        for (i, proc) in ready_queue.iter().enumerate() {
            if proc.is_kernel_daemon() {
                continue;
            }
            if proc.remaining_time < best_remaining {
                best_remaining = proc.remaining_time;
                best_idx = Some(i);
            }
        }

        best_idx.or(Some(0))
    }

    fn should_preempt(&self, current: &PCB, ready_queue: &VecDeque<PCB>) -> bool {
        if current.is_kernel_daemon() {
            // Kernel daemon should yield if there are user processes
            return ready_queue.iter().any(|p| !p.is_kernel_daemon());
        }

        // Check if any process in ready queue has shorter remaining time
        ready_queue
            .iter()
            .filter(|p| !p.is_kernel_daemon())
            .any(|p| p.remaining_time < current.remaining_time)
    }

    fn name(&self) -> &'static str {
        "SRTF"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::ProcessState;

    fn make_pcb(pid: u32, remaining: u32) -> PCB {
        PCB {
            pid,
            name: format!("P{}", pid),
            state: ProcessState::Ready,
            burst_time: remaining + 5,
            remaining_time: remaining,
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
    fn preempts_when_shorter_arrives() {
        let algo = Srtf;
        let current = make_pcb(1, 10);
        let mut queue = VecDeque::new();
        queue.push_back(make_pcb(2, 3));

        assert!(algo.should_preempt(&current, &queue));
    }

    #[test]
    fn no_preempt_when_current_is_shortest() {
        let algo = Srtf;
        let current = make_pcb(1, 2);
        let mut queue = VecDeque::new();
        queue.push_back(make_pcb(2, 10));

        assert!(!algo.should_preempt(&current, &queue));
    }
}
