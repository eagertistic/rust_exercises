// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example: add sally to engineering
// Let user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically

use std::{
    collections::{BTreeMap, BTreeSet},
    io::{self, Write},
};

const HELP_MESSAGE: &str = r#"
Available commands:
  - 'Add <Firstname> <Lastname> to <Department>' to do exactly that
  - 'List <department>` to list every employee within this department
  - 'List all' to list every employee in the company
  - 'Quit' to stop AdminCLI
"#;

enum Command {
    Add {
        firstname: String,
        lastname: String,
        dept: String,
    },
    ListDepartment(String),
    ListAll,
    Quit,
    Unknown,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Employee {
    firstname: String,
    lastname: String,
}

fn main() {
    println!("Welcome to AdminCLI!");

    let mut company: BTreeMap<String, BTreeSet<Employee>> = BTreeMap::new();
    let mut input = String::new();

    loop {
        println!("{}", HELP_MESSAGE);
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("\nerror: unable to read your input");

        let command = parse_command(&input);

        match command {
            Command::Add {
                firstname,
                lastname,
                dept,
            } => {
                company.entry(dept).or_default().insert(Employee {
                    firstname,
                    lastname,
                });
                println!("Successfully added the employee!");
            }
            Command::ListAll => {
                for (dept, names) in &company {
                    println!("\n[{}]", dept);
                    for name in names {
                        println!("{} {}", name.firstname, name.lastname);
                    }
                }
            }
            Command::ListDepartment(dept) => match company.get(&dept) {
                Some(names) => {
                    println!("\n[{}]", dept);
                    for name in names {
                        println!("    {} {}", name.firstname, name.lastname);
                    }
                }
                None => {
                    println!("\n'{}' department is not recognized", dept);
                    continue;
                }
            },
            Command::Quit => {
                println!("\nAdminCLI stopped ... Have a nice day\n");
                break;
            }
            Command::Unknown => println!("\nunknown command, use only the defined commands"),
        }
    }
}

fn parse_command(input: &str) -> Command {
    let words: Vec<&str> = input.split_whitespace().collect();

    match words.as_slice() {
        [cmd, firstname, lastname, to, dept]
            if cmd.eq_ignore_ascii_case("add") && to.eq_ignore_ascii_case("to") =>
        {
            Command::Add {
                firstname: firstname.to_ascii_lowercase(),
                lastname: lastname.to_ascii_lowercase(),
                dept: dept.to_ascii_lowercase(),
            }
        }
        [cmd, all] if cmd.eq_ignore_ascii_case("list") && all.eq_ignore_ascii_case("all") => {
            Command::ListAll
        }
        [cmd, dept] if cmd.eq_ignore_ascii_case("list") => {
            Command::ListDepartment(dept.to_ascii_lowercase())
        }
        [cmd] if cmd.eq_ignore_ascii_case("quit") => Command::Quit,
        _ => Command::Unknown,
    }
}
