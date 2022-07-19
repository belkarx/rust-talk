use clap::Parser;
use std::net::{IpAddr, TcpStream, TcpListener};

use std::io::*;
use ncurses::*;
use std::thread;
use std::sync::{Arc, mpsc::channel};
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

fn set_up_tui(lines: i32, cols: i32) {
    initscr();
    mvvline(0, cols/2, '+' as u32, lines);
    mvaddstr(1, 1, "me (q to quit)> ");
    mvaddstr(1, (cols/2)+3, "them> ");
    refresh();
}


fn main() /*-> io::Result<()>*/ {
    let cli = Cli::parse();
    let ip = cli.ip.unwrap(); 
    let lines = LINES();
    let cols = COLS();

    let (mut stream, addr) = get_stream(ip);
    //let stream = Arc::new(stream);

    println!("{:?}, {:?}", stream, addr);

    set_up_tui(lines, cols);
    
    let mut stream2 = stream.try_clone().unwrap();
    
    //recv
    thread::spawn(move|| {
        let w = newwin(lines, cols, 0, 0);
        //let mut i = 0;
        //let mut k:u8 = 0;
        let mut buf = [0; 1];
        loop {
            stream2.read_exact(&mut buf).unwrap();
            addch(w, buf[0] as u32);
            refresh();
        }
    });

    //send
    thread::spawn(move || {  
        let mut k: i32 = 0;   
        while k != 113 {
            k = getch();
            /*if k == KEY_BACKSPACE {
                k = 10;
            }*/
            stream.write(&mut [k as u8]).unwrap();
            refresh();
        }
    });
    

    std::thread::sleep(std::time::Duration::from_secs(5));

    //clean up TUI
    endwin();
    refresh();
}

fn get_stream(ip: IpAddr) -> (TcpStream, net::SocketAddr) {
    let addr = net::SocketAddr::new(ip, 1337);
    match TcpStream::connect(addr) {
        Ok(stream) => (stream, addr),
        Err(e) => {
            println!("LISTENING - {:?}", e);
            TcpListener::bind(addr)
            .unwrap()
            .accept()
            .unwrap()
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn 
}*/
