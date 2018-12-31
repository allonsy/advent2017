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

fn compare_particles(p1: &Particle, p2: &Particle) -> i32 {
    let p1_a_dist = manhattan_distance(p1.ax, p1.ay, p1.az);
    let p2_a_dist = manhattan_distance(p2.ax, p2.ay, p2.az);
    if p1_a_dist > p2_a_dist {
        return 1;
    } else if p1_a_dist < p2_a_dist {
        return -1;
    } else {
        let p1_v_dist = manhattan_distance(p1.vx, p1.vy, p1.vz);
        let p2_v_dist = manhattan_distance(p2.vx, p2.vy, p2.vz);
        if p1_v_dist > p2_v_dist {
            return 1;
        } else if p1_v_dist < p2_v_dist {
            return -1;
        } else {
            let p1_d_dist = manhattan_distance(p1.x, p1.y, p1.z);
            let p2_d_dist = manhattan_distance(p2.x, p2.y, p2.z);
            if p1_d_dist > p2_d_dist {
                return 1;
            } else if p1_d_dist < p2_d_dist {
                return -1;
            } else {
                return 0;
            }
        }
    }
}

fn manhattan_distance(a: i64, b: i64, c: i64) -> i64 {
    return a.abs() + b.abs() + c.abs();
}

fn main() {
    let particles = get_particles();
    let mut closest_p_index = 0;
    let mut closest_particle: &Particle = &particles[0];

    for idx in 1..particles.len() {
        let this_particle = &particles[idx];
        if compare_particles(closest_particle, this_particle) > 0 {
            closest_particle = this_particle;
            closest_p_index = idx;
        } else if compare_particles(closest_particle, this_particle) == 0 {
            panic!("Equal particles at index: {} and {}", closest_p_index, idx);
        }
    }

    println!("Closest particle: {}", closest_p_index);
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
