
use raytracer::*;

//Chapter 1
// pub fn main(){
//   let mut projectile = Projectile{position:Point{x:0.0,y:1.0,z:0.0}, velocity:normalize(Vector{x:1.0,y:1.0,z:0.0})};
//   let environment = Environment{gravity:Vector { x: 0.0, y: -0.1, z: 0.0 },wind:Vector { x: -0.01, y: 0.0, z: 0.0 }};

//   while projectile.position.y > 0.0{
//     projectile = tick(&environment, &projectile);
//     println!("projectile y {}",projectile.position.y);
//   }

// }

//Chapter 2
// pub fn main(){
//     let mut canvas= Canvas::new(900, 550);
//     let start= Point{x:0.0,y:1.0,z:0.0};
//     let velocity= normalize(Vector{ x:1.0, y:1.8, z:0.0 }) * 11.25;
//     let mut projectile = Projectile{position:start,velocity:velocity};

//     let gravity = Vector{x:0.0, y:-0.1, z:0.0};
//     let wind = Vector{ x:-0.01,y:0.0,z:0.0};
//     let environment = Environment{gravity:gravity,wind:wind};
//     let red = Color{red:1.0,green:0.0,blue:0.0};

//     while projectile.position.y > 0.0 {
//         let canvas_x = projectile.position.x.floor() as usize;
//         let canvas_y = (550.0 - projectile.position.y).floor() as usize;
//         write_pixel(&mut canvas, canvas_x, canvas_y, red);
//         projectile = tick(&environment, &projectile)
//     }

//     write_to_disk(&canvas, String::from("test.ppm"));

// }
//Chapter 4
pub fn main() {
    let width: usize = 500;
    let height = 500;
    let clock_size: f64 = 3.0 / 8.0;
    let mut canvas = Canvas::new(width, height);

    let radius: f64 = clock_size * 500.0;
    let twelve = Point {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    let three = Point {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let red = Color {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    }; 
    

    for i in 0..=12 {
        
        let next: f64 =  i.into();
        
        let r = Matrix::rotation_y(next * (std::f64::consts::PI / 6.0));
       
        let pixel = &r * &twelve;

        
        let x = (250.0 + (pixel.x * radius)).round() as usize;
        
        let  z = (250.0 - (pixel.z * radius)).round() as usize;
    
        // println!("------");
        // println!("i {}", i);
        // println!("x {}", x);
        // println!("z {}", z);
        // println!("z -radius: {}",((pixel.z * radius)as usize) );
        // println!("x: {}, y: {}, z:{}",pixel.x,pixel.y,pixel.z);
        // println!("x: {}, y: {}, z:{}",pixel.x.round(),pixel.y.round(),pixel.z.round());
        // println!("------");
        write_pixel(&mut canvas, x, z, red);
    }
    write_to_disk(&canvas, String::from("test.ppm"));
}


// fn tick(env: &Environment, proj: &Projectile) -> Projectile
// {
//     let position = proj.position + proj.velocity;
//     let velociy = proj.velocity + env.gravity + env.wind;
//     Projectile{
//         position:position,
//         velocity:velociy
//      }
// }
