mod day_14;
mod file_utils;

fn main() {
    day_14::day14();

    let mut v : Vec<char> = Vec::new();

    for i in 0..20{
        v.push('.');
    }

    for i in (3..6).rev(){
        print!("{}", i);
        v[i] = '#';
    }

    for c in v{
        print!("{}",c);
    }

    println!();


}
