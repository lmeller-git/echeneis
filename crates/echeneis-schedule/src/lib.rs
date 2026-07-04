pub trait Schedule {
    fn next_runnable(&mut self, &[&Task]) -> Option<Task>;
}


// assign checker a random prio at init,
// assign top N prios to init tasks
// run pct
// at point x checker runs

pub struct LazyPCT {
    threads: HashMap<TaskID, usize>,
    checker: Task,
    priorities: SmallVec<[usize; N]>,
    depth: usize,
    config: Config,
    change_points: SmallVec<[usize; N]>,
    next_prio: usize,
}


impl Schedule for LazyPCT {
    }
