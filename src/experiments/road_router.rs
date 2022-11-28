#[cfg(test)]
mod tests {
    use rust_road_router::{
        algo::{*, dijkstra::{*, query::dijkstra::Server as DijkServer}},
        datastr::graph::*,
    };

    fn get_graph() -> OwnedGraph {
        OwnedGraph::new(vec![0, 2, 4, 5], vec![1, 2, 1, 2, 0], vec![1, 2, 1, 4, 2])
    }

    #[test]
    fn test_simple_dijkstra() {
        let mut server = DijkServer::<_, DefaultOps>::new(get_graph());
        let mut result = server.query(Query { from: 2, to: 1 });
        assert_eq!(result.distance(), Some(3));
        println!("{:#?}", result.node_path())
    }
}