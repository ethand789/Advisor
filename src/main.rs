use std::io;

fn main() {
    let mut input = String::new();
    let last_command = String::from("begin new game");
    let exit_phrase = "exit";

    while(last_command != exit_phrase){

        //TODO: create command reader

        println!("please enter your next command (enter '{}' to exit)", exit_phrase);
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read command");

        let last_command = last_input(input.clone());

        print!("{} \n\n last command: {}",input,last_command);

    }
}

fn last_input(input:String) -> String {
    let mut line = input.lines();
    let mut next = line.next();
    let last = "if you see this text, Ethan fucked up";


    while next != None{
        let last: Option<String> = next.map(|s| s.to_string());
        next = line.next();
    }
    return String::from(last);
}

