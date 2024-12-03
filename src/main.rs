
fn sayHi(){
    println!("Hello Every One !!!!!");
}

fn sumValue(a:u32 , b:u32){
    println!("Sum of this Number is {}",a+b);
}

fn increment(i: &mut u32){
*i +=1;
}


fn returnNumber() -> u32{
    return 45;
}

fn checkAge(age:i32) -> bool{
    if age >=18 {
        return true;
    }
    return false; // without the return also return the value
    // Implicit return
}


struct Student {
    name: String,
    age: u32,
    grades: Vec<f32>,
    is_enrolled: bool,
}

// Implementation block for Student methods
impl Student {
    // Constructor method
    fn new(name: String, age: u32) -> Student {
        Student {
            name,
            age,
            grades: Vec::new(),
            is_enrolled: true,
        }
    }

    // Add a grade method
    fn add_grade(&mut self, grade: f32) {
        self.grades.push(grade);
    }

    // Calculate average grade
    fn get_average(&self) -> Option<f32> {
        if self.grades.is_empty() {
            None
        } else {
            let sum: f32 = self.grades.iter().sum();
            Some(sum / self.grades.len() as f32)
        }
    }

    // Display student info
    fn display_info(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Enrolled: {}", self.is_enrolled);
        println!("Grades: {:?}", self.grades);
        
        match self.get_average() {
            Some(avg) => println!("Average grade: {:.2}", avg),
            None => println!("No grades yet")
        }
    }
}

fn main() {
    let name = "Alice"; // immutable
    // name ="jai" // error due to changing the immutable value
    let mut age = 25; // mutable value
    println!("Hello, world! {} -- {}",name,age); // printing the value
    age = 40; // changing the mutable values
    let mut greeting = String::from("Hello");
    let welcome = String::from("KKKK");

    greeting.push_str(", world!");
    println!("Hello, world! {} -- {}-> {} --{} --{}",name, age, greeting, greeting.len(),welcome);

    let mut changingName = String::from("Good");
    println!("{}",changingName);

    changingName = String::from("Morning");
    println!("changed text ->  {}",changingName);

    let hello = String::from("Hello ");
    let mut world  = String::from("World!");
    let message = hello + &world;
    println!("Merged Text -1 {}",message);
    world = String::from("Man");
    println!("Merged Text -2 {} --{}",message,world);
    let value1:i8 = -98;
    let value:u64 = 89; // u (unsigned) only positive values
    println!("Int Data {} , {}", value,value1);


    // ARRAY
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:#?}",numbers); // to get in the mutiple lines
    println!("{:?}",numbers); // in single line

for i in numbers{
    println!("number is - {}",i);
}


let mut numbers = [1, 2, 3, 4, 5];

// Modify elements in place
for num in &mut numbers {
    println!("refff -- {}",num);
    *num *= 2; // *num is dereference to modify the actual value
}

println!("{:?}",numbers);


sayHi();
sumValue(10,20);
let mut k  = 5;
increment(&mut k);



let mut i = 5;
let ref_i = &mut i; // addrress of the i 
println!("ref value {}",ref_i);
*ref_i = 10;       // accessing the value in the ref_i can changing the value
println!("i is now {}", i);  // Prints "i is now 10"





println!("{}",returnNumber());

println!("Is ready to vote {}",checkAge(45));



let mut numbers = vec![1, 2, 3, 4, 5];         // Using vec! macro
let mut names: Vec<String> = Vec::new();       // Empty vector

// names.push("Rajesh"); we must us the String::from("Rajesh")
names.push(String::from("Rajesh"));
numbers.push(5);
println!("{:?}",names);
println!("{:?}",numbers);


let mut student1 = Student::new(String::from("Alice"), 20);
    
// Add some grades
student1.add_grade(85.0);
student1.add_grade(92.5);
student1.add_grade(88.0);

// Display student information
student1.display_info();

// Create another student directly
let mut student2 = Student {
    name: String::from("Bob"),
    age: 22,
    grades: vec![90.0, 85.5, 87.0],
    is_enrolled: true,
};

student2.display_info();


}
