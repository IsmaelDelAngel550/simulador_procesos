/// Priority (non-preemptive) scheduling algorithm.
///
/// Selects the process with the highest priority (lowest numeric value)
/// from the ready queue. Only evaluated when the CPU becomes free.
/// Priority 0 = highest (kernel), 1 = high, 10 = lowest.

use std::collections::VecDeque;

use crate::process::PCB;
use super::SchedulingAlgorithm;

pub struct PriorityNonPreemptive;

impl SchedulingAlgorithm for PriorityNonPreemptive {
    fn select_next(&self, ready_queue: &VecDeque<PCB>) -> Option<usize> {
        if ready_queue.is_empty() {
            return None;
        }

        let mut best_idx: Option<usize> = None;
        let mut best_priority = u8::MAX;

        for (i, proc) in ready_queue.iter().enumerate() {
            if proc.is_kernel_daemon() {
                continue;
            }
            if proc.priority < best_priority {
                best_priority = proc.priority;
                best_idx = Some(i);
            }
        }

        best_idx.or(Some(0))
    }

    fn name(&self) -> &'static str {
        "Prioridad"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::ProcessState;

    fn make_pcb(pid: u32, priority: u8) -> PCB {
        PCB {
            pid,
            name: format!("P{}", pid),
            state: ProcessState::Ready,
            burst_time: 10,
            remaining_time: 10,
            arrival_time: 0,
            priority,
            memory_mb: 64.0,
            io_burst: None,
            finish_time: None,
            turnaround_time: None,
            waiting_time: None,
        }
    }

    #[test]
    fn selects_highest_priority() {
        let algo = PriorityNonPreemptive;
        let mut queue = VecDeque::new();
        queue.push_back(make_pcb(1, 5));
        queue.push_back(make_pcb(2, 1));  // highest priority
        queue.push_back(make_pcb(3, 8));
        assert_eq!(algo.select_next(&queue), Some(1)); // PID 2, priority=1
    }
}
