use std::env;

fn main() {
    //get argument and convert to float
    let args: Vec<String> = env::args().collect();
    let num = args[1].parse::<f64>().unwrap();
    //create all lists
    let mut list = Vec::new();
    let mut uppers = upper_freqs(num);
    let mut lowers = lower_freqs(num);
    //append all lists together
    list.append(&mut lowers);
    list.push(num);
    list.append(&mut uppers);
    //sort the list vector
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("{:?}", list);
}

fn upper_freqs(mut num: f64) -> Vec<f64>{
    //multiples partial by 2 while under 20000 Hz
    let mut all_freqs = Vec::new();

    while num * 2.0 < 20000.0 {
        num *= 2.0;
        all_freqs.push(num);
    }

    all_freqs
}

fn lower_freqs(mut num: f64) -> Vec<f64>{
    //divides partials by 2 while over 20 Hz
    let mut all_freqs = Vec::new();

    while num / 2.0 > 20.0 {
        num /= 2.0;
        all_freqs.push(num);
    }

    all_freqs
}
