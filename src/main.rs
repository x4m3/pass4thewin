use std::fs;

fn add_corner_current_line(str: &mut String, cur_elem: usize, num_elements: usize) {
    if cur_elem + 1 == num_elements {
        str.push_str("└── ");
    } else {
        str.push_str("├── ");
    }
}

fn add_corner_other_line(str: &mut String, cur_elem: usize, num_elements: usize) {
    if cur_elem + 1 == num_elements {
        str.push(' ');
    } else {
        str.push('|');
    }
    str.push_str("   ");
}

fn clean_corner(str: &mut String) {
    for _ in 0..4 {
        str.pop();
    }
}

fn tree(path: &str, string: &mut String) {
    let mut cur_elem: usize = 0;
    let num_elements = fs::read_dir(path).unwrap().count();

    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        let current_path = dir.path();
        let current_name = dir.file_name();
        let current_name = current_name.to_str().unwrap();

        // if current path is a hidden folder / file, skip it
        if current_name.starts_with(".") {
            cur_elem += 1;
            continue;
        }

        // if next element is the last
        add_corner_current_line(string, cur_elem, num_elements);

        // display corner and current name
        println!("{}{}", string, current_name);

        // clean corner
        clean_corner(string);

        if current_path.is_dir() {
            // if next element is the last
            add_corner_other_line(string, cur_elem, num_elements);

            // pass through new folder
            tree(current_path.to_str().unwrap(), string);

            // clean corner
            clean_corner(string);
        }
        cur_elem += 1;
    }
}

fn main() {
    let path = "C:\\users\\x4m3\\.password-store\\";

    let mut string = String::new();
    println!("{}", path);
    tree(path, &mut string);
}
