mod util;

use std::collections::HashSet;

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

impl Particle {
    fn tick(&mut self) {
        self.vx += self.ax;
        self.vy += self.ay;
        self.vz += self.az;

        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn has_collided(&self, p1: &Particle) -> bool {
        self.x == p1.x && self.y == p1.y && self.z == p1.z
    }

    fn might_collide(&self, p1: &Particle) -> bool {
        let x_collide = Particle::dim_collide(self.x, self.vx, self.ax, p1.x, p1.vx, p1.ax);
        let y_collide = Particle::dim_collide(self.y, self.vy, self.ay, p1.y, p1.vy, p1.ay);
        let z_collide = Particle::dim_collide(self.z, self.vz, self.az, p1.z, p1.vz, p1.az);
        return x_collide && y_collide && z_collide;
    }

    fn dim_collide(x_0: i64, v_0: i64, a_0: i64, x_1: i64, v_1: i64, a_1: i64) -> bool {
        let b = (v_0 - v_1) as f64;
        let a = (0.5 * a_0 as f64) - (0.5 * a_1 as f64);
        let c = (x_0 - x_1) as f64;

        if a == 0.0 {
            if v_0 - v_1 == 0 {
                if x_0 == x_1 {
                    return true;
                }
                return false;
            }
            let t = (x_1 as f64 - x_0 as f64) / (v_0 as f64 - v_1 as f64);
            if t > 0.0 {
                return true;
            } else {
                return false;
            }
        }

        if (b * b) - (4.0 * a * c) < 0.0 {
            return false;
        }
        let t = max(
            (-b + ((b * b) - (4.0 * a * c)).sqrt()) / (2.0 * a),
            (-b - ((b * b) - (4.0 * a * c)).sqrt()) / (2.0 * a),
        );
        if t > 0.0 {
            return true;
        } else {
            return false;
        }
    }
}

fn max(v1: f64, v2: f64) -> f64 {
    if v1 >= v2 {
        return v1;
    }
    return v2;
}

fn main() {
    let mut particles = get_particles();
    let mut tick_num = 1;
    loop {
        for p in &mut particles {
            p.tick();
        }

        let mut collided = HashSet::new();
        let mut will_collide = false;

        for bubble_index in 0..(particles.len() - 1) {
            for idx in (bubble_index + 1)..particles.len() {
                if particles[bubble_index].has_collided(&particles[idx]) {
                    collided.insert(bubble_index);
                    collided.insert(idx);
                } else {
                    if particles[bubble_index].might_collide(&particles[idx]) {
                        will_collide = true;
                    }
                }
            }
        }

        let mut collisions: Vec<usize> = collided.into_iter().collect();
        collisions.sort();
        collisions.reverse();

        for col in collisions {
            particles.remove(col);
        }

        if !will_collide {
            break;
        }

        tick_num += 1;
    }

    println!("Number of remaining particles: {}", particles.len());
}

fn get_particles() -> Vec<Particle> {
    let mut particles = Vec::new();

    for line in util::read_file_lines("input.txt") {
        let outer_split: Vec<&str> = line.split("<").collect();
        let positions: Vec<&str> = outer_split[1].split(">").collect::<Vec<&str>>()[0]
            .split(",")
            .collect();
        let velocities: Vec<&str> = outer_split[2].split(">").collect::<Vec<&str>>()[0]
            .split(",")
            .collect();
        let accelerations: Vec<&str> = outer_split[3].split(">").collect::<Vec<&str>>()[0]
            .split(",")
            .collect();

        let x = positions[0].to_string().parse::<i64>().unwrap();
        let y = positions[1].to_string().parse::<i64>().unwrap();
        let z = positions[2].to_string().parse::<i64>().unwrap();

        let vx = velocities[0].to_string().parse::<i64>().unwrap();
        let vy = velocities[1].to_string().parse::<i64>().unwrap();
        let vz = velocities[2].to_string().parse::<i64>().unwrap();

        let ax = accelerations[0].to_string().parse::<i64>().unwrap();
        let ay = accelerations[1].to_string().parse::<i64>().unwrap();
        let az = accelerations[2].to_string().parse::<i64>().unwrap();

        particles.push(Particle {
            x: x,
            y: y,
            z: z,
            vx: vx,
            vy: vy,
            vz: vz,
            ax: ax,
            ay: ay,
            az: az,
        });
    }
    return particles;
}
