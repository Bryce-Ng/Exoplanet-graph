//This module is responsible for creating the graph of nodes and vertices and functions that use said graph
use std::collections::{HashMap, HashSet, VecDeque};
use csv::{ReaderBuilder, StringRecord};

#[derive(Debug, Clone)]
//The Planet struct stores information about each planet and they are used to display information for each exoplanet in the csv.
pub struct Planet {
    pub name: String,
    pub distance: f64,
    pub stellar_magnitude: f64,
    pub planet_type: String,
    pub discovery_year: u16,
    pub mass_multiplier: f64,
    pub mass_wrt: String,
    pub radius_multiplier: f64,
    pub radius_wrt: String,
    pub orbital_radius: f64,
}

//This struct models the graph of planets. it creates an adjacency list and stores the names of planets in the graph.
pub struct Graph {
    pub adj_list: HashMap<String, HashSet<String>>,
    pub planets: HashMap<String, Planet>,
}

impl Graph {
    //this function creates an empty graph using the Graph struct
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
            planets: HashMap::new(),
        }
    }
    //This function adds a planet to the graph and creates an empty neighbor in the adjacency list.
    pub fn add_planet(&mut self, planet: Planet) {
        let name = planet.name.clone();
        self.planets.insert(name.clone(), planet);
        self.adj_list.entry(name).or_insert_with(HashSet::new);
    }

    //This function creates and undirected edge between two planets and creates an empty neighbor list in the adjaceny list.
    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.adj_list
            .entry(from.to_string())
            .or_insert_with(HashSet::new)
            .insert(to.to_string());

        self.adj_list
            .entry(to.to_string())
            .or_insert_with(HashSet::new)
            .insert(from.to_string());
    }
    //This function performs a breadth-first search algorithm on the graph to find the shortest distance between two planets.
    //The input is a random starting planet and target planet. The output is the shortest distance.
    pub fn bfs_distance(&self, start: &str, target: &str) -> Option<usize> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        visited.insert(start.to_string(), 0);
        queue.push_back(start.to_string());

        //important loop that keeps popping the next node to explore at the front of the queue.
        //Also checks each node is the one we're looking for and return the distance if so.
        while let Some(current) = queue.pop_front() {
            if current == target {
                return visited.get(&current).copied();
            }

            //looks up neighbors, if there any, using the adjaceny list
            //Also important loop that loops through each neighbor to consider visiting and ensures we don't visit the same one twice.
            if let Some(neighbors) = self.adj_list.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains_key(neighbor) {
                        visited.insert(neighbor.clone(), visited[&current] + 1);
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        None
    }

    //This function returns the number of neighbors of a random exoplanet
    pub fn count_neighbors(&self, vertex: &str) -> usize {
        self.adj_list.get(vertex).map_or(0, |v| v.len())
    }

    //This function retrives the attributes of a given planet.
    pub fn get_planet_info(&self, name: &str) -> Option<&Planet> {
        self.planets.get(name)
    }

    //This function loads the csv file.
    //It parses each row into a planet struct and connects each planet to the next.
    pub fn from_csv(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
        let mut graph = Graph::new();
        let mut names = Vec::new();

        //Parsing the data
        for result in rdr.records() {
            let record: StringRecord = result?;
            let planet = Planet {
                name: record[0].to_string(),
                distance: record[1].parse().unwrap_or(0.0),
                stellar_magnitude: record[2].parse().unwrap_or(0.0),
                planet_type: record[3].to_string(),
                discovery_year: record[4].parse().unwrap_or(0),
                mass_multiplier: record[5].parse().unwrap_or(0.0),
                mass_wrt: record[6].to_string(),
                radius_multiplier: record[7].parse().unwrap_or(0.0),
                radius_wrt: record[8].to_string(),
                orbital_radius: record[9].parse().unwrap_or(0.0),
            };
            names.push(planet.name.clone());
            graph.add_planet(planet);
        }

        // Simple sparse graph where each planet links to next two
        for i in 0..names.len() {
            if i + 1 < names.len() {
                graph.add_edge(&names[i], &names[i + 1]);
            }
            if i + 2 < names.len() {
                graph.add_edge(&names[i], &names[i + 2]);
            }
        }

        Ok(graph)
    }
}

//Test functions where information is hard coded to check if each function works properly.
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_planet(name: &str) -> Planet {
        Planet {
            name: name.to_string(),
            distance: 10.0,
            stellar_magnitude: 5.5,
            planet_type: "Gas Giant".to_string(),
            discovery_year: 2020,
            mass_multiplier: 1.0,
            mass_wrt: "Jupiter".to_string(),
            radius_multiplier: 1.0,
            radius_wrt: "Jupiter".to_string(),
            orbital_radius: 1.0,
        }
    }

    #[test]
    fn test_add_planet() {
        let mut graph = Graph::new();
        let planet = sample_planet("Earth");

        graph.add_planet(planet.clone());

        assert!(graph.planets.contains_key("Earth"));
        assert!(graph.adj_list.contains_key("Earth"));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_planet(sample_planet("Earth"));
        graph.add_planet(sample_planet("Mars"));

        graph.add_edge("Earth", "Mars");

        assert!(graph.adj_list["Earth"].contains("Mars"));
        assert!(graph.adj_list["Mars"].contains("Earth"));
    }

    #[test]
    fn test_bfs_distance_connected() {
        let mut graph = Graph::new();
        graph.add_planet(sample_planet("A"));
        graph.add_planet(sample_planet("B"));
        graph.add_planet(sample_planet("C"));

        graph.add_edge("A", "B");
        graph.add_edge("B", "C");

        assert_eq!(graph.bfs_distance("A", "C"), Some(2));
    }

    #[test]
    fn test_bfs_distance_disconnected() {
        let mut graph = Graph::new();
        graph.add_planet(sample_planet("A"));
        graph.add_planet(sample_planet("B"));

        // No edge between A and B
        assert_eq!(graph.bfs_distance("A", "B"), None);
    }

    #[test]
    fn test_count_neighbors() {
        let mut graph = Graph::new();
        graph.add_planet(sample_planet("Earth"));
        graph.add_planet(sample_planet("Mars"));
        graph.add_edge("Earth", "Mars");

        assert_eq!(graph.count_neighbors("Earth"), 1);
        assert_eq!(graph.count_neighbors("Mars"), 1);
    }

    #[test]
    fn test_get_planet_info() {
        let mut graph = Graph::new();
        let planet = sample_planet("Neptune");
        graph.add_planet(planet.clone());

        let fetched = graph.get_planet_info("Neptune").unwrap();
        assert_eq!(fetched.discovery_year, 2020);
        assert_eq!(fetched.planet_type, "Gas Giant");
    }
}
