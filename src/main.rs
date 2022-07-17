use std::thread;
use clap::Parser;
use std::net::{IpAddr, TcpStream, TcpListener};
use std::fs;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


use std::io::Read;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, required = true, value_hint = clap::ValueHint::Hostname)]
    ip: Option<IpAddr>,

    #[clap(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 1337)]
    port: u16,
}

fn main() {
    //crossterm::terminal::enable_raw_mode().unwrap();
    //let port = client::notmain();
    //let addr = fs::read_to_string("/tmp/talk.txt").expect("Error in reading the file");
    //println!("{}", addr);
    let mut stream = TcpStream::connect("127.0.0.1:1337").unwrap();
    
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    termion::cursor::Goto(1, 1);
    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();
    let mut i = 0;

    //detecting keydown events
    let mut k: char;
    for c in stdin.keys() {
        k = match c {
            Ok(Key::Char(e)) => e,
            _ => '\n'
        };
        println!("{}",k);
        stream.write(&mut [k as u8]).unwrap();
        
        i += 1;
        if i == 20 {
            break;
        }
    }
    stdout.flush().unwrap();


    //let mut stream = std::io::BufWriter::with_capacity(1, stream);
    //let buf:u8 = 0;
    //std::io::stdin().read_exact(&mut [buf]).unwrap();
    //stream.write(&mut [buf]).unwrap();
//    std::io::copy(&mut std::io::stdin(), &mut stream).unwrap();
    //crossterm::terminal::disable_raw_mode().unwrap();
/*
    let cli = Cli::parse();

    println!("port: {}", cli.port);
    println!("Ip: {:?}", cli.ip.unwrap()); //unwrap will not fail because clap validates the IP
    

    //stream.set_nonblocking(true)?;
    let mut stream = TcpStream::connect((cli.ip.unwrap(), cli.port))?;
    loop {
        //io::copy(&mut io::stdin().lock(), &mut stream)?;
        io::copy(&mut stream, &mut io::stdout())?;
        if 1==2 {
            break;
        }
    }

/*
    if let Ok(stream2) = TcpStream::connect((cli.ip.unwrap(), cli.port)) {
        println!("Connected to 1");
    }*/
    Ok(())*/
}

