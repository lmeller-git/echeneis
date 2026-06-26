pub(crate) mod env;
pub(crate) mod thread;

pub(crate) trait TaskHandle {
    fn yield_now(&mut self, payload: YieldData);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum YieldData {
    AtomicTransition,
    Complete,
    Terminated,
}

pub(crate) struct CheckedTaskHandle {
    current_steps: usize,
    max_steps: usize,
}

impl CheckedTaskHandle {
    pub(crate) fn new(max_steps: usize) -> Self {
        Self {
            current_steps: 0,
            max_steps,
        }
    }

    pub(crate) fn step(&mut self) {
        self.current_steps += 1;

        if self.current_steps > self.max_steps {
            panic!(
                "Maximum steps exceeded\n\
                 Exceeded allowed limit of {} atomic transitions.
                 Did you call a spin-loop or similar somewhere?",
                self.max_steps
            );
        }
    }
}

impl TaskHandle for CheckedTaskHandle {
    fn yield_now(&mut self, payload: YieldData) {
        match payload {
            YieldData::AtomicTransition => self.step(),
            YieldData::Complete => {}
            YieldData::Terminated => panic!("terminated"),
        }
    }
}
