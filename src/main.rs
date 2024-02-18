use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    if args.len() <= 1  {
        // path with no inputted file
        todo!();
    }
    else {
        // path with a file in the params
        todo!();
    }
}
