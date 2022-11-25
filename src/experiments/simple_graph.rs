#[derive(Debug)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}

impl Graph {
    fn is_valid(&self) -> bool {
        let mut res: bool = true;
        for edge in self.edges.iter() {
            let list: Vec<&Edge> = self.edges.iter().filter(|e| (*e).id == edge.id).collect();
            res = res && list.len() == 1;
        }
        res
    }

    fn traverse_path_sum_weights(&self, path: Vec<i32>) -> f32 {
        0.0
    }
}

#[derive(Debug)]
pub struct Vertex {
    pub id: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub id: i32,
    pub from: i32,
    pub to: i32,
    pub weight: f32,
}

#[cfg(test)]
mod tests {
    use crate::experiments::simple_graph::{Edge, Graph, Vertex};

    #[test]
    fn print_graph() {
        let vertex_list = vec![Vertex { id: 1 }, Vertex { id: 2 }, Vertex { id: 3 }];
        let edge_list = vec![Edge { id: 1, from: 1, to: 2, weight: 0.5 },
                             Edge { id: 2, from: 1, to: 3, weight: 2.0 },
                             Edge { id: 3, from: 2, to: 2, weight: 1.0 },
                             Edge { id: 4, from: 2, to: 3, weight: 4.0 },
                             Edge { id: 5, from: 3, to: 1, weight: 2.0 }];
        let graph = Graph {
            vertices: vertex_list,
            edges: edge_list,
        };
        println!("{:?}", graph);
        println!("First edge is {:?}", graph.edges.get(0));
        println!("Graph is valid: {:?}", graph.is_valid());
    }
}