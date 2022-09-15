use std::io::{stdin, stdout, Write};
use std::process::Command;


fn input() -> String {
    let mut output = String::new();
    stdout().flush().expect("could not flush stdout");
    stdin().read_line(&mut output).expect("could not read line");
    return output.trim_end().to_string();
}


fn shutdown(shutdown_value: i32){
    let _ = Command::new("cmd").args(["/C", "shutdown -s -t ", &shutdown_value.to_string()])
        .output()
        .expect("failed to execute shutdown");
}


fn cls(){
    let _ = Command::new("cmd").args(["/C", "cls"]).status().expect("could not execute cls");
}


fn pause(){
    let _ = Command::new("cmd").args(["/C", "pause"]).status().expect("could not execute pause");
}


fn get_question(message: &str) -> String {
    return "                                    Tempo para desligar o sistema (em ".to_owned()+message+"):\n> ";
}


fn main() {
    let banner = "\n\n\n\n\n\n\n\n
                          ██████  ██░ ██  █    ██ ▄▄▄█████▓▓█████▄  ▒█████   █     █░███▄    █
                        ▒██    ▒ ▓██░ ██▒ ██  ▓██▒▓  ██▒ ▓▒▒██▀ ██▌▒██▒  ██▒▓█░ █ ░█░██ ▀█   █
                        ░ ▓██▄   ▒██▀▀██░▓██  ▒██░▒ ▓██░ ▒░░██   █▌▒██░  ██▒▒█░ █ ░█▓██  ▀█ ██▒
                          ▒   ██▒░▓█ ░██ ▓▓█  ░██░░ ▓██▓ ░ ░▓█▄   ▌▒██   ██░░█░ █ ░█▓██▒  ▐▌██▒
                        ▒██████▒▒░▓█▒░██▓▒▒█████▓   ▒██▒ ░ ░▒████▓ ░ ████▓▒░░░██▒██▓▒██░   ▓██░
                        ▒ ▒▓▒ ▒ ░ ▒ ░░▒░▒░▒▓▒ ▒ ▒   ▒ ░░    ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▓░▒ ▒ ░ ▒░   ▒ ▒
                        ░  ░  ░   ░  ░░ ░ ░░░ ░ ░   ░       ░ ░  ░ ░ ░ ░ ▒    ░   ░    ░   ░ ░
                              ░   ░  ░  ░   ░                 ░        ░ ░      ░            ░\n";
    cls();
    print!("{}                            [1] Agendar desligamento             [2] Desfazer desligamento:\n> ", banner);
    let schedule_cancel = input();
    if schedule_cancel == "1" {
        cls();
        print!("{}                                      [1] Horas     [2] Minutos     [3] Segundos:\n> ", banner);
        let shutdown_task_question = input();
        if shutdown_task_question == "1" {
            cls();
            print!("{}{}", banner, get_question("horas"));
            let shutdown_hours: i32 = input().parse().unwrap();
            shutdown(shutdown_hours*3600);
            println!("shutdown {}", shutdown_hours*3600)
        }
        else if shutdown_task_question == "2"{
            cls();
            print!("{}{}", banner, get_question("minutos"));
            let shutdown_minutes: i32 = input().parse().unwrap();
            shutdown(shutdown_minutes*60);
            println!("shutdown {}", shutdown_minutes*60)
        }
        else if shutdown_task_question == "3"{
            cls();
            print!("{}{}", banner, get_question("segundos"));
            let shutdown_seconds: i32 = input().parse().unwrap();
            shutdown(shutdown_seconds);
            println!("shutdown {}", shutdown_seconds)
        }
        else {
            println!("\nUm erro ocorreu, você deve digitar 1, 2 ou 3");
        }
    }
    else if schedule_cancel == "2" {
        println!("shutdown -a");
        let _ = Command::new("cmd").args(["/C", "shutdown -a"])
            .output()
            .expect("failed to execute shutdown");
    }
    else {
        println!("\nUm erro ocorreu, você deve digitar 1 ou 2");
    }
    pause()
}