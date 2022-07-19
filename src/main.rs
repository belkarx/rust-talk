use clap::Parser;
use std::net::{IpAddr, TcpStream, TcpListener};

use std::io::*;
use ncurses::*;
use std::thread;
use std::sync::mpsc::channel;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, required = true, value_hint = clap::ValueHint::Hostname)]
    ip: Option<IpAddr>,

    #[clap(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 1331)]
    lport: u16,

    #[clap(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 1337)]
    rport: u16,

}

fn set_up_tui() -> WINDOW {
    initscr();
    let lines = LINES();
    let cols = COLS();
    let w = newwin(lines, cols, 0, 0);
    mvvline(0, cols/2, '+' as u32, lines);
    keypad(w, true);
    mvaddstr(lines-1, (cols/2)+3, "them> ");
    mvaddstr(lines-1, 1, "me (q to quit)> ");
    refresh();
    w
}


fn main() /*-> io::Result<()>*/ {
    let cli = Cli::parse();
    let (mut stream, port) = get_stream(cli);
   /* 
    let w = set_up_tui();
    //std::thread::sleep(std::time::Duration::from_secs(5));
    //send
    thread::spawn(move || {  
        let mut k: i32 = 0;   
        while k != 113 {
            k = getch();
            stream.write(&mut [k as u8]).unwrap();
            refresh();
        }
    });
    
    //recv
    thread::spawn(move|| {
       std::io::copy(&mut stream, &mut std::io::stdout());
    });



    //clean up TUI
    delwin(w);
    endwin();
    refresh();*/
}


fn get_stream(cli: Cli) -> (TcpStream, u16) {
    let (tx, rx) = channel();
    let tx2 = tx.clone();

    //try to connect to recieving port
    thread::spawn(move|| {
        tx.send(TcpStream::connect((cli.ip.unwrap(), cli.rport))).unwrap();
    });
    
    //listen
    thread::spawn(move|| {     
        tx2.send(
            Ok(TcpListener::bind((cli.ip.unwrap(), cli.lport))
            .unwrap()
            .accept()
            .unwrap().0
            ))
    });
    
    let stream = rx.recv().unwrap();
    match stream {
        Ok(stream) => (stream, cli.rport),
        Err(_) => {
            println!("listening");
            (rx.recv().unwrap().unwrap(), cli.lport)
        }
    }
    //other thread goes out of scope once this returns
}
