struct Tcat {
    name: String,
    age: u32,
    grades: Vec<u32>,
}

impl Tcat {
    fn new(name: &str, age: u32, grades: Vec<u32>) -> Tcat {
        Tcat {
            name: name.to_string(),
            age,
            grades,
        }
    }

    fn average_grade(&self) -> f64 {
        let sum: u32 = self.grades.iter().sum();
        let count = self.grades.len() as f64;
        sum as f64 / count
    }

    fn add_grade(&mut self, grade: u32) {
        self.grades.push(grade);
    }
}

fn main() {
    // println!("Hello, world!");
    // Test new
    let mut tcat1 = Tcat::new("Kyoyasuo", 20, vec![85, 90, 78]);
    assert_eq!(tcat1.name, "Kyoyasuo");
    assert_eq!(tcat1.age, 20);
    assert_eq!(tcat1.grades, vec![85, 90, 78]);

    let tcat2 = Tcat::new("KMS", 22, vec![70, 80, 90]);
    assert_eq!(tcat2.name, "KMS");
    assert_eq!(tcat2.age, 22);
    assert_eq!(tcat2.grades, vec![70, 80, 90]);

    //Test average_grade
    assert_eq!(tcat1.average_grade(), 84.33333333333333);
    assert_eq!(tcat2.average_grade(), 80.0);

    //Test add_grade
    tcat1.add_grade(95);
    assert_eq!(tcat1.grades, vec![85, 90, 78, 95]);

    println!("All tests passed");
}
