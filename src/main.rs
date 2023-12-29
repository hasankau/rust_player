use std::io;
use std::io::prelude::*;
use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;

fn play_mp3(file_path: &str) -> io::Result<()> {
    // Create an output stream
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(output) => output,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to create output stream: {}", err))),
    };

    // Load the MP3 file
    let file = match std::fs::File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(io::Error::new(io::ErrorKind::NotFound, format!("Failed to open file: {}", err))),
    };
    let source = match Decoder::new(io::BufReader::new(file)) {
        Ok(decoder) => decoder,
        Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Failed to create decoder: {}", err))),
    };

    // Create a sink to play the audio
    let sink = match Sink::try_new(&stream_handle) {
        Ok(sink) => sink,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to create sink: {}", err))),
    };
    sink.append(source);

    println!("Playing {}", file_path);
    println!("Press Enter to stop...");

    // Wait for user input (Enter key) to stop the player
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();

    Ok(())
}

fn main() {
    let file_path = "its_only_love.mp3";

    match play_mp3(file_path) {
        Ok(_) => println!("Playback finished."),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
