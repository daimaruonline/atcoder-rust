use proconio::input;

fn main() {
    input!{
        s: String,
    }
    let v: Vec<&str> = s.rsplit('.').collect();
    println!("{}", v[0]);
}
