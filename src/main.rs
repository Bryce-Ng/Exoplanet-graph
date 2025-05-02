//This is the main file that houses the main function.

//importing crates
use std::error::Error; // Allows the program to return an Error using Result
use rand::seq::SliceRandom; // Allows the program to randomly choose items from a list
use rand::rng; 
use rand::prelude::IndexedRandom;

mod graph; // Includes the graph.rs module
use graph::Graph; // Brings Graph into scope

fn main() -> Result<(), Box<dyn Error>> {
    // loads the Exoplanet.csv file
    let file_path = "Exoplanet.csv";

    //Creates the graph using the loaded csv file
    let graph = match Graph::from_csv(file_path) {
        Ok(g) => g,
        Err(e) => {
            println!("Failed to read CSV: {}", e);
            return Ok(());
        }
    };

    // Collects all planets' names as a list 
    let vertices: Vec<_> = graph.planets.keys().cloned().collect();
    if vertices.len() < 2 {
        println!("Not enough planets in dataset.");
        return Ok(());
    }

    //Randomly chooses a starting exoplanet and target exoplanet
    let mut rng = rng();
    let start = vertices.choose(&mut rng).unwrap();
    let mut target = vertices.choose(&mut rng).unwrap();
    while start == target {
        target = vertices.choose(&mut rng).unwrap();
    }

    // Outputs the information of the starting planet
    println!("Start planet: {}", start);
    if let Some(info) = graph.get_planet_info(start) {
        println!("{:#?}", info);
    }

    // Outputs the information of the target planet
    println!("Target planet: {}", target);
    if let Some(info) = graph.get_planet_info(target) {
        println!("{:#?}", info);
    }

    // Performs BFS on the starting and target planets and returns the shortest distance
    match graph.bfs_distance(start, target) {
        Some(dist) => println!("Breadth-first Search distance from {} to {} is: {}", start, target, dist),
        None => println!("No path found between {} and {}", start, target),
    }

    // Chooses another random exoplanet as well as counts and outputs the neighbors of that exoplanet
    let random_vertex = vertices.choose(&mut rng).unwrap();
    let neighbors = graph.count_neighbors(random_vertex);
    println!("{} has {} neighbors.", random_vertex, neighbors);

    if let Some(neighbor_list) = graph.adj_list.get(random_vertex) {
        println!("Neighbors of {}:", random_vertex);
        for neighbor in neighbor_list {
            println!(" - {}", neighbor);
        }
    }

    Ok(())
}
