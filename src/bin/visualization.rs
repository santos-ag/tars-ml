use petgraph::graph::UnGraph;
use std::io::Result;
use std::process::Command;
use tars::view::plot::*;

// fn network_to_ungraph() {}

fn main() -> Result<()> {
    println!("Hello from visualization module!");

    let mut graph = UnGraph::<String, f32>::new_undirected();

    let input = graph.add_node("Arroz".to_string());
    let hidden = graph.add_node("Sorvete".to_string());
    let output = graph.add_node("Arroz Com Sorvete".to_string());

    graph.add_edge(input, hidden, 1.0);
    graph.add_edge(hidden, output, 1.0);

    let net: NetGraph<String> = NetGraph::new(graph);
    let _dot = net.to_dot();

    let default_path: &str = "src/view/graph/network.dot";

    net.save_dot(default_path)?;

    Command::new("dot")
        .arg("src/view/graph/network.dot")
        .arg("-Tsvg")
        .arg("-o")
        .arg("src/view/graph/network.svg")
        .status()
        .expect("Falha ao executar o comando 'dot'. O Graphviz está instalado e no PATH?");

    net.plot();

    Ok(())
}
