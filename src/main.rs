use std::io;

fn main() {
    let mut input = String::new();
    let exit_phrase = String::from("exit");
    let mut command_list:Vec<String> = Vec::new();

    while(String::from(command_list.last().unwrap()) != exit_phrase){ //exiting while loop terminates program

        println!("please enter your next command (enter '{}' to exit)", exit_phrase); //get input from user
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read command");

        command_list.push(String::from(input.trim())); //add last command to command_list

        input = String::new(); //clear input buffer

       for x in command_list.iter(){ //print all commands in command_list
           println!("{}",x);
       }
        print!("\n\n last command: {}",command_list.last().unwrap()); //print last command in command_list
    }
}


