use std::io::{self, Write};

use yeold::{AutoRP, AUTORP};

fn main() {
    let arp = keyvalues_serde::from_str::<AutoRP>(AUTORP).unwrap();
    let stdin: io::Stdin = io::stdin();
    let mut buf: String = String::with_capacity(1024);
    let mut outbuf = String::with_capacity(1024);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let bytes: usize = stdin.read_line(&mut buf).unwrap();

        // EOF
        if bytes == 0 {
            break;
        }

        let input = buf.trim_end();

        println!("{}", arp.translate(input));
        outbuf.clear();
        buf.clear();
    }
}
