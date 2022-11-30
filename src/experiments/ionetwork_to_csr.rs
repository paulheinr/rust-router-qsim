use rust_q_sim::io::network::{IOLink, IONetwork, IONode};
use rust_road_router::datastr::graph::{EdgeId, NodeId, Weight};

fn convert_network(path: &str) -> RoutingKitNetwork {
    let mut network = IONetwork::from_file(path);

    let mut first_out: Vec<EdgeId> = Vec::new();
    let mut head: Vec<NodeId> = Vec::new();
    let mut travel_time: Vec<Weight> = Vec::new();
    let mut latitude: Vec<f32> = Vec::new();
    let mut longitude: Vec<f32> = Vec::new();

    check_network_valid(&network);

    //sort links by from id
    network.links_mut().sort_by_key(|link: &IOLink| link.from.to_lowercase());
    //sort nodes by id
    network.nodes_mut().sort_by_key(|node: &IONode| node.id.to_lowercase());

    let mut links_before = 0;

    for node in network.nodes() {
        //TODO: make sure, that the coordinate system is correct
        longitude.push(node.x);
        latitude.push(node.y);

        first_out.push((links_before) as EdgeId);

        let links: Vec<&IOLink> = network.links().iter().filter(|link: &&IOLink| *link.from == node.id).collect();
        links_before += links.len();
        for link in links {
            head.push(get_node_index(&network, &link.to) as NodeId);
            travel_time.push((link.length / link.freespeed) as Weight);
        }
    }
    first_out.push(head.len() as EdgeId);

    RoutingKitNetwork {
        first_out,
        head,
        travel_time,
        latitude,
        longitude,
    }
}

//checks whether network consists of unique node ids
fn check_network_valid(network: &IONetwork) {
    let mut node_ids: Vec<String> = network.nodes().iter().map(|n| String::from(&n.id)).collect();
    node_ids.dedup();
    assert_eq!(node_ids.len(), network.nodes().len());
}

fn get_node_index(network: &IONetwork, id: &String) -> usize {
    network.nodes().iter().position(|node| node.id == *id).unwrap()
}

#[derive(Debug)]
struct RoutingKitNetwork {
    //CSR graph representation
    first_out: Vec<EdgeId>,
    head: Vec<NodeId>,
    travel_time: Vec<Weight>,
    latitude: Vec<f32>,
    longitude: Vec<f32>,
}

#[cfg(test)]
mod test {
    use crate::experiments::ionetwork_to_csr::convert_network;

    #[test]
    fn test_simple_network() {
        let network = convert_network("./assets/simple-network.xml");
        println!("{network:#?}");

        assert_eq!(network.first_out, vec![0, 0, 2, 4, 5]);
        assert_eq!(network.head, vec![2, 3, 2, 3, 1]);
        assert_eq!(network.travel_time, vec![1, 2, 1, 4, 2]);
        //we don't check latitude and longitude so far
    }
}