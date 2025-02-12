use cucumber::{given,when,then, World};
use raytracer::{canvas::write_pixel, canvas::canvas_to_ppm,*};

#[derive(Debug, Default, World)]
pub struct CanvasWorld {
    canvas: Canvas,
    color: Color,
    color_2: Color,
    color_3: Color,
    ppm: String,
}

const RED: Color = Color{
    red:1.0,
    green:0.0,
    blue:0.0,
};

#[given(regex = r#"c ← canvas\(([\d]+), ([\d.]+)\)"#)]
async fn set_canvas(world: &mut CanvasWorld, width: usize, height: usize) {
    let canvas= Canvas::new(width, height);
    world.canvas = canvas;
}


#[given(regex = r#"red ← color\((-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*)\)"#)]
#[given(regex = r#"c1 ← color\((-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*)\)"#)]
async fn set_color(world: &mut CanvasWorld,red: f64, green: f64, blue: f64) {
    let color = Color{
        red:red,
        green:green,
        blue:blue,
    };
    world.color = color;
}

#[given(regex = r#"c2 ← color\((-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*)\)"#)]
async fn set_color_c2(world: &mut CanvasWorld,red: f64, green: f64, blue: f64) {
    let color = Color{
        red:red,
        green:green,
        blue:blue,
    };
    world.color_2 = color;
}

#[given(regex = r#"c3 ← color\((-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*)\)"#)]
async fn set_color_c3(world: &mut CanvasWorld,red: f64, green: f64, blue: f64) {
    let color = Color{
        red:red,
        green:green,
        blue:blue,
    };
    world.color_3 = color;
}

#[when(regex = r#"write_pixel\(c, ([\d]+), ([\d]+), red\)"#)]
#[when(regex = r#"write_pixel\(c, ([\d]+), ([\d]+), c1\)"#)]
async fn write_color(world: &mut CanvasWorld,x: usize, y: usize) {
  write_pixel(&mut world.canvas, x, y,world.color)
}

#[when(regex = r#"write_pixel\(c, ([\d]+), ([\d]+), c2\)"#)]
async fn write_color_2(world: &mut CanvasWorld,x: usize, y: usize) {
  write_pixel(&mut world.canvas, x, y,world.color_2)
}

#[when(regex = r#"write_pixel\(c, ([\d]+), ([\d]+), c3\)"#)]
async fn write_color_3(world: &mut CanvasWorld,x: usize, y: usize) {
  write_pixel(&mut world.canvas, x, y,world.color_3)
}

#[when(regex = r#"ppm ← canvas_to_ppm\(c\)"#)]
async fn when_canvas_to_ppm(world: &mut CanvasWorld) {
  world.ppm = canvas_to_ppm(&world.canvas);
}


#[when(regex = r#"every pixel of c is set to color\((-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*)\)"#)]
async fn set_every_pixel(world: &mut CanvasWorld,red: f64, green: f64, blue: f64) {
    let color = Color{
        red:red,
        green:green,
        blue:blue,
    };
    for i  in 0..world.canvas.height{
        for j in 0..world.canvas.width{
            write_pixel(&mut world.canvas,j, i,color) 
        }
       
    }
}


#[then(regex = r#"c.width = ([\d]+)"#)]
async fn test_width(world: &mut CanvasWorld, width: usize) {
    assert_eq!(world.canvas.width,width);
}

#[then(regex = r#"c.height = ([\d]+)"#)]
async fn test_height(world: &mut CanvasWorld, height: usize) {
    assert_eq!(world.canvas.height,height);
}

#[then(regex = r#"every pixel of c is color\((-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*)\)"#)]
async fn test_everypixel_color(world: &mut CanvasWorld,red: f64, green: f64, blue: f64) {
    let color = Color{
        red:red,
        green:green,
        blue:blue,
    };
    world.canvas.pixels.iter().flatten().for_each(|&entry| {
        assert_eq!(entry,color);
    }); 
}

#[then(regex = r#"pixel_at\(c, ([\d]+), ([\d]+)\) = red"#)]
async fn test_color(world: &mut CanvasWorld,x: usize, y: usize) {
  assert_eq!(world.canvas.pixels[y][x],RED);
}

#[then(regex = r#"lines 1-3 of ppm are"#)]
async fn test_line_1_3_color(world: &mut CanvasWorld) {
  
  let lines: Vec<&str> = world.ppm.lines().collect(); // Split into lines

  assert_eq!(lines[0], "P3");
  assert_eq!(lines[1], "5 3");
  assert_eq!(lines[2], "255");
}

#[then(regex = r#"lines 4-6 of ppm are"#)]
async fn test_line_4_6_color(world: &mut CanvasWorld) {
  let lines: Vec<&str> = world.ppm.lines().collect(); // Split into lines

  assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
  assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
  assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");

}

#[then(regex = r#"lines 4-7 of ppm are"#)]
async fn test_line_4_7_color(world: &mut CanvasWorld) {
  let lines: Vec<&str> = world.ppm.lines().collect(); // Split into lines

  assert_eq!(lines[3], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
  assert_eq!(lines[4], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
  assert_eq!(lines[5], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
  assert_eq!(lines[6], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");

}


#[then(regex = r#"ppm ends with a newline character"#)]
async fn test_ppm_end_with(world: &mut CanvasWorld) {
   world.ppm.ends_with("\n");

}

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(CanvasWorld::run(
        "tests/features/canvas.feature",
    ));
}