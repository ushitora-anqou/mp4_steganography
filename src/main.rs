use byteorder::ByteOrder;
use std::io::{Read, Write};

fn bigendian(src: &[u8]) -> u64 {
    src.iter().fold(0u64, |acc, &x| (acc << 8) + x as u64)
}

fn insert(dst_file: &mut std::fs::File, message: String) -> std::io::Result<()> {
    let mut buf = ["\0\0\0\0uuidabcdabcdabcdabcd".to_string(), message]
        .concat()
        .into_bytes();
    let size = buf.len() as u32;
    byteorder::BigEndian::write_u32(&mut buf[0..4], size);
    byteorder::BigEndian::write_u64(&mut buf[8..16], 0x4362154cac6343cc);
    byteorder::BigEndian::write_u64(&mut buf[16..24], 0x8376e300e8723119);
    dst_file.write_all(&buf)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} IN-FILE OUT-FILE MESSAGE", args[0]);
        std::process::exit(1);
    }
    let mut src_file = std::fs::File::open(&args[1])?;
    let mut dst_file = std::fs::File::create(&args[2])?;
    let message = &args[3];
    let mut inserted = false;

    loop {
        let mut buf = [0; 8];
        match src_file.read_exact(&mut buf) {
            Ok(_) => (),
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
        let box_size = bigendian(&buf[0..4]);
        if box_size == 0 {
            insert(&mut dst_file, message.clone())?;
            inserted = true;
        }
        dst_file.write_all(&buf)?;
        let mut buf = vec![0; (box_size - 8) as usize];
        src_file.read_exact(&mut buf)?;
        dst_file.write_all(&buf)?;
    }

    if !inserted {
        insert(&mut dst_file, message.clone())?;
    }

    Ok(())
}
