//! Types related to task management

use super::TaskContext;
use super::MAX_SYSCALL_NUM;
use alloc::collections::BTreeMap;

#[derive(Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    pub exec_start_time: usize,
    pub syscall_times: BTreeMap<usize, u32>,
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
