use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args[1].parse::<f64>().unwrap();
    let mut list = Vec::new();
    let mut uppers = upper_freqs(num);
    let mut lowers = lower_freqs(num);

    list.append(&mut lowers);
    list.push(num);
    list.append(&mut uppers);

    println!("{:?}", list);
}

fn upper_freqs(mut num: f64) -> Vec<f64>{
    let mut all_freqs = Vec::new();

    while num * 2.0 < 20000.0 {
        num *= 2.0;
        all_freqs.push(num);
    }

    all_freqs
}

fn lower_freqs(mut num: f64) -> Vec<f64>{
    let mut all_freqs = Vec::new();

    while num / 2.0 > 20.0 {
        num /= 2.0;
        all_freqs.push(num);
    }

    all_freqs
}
