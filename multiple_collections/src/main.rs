use std::collections::HashMap;

/// Example: Using Multiple Collections
///
/// This example demonstrates how to use a `HashMap` to store key-value pairs
/// and handle potential errors using `Result`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// fn main() {
///     let mut map = HashMap::new();
///     map.insert("a", 10);
///
///     // Using Result to handle potential errors
///     let result = map.get("a").ok_or("Key not found");
///
///     match result {
///         Ok(value) => println!("Found: {}", value),
///         Err(e) => println!("Error: {}", e),
///     }
/// }
/// main();
/// ```
///
/// Here’s a more complex example that combines several collections to model a graph
/// with weighted edges, using a `HashMap` for nodes and a `Vec` for edges.
///
/// ```
/// use std::collections::HashMap;
///
/// struct Graph {
///     nodes: HashMap<&'static str, Vec<(&'static str, i32)>>,
/// }
///
/// impl Graph {
///     fn new() -> Self {
///         Graph {
///             nodes: HashMap::new(),
///         }
///     }
///
///     fn add_edge(&mut self, from: &'static str, to: &'static str, weight: i32) {
///         self.nodes.entry(from).or_insert(Vec::new()).push((to, weight));
///     }
///
///     fn get_edges(&self, node: &'static str) -> Option<&Vec<(&'static str, i32)>> {
///         self.nodes.get(node)
///     }
/// }
///
/// fn main() {
///     let mut graph = Graph::new();
///     graph.add_edge("A", "B", 5);
///     graph.add_edge("A", "C", 10);
///
///     if let Some(edges) = graph.get_edges("A") {
///         for (to, weight) in edges {
///             println!("Edge from A to {}: weight {}", to, weight);
///         }
///     }
/// }
/// main();
/// ```

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 10);

    // Using Result to handle potential errors
    let result = map.get("a").ok_or("Key not found");

    match result {
        Ok(value) => println!("Found: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_main_output() {
        let output = main();
        assert_eq!(output, "Found: 10\n");
    }
}
