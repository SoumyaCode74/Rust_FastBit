fn main() {
    print!("Hello, ");
    println!("world!");
    eprintln!("Sorry! An error occurred!");
    let name = "Soumyadeep";
    let age = 30;
    let marks = 7.81;
    let message = format!(
        "My name is {} and I am {} years old, and secured {} GPA",
        name, age, marks
    );
    println!("{}", message);
}
