
mod Student;

fn main() {
    println!("Hello, world!");
    
    Student::static_hello();
    
    let mut s1 = Student::Student::new(1, "Don", "Quixote");
    
    println!("{}", s1.toString());
    
    s1.addGrade(50, 37);
    
    println!("{}", s1.toString());
}
