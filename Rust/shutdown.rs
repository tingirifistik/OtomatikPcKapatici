use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};
use std::process::{Command, exit};

fn main() {
    let mut shell = "sh";
    let mut arg1 = "-c";
    let mut arg2 = "shutdown -h +";
    let mut cancel = "shutdown -c";
    let mut clear = "clear";
    let mut time = 1;
    if cfg!(windows) {
        shell = "cmd";
        arg1 = "/C";
        arg2 = "shutdown -s -f -t ";
        cancel = "shutdown -a";
        clear = "cls";
        time = 60;
    } 

    loop {
        Command::new(shell).arg(arg1).arg(clear).status().unwrap();
        println!("1- Set the shutdown time");
        println!("2- Cancel the shutdown");
        println!("3- Exit");
        print!("\n>>>");
        io::stdout().flush().unwrap();
        let mut menu = String::new();
        io::stdin().read_line(&mut menu).expect("Error!");
        let menu: u32 = match menu.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue
        };

        Command::new(shell).arg(arg1).arg(clear).status().unwrap();

        match menu {
            1 => {
                print!("How many minutes later do you want the computer to shutdown: ");
                io::stdout().flush().unwrap();
                let mut minute = String::new();
                io::stdin().read_line(&mut minute).expect("Error!");
                let minute: u32 = match minute.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let minute = time*minute;
                
                let output = Command::new(shell).arg(arg1).arg(format!("{}{}", arg2,minute)).output().expect("Error!");
                if let Ok(stderr) = String::from_utf8(output.stderr){
                    Command::new(shell).arg(arg1).arg(clear).status().unwrap();
                    println!("{}", stderr);
                    sleep(Duration::from_secs(3));
                }
            }
            2 => {
                Command::new(shell).arg(arg1).arg(cancel).status().expect("Error!");
                println!("The shutdown has been canceled..");
                sleep(Duration::from_secs(1));
            }
            3 => exit(0),
            _ => {
                println!("Please make a valid choice..");
                sleep(Duration::from_secs(1));
            }
        }
    }
}