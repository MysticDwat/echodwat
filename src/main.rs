use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let ignore_new_line_flag_indx = args.iter().position(|elem| elem == "-n" || elem == "--ignore_new_line");
    let ignore_new_line_flag = ignore_new_line_flag_indx != None;

    if ignore_new_line_flag {
        args.remove(ignore_new_line_flag_indx.unwrap());
    }

    print!("{}{}", args[1..].join(" "), "\n".repeat(!ignore_new_line_flag as usize));
}
