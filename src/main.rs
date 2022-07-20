use clap::Parser;
use std::net::{IpAddr, TcpListener, TcpStream};

use ncurses::*;
use std::io::*;
use std::net;
use std::thread;

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

fn set_up_tui(lines: i32, cols: i32) -> WINDOW {
    initscr();
    let w = newwin(lines, cols, 0, 0);
    keypad(w, true);
    start_color();
    use_default_colors();
    mvaddstr(1, 1, "me (q to quit)> ");
    refresh();
    w
}

fn main() /*-> io::Result<()>*/
{
    let cli = Cli::parse();
    let ip = cli.ip.unwrap();
    let lines = LINES();
    let cols = COLS();

    let (mut stream, addr) = get_stream(ip);
    //let stream = Arc::new(stream);

    println!("{:?}, {:?}", stream, addr);

    let w = set_up_tui(lines, cols);

    let mut stream2 = stream.try_clone().unwrap();
    mv(1, 18);

    //send
    let res = thread::spawn(move || {
        let mut k;
        loop {
            k = getch();
            if k == 10 {
                insertln();
            }
            stream.write(&mut [k as u8]).unwrap();
            refresh();
        }
    });

    //recv
    thread::spawn(move || {
        init_pair(1, COLOR_WHITE, COLOR_BLUE);
        let mut buf = [0; 1];
        loop {
            stream2.read_exact(&mut buf).unwrap();
            //println!("{:?}", buf);
            if buf[0] == 10 {
                //newline
                insertln();
            } else {
                attron(COLOR_PAIR(1));
                addch(buf[0] as u32);
                attroff(COLOR_PAIR(1));
            }
            refresh();
        }
    });

    //hang until one of the threads encounters an error (typically because someone left the chat) and then exit gracefully
    let _ = res.join();

    //clean up TUI
    delwin(w);
    endwin();
    refresh();
}

fn get_stream(ip: IpAddr) -> (TcpStream, net::SocketAddr) {
    let addr = net::SocketAddr::new(ip, 1337);
    match TcpStream::connect(addr) {
        Ok(stream) => (stream, addr),
        Err(e) => {
            println!("LISTENING - {:?}", e);
            TcpListener::bind(addr).unwrap().accept().unwrap()
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
