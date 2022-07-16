#!/usr/bin/env rust-script

mod client;
use std::thread;
use clap::Parser;
use std::net::{IpAddr, TcpStream, TcpListener};
use std::fs;

use std::io::Write;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, required = true, value_hint = clap::ValueHint::Hostname)]
    ip: Option<IpAddr>,

    #[clap(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 1337)]
    port: u16,
}

fn main() {
    //let port = client::notmain();
    let addr = fs::read_to_string("test.txt").expect("Error in reading the file");
    println!("{}", addr);
    let mut stream = TcpStream::connect(addr).unwrap();
    std::io::copy(&mut std::io::stdin(), &mut stream).unwrap();
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
