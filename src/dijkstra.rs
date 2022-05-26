use std::collections::{HashMap};

extern crate minheap;
use minheap::MinHeap;
use crate::graph::Graph;


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
