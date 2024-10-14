//! Types related to task management

use crate::{config::MAX_SYSCALL_NUM, timer::get_time_ms};

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub enum TaskControlBlock {
    /// The task is uninitialized
    UnInit(TaskInfo),
    /// The task is ready to run
    Ready(TaskInfo),
    /// The task is running
    Running(TaskInfo),
    /// The task has exited
    Exited(TaskInfo),
}

impl TaskControlBlock {
    /// Turn the task into `TaskControlBlock::Running`
    pub fn turn_to_running(&mut self) -> Result<(), &'static str> {
        match self {
            TaskControlBlock::Ready(info) => {
                if info.start_time == 0 {
                    info.start_time = get_time_ms();
                }
                *self = TaskControlBlock::Running(*info);
                Ok(())
            },
            _ => Err("TaskControlBlock::turn_to_running: not a Ready task"),
        }
    }

    /// Turn the task into `TaskControlBlock::Ready`
    pub fn try_turn_to_ready(&mut self) -> Result<(), &'static str> {
        match self {
            TaskControlBlock::Running(info) => {
                *self = TaskControlBlock::Ready(*info);
                Ok(())
            },
            _ => Err("TaskControlBlock::turn_to_ready: not a Running task"),
        }
    }

    /// Turn the task into `TaskControlBlock::Exited`
    pub fn turn_to_exited(&mut self) {
        if let TaskControlBlock::Running(info)
            | TaskControlBlock::Ready(info)
            | TaskControlBlock::UnInit(info) = self {
            *self = TaskControlBlock::Exited(*info);
        }
    }

    /// Get the task context
    pub fn cx(&self) -> &TaskContext {
        match self {
            TaskControlBlock::Ready(info) 
            | TaskControlBlock::Running(info) 
            | TaskControlBlock::Exited(info)
            | TaskControlBlock::UnInit(info) => &info.task_cx,
        }
    }

    /// Check if the task is ready
    pub fn is_ready(&self) -> bool {
        match self {
            TaskControlBlock::Ready(_) => true,
            _ => false,
        }
    }

    /// Get the status of the task
    pub fn status(&self) -> TaskStatus {
        match self {
            TaskControlBlock::UnInit(_) => TaskStatus::UnInit,
            TaskControlBlock::Ready(_) => TaskStatus::Ready,
            TaskControlBlock::Running(_) => TaskStatus::Running,
            TaskControlBlock::Exited(_) => TaskStatus::Exited,
        }
    }

    /// Get the task information
    pub fn info(&self) -> Option<&TaskInfo> {
        match self {
            TaskControlBlock::Ready(info) | TaskControlBlock::Running(info) => Some(info),
            _ => None,
        }
    }

    /// Increase the syscall times
    pub fn sys_call_inc(&mut self, syscall_id: usize) -> Option<()> {
        match self {
            TaskControlBlock::Ready(info) | TaskControlBlock::Running(info) => {
                info.sys_call_inc(syscall_id);
                Some(())
            },
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// The task context
    pub task_cx: TaskContext,
    /// The start time of the task
    pub start_time: usize,
    /// The number of syscalls called by the task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
}

impl TaskInfo {
    pub fn new(task_cx: TaskContext, start_time: usize) -> Self {
        Self {
            task_cx,
            start_time,
            syscall_times: [0; MAX_SYSCALL_NUM],
        }
    }

    /// increase the syscall times
    pub fn sys_call_inc(&mut self, syscall_id: usize) {
        self.syscall_times[syscall_id] += 1;
    }
}

/// The status of a task
#[derive(Copy, Clone, Debug)]
pub enum TaskStatus {
    /// The task is uninitialized
    UnInit,
    /// The task is ready to run
    Ready,
    /// The task is running
    Running,
    /// The task has exited
    Exited,
}
