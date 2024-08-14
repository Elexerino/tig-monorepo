/*!
Copyright 2024 Louis Silva

Licensed under the TIG Commercial License v1.0 (the "License"); you
may not use this file except in compliance with the License. You may obtain a copy
of the License at

https://github.com/tig-foundation/tig-monorepo/tree/main/docs/licenses

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the specific
language governing permissions and limitations under the License.
 */

use anyhow::Result;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use tig_challenges::vehicle_routing::{Challenge, Solution};

type SolutionWithRating = (Vec<Vec<usize>>, i32);

const ALFA: f64 = 2.0;
const BETA: f64 = 5.0;
const GAMMA: f64 = 9.0;
const RHO: f64 = 0.8;
const THETA: f64 = 80.0;
const SIGMA: usize = 3;
const ITERATIONS: usize = 50000;
const NO_IMPROVEMENT_THRESHOLD: usize = 100;
const USE_TWO_OPT: bool = false;
const USE_THREE_OPT: bool = true;
const USE_SWAP: bool = true;

const DEBUG: bool = false;

macro_rules! debug_println {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
        }
    };
}

pub fn solve_challenge(challenge: &Challenge) -> Result<Option<Solution>> {
    let num_nodes: usize = challenge.difficulty.num_nodes;
    let vehicle_capacity: i32 = challenge.max_capacity;
    let demands: &Vec<i32> = &challenge.demands;
    let distance_matrix: &Vec<Vec<i32>> = &challenge.distance_matrix;
    let num_ants: usize = num_nodes;

    let savings_matrix = compute_savings_matrix(distance_matrix, num_nodes);
    let initial_solution = nearest_neighbor_heuristic_with_candidates(
        num_nodes,
        vehicle_capacity,
        demands,
        distance_matrix,
    );
    let initial_solution_rating: i32 = rate_solution(&initial_solution, distance_matrix);

    let mut best_solution: Option<SolutionWithRating> =
        Some((initial_solution.clone(), initial_solution_rating));

    // Populate pheromone matrix with initial pheromones generated by initial solution
    let initial_total_distance: i32 = initial_solution
        .iter()
        .map(|route| calculate_route_distance(route, distance_matrix))
        .sum();
    let tau_0: f64 = num_ants as f64 / initial_total_distance as f64;
    let mut pheromones: Vec<Vec<f64>> = vec![vec![tau_0 * 0.1; num_nodes]; num_nodes];

    for route in &initial_solution {
        for w in route.windows(2) {
            let u = w[0];
            let v = w[1];
            pheromones[u][v] = tau_0;
            pheromones[v][u] = tau_0;
        }
    }

    let mut iterations_without_improvement: usize = 0;
    let mut previous_best_cost = best_solution.as_ref().unwrap().1;

    for i in 0..ITERATIONS {
        let mut solutions: Vec<SolutionWithRating> = Vec::with_capacity(num_ants);

        let mut alfa: f64 = ALFA;
        let mut beta: f64 = BETA;
        let mut rho: f64 = RHO;
        if iterations_without_improvement > NO_IMPROVEMENT_THRESHOLD / 2 {
            alfa *= 1.1;
            beta *= 0.9;
            rho *= 0.95;
        }

        for ant_index in 0..num_ants {
            let ant_seed = challenge.seed as u32 + ant_index as u32;
            let solution = solution_of_one_ant(
                num_nodes,
                vehicle_capacity,
                demands,
                distance_matrix,
                &pheromones,
                &savings_matrix,
                ant_seed,
                alfa,
                beta,
                GAMMA,
            )?;
            let solution_rating = rate_solution(&solution, distance_matrix);
            solutions.push((solution, solution_rating));
        }

        update_pheromone(&mut pheromones, &mut solutions, &mut best_solution, rho);
        let solution_ratings: Vec<i32> = solutions.iter().map(|(_, rating)| *rating).collect();

        debug_println!("Iteration: {}", i);
        debug_println!("Current solutions: {:?}", solution_ratings);

        if let Some((_, best_cost)) = best_solution {
            debug_println!("Current best solution cost: {}", best_cost);

            if best_cost < previous_best_cost {
                previous_best_cost = best_cost;
                iterations_without_improvement = 0;
            } else {
                iterations_without_improvement += 1;
            }

            if iterations_without_improvement >= NO_IMPROVEMENT_THRESHOLD {
                debug_println!(
                    "Terminating due to no improvement for {} iterations.",
                    NO_IMPROVEMENT_THRESHOLD
                );
                break;
            }
        }
    }

    Ok(best_solution.map(|solution| Solution { routes: solution.0 }))
    //Ok(Some(Solution{routes: best_solution.unwrap().0}))
}

fn update_pheromone(
    pheromones: &mut Vec<Vec<f64>>,
    solutions: &mut Vec<SolutionWithRating>,
    best_solution: &mut Option<SolutionWithRating>,
    rho: f64,
) {
    let l_avg: f64 =
        solutions.iter().map(|&(_, cost)| cost).sum::<i32>() as f64 / solutions.len() as f64;
    let theta_over_l_avg = THETA / l_avg;

    for row in pheromones.iter_mut() {
        for pheromone in row.iter_mut() {
            *pheromone = (*pheromone * rho + theta_over_l_avg).min(1.0);
        }
    }

    solutions.sort_by(|a, b| a.1.cmp(&b.1));

    if let Some(ref mut best) = best_solution {
        if solutions[0].1 < best.1 {
            *best = solutions[0].clone();
        }
    } else {
        *best_solution = Some(solutions[0].clone());
    }

    if let Some(ref best) = best_solution {
        let best_cost = best.1 as f64;
        for route in &best.0 {
            if route.len() >= 2 {
                for window in route.windows(2) {
                    let (u, v) = (window[0], window[1]);
                    let min_uv = u.min(v);
                    let max_uv = u.max(v);
                    pheromones[min_uv][max_uv] += SIGMA as f64 / best_cost;
                    pheromones[max_uv][min_uv] = pheromones[min_uv][max_uv];
                }
            }
        }
    }

    let top_solutions = solutions.iter().take((SIGMA - 1).min(solutions.len()));
    for (l, solution) in top_solutions.enumerate() {
        let deposit_amount: f64 = (THETA * (SIGMA as f64 - l as f64)) / solution.1 as f64;
        for route in &solution.0 {
            if route.len() >= 2 {
                for window in route.windows(2) {
                    let (u, v) = (window[0], window[1]);
                    let min_uv = u.min(v);
                    let max_uv = u.max(v);
                    pheromones[min_uv][max_uv] += deposit_amount;
                    pheromones[max_uv][min_uv] = pheromones[min_uv][max_uv];
                }
            }
        }
    }
}

fn solution_of_one_ant(
    num_nodes: usize,
    vehicle_capacity: i32,
    demands: &Vec<i32>,
    distance_matrix: &Vec<Vec<i32>>,
    pheromones: &Vec<Vec<f64>>,
    savings: &Vec<Vec<f64>>,
    seed: u32,
    alfa: f64,
    beta: f64,
    gamma: f64,
) -> Result<Vec<Vec<usize>>> {
    let mut solution: Vec<Vec<usize>> = Vec::new();
    let mut vertices: Vec<usize> = (1..num_nodes).collect();

    let mut rng = StdRng::seed_from_u64(seed as u64);

    while !vertices.is_empty() {
        let mut route: Vec<usize> = vec![0];
        let current_node_index: usize = rng.gen_range(0..vertices.len());
        let mut current_node = vertices.remove(current_node_index);
        let mut current_capacity = vehicle_capacity - demands[current_node];
        route.push(current_node);

        while !vertices.is_empty() {
            let epsilon = 1e-10;
            let mut probabilities: Vec<f64> = Vec::new();
            let mut vertex_pairs: Vec<usize> = Vec::new();

            for &x in &vertices {
                if x != current_node && current_capacity >= demands[x] {
                    let key = (x.min(current_node), x.max(current_node));
                    let tau = pheromones[key.0][key.1];
                    let eta = 1.0 / (distance_matrix[key.0][key.1] as f64 + epsilon);
                    let mu = savings[key.0][key.1];
                    let probability = (tau.powf(alfa) * eta.powf(beta) * mu.powf(gamma)).max(0.0);
                    probabilities.push(probability);
                    vertex_pairs.push(x);
                }
            }

            if probabilities.is_empty() {
                break;
            }

            let sum: f64 = probabilities.iter().sum();
            if sum == 0.0 {
                break;
            }

            for prob in &mut probabilities {
                *prob /= sum;
            }

            if let Some(chosen_index) = sample_from_probabilities(&probabilities, &mut rng) {
                let next_node = vertex_pairs[chosen_index];
                if current_capacity >= demands[next_node] {
                    current_capacity -= demands[next_node];
                    route.push(next_node);
                    current_node = next_node;
                    vertices.retain(|&x| x != next_node);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        route.push(0);
        if USE_TWO_OPT {
            route = two_opt(&route, distance_matrix, demands, vehicle_capacity);
        }
        solution.push(route);
    }

    if USE_THREE_OPT {
        three_opt(&mut solution, distance_matrix, demands, vehicle_capacity);
    }

    if USE_SWAP {
        try_swap(&mut solution, distance_matrix, demands, vehicle_capacity);
    }

    Ok(solution)
}

fn rate_solution(solution: &Vec<Vec<usize>>, distance_matrix: &Vec<Vec<i32>>) -> i32 {
    solution
        .iter()
        .map(|route| {
            route
                .windows(2)
                .map(|w| distance_matrix[w[0].min(w[1])][w[0].max(w[1])])
                .sum::<i32>()
                + distance_matrix[*route.last().unwrap_or(&0).min(&0)]
                    [*route.last().unwrap_or(&0).max(&0)]
        })
        .sum()
}

fn two_opt(
    route: &Vec<usize>,
    distance_matrix: &Vec<Vec<i32>>,
    demands: &Vec<i32>,
    vehicle_capacity: i32,
) -> Vec<usize> {
    let mut best_route = route.clone();
    let mut improved = true;

    while improved {
        improved = false;
        let mut best_distance = calculate_route_distance(&best_route, distance_matrix);

        for i in 1..best_route.len() - 2 {
            for j in i + 1..best_route.len() - 1 {
                let mut new_route = best_route[..i].to_vec();
                let mut middle_segment = best_route[i..=j].to_vec();
                middle_segment.reverse();
                new_route.extend(middle_segment);
                new_route.extend(best_route[j + 1..].to_vec());

                if compute_route_demand(&new_route, demands) > vehicle_capacity {
                    continue; // Skip this new route if it exceeds capacity
                }

                let new_distance = calculate_route_distance(&new_route, distance_matrix);

                if new_distance < best_distance {
                    best_route = new_route;
                    best_distance = new_distance;
                    improved = true;
                }
            }
        }
    }

    best_route
}

fn three_opt(
    solution: &mut Vec<Vec<usize>>,
    distance: &Vec<Vec<i32>>,
    demands: &Vec<i32>,
    vehicle_capacity: i32,
) -> bool {
    let mut improved = false;

    for r in 0..solution.len() {
        let n = solution[r].len();

        let mut route_changed = true;
        while route_changed {
            route_changed = false;

            let original_route = solution[r].clone();

            for i in 0..n - 3 {
                for j in i + 1..n - 2 {
                    for k in j + 1..n - 1 {
                        let segments = [
                            &original_route[0..=i],
                            &original_route[i + 1..=j],
                            &original_route[j + 1..=k],
                            &original_route[k + 1..n],
                        ];

                        let new_routes = vec![
                            segments[0]
                                .iter()
                                .chain(segments[1].iter().rev())
                                .chain(segments[2].iter())
                                .chain(segments[3].iter())
                                .cloned()
                                .collect(),
                            segments[0]
                                .iter()
                                .chain(segments[1].iter())
                                .chain(segments[2].iter().rev())
                                .chain(segments[3].iter())
                                .cloned()
                                .collect(),
                            segments[0]
                                .iter()
                                .chain(segments[1].iter().rev())
                                .chain(segments[2].iter().rev())
                                .chain(segments[3].iter())
                                .cloned()
                                .collect(),
                            segments[0]
                                .iter()
                                .chain(segments[2].iter())
                                .chain(segments[1].iter().rev())
                                .chain(segments[3].iter())
                                .cloned()
                                .collect(),
                            segments[0]
                                .iter()
                                .chain(segments[2].iter().rev())
                                .chain(segments[1].iter().rev())
                                .chain(segments[3].iter())
                                .cloned()
                                .collect(),
                            segments[0]
                                .iter()
                                .chain(segments[2].iter())
                                .chain(segments[1].iter().rev())
                                .chain(segments[3].iter())
                                .cloned()
                                .collect(),
                        ];

                        for new_route in new_routes {
                            let new_demand = compute_route_demand(&new_route, demands);

                            if new_demand <= vehicle_capacity {
                                let current_cost = calculate_route_distance(&solution[r], distance);
                                let new_cost = calculate_route_distance(&new_route, distance);

                                if new_cost < current_cost {
                                    solution[r] = new_route;
                                    improved = true;
                                    route_changed = true;
                                    break; // Early exit once improvement is found
                                }
                            }
                        }

                        if route_changed {
                            break; // Early exit from the middle loop
                        }
                    }

                    if route_changed {
                        break; // Early exit from the outer loop
                    }
                }

                if route_changed {
                    break; // Reinitialize with the new route
                }
            }
        }
    }

    improved
}

fn try_swap(
    solution: &mut Vec<Vec<usize>>,
    distance_matrix: &Vec<Vec<i32>>,
    demands: &Vec<i32>,
    vehicle_capacity: i32,
) -> bool {
    fn compute_route_demand_with_swap(
        route: &Vec<usize>,
        demands: &Vec<i32>,
        swap_index: usize,
        new_customer: usize,
    ) -> i32 {
        let mut total_demand = 0;
        for (i, &node) in route.iter().enumerate() {
            if i == swap_index {
                total_demand += demands[new_customer];
            } else {
                total_demand += demands[node];
            }
        }
        total_demand
    }

    let mut improved = true;

    while improved {
        improved = false;
        for i in 0..solution.len() {
            for j in i + 1..solution.len() {
                for k in 1..solution[i].len() - 1 {
                    for l in 1..solution[j].len() - 1 {
                        let customer_i = solution[i][k];
                        let customer_j = solution[j][l];

                        let new_demand_i =
                            compute_route_demand_with_swap(&solution[i], demands, k, customer_j);
                        let new_demand_j =
                            compute_route_demand_with_swap(&solution[j], demands, l, customer_i);

                        if new_demand_i <= vehicle_capacity && new_demand_j <= vehicle_capacity {
                            let current_cost = distance_matrix[solution[i][k - 1]][solution[i][k]]
                                + distance_matrix[solution[i][k]][solution[i][k + 1]]
                                + distance_matrix[solution[j][l - 1]][solution[j][l]]
                                + distance_matrix[solution[j][l]][solution[j][l + 1]];

                            let new_cost = distance_matrix[solution[i][k - 1]][solution[j][l]]
                                + distance_matrix[solution[j][l]][solution[i][k + 1]]
                                + distance_matrix[solution[j][l - 1]][solution[i][k]]
                                + distance_matrix[solution[i][k]][solution[j][l + 1]];

                            if new_cost < current_cost {
                                // Perform the swap
                                solution[i][k] = customer_j;
                                solution[j][l] = customer_i;

                                improved = true;
                            }
                        }
                    }
                }
            }
        }
    }

    improved
}

fn nearest_neighbor_heuristic(
    num_nodes: usize,
    vehicle_capacity: i32,
    demands: &Vec<i32>,
    distance_matrix: &Vec<Vec<i32>>,
) -> Vec<Vec<usize>> {
    let mut solution: Vec<Vec<usize>> = Vec::new();
    let mut visited: Vec<bool> = vec![false; num_nodes];
    let mut remaining_capacity: i32;

    // Initialize depot as visited
    visited[0] = true;

    while visited.iter().any(|&v| !v) {
        let mut route: Vec<usize> = vec![0]; // Start route at the depot
        remaining_capacity = vehicle_capacity;

        loop {
            let last_node = *route.last().unwrap();
            let mut nearest_neighbor: Option<usize> = None;
            let mut nearest_distance: i32 = i32::MAX;

            for i in 1..num_nodes {
                if !visited[i] && demands[i] <= remaining_capacity {
                    let distance = distance_matrix[last_node][i];
                    if distance < nearest_distance {
                        nearest_distance = distance;
                        nearest_neighbor = Some(i);
                    }
                }
            }

            match nearest_neighbor {
                Some(neighbor) => {
                    route.push(neighbor);
                    visited[neighbor] = true;
                    remaining_capacity -= demands[neighbor];
                }
                None => break,
            }
        }

        route.push(0); // End route at the depot
        solution.push(route);
    }

    solution
}

fn nearest_neighbor_heuristic_with_candidates(
    num_nodes: usize,
    vehicle_capacity: i32,
    demands: &Vec<i32>,
    distance_matrix: &Vec<Vec<i32>>,
) -> Vec<Vec<usize>> {
    // Create candidates
    let num_candidates: usize = num_nodes / 3;
    let mut candidate_list = vec![Vec::with_capacity(num_candidates); num_nodes];
    for i in 0..num_nodes {
        let mut distances: Vec<(usize, i32)> = Vec::with_capacity(num_nodes - 1);
        for j in 0..num_nodes {
            if i != j {
                distances.push((j, distance_matrix[i][j]));
            }
        }
        distances.sort_by_key(|&(_, dist)| dist);
        candidate_list[i] = distances
            .into_iter()
            .take(num_candidates)
            .map(|(j, _)| j)
            .collect();
    }

    let mut solution: Vec<Vec<usize>> = Vec::new();
    let mut visited: Vec<bool> = vec![false; num_nodes];
    let mut remaining_capacity: i32;

    // Initialize depot as visited
    visited[0] = true;

    while visited.iter().any(|&v| !v) {
        let mut route: Vec<usize> = vec![0]; // Start route at the depot
        remaining_capacity = vehicle_capacity;

        loop {
            let last_node = *route.last().unwrap();
            let mut nearest_neighbor: Option<usize> = None;
            let mut nearest_distance: i32 = i32::MAX;

            // Check candidate list
            for &neighbor in &candidate_list[last_node] {
                if !visited[neighbor] && demands[neighbor] <= remaining_capacity {
                    let distance = distance_matrix[last_node][neighbor];
                    if distance < nearest_distance {
                        nearest_distance = distance;
                        nearest_neighbor = Some(neighbor);
                    }
                }
            }

            // Fallback to full nearest neighbor search if no candidate found
            if nearest_neighbor.is_none() {
                for i in 1..num_nodes {
                    if !visited[i] && demands[i] <= remaining_capacity {
                        let distance = distance_matrix[last_node][i];
                        if distance < nearest_distance {
                            nearest_distance = distance;
                            nearest_neighbor = Some(i);
                        }
                    }
                }
            }

            match nearest_neighbor {
                Some(neighbor) => {
                    route.push(neighbor);
                    visited[neighbor] = true;
                    remaining_capacity -= demands[neighbor];
                }
                None => break,
            }
        }

        route.push(0); // End route at the depot
        solution.push(route);
    }

    solution
}

fn compute_route_demand(route: &Vec<usize>, demands: &Vec<i32>) -> i32 {
    route.iter().map(|&customer| demands[customer]).sum()
}

fn calculate_route_distance(route: &Vec<usize>, distance_matrix: &Vec<Vec<i32>>) -> i32 {
    route
        .windows(2)
        .map(|window| distance_matrix[window[0]][window[1]])
        .sum()
}

fn compute_savings_matrix(distance_matrix: &Vec<Vec<i32>>, num_nodes: usize) -> Vec<Vec<f64>> {
    let mut savings_matrix = vec![vec![0.0; num_nodes]; num_nodes];

    for i in 1..num_nodes {
        let dist_i0 = distance_matrix[i][0] as f64;
        for j in 1..num_nodes {
            if i != j {
                let dist_0j = distance_matrix[0][j] as f64;
                let dist_ij = distance_matrix[i][j] as f64;
                savings_matrix[i][j] = dist_i0 + dist_0j - dist_ij;
            }
        }
    }

    savings_matrix
}

fn sample_from_probabilities(probabilities: &[f64], rng: &mut StdRng) -> Option<usize> {
    let total_weight: f64 = probabilities.iter().sum();
    if total_weight == 0.0 {
        return None;
    }

    let sample = rng.gen_range(0.0..total_weight);
    let mut cumulative_sum = 0.0;

    for (index, &weight) in probabilities.iter().enumerate() {
        cumulative_sum += weight;
        if sample < cumulative_sum {
            return Some(index);
        }
    }

    None
}
#[cfg(feature = "cuda")]
mod gpu_optimisation {
    use super::*;
    use cudarc::driver::*;
    use std::{collections::HashMap, sync::Arc};
    use tig_challenges::CudaKernel;

    // set KERNEL to None if algorithm only has a CPU implementation
    pub const KERNEL: Option<CudaKernel> = None;

    // Important! your GPU and CPU version of the algorithm should return the same result
    pub fn cuda_solve_challenge(
        challenge: &Challenge,
        dev: &Arc<CudaDevice>,
        mut funcs: HashMap<&'static str, CudaFunction>,
    ) -> anyhow::Result<Option<Solution>> {
        solve_challenge(challenge)
    }
}
#[cfg(feature = "cuda")]
pub use gpu_optimisation::{cuda_solve_challenge, KERNEL};
