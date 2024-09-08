pub enum Command{
    help,
    new,
    lists,
    load,
    add,
    events,
    finish,
    unknown,
}

pub fn parse_command(args:&Vec<String>)->Command{
    if args.len()<1{
        return Command::unknown;
    }
    let command_str=&args[0];
    if command_str.len()<1{
        return Command::unknown;
    }
    if command_str.starts_with('-') {
        if command_str.len() > 1 && command_str.chars().nth(1) == Some('-') {
            return match command_str.get(2..){
                Some(cmd) if cmd=="help"=>Command::help,
                Some(cmd) if cmd=="new"=>Command::new,
                Some(cmd) if cmd=="lists"=>Command::lists,
                Some(cmd) if cmd=="load"=>Command::load,
                Some(cmd) if cmd=="add"=>Command::add,
                Some(cmd) if cmd=="events"=>Command::events,
                Some(cmd) if cmd=="finish"=>Command::finish,
                _=>Command::unknown,
            }
        }else{
            return match command_str.get(1..){
                Some(cmd) if cmd=="h"=>Command::help,
                Some(cmd) if cmd=="n"=>Command::new,
                Some(cmd) if cmd=="ls"=>Command::lists,
                Some(cmd) if cmd=="ld"=>Command::load,
                Some(cmd) if cmd=="a"=>Command::add,
                Some(cmd) if cmd=="es"=>Command::events,
                Some(cmd) if cmd=="f"=>Command::finish,
                _=>Command::unknown,
            }
        }
    }
    Command::unknown
}

pub fn print_help(){
    
}