use crate::schema::{self, ExercisePublic};
struct Repository {}

impl Repository {
    pub fn initialiaze_repository() {
        todo!();
    }
    pub fn get_exercise_by_id<'a>(id: u32) -> ExercisePublic<'a> {
        todo!();
    }
    pub fn create_exercise(exercise: schema::ExerciseCreate) {
        todo!();
    }
    pub fn edit_exercise(exercise: schema::ExerciseEdit) {
        todo!();
    }
}
