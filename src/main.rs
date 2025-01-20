use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process::{ChildStdin, Command, Stdio};
fn play(reader: &mut BufReader<File>, buffer: &mut [u8; 4096], aplay_stdin: &mut ChildStdin) {
    loop {
        //read in blocks of 4096 bytes and store it into buffer
        let bytes_read = reader.read(buffer).expect("Failed to read the file");

        if bytes_read == 0 {
            break;
        }
        aplay_stdin
            .write_all(&buffer[..bytes_read])
            .expect("Failed streaming file");
    }
}

fn main() {
    //skip first argument which is executable path
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: cargo run <file.wav>");
        return;
    }
    //This allows to open the file without loading it all in memory
    let file = File::open(&args[0]).expect("Should be able to read file");
    //Create a buffer for the file
    let mut reader = BufReader::new(file);
    // buffer of 4096 bytes
    let mut buffer = [0u8; 4096];
    //skip header of wav
    let mut header = [0u8; 20];
    // skip header info
    reader
        .seek_relative(24)
        .expect("Failed to seek to sample rate");
    // Read 4 bytes of Sample Rate
    let mut sample_rate_bytes = [0u8; 4];
    reader
        .read_exact(&mut sample_rate_bytes)
        .expect("Failed to read sample rate");

    // Convert Little Endian to u32
    let sample_rate = u32::from_le_bytes(sample_rate_bytes);
    reader
        .read_exact(&mut header)
        .expect("Failed to read header");
    let mut child = Command::new("aplay")
        .arg("-f")
        .arg("S16_LE") // Formato: 16-bit PCM Little Endian
        .arg("-r")
        .arg(sample_rate.to_string()) // Frecuencia de muestreo: 44100 Hz
        .arg("-c")
        .arg("2") // Canales: 2 (estéreo)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start aplay");
    let mut aplay_stdin = child.stdin.take().expect("Failed to open aplay_stdin");
    play(&mut reader, &mut buffer, &mut aplay_stdin);
    println!("Finished");
    return;
}
