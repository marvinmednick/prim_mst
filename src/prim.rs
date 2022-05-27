use std::collections::{HashMap};
extern crate minheap;
use minheap::MinHeap;
use crate::graph::Graph;



pub struct Prim {
    pub unprocessed_vertex : MinHeap::<i32>,
    pub processed_vertex : HashMap::<u32,i32>,
}
            

impl Prim {

    pub fn new() -> Self {

        Prim  {
            unprocessed_vertex : MinHeap::<i32>::new(),
            processed_vertex : HashMap::<u32,i32>::new(),
        }
    }


    // update scoring for Prim MST  
    
    pub fn update_scoring(&mut self, graph: &mut Graph, id: u32) {
        // get a vector of outgoing edges... (comprised of vertex and weight
        let adj_vertexes = graph.get_outgoing(id);
        
        // update each of this nodes adjancent vertexes, 
        // setting their score to their weight
        for edge in adj_vertexes {
            if let Some(cur_score) = self.unprocessed_vertex.peek_id_data(edge.vertex) {
            //    println!("Edge to vertex {} has weight {}",edge.vertex,cur_score);
                if edge.weight < cur_score {
                    let vertex_index= self.unprocessed_vertex.get_id_index(edge.vertex).unwrap().clone() ;
                    self.unprocessed_vertex.update(vertex_index,edge.weight);
                }
            }
            else {
             //   println!("skipping... already processed");
            }
            //println!("Done with {:?}",edge)

        }


    }

    pub fn display(&self) {

        println!("Unprocessed:");
        println!("{:?}", self.unprocessed_vertex);

        println!("Processed:");
        let mut cnt = 0;
        for edge in &self.processed_vertex {
            cnt += 1;
            if cnt % 10 == 0 {
                println!("[v {} w{} ]",edge.0,edge.1);
            }
            else {
                print!("[v {} w{} ] ",edge.0,edge.1);
            }
            
        }
        println!("");

    }

    pub fn min_span_tree(&mut self, graph: &mut Graph, starting_vertex: u32) {
        println!("Starting Min Span Tree path with {}",starting_vertex);
        //println!("Unprocessed: {:?}",self.unprocessed_vertex);

        if let Some(starting_index) = self.unprocessed_vertex.get_id_index(starting_vertex) {

            let index = starting_index.clone();
            self.unprocessed_vertex.delete(index);
         //   println!("Unprocessed After Delete: {:?}",self.unprocessed_vertex);
            
            // setup the initial distance for the starting vertex to 0 (to itself)
            self.processed_vertex.insert(starting_vertex,0);

            // update the scoring in the unprocessed heap so the next vertex is that the top
            self.update_scoring(graph,starting_vertex);

            
        //    println!("Afer initial vertex selection");
        //   self.display();
            // pull out each vertex from the heap, add it to processed list, and
            // update the weights based on the adjacent vertexes, and then select the 
            // closeset one, repeating until the heap is empty (all vertexes have been processed)
            while let Some((next_vertex,next_vertex_score)) = self.unprocessed_vertex.get_min_entry() {
          //      println!("Processing vertex {} score: {}",next_vertex,next_vertex_score);
                self.processed_vertex.insert(next_vertex,next_vertex_score);
                self.update_scoring(graph,next_vertex);
//                self.display();
            }
         }       
        else {
            println!("Starting vertex {} is not in the graph",starting_vertex);
        }

    }


}
