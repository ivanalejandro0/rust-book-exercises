use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        // format: add person to department
        println!("Please enter instruction:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input == "end" {
            break;
        }

        let tokens: Vec<&str> = input.split(' ').collect();
        println!("{:?}", tokens);

        let person = String::from(tokens[1]);
        let department_name = String::from(tokens[3]);

        let department = company.entry(department_name).or_insert(vec![]);
        department.push(person);
    }

    println!("{:#?}", company);
}
