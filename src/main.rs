use rand::{self, Rng};
use raylib::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Unit {
    position: Vector2,
    velocity: Vector2,
    color: Color,
}

fn generate_units_in_circle(center: Vector2, radius: f32, count: usize) -> Vec<Unit> {
    let mut units = Vec::with_capacity(count);
    for i in 0..count {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (count as f32);
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        let mut rng = rand::thread_rng();
        units.push(Unit {
            position: Vector2::new(x, y),
            velocity: Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
            color: Color::RED,
        });
    }
    units
}

fn distance(vec1: Vector2, vec2: Vector2) -> f32 {
    ((vec1.x - vec2.x).powi(2) + (vec1.y - vec2.y).powi(2)).sqrt()
}

fn separation(unit: &Unit, units: Vec<Unit>, separation_distance: f32) -> Vector2 {
    let mut separation = Vector2::new(0.0, 0.0);
    for other in units {
        if other.position != unit.position {
            let distance = distance(unit.position, other.position);
            if distance < separation_distance {
                separation += (unit.position - other.position).normalized() / distance;
            }
        }
    }

    separation
}

fn alignment(unit: &Unit, units: Vec<Unit>, alignment_distance: f32) -> Vector2 {
    let mut alignment = Vector2::new(0.0, 0.0);
    let mut total = 0;
    for other in units {
        if other.position != unit.position {
            let distance = distance(unit.position, other.position);
            if distance < alignment_distance {
                alignment += other.velocity;
                total += 1;
            }
        }
    }
    if total > 0 {
        alignment /= total as f32;
    }

    alignment
}

fn cohesion(unit: &Unit, units: Vec<Unit>, cohesion_distance: f32) -> Vector2 {
    let mut cohesion = Vector2::new(0.0, 0.0);
    let mut total = 0;
    for other in units {
        if other.position != unit.position {
            let distance = distance(unit.position, other.position);
            if distance < cohesion_distance {
                cohesion += other.position;
                total += 1;
            }
        }
    }
    if total > 0 {
        cohesion /= total as f32;
        cohesion = (cohesion - unit.position).normalized();
    }
    cohesion
}

fn steer(unit: &Unit, target: Vector2) -> Vector2 {
    let desired = (target - unit.position).normalized();
    desired - unit.velocity
}

fn main() {
    let mut units = generate_units_in_circle(Vector2::new(400.0, 400.0), 200.0, 30);

    let (mut rl, thread) = raylib::init().size(800, 800).title("boids").build();

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        let units_to_check = units.clone();

        let mouse_position = rl.get_mouse_position();
        let threshold_distance = 100.0;

        for unit in &mut units {
            let separation_force = separation(unit, units_to_check.clone(), 100.0);
            let alignment_force = alignment(unit, units_to_check.clone(), 50.0);
            let cohesion_force = cohesion(unit, units_to_check.clone(), 40.0);
            let steer_force = steer(unit, mouse_position);

            let distance_to_mouse = distance(unit.position, mouse_position);
            if distance_to_mouse > threshold_distance {
                unit.velocity += separation_force.scale_by(1.5)
                    + alignment_force.scale_by(1.0)
                    + cohesion_force.scale_by(0.5)
                    + steer_force.scale_by(1.0);
            } else {
                unit.velocity += separation_force.scale_by(5.5)
                    + alignment_force.scale_by(0.8)
                    + cohesion_force.scale_by(0.0)
                    + steer_force.scale_by(0.9);
            }

            if unit.velocity.length() > 2.0 {
                unit.velocity = unit.velocity.normalized() * 2.0;
            }
            unit.position += unit.velocity * delta_time * 50.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        for unit in &units {
            d.draw_circle_v(unit.position, 10.0, unit.color);
        }
    }
}
