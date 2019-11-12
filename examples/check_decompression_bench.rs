use laz::checking::check_decompression_bench;
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let laz_path = args.get(1).expect("Path to laz file as first argument");
    let laz_file = std::io::BufReader::new(File::open(laz_path).unwrap());

    check_decompression_bench(laz_file);
}
