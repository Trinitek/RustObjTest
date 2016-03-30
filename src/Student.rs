#![allow(non_snake_case)]

pub struct Student<'a> {
    id:             u32,
    firstname:      &'a str,
    lastname:       &'a str,
    pointsPossible: u32,
    pointsEarned:   u32,
}

impl<'a> Student<'a> {
    pub fn new(
        id:             u32,
        firstname:      &'a str,
        lastname:       &'a str,
    ) -> Student<'a>
    {
        Student {
            id:             id,
            firstname:      firstname,
            lastname:       lastname,
            pointsPossible: 0,
            pointsEarned:   0,
        }
    }
    
    pub fn addGrade(
        &mut self,
        pointsPossible: u32,
        pointsEarned:   u32,
    )
    {
        self.pointsPossible += pointsPossible;
        self.pointsEarned += pointsEarned;
    }

    fn gradeAverage(&self) -> f64 {
        if self.pointsPossible != 0 {
            (self.pointsEarned as f64 / self.pointsPossible as f64) * 100.0
        } else {
            0.0
        }
    }
    
    fn letterGrade(&self) -> char {
        let grade = self.gradeAverage();
        if grade >= 90.0 {
            'A'
        } else if grade >= 80.0 {
            'B'
        } else if grade >= 70.0 {
            'C'
        } else if grade >= 60.0 {
            'D'
        } else {
            'F'
        }
    }
    
    pub fn toString(&self) -> String {
        format!(
            "{}: {}, {}: {}% ({})",
            self.id, self.lastname, self.firstname,
            self.gradeAverage(), self.letterGrade()
        )
    }
}

pub fn static_hello() {
    println!("Hello from the Student.rs file!");
}
