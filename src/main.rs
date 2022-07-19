use clap::Parser;
use std::net::{IpAddr, TcpStream, TcpListener};

use std::io::*;
use ncurses::*;
use std::thread;
use std::sync::mpsc::channel;
use std::net;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, required = true, value_hint = clap::ValueHint::Hostname)]
    ip: Option<IpAddr>,

    /*#[clap(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 1331)]
    lport: u16,

    #[clap(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 1337)]
    rport: u16,*/

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
    let (mut stream, addr) = get_stream(cli);

    let mut addr = net::SocketAddr::new(cli.ip.unwrap(), 1337);
    let (stream, addr) = match TcpStream::connect(addr) {
        Ok(stream) => (stream, addr),
        Err(e) => {
            println!("LISTENING - {:?}", e);
            TcpListener::bind((cli.ip.unwrap(), 1331))
            .unwrap()
            .accept()
            .unwrap()
        }
    };

    println!("{:?}, {:?}", stream, addr);

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


fn get_stream(cli: Cli) -> (TcpStream, net::SocketAddr) {
    let addr = net::SocketAddr::new(cli.ip.unwrap(), 1337);
    match TcpStream::connect(addr) {
        Ok(stream) => (stream, ()),
        Err(e) => {
            println!("LISTENING - {:?}", e);
            TcpListener::bind((cli.ip.unwrap(), 1331))
            .unwrap()
            .accept()
            .unwrap()
        }
    }
}
