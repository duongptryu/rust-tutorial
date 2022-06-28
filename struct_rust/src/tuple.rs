fn main () {
    let dai = 30;
    let rong = 50;
    println!("Dien tich cua hinh chu nhat = {}", dien_tich(dai, rong))
}

fn dien_tich (dai: u32, rong: u32) -> u32 {
    dai * rong
}
