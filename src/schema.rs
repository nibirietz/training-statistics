pub trait Exercise {
    fn get_name(&self) -> &str;
    fn get_approaches(&self) -> u32;
    fn get_repetitions(&self) -> u32;
}

impl<'a> Exercise for ExerciseCreate<'a> {
    fn get_name(&self) -> &str {
        self.name
    }
    fn get_approaches(&self) -> u32 {
        self.approaches
    }
    fn get_repetitions(&self) -> u32 {
        self.repetitions
    }
}
impl<'a> Exercise for ExercisePublic<'a> {
    fn get_name(&self) -> &str {
        self.name
    }
    fn get_approaches(&self) -> u32 {
        self.approaches
    }
    fn get_repetitions(&self) -> u32 {
        self.repetitions
    }
}
impl<'a> Exercise for ExerciseEdit<'a> {
    fn get_name(&self) -> &str {
        self.name
    }
    fn get_approaches(&self) -> u32 {
        self.approaches
    }
    fn get_repetitions(&self) -> u32 {
        self.repetitions
    }
}

pub struct ExercisePublic<'a> {
    id: u32,
    name: &'a str,
    approaches: u32,
    repetitions: u32,
}

pub struct ExerciseCreate<'a> {
    name: &'a str,
    approaches: u32,
    repetitions: u32,
}

pub struct ExerciseEdit<'a> {
    id: u32,
    name: &'a str,
    approaches: u32,
    repetitions: u32,
}

impl<'a> ExerciseCreate<'a> {
    fn new(name: &'a str, approaches: u32, repetitions: u32) -> ExerciseCreate<'a> {
        Self {
            name: name,
            approaches: approaches,
            repetitions: repetitions,
        }
    }
}

impl<'a> ExercisePublic<'a> {
    fn new(id: u32, name: &'a str, approaches: u32, repetitions: u32) -> ExercisePublic<'a> {
        Self {
            id: id,
            name: name,
            approaches: approaches,
            repetitions: repetitions,
        }
    }
    pub fn get_id(&self) -> &str {
        self.name
    }
}
