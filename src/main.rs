use clap::Parser;
use std::net::{IpAddr, TcpStream};

use std::io::{self, stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, required = true, value_hint = clap::ValueHint::Hostname)]
    ip: Option<IpAddr>,

    #[clap(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 1337)]
    port: u16,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    println!("// SENDING CLIENT //");
    println!("<Esc> to quit");
    let mut stream = TcpStream::connect((cli.ip.unwrap(), cli.port)).expect("Run the server/listening client first!");
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;
    //termion::cursor::Goto(1, 1);
    //write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush()?;

    //detecting keydown events
    let mut k: char;
    for c in stdin.keys() {
        k = match c {
            Ok(Key::Char(e)) => e,
            Ok(Key::Esc) => break,
            _ => '\n',
        };
        println!("{}", k);
        stream.write(&mut [k as u8])?;
    }
    stdout.flush()?;
    Ok(())
}
