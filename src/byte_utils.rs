use std::io;
use std::io::BufRead;


pub fn read_until<R: BufRead>(reader: &mut R, seperator: &str) -> io::Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::with_capacity(128);
    let mut consumed;
    let mut quit = false;
    loop {
        {
            let available = match reader.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            consumed = available.len();
            if consumed == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "end of buf"));
            }
            if let Some(index) = available.windows(2).position(|s| s == seperator.as_bytes()) {
                buf.extend_from_slice(&available[..index]);
                consumed = index + 2;
                quit = true;
            }
        }
        reader.consume(consumed);
        if quit {
            break;
        }
    }
    Ok(buf)
}


pub fn peek_until<R: BufRead>(reader: &mut R, seperator: &str) -> io::Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::with_capacity(128);
    let mut quit = false;
    loop {
        {
            let available = match reader.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            if available.len() == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "end of buf"));
            }
            if let Some(index) = available.windows(2).position(|s| s == seperator.as_bytes()) {
                buf.extend_from_slice(&available[..index]);
                quit = true;
            }
        }
        if quit {
            break;
        }
    }
    Ok(buf)
}
