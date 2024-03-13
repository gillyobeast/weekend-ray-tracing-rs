fn main() {

    // image
    let image_width = 256;
    let image_height = 256;

    // render

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let r: f64 = (i as f64) / (image_width as f64 - 1.0);
            let g: f64 = (j as f64) / (image_width as f64 - 1.0);
            let b = 0.0;

            let r = (255.99 * r) as i32;
            let g = (255.99 * g) as i32;
            let b = (255.99 * b) as i32;

            println!("{r} {g} {b}");
        }
    }
}
