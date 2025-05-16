use crate::database;
use crate::schema::{self, ExercisePublic};
pub struct Repository {}

impl Repository {
    pub fn initialiaze_repository() -> Repository {
        todo!();
    }
    pub fn get_exercise_by_id<'a>(&self, name: &str, id: u32) -> ExercisePublic<'a> {
        todo!();
    }
    pub fn create_exercise(&self, exercise: schema::ExerciseCreate) {
        todo!();
    }
    pub fn edit_exercise(&self, exercise: schema::ExerciseEdit) {
        todo!();
    }
    pub fn get_id_of_last_exercise(&self, name: &str) -> u32 {
        todo!();
    }
}
