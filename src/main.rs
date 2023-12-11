mod pentry;

use std::io;
use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;
use crate::pentry::ServiceInfo;

fn clr(){
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"
     _____  _______ _______ _______      _    _ _______ _     _        _______
    |_____] |_____| |______ |______       \  /  |_____| |     | |         |   
    |       |     | ______| ______|        \/   |     | |_____| |_____    |   

    "#;
    let goodbye = r#"
     ______  _____   _____  ______  ______  __   __ _______   /
    |  ____ |     | |     | |     \ |_____]   \_/   |______  / 
    |_____| |_____| |_____| |_____/ |_____]    |    |______ .                                                          
    "#;
    println!("{}",ascii);
    loop{
        println!("Password manager menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim(){
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service:"),
                    prompt("Username:"),
                    prompt("Password:"),
                );
                println!("Entry added Successfuly");
                entry.write_to_file()
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords:{}", err);
                    Vec::new()
                });
                for item in &services{
                    println!("
                        Service = {}
                        - Username : {}
                        - Password : {}",
                        item.service, item.username, item.password
                    ); 
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords:{}", err);
                    Vec::new()
                });
                let search = prompt("Search :");
                for item in &services{
                    if item.service.as_str() == search.as_str(){
                        println!("
                            Service: {}
                            - Username: {}
                            - Password: {}",
                        item.service, item.username, item.password
                        );  
                    }
                }
            }
            "4" => {
                clr();
                println!("{}", goodbye);
                break;
            }
            _ => {
                println!("Invalid Choice");
            }
        }
        println!("\n\n");
    }
}
