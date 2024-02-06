use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};
use std::process::{Command, exit};


fn main() {
    let mut program = "sh";
    let mut arg1 = "-c";
    let mut arg2 = "shutdown -h +";
    let mut kapa = "shutdown -c";
    let mut sil = "clear";
    let mut zaman = 1;
    
    if cfg!(windows) {
        program = "cmd";
        arg1 = "/C";
        arg2 = "shutdown -s -f -t ";
        kapa = "shutdown -a";
        sil = "cls";
        zaman = 60;
    } 

    loop {
        Command::new(program).arg(arg1).arg(sil).status().unwrap();
        println!("1- Otomatik kapatma zamanını ayarla");
        println!("2- Otomatik kapatmayı iptal et");
        println!("3- Çıkış");
        print!("\nSeçim: ");
        io::stdout().flush().unwrap();

        let mut sec = String::new();
        io::stdin().read_line(&mut sec).expect("Hata!");

        let sec: u32 = match sec.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue
        };

        Command::new(program).arg(arg1).arg(sil).status().unwrap();

        match sec {
            1 => {
                print!("Bilgisayarın kaç dakika sonra kapanmasını istiyorsun: ");
                io::stdout().flush().unwrap();

                let mut dakika = String::new();
                io::stdin().read_line(&mut dakika).expect("Hata!");

                let dakika: u32 = match dakika.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let dakika = zaman*dakika;
                let output = Command::new(program).arg(arg1).arg(format!("{}{}", arg2,dakika)).output().expect("Hata!");
                if let Ok(stderr) = String::from_utf8(output.stderr){
                    Command::new(program).arg(arg1).arg(sil).status().unwrap();
                    println!("{}", stderr);
                    sleep(Duration::from_secs(2));
                }
            }
            2 => {
                Command::new(program).arg(arg1).arg(kapa).status().expect("Hata!");
                println!("Otomatik kapatma iptal edildi..");
                sleep(Duration::from_secs(1));
            }
            3 => exit(0),
            _ => {
                println!("Doğru bir seçim yapınız..");
                sleep(Duration::from_secs(1));
            }
        }
    }
}