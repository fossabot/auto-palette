use crate::math::clustering::cluster::Cluster;
use crate::math::metrics::DistanceMetric;
use crate::math::point::Point;
use rand::Rng;
use rand_distr::{Distribution, WeightedAliasIndex};

/// Kmeans represents the K-means clustering algorithm.
///
/// # Type Parameters
/// * `R` - The random number generator.
#[derive(Debug)]
pub struct Kmeans<R: Rng + Clone> {
    k: usize,
    max_iter: usize,
    tolerance: f32,
    rng: R,
    metric: DistanceMetric,
}

impl<R: Rng + Clone> Kmeans<R> {
    /// Creates a new `Kmeans` instance.
    ///
    /// # Arguments
    /// * `k` - The number of clusters.
    /// * `max_iter` - The maximum number of iterations.
    /// * `tolerance` - The tolerance for convergence conditions.
    /// * `rng` - The random number generator.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `Kmeans` instance.
    ///
    /// # Errors
    /// Returns an error if the number of clusters is zero, the maximum number of iterations is zero,
    /// or the tolerance is less than or equal to zero.
    pub fn new(
        k: usize,
        max_iter: usize,
        tolerance: f32,
        rng: R,
        metric: DistanceMetric,
    ) -> Result<Self, &'static str> {
        if k == 0 {
            return Err("The number of clusters must be greater than zero.");
        }
        if max_iter == 0 {
            return Err("The maximum number of iterations must be greater than zero.");
        }
        if tolerance <= 0.0 {
            return Err("The tolerance must be greater than zero.");
        }
        Ok(Self {
            k,
            max_iter,
            tolerance,
            rng,
            metric,
        })
    }

    /// Fits the K-means algorithm to the given points.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters of the points.
    pub fn fit<const N: usize>(&self, points: &[Point<N>]) -> Vec<Cluster<N>> {
        if points.is_empty() {
            return Vec::new();
        }

        if self.k >= points.len() {
            return points
                .iter()
                .enumerate()
                .map(|(index, point)| {
                    let mut cluster = Cluster::new();
                    cluster.add_point(index, point);
                    cluster
                })
                .collect();
        }

        let mut rng = self.rng.clone();
        let index = rng.gen_range(0..points.len());

        // Initialize the centroids.
        let mut centroids = Vec::with_capacity(self.k);
        centroids.push(points[index]);

        for _ in 1..self.k {
            let mut distances = vec![f32::INFINITY; points.len()];
            for (i, point) in points.iter().enumerate() {
                for centroid in &centroids {
                    let distance = self.metric.measure(point, centroid);
                    distances[i] = distances[i].min(distance);
                }
            }

            let weighted_index = WeightedAliasIndex::new(distances).unwrap();
            let index = weighted_index.sample(&mut rng);
            centroids.push(points[index]);
        }

        let mut clusters = vec![Cluster::new(); self.k];
        for _ in 0..self.max_iter {
            for cluster in &mut clusters {
                cluster.clear();
            }

            for (index, point) in points.iter().enumerate() {
                let mut min_distance = f32::INFINITY;
                let mut cluster_id = 0;
                for (i, centroid) in centroids.iter().enumerate() {
                    let distance = self.metric.measure(point, centroid);
                    if distance < min_distance {
                        min_distance = distance;
                        cluster_id = i;
                    }
                }
                clusters[cluster_id].add_point(index, point);
            }

            let mut max_shift = 0.0_f32;
            let new_centroids = clusters.iter().map(|cluster| *cluster.centroid()).collect();
            for (old, new) in centroids.iter().zip(&new_centroids) {
                let distance = self.metric.measure(old, new);
                max_shift = max_shift.max(distance);
            }

            if max_shift < self.tolerance {
                break;
            }
            centroids = new_centroids;
        }
        clusters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_kmeans() {
        // Act
        let kmeans =
            Kmeans::new(3, 10, 1e-3, rand::thread_rng(), DistanceMetric::Euclidean).unwrap();

        // Assert
        assert_eq!(kmeans.k, 3);
        assert_eq!(kmeans.max_iter, 10);
        assert_eq!(kmeans.tolerance, 1e-3);
        assert_eq!(kmeans.metric, DistanceMetric::Euclidean);
    }
}
