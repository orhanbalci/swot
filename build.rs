use phf_codegen::Set as PSet;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::{env, io};

fn main() {
    create_tlds_set();
    create_stoplist_set();
}

fn create_tlds_set() {
    let tlds_file_path = Path::new("domains").join("tlds.txt");
    let tlds_lines = File::open(tlds_file_path).map(|file| io::BufReader::new(file).lines());
    if let Ok(lines) = tlds_lines {
        let tlds_source_path = Path::new(&env::var("OUT_DIR").unwrap()).join("tlds.rs");
        let mut file = BufWriter::new(File::create(&tlds_source_path).unwrap());
        let mut tlds_set: PSet<String> = PSet::new();
        for line in lines {
            if let Ok(l) = line {
                tlds_set.entry(l);
            } else {
                println!("Error on tlds line {:?}", line);
            }
        }

        writeln!(
            &mut file,
            "static TLDS: phf::Set<&'static str> = \n{};\n",
            tlds_set.build()
        )
        .unwrap();
    } else {
        println!("Can not read tlds file");
    }
}

fn create_stoplist_set() {
    let stoplist_file_path = Path::new("domains").join("stoplist.txt");
    let stoplist_lines =
        File::open(stoplist_file_path).map(|file| io::BufReader::new(file).lines());
    if let Ok(lines) = stoplist_lines {
        let stoplist_source_path = Path::new(&env::var("OUT_DIR").unwrap()).join("stoplist.rs");
        let mut file = BufWriter::new(File::create(&stoplist_source_path).unwrap());
        let mut stoplist_set: PSet<String> = PSet::new();
        for line in lines {
            if let Ok(l) = line {
                stoplist_set.entry(l);
            } else {
                println!("Error on stoplist_set line {:?}", line);
            }
        }

        writeln!(
            &mut file,
            "static STOPLIST: phf::Set<&'static str> = \n{};\n",
            stoplist_set.build()
        )
        .unwrap();
    } else {
        println!("Can not read stoplist file");
    }
}
