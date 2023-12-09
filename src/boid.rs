use std::iter;

use rand::Rng;

use crate::{math::{Vector2D, self, WeightedMean}, settings::{Settings, self}, simulation::SIZE};

#[derive(Clone, Debug, PartialEq)]
pub struct Boid {
    position: Vector2D,
    velocity: Vector2D,
    radius: f64,
    hue: f64,
}

impl Boid {
    pub fn new_random(settings: &Settings) -> Self {
        let mut rng = rand::thread_rng();

        let max_radius = settings.min_distance / 2.0;
        let min_radius = max_radius / 6.0;
        // by using the third power large boids become rarer
        let radius = min_radius + rng.gen::<f64>().powi(3) * (max_radius - min_radius);

        Self {
            position: Vector2D::new(rng.gen::<f64>() * SIZE.x, rng.gen::<f64>() * SIZE.y),
            velocity: Vector2D::from_polar(rng.gen::<f64>() * math::TAU, settings.max_speed),
            radius,
            hue: rng.gen::<f64>() * math::TAU
        }
    }

    fn coherence(&self, boids: VisibleBoidIter, factor: f64) -> Vector2D {
        Vector2D::weighted_mean(
            boids.map(|other| (other.boid.position, other.boid.radius * other.boid.radius)),
        )
        .map(|mean| (mean - self.position) * factor)
        .unwrap_or_default()
    }
}

#[derive(Debug)]
struct VisibleBoid<'a> {
    boid: &'a Boid,
    offset: Vector2D,
    distance: f64,
}

#[derive(Clone, Debug)]
struct VisibleBoidIter<'boid> {
    // Pay no mind to this mess of a type.
    // It's just `before` and `after` joined together
    it: iter::Chain<std::slice::Iter<'boid, Boid>, std::slice::Iter<'boid, Boid>>,
    position: Vector2D,
    visible_range: f64,
}

impl<'boid> VisibleBoidIter<'boid> {
    fn new(
        before: &'boid [Boid],
        after: &'boid [Boid],
        position: Vector2D,
        visible_range: f64,
    ) -> Self {
        Self {
            it: before.iter().chain(after),
            position,
            visible_range,
        }
    }
}

impl <'boid> Iterator for VisibleBoidIter<'boid> {
    type Item = VisibleBoid<'boid>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            ref mut it,
            position,
            visible_range,
        } = *self;

        it.find_map(move |other| {
            let offset = other.position - position;
            let distance = offset.magnitude();

            if distance > visible_range {
                None
            } else {
                Some(VisibleBoid {
                    boid: other,
                    offset,
                    distance
                })
            }
        })
    }
}