use std::{
    fs::File,
    io::{Cursor, Read, Write, Seek},
};

use atty::Stream;
use clap::{App, Arg};
use wav::{BitDepth, WAV_FORMAT_PCM};

struct Args {
    infile: File,
    outfile: File,
    //    verbose: bool,
}

impl Args {
    fn init() -> Self {
        let mut app = App::new("sound-mush")
            .version("0.1.0")
            .about("Manipulate image files using audio compression")
            .author("Mae Dartmann");

        let stdin_is_piped = atty::isnt(Stream::Stdin);
        let stdout_is_piped = atty::isnt(Stream::Stdout);

        let args = [
            Arg::new("input")
                .short('i')
                .long("input")
                .takes_value(true)
                .help("What file to process")
                .required(!stdin_is_piped),
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .help("Where to write the result")
                .required(true),
            Arg::new("debug")
                .short('v')
                .long("verbose")
                .takes_value(false)
                .help("Verbose output")
                .required(false),
        ];

        for arg in args {
            app = app.arg(arg);
        }

        let matches = app.get_matches();

        // Open input source (stdin on pipe)
        let in_file = if stdin_is_piped {
            match File::open("/dev/stdin") {
                Err(why) => panic!("Could not open {}: {}", "/dev/stdin", &why),
                Ok(file) => file,
            }
        } else {
            let filename = matches
                .get_one::<String>("input")
                .expect("Missing input file!");
            match File::open(filename) {
                Err(why) => panic!("Could not open {}: {}", filename, &why),
                Ok(file) => file,
            }
        };

        let out_file = if stdout_is_piped {
            match File::open("/dev/stdout") {
                Err(why) => panic!("Could not open {}: {}", "/dev/stdout", &why),
                Ok(file) => file,
            }
        } else {
            let filename = matches
                .get_one::<String>("output")
                .expect("Missing input file!");
            match File::create(filename) {
                Err(why) => panic!("Could not open {}: {}", filename, &why),
                Ok(file) => file,
            }
        };

        //        let debug_mode = matches.is_present("debug");
        Args {
            infile: in_file,
            outfile: out_file,
            //    verbose: debug_mode,
        }
    }
}

fn main() {
    // Parse arguments.
    let args = Args::init();

    // Variables for in- and output.
    let mut infile = args.infile;
    let mut outfile = args.outfile;

    // Get file contents into byte buffer.
    let mut inbuf = Vec::<u8>::new();
    infile.read_to_end(&mut inbuf).expect("Could not read input!");

    // Make a wav "file" out of the input buffer.

    // The wav header.
    let header = wav::Header::new(WAV_FORMAT_PCM, 1, 8000, 8);
    // The "audio" to write (as 8-bit integers)
    let buf_to_write = BitDepth::Eight(inbuf);
    // The output buffer (Cursor implements Seek and Read)
    let mut out_buf = Cursor::new(Vec::<u8>::new());

    // write the wav file to the output buffer.
    wav::write(header, &buf_to_write, &mut out_buf).expect("Wave file generation failed!");

    // Get contents of cursor back into u8 vec.
    out_buf.rewind().unwrap();
    let mut out_buf_vec = Vec::<u8>::new();
    out_buf.read_to_end(&mut out_buf_vec).unwrap();
    
    // write result out to outfile.
    outfile.write(&out_buf_vec).unwrap();
}
