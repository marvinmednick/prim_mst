//use std::env;
//use std::process; 
//use std::io::{self, Write}; // use std::error::Error;
//use std::cmp;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap};
//use std::thread;
use regex::Regex;
//use std::fmt;

mod graph;
use crate::graph::{Graph};
extern crate minheap;
use minheap::MinHeap;
mod cmd_line;
use crate::cmd_line::CommandArgs;


pub struct Dijkstra {
    pub unprocessed_vertex : MinHeap::<i32>,
    pub processed_vertex : HashMap::<u32,i32>,
}
            

impl Dijkstra {

    pub fn new() -> Self {

        Dijkstra  {
            unprocessed_vertex : MinHeap::<i32>::new(),
            processed_vertex : HashMap::<u32,i32>::new(),
        }
    }


    // update scoring for dijkstra shortest path
    
    pub fn update_scoring(&mut self, graph: &mut Graph, id: u32) {
   //     println!("Dijsktra scoring {}",id);
        let adj_vertexes = graph.get_outgoing(id);
        
        // get the distance/score from the current vertex as the base
        let cur_vertex_distance = self.processed_vertex.get(&id).unwrap().clone();

        // update each of this nodes adjancent vertexes, if the new distance
        // is < the current distance
        for v in adj_vertexes {
  //          println!("Dijsktra updating adjacent {:?}",v);
            // if the adjacent vertex is still in the unprocessed list, then 
            // update the scoring, otherwise skip it (since its already in the processed list)
            if let Some(cur_score) = self.unprocessed_vertex.peek_id_data(v.vertex) {
                let new_score = cur_vertex_distance + v.weight;
                if new_score < cur_score {
//                    println!("Update scoring on {} from {} to {}",v.vertex,cur_score,new_score);
                    let vertex_index = self.unprocessed_vertex.get_id_index(v.vertex).unwrap().clone();
                    self.unprocessed_vertex.update(vertex_index,new_score);
 //                   println!("Unprocessed: {:?}",self.unprocessed_vertex)
                }
             }       
            
        }

    }

    pub fn shortest_paths(&mut self, graph: &mut Graph, starting_vertex: u32) {
        println!("Starting shortest path with {}",starting_vertex);

        if let Some(starting_index) = self.unprocessed_vertex.get_id_index(starting_vertex) {

            let index = starting_index.clone();
            self.unprocessed_vertex.delete(index);
            
            // setup the initial distance for the starting vertex to 0 (to itself)
            self.processed_vertex.insert(starting_vertex,0);

            self.update_scoring(graph,starting_vertex);

            while let Some((next_vertex,next_vertex_score)) = self.unprocessed_vertex.get_min_entry() {
 //               println!("Processing vertex {} score: {}",next_vertex,next_vertex_score);
                self.processed_vertex.insert(next_vertex,next_vertex_score);
                self.update_scoring(graph,next_vertex);
            }
         }       
        else {
            println!("Starting vertex {} is not in the graph",starting_vertex);
        }

    }

}

fn main() {


    let cmd_line = CommandArgs::new();

    println!("Hello, {:?}!",cmd_line);

    println!("Calulating shortest path from Vertex {} to all other vertexes",cmd_line.start_vertex);
  // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();


    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);

	let mut g = Graph::new();
	let mut d = Dijkstra::new();

    // read the first line
    let mut line = String::new();
    let _len = reader.read_line(&mut line).unwrap();

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let line_data = line.unwrap();
        println!("Processing {}",line_data);

        // split the line into the vertex and the list of adjacent vertexes/weight pairs
        let re_vertex = Regex::new(r"\s*(?P<src>\d+)\s+(?P<dest>\d+)\s+(?P<weight>-*\d+).*$").unwrap();
        // adjacent vertexes are in the format vertex,weight   - and regex below allows for
        // whitespace
        let caps = re_vertex.captures(&line_data).unwrap();
        let src_vertex = caps["src"].parse::<u32>().unwrap(); 
        let dest_vertex = caps["dest"].parse::<u32>().unwrap(); 
        let weight = caps["weight"].parse::<i32>().unwrap(); 
        g.add_edge(src_vertex,dest_vertex,weight);
        println!("Added Edge #{}: from {} - {} wgt: {} --  ",_count,src_vertex,dest_vertex,weight);
    }

    for v in g.vertex_map.keys() {
        d.unprocessed_vertex.insert(g.vertex_map[v].vertex_id,100000000);
    }


    g.print_vertexes();
    d.shortest_paths(&mut g,cmd_line.start_vertex);

    for v in g.vertex_map.keys() {
        println!("v {} - {}", v, d.processed_vertex[v]);
    }

}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Edge;

	fn setup_basic1() -> Graph {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2,1),Some(1));
        assert_eq!(g.add_edge(1,3,1),Some(2));
        assert_eq!(g.add_edge(2,3,1),Some(1));
        assert_eq!(g.add_edge(2,4,1),Some(2));
        assert_eq!(g.add_edge(3,4,1),Some(1));
        assert_eq!(g.get_outgoing(1),&[Edge::new(2,1),Edge::new(3,1)]);
		assert_eq!(g.get_outgoing(2),&[Edge::new(3,1),Edge::new(4,1)]);
		assert_eq!(g.get_outgoing(3),&[Edge::new(4,1)]);
		assert_eq!(g.get_outgoing(4),&[]);
		g
	} 

    #[test]
    fn basic() {
		let mut g = Graph::new();
		assert_eq!(g.create_vertex(&1),Some(1));
		assert_eq!(g.create_vertex(&2),Some(2));
		assert_eq!(g.add_edge(1,2,1),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2));
		assert_eq!(g.create_vertex(&3),Some(3));
		assert_eq!(g.add_edge(1,3,1),Some(2));
		assert_eq!(g.add_edge(2,3,1),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2,3));
		assert_eq!(g.add_edge(1,4,1),Some(3));
		assert_eq!(g.get_vertexes(),vec!(1,2,3,4));
		println!("{:?}",g);

    }

	#[test]
	fn test_add() {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2,1),Some(1));
		assert_eq!(g.get_outgoing(1),&[Edge::new(2,1)]);
		assert_eq!(g.get_incoming(2),&[Edge::new(1,1)]);
		assert_eq!(g.add_edge(1,3,1),Some(2));
		assert_eq!(g.get_outgoing(1),&[Edge::new(2,1),Edge::new(3,1)]);
		assert_eq!(g.get_incoming(2),&[Edge::new(1,1)]);
	}

	#[test]
	fn test_add_del() {
		let mut g = setup_basic1();
		assert_eq!(g.get_outgoing(1),&[Edge::new(2,1),Edge::new(3,1)]);
		assert_eq!(g.add_edge(1,2,1),Some(3));
		assert_eq!(g.get_outgoing(1),&[Edge::new(2,1),Edge::new(3,1)]);
		assert_eq!(g.get_outgoing(2),&[Edge::new(3,1),Edge::new(4,1)]);
		assert_eq!(g.get_outgoing(3),&[Edge::new(4,1)]);
		assert_eq!(g.delete_edge(1,2,1),Ok(()));
		assert_eq!(g.get_outgoing(1),&[Edge::new(2,1),Edge::new(3,1)]);
		assert_eq!(g.delete_edge(1,2,1),Ok(()));
		assert_eq!(g.get_outgoing(1),&[Edge::new(3,1)]);
		
	}


 }
