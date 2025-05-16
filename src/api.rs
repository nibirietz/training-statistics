use crate::repository::{self, Repository};
use crate::schema::{self, ExerciseCreate, ExercisePublic};

pub struct API {
    repository: repository::Repository,
}

impl API {
    fn new() -> API {
        Self {
            repository: repository::Repository::initialiaze_repository(),
        }
    }
    fn create_exercise(&self, name: &str, approaches: u32, repetitions: u32) {
        let exercise = schema::ExerciseCreate::new(name, approaches, repetitions);
        self.repository.create_exercise(exercise);
    }
    fn edit_exercise(&self, id: u32, name: &str) {
        todo!();
    }
    fn edit_last_exercise(&self, name: &str) {
        let id = self.repository.get_id_of_last_exercise(name);
        self.edit_exercise(id, name);
    }
    fn get_exercise_by_id(&self, name: &str, id: u32) -> ExercisePublic {
        self.repository.get_exercise_by_id(name, id)
    }
}
