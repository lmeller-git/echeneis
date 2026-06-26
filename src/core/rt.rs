use std::panic::Location;

pub(crate) mod env;
pub(crate) mod thread;

pub(crate) trait TaskHandle {
    fn yield_now(&mut self, payload: YieldData);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum YieldData {
    AtomicTransition(Option<&'static Location<'static>>),
    Complete,
    Terminated,
}

pub(crate) struct CheckedTaskHandle {
    current_steps: usize,
    max_steps: usize,
    preempted_location: Option<&'static Location<'static>>,
}

impl CheckedTaskHandle {
    pub(crate) fn new(max_steps: usize) -> Self {
        Self {
            current_steps: 0,
            max_steps,
            preempted_location: None,
        }
    }

    pub(crate) fn with_preempted_loc(
        mut self,
        preempted_location: &'static Location<'static>,
    ) -> Self {
        self.preempted_location.replace(preempted_location);
        self
    }

    pub(crate) fn step(&mut self, loc: Option<&'static Location<'static>>) {
        self.current_steps += 1;
        if self.current_steps > self.max_steps {
            panic!(
                "Maximum steps exceeded {}.\n\
                 Exceeded allowed limit of {} atomic transitions {}.
                 Did you call a spin-loop or similar somewhere?",
                if let Some(loc) = self.preempted_location {
                    format!(
                        "while another thread was preempted before completing the operation at {}",
                        loc
                    )
                } else {
                    "".into()
                },
                self.max_steps,
                if let Some(loc) = loc {
                    format!("while trying to evaluate the operation at {}", loc)
                } else {
                    "".into()
                }
            );
        }
    }
}

impl TaskHandle for CheckedTaskHandle {
    fn yield_now(&mut self, payload: YieldData) {
        match payload {
            YieldData::AtomicTransition(loc) => self.step(loc),
            YieldData::Complete => {}
            YieldData::Terminated => panic!("terminated"),
        }
    }
}
