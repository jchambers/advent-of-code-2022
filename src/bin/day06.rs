use std::error::Error;
use std::fs;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let data_stream = DataStream::from(fs::read_to_string(path)?.as_str());

        println!("Start of packet: {}", data_stream.start_of_packet().unwrap());
        println!("Start of message: {}", data_stream.start_of_message().unwrap());

        Ok(())
    } else {
        Err("Usage: day06 INPUT_FILE_PATH".into())
    }
}

struct DataStream {
    characters: Vec<char>,
}

impl From<&str> for DataStream {
    fn from(string: &str) -> Self {
        DataStream { characters: string.chars().collect() }
    }
}

impl DataStream {
    fn start_of_packet(&self) -> Option<usize> {
        self.start_of_segment(4)
    }

    fn start_of_message(&self) -> Option<usize> {
        self.start_of_segment(14)
    }

    fn start_of_segment(&self, marker_length: usize) -> Option<usize> {
        self.characters
            .windows(marker_length)
            .enumerate()
            .find(|(_, chars)| chars.iter().unique().count() == marker_length)
            .map(|(i, _)| i + marker_length)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_start_of_packet() {
        assert_eq!(7, DataStream::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb").start_of_packet().unwrap());
        assert_eq!(5, DataStream::from("bvwbjplbgvbhsrlpgdmjqwftvncz").start_of_packet().unwrap());
        assert_eq!(6, DataStream::from("nppdvjthqldpwncqszvftbrmjlhg").start_of_packet().unwrap());
        assert_eq!(10, DataStream::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start_of_packet().unwrap());
        assert_eq!(11, DataStream::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").start_of_packet().unwrap());
    }

    #[test]
    fn test_start_of_message() {
        assert_eq!(19, DataStream::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb").start_of_message().unwrap());
        assert_eq!(23, DataStream::from("bvwbjplbgvbhsrlpgdmjqwftvncz").start_of_message().unwrap());
        assert_eq!(23, DataStream::from("nppdvjthqldpwncqszvftbrmjlhg").start_of_message().unwrap());
        assert_eq!(29, DataStream::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start_of_message().unwrap());
        assert_eq!(26, DataStream::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").start_of_message().unwrap());
    }
}
