mod util;

#[derive(Debug)]
struct Particle {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
    ax: i64,
    ay: i64,
    az: i64,
}

fn max(v1: f64, v2: f64) -> f64 {
    if v1 >= v2 {
        return v1;
    }
    return v2;
}


fn dim_collide(x_0: i64, v_0: i64, a_0: i64, x_1: i64, v_1: i64, a_1: i64) -> Option<f64> {
    let b = (v_0 - v_1) as f64;
    let a = (0.5 * a_0 as f64) - (0.5 * a_1 as f64);
    let c = (x_0 - x_1) as f64;

    if a == 0.0 {
        if v_0 - v_1 == 0 {
            if x_0 == x_1 {
                return Some(-1.0);
            }
            return None;
        }
        let t = (x_1 as f64 - x_0 as f64) / (v_0 as f64 - v_1 as f64);
        if t >= 0.0 {
            //println!("collision at t: {}", t);
            return Some(t)
        } else {
            return None;
        }
    }

    if (b * b) - (4.0 * a * c) < 0.0 {
        return None;
    }
    //println!("a is: {}", a);
    let t = max( 
        (-b + ((b*b) - (4.0 * a * c)).sqrt()) / (2.0 * a),
        (-b - ((b*b) - (4.0 * a * c)).sqrt()) / (2.0 * a)
    );
    //println!("collision at t: {}", t);
    if t >= 0.0 {
        return Some(t);
    } else {
        return None;
    }
}

fn is_integer(val: f64) -> bool {
    return ((val as i64) as f64) == val;
}

fn collide(p1: &Particle, p2: &Particle) -> bool {
    let x_collide_opt = dim_collide(p1.x, p1.vx, p1.ax, p2.x, p2.vx, p2.ax);
    if x_collide_opt.is_none() {
        return false;
    }
    let x_collide = x_collide_opt.unwrap();
    if !is_integer(x_collide) {
        //println!("throwing out collision");
        return false;
    }

    let y_collide_opt = dim_collide(p1.y, p1.vy, p1.ay, p2.y, p2.vy, p2.ay);
    if y_collide_opt.is_none() {
        return false;
    }
    let y_collide = y_collide_opt.unwrap();
    if !is_integer(y_collide) {
        return false;
    }

    let z_collide_opt = dim_collide(p1.z, p1.vz, p1.az, p2.z, p2.vz, p2.az);
    if z_collide_opt.is_none() {
        return false;
    }
    let z_collide = z_collide_opt.unwrap();
    if !is_integer(z_collide) {
        return false;
    }

    return (x_collide == y_collide || x_collide == -1.0 || y_collide == -1.0) 
        && (y_collide == z_collide || y_collide == -1.0 || z_collide == -1.0);
}

fn main() {
    let mut particles = get_particles();
    let mut bubble_index = 0;
    while bubble_index < particles.len() {
        let mut did_collide = false;
        if particles.is_empty() {
            break;
        }

        let mut collisions = Vec::new();
        for idx in bubble_index + 1..particles.len()  {
            if collide(&particles[0], &particles[idx]) {
                //println!("collision!");
                did_collide = true;
                collisions.push(idx);
            }
        }

        if did_collide {
            collisions.reverse();
            for p_collision in collisions {
                particles.remove(p_collision);
            }
            particles.remove(0);
        } else {
            bubble_index += 1;
        }
    }

    println!("Number of remaining particles: {}", particles.len());
}

fn get_particles() -> Vec<Particle> {
    let mut particles = Vec::new();
    
    for line in util::read_file_lines("input.txt") {
        let outer_split:Vec<&str> = line.split("<").collect();
        let positions: Vec<&str> = outer_split[1].split(">").collect::<Vec<&str>>()[0].split(",").collect();
        let velocities: Vec<&str> = outer_split[2].split(">").collect::<Vec<&str>>()[0].split(",").collect();
        let accelerations: Vec<&str> = outer_split[3].split(">").collect::<Vec<&str>>()[0].split(",").collect();

        let x = positions[0].to_string().parse::<i64>().unwrap();
        let y = positions[1].to_string().parse::<i64>().unwrap();
        let z = positions[2].to_string().parse::<i64>().unwrap();

        let vx = velocities[0].to_string().parse::<i64>().unwrap();
        let vy = velocities[1].to_string().parse::<i64>().unwrap();
        let vz = velocities[2].to_string().parse::<i64>().unwrap();

        let ax = accelerations[0].to_string().parse::<i64>().unwrap();
        let ay = accelerations[1].to_string().parse::<i64>().unwrap();
        let az = accelerations[2].to_string().parse::<i64>().unwrap();

        particles.push( Particle {
            x: x,
            y: y,
            z: z,
            vx: vx,
            vy: vy,
            vz: vz,
            ax: ax,
            ay: ay,
            az: az
        });
    }
    return particles;
}