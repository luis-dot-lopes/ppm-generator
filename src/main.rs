use ndarray::prelude::*;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::vec;

#[derive(Debug, Clone)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Copy for Pixel {}

#[derive(Debug)]
struct Screen {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

fn write_image_grayscale(
    filename: &str,
    height: usize,
    width: usize,
    pixels: &[u8],
) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    write!(file, "P6\n{} {} 255\n", height, width)?;
    for y in 0..height {
        for x in 0..width {
            let pixel = pixels[y * height + x];
            file.write(&[pixel, pixel, pixel])?;
        }
    }
    Ok(())
}

fn write_image(
    filename: &str,
    height: usize,
    width: usize,
    pixels: &[Pixel],
) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    write!(file, "P6\n{} {} 255\n", height, width)?;
    for y in 0..height {
        for x in 0..width {
            let Pixel {
                red: r,
                green: g,
                blue: b,
            } = pixels[y * height + x];
            file.write(&[r, g, b])?;
        }
    }
    Ok(())
}

fn read_image(filename: &str) -> std::io::Result<Screen> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let mut lines = data.lines();
    lines.next();
    let mut metadata = lines
        .next()
        .expect("file doesn't have enough lines")
        .split(" ")
        .map(|x| x.parse::<usize>());
    let width = metadata.next().unwrap().unwrap();
    let height = metadata.next().unwrap().unwrap();
    let image_data = lines.next().unwrap();
    let mut pixels: Vec<Pixel> = Vec::new();
    image_data.as_bytes().chunks(3).for_each(|chunck| {
        pixels.push(Pixel {
            red: chunck[0],
            green: chunck[1],
            blue: chunck[2],
        })
    });

    Ok(Screen {
        width: width,
        height: height,
        pixels: pixels,
    })
}

fn draw_rect(x: usize, y: usize, width: usize, height: usize, color: Pixel, screen: &mut Screen) {
    for i in 0..height {
        for j in 0..width {
            screen.pixels[(y + i) * screen.width + x + j] = color;
        }
    }
}

fn draw_circle(x: usize, y: usize, radius: usize, color: Pixel, screen: &mut Screen) {
    let lowy = if y < radius {
        0
    } else {
        y - radius
    };

    let lowx = if x < radius {
        0
    } else {
        x - radius
    };

    let highy = if y + radius >= screen.height {
        screen.height - 1
    } else {
        y + radius
    };

    let highx = if x + radius >= screen.width {
        screen.width - 1
    } else {
        x + radius
    };

    for i in lowy..highy {
        for j in lowx..highx {
            let cy = (i as i64) - (y as i64);
            let cx = (j as i64) - (x as i64);
            if(cx * cx + cy * cy <= (radius as i64) * (radius as i64)) {
                screen.pixels[i * screen.width + j] = color;
            }
        }
    }
}

fn apply_matrix(matrix: Array<f64, Dim<[usize; 2]>>, screen: &Screen) -> Screen {
    let new_pixels = vec![
        Pixel {
            red: 0,
            green: 0,
            blue: 0
        };
        screen.width * screen.height
    ];

    let mut new_screen = Screen {
        width: screen.width,
        height: screen.height,
        pixels: new_pixels,
    };

    for y in 0..screen.height {
        for x in 0..screen.width {
            let pos = Array::from_shape_vec((2, 1), vec![x as f64, y as f64])
                .expect("No idea what happened");
            let new_pos = matrix.dot(&pos);
            let new_x = new_pos[[0, 0]].floor() as usize;
            let new_y = new_pos[[1, 0]].floor() as usize;
            if new_x > 0 && new_x < screen.width && new_y > 0 && new_y < screen.height {
                new_screen.pixels[new_y * screen.width + new_x] =
                    screen.pixels[y * screen.width + x];
            }
        }
    }

    new_screen
}

fn rotation_matrix(radians: f64) -> Array<f64, Dim<[usize; 2]>> {
    array![
        [radians.cos(), radians.sin()],
        [-(radians.sin()), radians.cos()]
    ]
}

fn main() -> std::io::Result<()> {
    const SCREEN_SIDE: usize = 100;
    let _arr1 = rotation_matrix(0.8f64);
    let _arr2 = array![[1.3, 0.], [0., 1.3]];
    let arr3 = rotation_matrix(0.3f64);
    let arr4 = array![[1., 0.], [0.2, 1.]];
    let pixels = vec![0; SCREEN_SIDE * SCREEN_SIDE];
    write_image_grayscale("foo.ppm", SCREEN_SIDE, SCREEN_SIDE, &pixels)?;

    let pixels2 = vec![
        Pixel {
            red: 255,
            green: 0,
            blue: 0,
        };
        SCREEN_SIDE * SCREEN_SIDE
    ];

    let mut screen = Screen {
        width: SCREEN_SIDE,
        height: SCREEN_SIDE,
        pixels: pixels2,
    };

    /*
    draw_rect(
        10,
        10,
        30,
        30,
        Pixel {
            red: 0,
            green: 255,
            blue: 0,
        },
        &mut screen,
    );
    */

    draw_rect(
        50,
        50,
        50,
        50,
        Pixel {
            red: 0,
            green: 255,
            blue: 0,
        },
        &mut screen,
    );

    draw_circle(
        75,
        75,
        25,
        Pixel {
            red: 0,
            green: 0,
            blue: 255,
        },
        &mut screen,
    );

    //let screen = apply_matrix(arr4, &apply_matrix(arr3, &screen));

    write_image("bar.ppm", screen.height, screen.width, &screen.pixels)
}
