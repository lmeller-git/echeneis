use crate::build_test::Model;

pub(crate) mod linear_pairwise;

pub(crate) trait TestSchedule<I, F, D> {
    fn check_model(&mut self, model: Model<I, F, D>);
}
