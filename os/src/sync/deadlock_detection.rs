use alloc::{collections::vec_deque::VecDeque, vec::Vec};

/// Deadlock detection
#[derive(Debug)]
pub struct Banker<R> {
    // resource id is the index in the resources vector
    resources: Vec<Option<R>>, // None means the resource is deallocated
    recycle: VecDeque<usize>, // recycle resource id
    num_tasks: usize,
    // [task][resource]
    max: Vec<Vec<usize>>,
    allocated: Vec<Vec<usize>>,
    available: Vec<usize>,
}

impl <R: Eq + Copy> Banker<R> {
    /// Create a new DeadlockDetection
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            recycle: VecDeque::new(),
            num_tasks: 0,
            max: Vec::new(),
            allocated: Vec::new(),
            available: Vec::new(),
        }
    }

    /// Add a task to the allocated list
    pub fn add_task(&mut self) -> usize {
        self.num_tasks += 1;
        self.max.push(alloc::vec![0; self.resources.len()]);
        self.allocated.push(alloc::vec![0; self.resources.len()]);
        self.num_tasks
    }

    /// Remove a task from the allocated list
    pub fn remove_task(&mut self, task_id: usize) -> bool {
        if task_id >= self.num_tasks {
            return false;
        }
        self.max[task_id].fill(0);
        self.allocated[task_id].fill(0);
        true
    }

    /// Add a task to the allocated list
    pub fn add_resource(&mut self, resource: R, total: usize) {
        if let Some(id) = self.recycle.pop_front() {
            self.resources[id] = Some(resource);
            self.available[id] = total;
            return;
        }

        self.resources.push(Some(resource));
        self.available.push(total);
    }

    pub fn remove_resource(&mut self, resource: R) -> bool {
        if let Some(id) = self.resource_id(resource) {
            self.resources[id] = None;
            self.recycle.push_back(id);
            return true;
        }
        false
    }

    pub fn resource_id(&self, resource: R) -> Option<usize> {
        self.resources.iter().position(|res| res == &Some(resource))
    }

    pub fn request(&mut self, task_id: usize, resource: R, amount: usize) -> bool {
        if task_id >= self.num_tasks {
            return false;
        }

        let Some(resource_id) = self.resource_id(resource) else {
            return false;
        };

        if self.allocated[task_id][resource_id] + amount > self.max[task_id][resource_id] {
            return false;
        }

        if amount > self.available[resource_id] {
            return false;
        }

        self.allocated[task_id][resource_id] += amount;
        true
    }

    pub fn release(&mut self, task_id: usize, resource: R, amount: usize) -> bool {
        if task_id >= self.num_tasks {
            return false;
        }

        let Some(resource_id) = self.resource_id(resource) else {
            return false;
        };

        if amount > self.allocated[task_id][resource_id] {
            return false;
        }

        self.allocated[task_id][resource_id] -= amount;
        true
    }

    pub fn is_safe(&self, task_id: usize) -> bool {
        let mut work = self.available.clone();
        let mut finish = alloc::vec![false; self.num_tasks];

        for i in 0..self.num_tasks {
            if finish[i] {
                continue;
            }

            let mut can_finish = true;
            for j in 0..self.resources.len() {
                if self.max[i][j] - self.allocated[i][j] > work[j] {
                    can_finish = false;
                    break;
                }
            }

            if can_finish {
                for j in 0..self.resources.len() {
                    work[j] += self.allocated[i][j];
                }
                finish[i] = true;
            }
        }

        finish[task_id]
    }
}