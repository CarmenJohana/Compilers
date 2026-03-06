use std::{collections::HashMap, hash::Hash};

struct Node{
    
	acceptation: bool,
	transitions: HashMap<String, Vec<usize>>, // direct transitions by character
    e_transitions: Vec<usize>, // epsilon transitions
}

struct Automata{

	states :Vec<Node>, // s_0, ..., s_n where n = |states|
	alphabet: Vec<String>, // Necessary?

}


impl Automata{
	// getNextAt(No., Str) -> Vec<Node>

	fn get_next_at(&self, n:usize, s:&str)-> &Vec<usize>{
		return self.states[n].transitions.get(s).unwrap();

	}	
    
    fn e_clousure(&self, sts: Vec<usize>) -> HashMap<usize, Vec<usize>>{
        
        
        let mut m= HashMap::<usize, Vec<usize>>::new();

        for i in 0..sts.len() {
            m.insert(sts[i], vec![i]);    
        }
        
        let mut worklist = sts.clone();

        while !worklist.is_empty() {
            let n = worklist.pop().unwrap();
            let mut eps_n = self.states[n].e_transitions.clone();
            let mut temp: Vec<usize> = vec![n];
			temp.append(&mut eps_n);

            if temp != *m.get(&n).unwrap(){
                m.insert(n, temp);
                for st in &sts{
                    if self.states[*st].e_transitions.contains(&n){
						worklist.push(*st);
					}
                    
                }
            }

        }
		return m;

    }
}




fn main(){
	
	
	let si: Node = Node{
		acceptation: false,
		transitions: HashMap::from([(String::from("a"), Vec::<usize>::from([1]))]),	
		e_transitions: Vec::<usize>::from([1]),
	};
	
	let sf: Node = Node{
		acceptation: true,
		transitions: HashMap::from([
			(String::from("b"), Vec::<usize>::from([1])), 
			(String::from("c"), Vec::<usize>::from([1]))
		]),  
		e_transitions: Vec::<usize>::from([]),
	}; 
	

	let aut0 = Automata{
		states: vec![si, sf],
		alphabet: vec![String::from("a"), String::from("b"), String::from("c")],
	};

	println!("{:?}", aut0.get_next_at(0, "a"));
	println!("{:?}", aut0.e_clousure(vec![0]));
}