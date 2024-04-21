use std::env;

mod xaml_reader;
mod UIElements;

fn usage() {
    let args: Vec<String> = env::args().collect();
    println!("USAGE: <xaml>");
    for s in args {
        println!("arg: {}", s);
    }
    std::process::exit(1);
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }

    let filename = &args[1];

    let tree = xaml_reader::read_xaml(filename);
    if let Result::Ok(t) = tree {
        println!("TREE ---> ");
        t.lock().unwrap().dump(0)
    } else {
        println!("no tree returned from xaml parse?");
    }
}