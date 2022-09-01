use std::collections::{HashMap,BTreeMap};


static mut MAX_OUT_LEVEL : u32= 0;
static mut MAX_IN_LEVEL : u32 = 0;

#[derive(Debug, Clone)]
pub struct Vertex {
	pub vertex_id: usize,
    // list of unique incoming edges along with a count of how many duplicates there are
	incoming: BTreeMap<Edge,usize>,
	incoming_cnt: usize,
    // list of unique outgoing edges along with a count of how many duplicates there are
	outgoing: BTreeMap<Edge,usize>,
	outgoing_cnt: usize,
}

#[derive(Debug,Clone,Ord,PartialOrd,Eq,PartialEq)]
pub struct Edge {
    pub vertex: usize,
    pub weight: i32
}

impl Edge {
    pub fn new(v : usize, w: i32 ) -> Self {
        Edge { vertex: v, weight: w }
    }
}

impl Vertex {

	pub fn new(id : &usize) -> Vertex {
		let incoming = BTreeMap::<Edge,usize>::new();
		let outgoing = BTreeMap::<Edge,usize>::new();
		Vertex {vertex_id: id.clone(), 
				incoming: incoming, 
				outgoing: outgoing,
				incoming_cnt : 0,
				outgoing_cnt : 0,
				}
	}
	
	pub fn add_outgoing(&mut self, vertex_id: usize, weight: i32) {
        let edge = Edge {vertex: vertex_id, weight: weight };
		let counter = self.outgoing.entry(edge).or_insert(0);
		*counter += 1;
		self.outgoing_cnt += 1;
	}

	pub fn del_outgoing (&mut self, vertex_id: usize, weight: i32) ->  Result <(), String> {

        let edge = Edge {vertex: vertex_id, weight: weight };

		match self.outgoing.get_mut(&edge) {
			None | Some(0)  => Err("Invalid Vertex".to_string()),
			Some(1)        =>  	{ 	
									self.outgoing.remove(&edge); 
									self.outgoing_cnt -= 1;
									Ok(())
								}, 
			Some(x)        => 	{	*x -=1;  
								 	self.outgoing_cnt -= 1;
								 	Ok(())
								},
		}
	}

	pub fn add_incoming(&mut self, vertex_id: usize, weight: i32) {
        let edge = Edge {vertex: vertex_id, weight: weight };
		let counter = self.incoming.entry(edge).or_insert(0);
		*counter += 1;
		self.incoming_cnt += 1;
	}

	pub fn del_incoming (&mut self, vertex_id: usize, weight: i32) -> Result<(),String> {
	
        let edge = Edge {vertex: vertex_id, weight: weight };
		match self.incoming.get_mut(&edge) {
			None | Some(0)  => Err("Invalid Vertex".to_string()),
			Some(1)        =>	{ 
									self.incoming.remove(&edge); 
									self.incoming_cnt -= 1;
									Ok(())
								}, 
			Some(x)        => 	{
									*x -=1;
									self.incoming_cnt -= 1;
									Ok(())
								},
		}

	}
}


#[derive(Debug,Clone)]
pub struct Graph {
	pub vertex_map:  BTreeMap::<usize, Vertex>,
	edge_count:  usize,
	explored:  HashMap::<usize,bool>,
	pub finished_order:  Vec::<usize>,
	pub start_search:  HashMap::<usize,Vec::<usize>>,
	top_search_cnts:  HashMap::<usize, usize>,
}


impl Graph {
	pub fn new() -> Graph {
		let v_map = BTreeMap::<usize, Vertex>::new();
		Graph {
				vertex_map: v_map,
				edge_count: 0,
				explored:  HashMap::<usize,bool>::new(),
				finished_order:  Vec::<usize>::new(),
				start_search : HashMap::<usize,Vec::<usize>>::new(),
				top_search_cnts : HashMap::<usize,usize>::new(),
		}
	}


	pub fn get_outgoing(&self, vertex: usize) -> Vec<Edge>{
		let v = self.vertex_map.get(&vertex).unwrap();
		v.outgoing.keys().cloned().collect()
		
	}

	pub fn get_incoming(&self,vertex: usize) -> Vec<Edge> {
		let v = self.vertex_map.get(&vertex).unwrap();
		v.incoming.keys().cloned().collect()
		
	}


	pub fn get_vertexes(&self) -> Vec<usize> {
		self.vertex_map.keys().cloned().collect()
			
	}

	pub fn print_vertexes(&self) {
		for (key, value) in &self.vertex_map {
			let out_list : String = value.outgoing.iter().map(|(x, y)| if y > &1 {format!("{:?}({}) ; ",x,y) } else { format!("{:?} ;",x)}).collect();
			println!("Vertex {} ({}) :  {}",key,value.vertex_id,out_list);
		}
					
	}

	pub fn create_vertex(&mut self,id: &usize) -> Option<usize> {

		if self.vertex_map.contains_key(&id) {
			None
		} 
		else { 
			let v = Vertex::new(&id);
			self.vertex_map.insert(id.clone(),v.clone());
			Some(self.vertex_map.len())  
		}
	}

	pub fn add_search_entry(&mut self, vertex: usize, count: usize) {

			self.top_search_cnts.insert(vertex,count);
			let mut removed = None;
			if self.top_search_cnts.len() > 10 {
				let top_search_iter = self.top_search_cnts.iter();
				let mut top_search_count_vec : Vec::<(usize, usize)> = top_search_iter.map(|(k,v)| (*k, *v)).collect();
				top_search_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
				removed = top_search_count_vec.pop();
			}
			if let Some(entry) = removed {
				self.top_search_cnts.remove(&entry.0);
				
			}
			
	}

	pub fn dfs_outgoing(&mut self, vertex_id:  usize, start_vertex: usize, level: u32) {
			
//			let spacer = (0..level*5).map(|_| " ").collect::<String>();
			unsafe {
			if level > MAX_OUT_LEVEL {
				MAX_OUT_LEVEL = level;
//					println!("reached level {}", MAX_OUT_LEVEL);
			}
			}
			
			// Set current node to explored
			self.explored.insert(vertex_id,true);

			let cur_len: usize;
		
			{
				let group_list = self.start_search.entry(start_vertex).or_insert(Vec::<usize>::new());
				group_list.push(vertex_id);
				cur_len = group_list.len();
			}
			self.add_search_entry(start_vertex,cur_len);

			
			let next_v : Vertex;

			if let Some(vertex) = self.vertex_map.get(&vertex_id) {

				next_v = vertex.clone();
			}

			else {
				panic!("invalid vertex");
			}

			// Search through each edge
			for edge in next_v.outgoing.keys() {
				let next_vertex = edge.vertex.clone();
				if !self.explored.contains_key(&edge.vertex) {
					self.dfs_outgoing(next_vertex,start_vertex,level+1);
				}
				else {
			//		println!("{}Vertex {} is already explored",spacer,edge);
				}
			}
			// so add it to the finished list
			self.finished_order.push(vertex_id);
	}

	pub fn dfs_incoming(&mut self, vertex_id:  usize, start_vertex: usize, level: u32) {
			
//			let spacer = (0..level*5).map(|_| " ").collect::<String>();
			unsafe {
			if level > MAX_IN_LEVEL {
				MAX_IN_LEVEL = level;
//				println!("reached level {}", MAX_IN_LEVEL);
			}
			}
			
			// Set current node to explored
			self.explored.insert(vertex_id,true);

			let group_list = self.start_search.entry(start_vertex).or_insert(Vec::<usize>::new());
			group_list.push(vertex_id);
			let cur_len = group_list.len();
			self.add_search_entry(start_vertex,cur_len);

			let next_v : Vertex;

			if let Some(vertex) = self.vertex_map.get(&vertex_id) {

				next_v = vertex.clone();
			}

			else {
				panic!("invalid vertex");
			}

			// Search through each edge
			for edge in next_v.incoming.keys() {
				let next_vertex = edge.vertex.clone();
				if !self.explored.contains_key(&edge.vertex) {
					self.dfs_incoming(next_vertex,start_vertex,level+1);
				}
				else {
			//		println!("{}Vertex {} is already explored",spacer,edge);
				}
			}
			// so add it to the finished list
			self.finished_order.push(vertex_id);
	}

	pub fn dfs_loop_incoming(&mut self, list: &Vec<usize>) {

//		println!("Looping on incoming DFS");
		self.finished_order = Vec::<usize>::new();
		self.start_search = HashMap::<usize,Vec::<usize>>::new();
		self.explored = HashMap::<usize,bool>::new();
		self.top_search_cnts = HashMap::<usize,usize>::new();

		let mut _count : usize = 0;
		for v in list {
/*			if _count % 1000000 == 0 {
				print!("*");
				io::stdout().flush().unwrap();
			} */
			let vertex = v.clone();
//			println!("Looping on {}",vertex);
			if !self.explored.contains_key(&vertex) {
				self.dfs_incoming(vertex,vertex,0);
			}
			_count += 1;
		}
	}

	pub fn dfs_loop_outgoing(&mut self, list: &Vec<usize>) {
//		println!("Looping on outgoing DFS");
		self.finished_order = Vec::<usize>::new();
		self.start_search = HashMap::<usize,Vec::<usize>>::new();
		self.explored = HashMap::<usize,bool>::new();
		self.top_search_cnts = HashMap::<usize,usize>::new();

		let mut _count : usize = 0;
		for v in list {
/*			if _count % 1000000 == 0 {
				print!("#");
				io::stdout().flush().unwrap();
			} */
			let vertex = v.clone();
//			println!("Looping on {}",vertex);
			if !self.explored.contains_key(&vertex) {
				self.dfs_outgoing(vertex,vertex,0);
			}
		}
	}

	pub fn add_edge(&mut self, v1: usize, v2: usize, weight: i32) -> Option<usize> {

		//create the vertexes, if the don't exist
		self.create_vertex(&v1);
		self.create_vertex(&v2);

		let v_map = &mut self.vertex_map;
		// add the edge to the first vertex's adjanceny list
		let vert = v_map.get_mut(&v1).unwrap(); 
		vert.add_outgoing(v2,weight);
		let new_cnt = vert.outgoing_cnt.clone();

		// add the edge to the second vertex adjacentcy list
		let vert2 = v_map.get_mut(&v2).unwrap(); 
		vert2.add_incoming(v1,weight);

		self.edge_count += 1;
		Some(new_cnt)

	}

	pub fn delete_edge(&mut self,v1 : usize, v2 : usize, weight: i32) -> Result<(),String>  {
	
		self.vertex_map.get_mut(&v1).unwrap().del_outgoing(v2,weight)?	;
		self.vertex_map.get_mut(&v2).unwrap().del_incoming(v1,weight)?;
		self.edge_count -= 1;
		Ok(())

	}

}

            
