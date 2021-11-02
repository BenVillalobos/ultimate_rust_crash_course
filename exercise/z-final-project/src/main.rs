// FINAL PROJECT
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

// extern crate clap;
// use clap::{Arg, App, SubCommand};

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/

    /*let matches = App::new("Image Processor!")
    .version("1.0")
    .author("Ben V.")
    .about("Processes images of course!")
    .arg(Arg::with_name("blur")
         .short("b")
         .long("blur")
         .value_name("FILE")
         .help("Blurs an image"))
    .get_matches();

    let blurring = matches.value_of("blur").unwrap_or("blur");
    println!("{}", blurring);
*/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount: f32 = args.remove(0).parse().expect("Failed to parse the amount");
            blur(infile, outfile, amount);
        },
        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount: i32 = args.remove(0).parse().expect("Failed to parse the amount");
            brighten(infile, outfile, amount);
        }
        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x: u32 = args.remove(0).parse().expect("Failed to parse X");
            let y: u32 = args.remove(0).parse().expect("Failed to parse Y");
            let width: u32 = args.remove(0).parse().expect("Failed to parse WIDTH");
            let height:u32 = args.remove(0).parse().expect("Failed to parse HEIGHT");
            crop(infile, outfile, x, y, width, height);
        }
        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let rotation: i32 = args.remove(0).parse().expect("Failed to parse rotation");
            rotate(infile, outfile, rotation);
        }
        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            invert(infile, outfile);
        }
        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale(infile, outfile);
        }
        "generate" => {
            if args.len() != 4 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            let r: u8 = args.remove(0).parse().expect("Failed to parse R");
            let g: u8 = args.remove(0).parse().expect("Failed to parse G");
            let b: u8 = args.remove(0).parse().expect("Failed to parse B");
            generate(outfile, r, g, b);
        }
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

        // For everything else...
        _ => print_usage_and_exit()
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE AMOUNT");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, amount: f32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    img.blur(amount).save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, amount: i32) {
    let img = image::open(infile).expect("Failed to open INFILE");
    img.brighten(amount).save(outfile).expect("Failed writing OUTFILE");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let img = image::open(infile).expect("Failed to open INFILE");
    img.crop_imm(x, y, width, height).save(outfile).expect("Failed writing OUTFILE");
}

fn rotate(infile: String, outfile: String, angle: i32) {
    let img = image::open(infile).expect("Failed to open INFILE");
    if angle == 90 {
        img.rotate90().save(outfile).expect("Failed writing OUTFILE");
    }
    else if angle == 180 {
        img.rotate180().save(outfile).expect("Failed writing OUTFILE");
    }
    else if angle == 270 {
        img.rotate270().save(outfile).expect("Failed writing OUTFILE");
    }
    else {
        print_usage_and_exit();
    }
}


fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE");
    img.grayscale().save(outfile).expect("Failed writing OUTFILE");
}

fn generate(outfile: String, r: u8, g: u8, b: u8) {
    let width = 500;
    let height = 500;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([r, g, b]);
    }

    // Challenge 2: Generate something more interesting!

    imgbuf.save(outfile).expect("Failed writing OUTFILE");
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
