#!/usr/bin/env rust-script
use std::thread;
use clap::Parser;
use std::net::{IpAddr, TcpStream, TcpListener};
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
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    let (mut socket, addr) = match listener.accept() {
        Ok((socket, addr)) => {
            println!("new client: {addr:?}");
            (socket, addr)
        }
        Err(e) => panic!("socket err: {e}")
    };
    std::fs::write("test.txt", format!("{:?}", addr)).unwrap();

    let thread_res = thread::spawn (move || {
        loop {
            std::io::copy(&mut socket, &mut std::io::stdout()).unwrap();
        }
    });
}
