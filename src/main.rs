
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
pub fn main(){
    let mut canvas= Canvas::new(900, 550);
    let start= Point{x:0.0,y:1.0,z:0.0};
    let velocity= normalize(Vector{ x:1.0, y:1.8, z:0.0 }) * 11.25;
    let mut projectile = Projectile{position:start,velocity:velocity};
    
    let gravity = Vector{x:0.0, y:-0.1, z:0.0};
    let wind = Vector{ x:-0.01,y:0.0,z:0.0};
    let environment = Environment{gravity:gravity,wind:wind};
    let red = Color{red:1.0,green:0.0,blue:0.0};

    while projectile.position.y > 0.0 {
        let canvas_x = projectile.position.x.floor() as usize;
        let canvas_y = (550.0 - projectile.position.y).floor() as usize;
        write_pixel(&mut canvas, canvas_x, canvas_y, red);
        projectile = tick(&environment, &projectile)
    }
    
    write_to_disk(&canvas, String::from("test.ppm"));
    
  
}


fn tick(env: &Environment, proj: &Projectile) -> Projectile
{
    let position = proj.position + proj.velocity;
    let velociy = proj.velocity + env.gravity + env.wind;
    Projectile{
        position:position,
        velocity:velociy
     }
}
