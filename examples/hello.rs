fn main() {
    let image_width = 256;
    let image_hight = 256;
    println!("P3\n{} {}\n255", image_width, image_hight);

    let mut j = 256.0;

    while j >= 0.0 {
        let mut i = 0.0;
        while i < 256.0 {
            let r = i / (image_width - 1) as f64;
            let g = j / (image_hight - 1) as f64;
            let b = 0.25;
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
            i += 1.0;
        }
        j -= 1.0;
    }
}
