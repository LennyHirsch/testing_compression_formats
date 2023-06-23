use zstd::stream::{copy_encode, copy_decode};
use std::io::{self, BufReader, BufWriter, Write};
use std::fs::File;
use std::time::Instant;
use std::path::Path;
use std::env;

const SIZE: usize = 1024 * 1024 * 8;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 3 { panic!("Usage: flag filename\nflag: c (compress) or d (decompress)") }
    // let flag = args[1].as_str();
    // let filename = &args[2];
    let test_file = read_hdf5().unwrap();
    let test_file = test_file.as_reader();
    let test_arr = test_file.read_2d::<u8>().unwrap();
    println!("{:#?}", test_arr);

    // let now = Instant::now();

    // match flag {
    //     "c" => {
    //         match compress(filename) {
    //             Ok(_) => println!("File compressed."),
    //             Err(e) => println!("An error occured: {}", e),
    //         }
    //     },
    //     "d" => {
    //         match decompress(filename) {
    //             Ok(_) => println!("File decompressed."),
    //             Err(e) => println!("An error occured: {}", e),
    //         }
    //     },
    //     &_ => panic!("Usage: flag filename\nflag: c (compress) or d (decompress)"),
    // }

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}

fn create_hdf5() -> hdf5::Result<()> {
    let file = hdf5::File::create("testhdf.h5")?;
    let testarr = ndarray::array![[1,2,3,4],[5,6,7,8]];
    let anotherarr = ndarray::array![[10,20,30,40],[50,60,70,80]];
    let colors = file.new_dataset::<u8>().create("testArray")?;
    colors.write(&testarr)?;

    let group = file.create_group("testGroup")?;
    let pixels = group.new_dataset::<u8>().create("anotherArray")?;
    pixels.write(&anotherarr)?;

    Ok(())
}

fn read_hdf5() -> hdf5::Result<hdf5::Dataset> {
    let file = hdf5::File::open("testhdf.h5")?;
    let arr = file.dataset("testArray")?;
    Ok(arr)
}

fn compress(input_file: &str) -> io::Result<()> {
    let mut output_name = String::from(input_file.to_string());
    output_name.push_str(".zstd");
    let output_name = Path::new(&output_name);
    
    let input_file = File::open(input_file)?;
    let output_file = File::create(output_name)?;

    let mut reader = BufReader::with_capacity(SIZE, input_file);
    let mut writer = BufWriter::new(output_file);

    copy_encode(&mut reader, &mut writer, 0)?;

    writer.flush()?;

    Ok(())
}

fn decompress(input_file: &str) -> io::Result<()> {
    let output_name = Path::new(input_file).file_stem().unwrap();
    
    let input_file = File::open(input_file)?;
    let output_file = File::create(output_name)?;

    let mut reader = BufReader::with_capacity(SIZE, input_file);
    let mut writer = BufWriter::new(output_file);

    copy_decode(&mut reader, &mut writer)?;

    writer.flush()?;

    Ok(())
}
