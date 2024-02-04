use rand::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: u8,
    pub y: u8,
    pub color: (u8, u8, u8)
}

pub struct Cluster {
    pub centroid: Point,
    pub points: Vec<Point>
}

impl Cluster {
    fn update_centroid(&mut self) {
        if !self.points.is_empty() {
            let sum_x: u32 = self.points.iter().map(|p| p.x as u32).sum();
            let sum_y: u32 = self.points.iter().map(|p| p.y as u32).sum();
            let avg_x: u8 = (sum_x / self.points.len() as u32) as u8;
            let avg_y: u8 = (sum_y / self.points.len() as u32) as u8;

            self.centroid.x = avg_x;
            self.centroid.y = avg_y;
        }
    }
}

fn assign_to_clusters(points: &[Point], clusters: &mut [Cluster]) {
    for cluster in clusters.iter_mut() {
        cluster.points.clear()
    }

    for point in points {
        let closet_cluster = clusters.into_iter().min_by_key(|cluster| {
            let dx = point.x as i32 - cluster.centroid.x as i32;
            let dy = point.y as i32 - cluster.centroid.y as i32;
            (dx * dx + dy * dy) as u32
        });

        if let Some(cluster) = closet_cluster {
            cluster.points.push(point.clone());
        }
    }
}

pub fn k_means(points: Vec<Point>, k: usize, max_iterations: usize) -> Vec<Cluster> {
    let mut rng = thread_rng();

    // Initialize clusters randomly
    let mut clusters: Vec<Cluster> = (0..k)
        .map(|_| Cluster {
            centroid: points.choose(&mut rng).unwrap().clone(),
            points: Vec::new(),
        })
        .collect();

    for _ in 0..max_iterations {
        assign_to_clusters(&points, &mut clusters);

        // Update centroids
        for cluster in clusters.iter_mut() {
            cluster.update_centroid();
        }
    }

    clusters
}
