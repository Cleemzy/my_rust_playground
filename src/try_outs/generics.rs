struct User<T>{
    name: String,
    notes: T
}

impl<T: std::fmt::Display + std::fmt::Debug> User<T>{
    pub fn take_note_from_other(self, other: User<T>){
        let my_current_notes = self.notes;
        let other_s_name = other.name;
        let other_s_notes = other.notes;
        let new_notes = format!("My current notes : {:?} \n - Notes from {:?}  : {:?}", my_current_notes, other_s_name, other_s_notes);
    
        // self.notes = new_notes as T;
    }
}