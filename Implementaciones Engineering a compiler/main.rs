use std::{collections::HashMap, hash::Hash};

struct Node{
    
	acceptation: bool,
	transitions: HashMap<String, Vec<usize>>, // direct transitions by character
    e_transitions: Vec<usize>, // epsilon transitions
}

struct Automata{

	states :Vec<Node>, // s_0, ..., s_n where n = |states|
	alphabet: Vec<String>, // Necessary?
	accept_states: Vec<usize>,

}


impl Automata{
	// getNextAt(No., Str) -> Vec<Node>

	fn get_next_at(&self, n:usize, s:&str)-> Vec<usize>{	//Le hice cambio para que devolviera vacío si no había transicion
		let in_transition = self.states[n].transitions.keys().any(|x| x==s);
		
		if !in_transition{
			return vec![];
		}
		return self.states[n].transitions.get(s).unwrap().clone();

	}
    
    fn e_clousure(&self) -> HashMap<usize, Vec<usize>>{   //Le hice cambios porque no estaba haciendo el paso de unir el singulete con los E(n)
        let sts: Vec<usize>  = (0..self.states.len()).collect();
        
        let mut m= HashMap::<usize, Vec<usize>>::new();

        for i in 0..sts.len() {
            m.insert(sts[i], vec![i]);    
        }
        
        let mut worklist = sts.clone();
		
        while !worklist.is_empty() {
            let n = worklist.pop().unwrap();
			let eps_n = self.states[n].e_transitions.clone();
			let mut temp: Vec<usize> = vec![n];
			for i in eps_n{
				temp.extend(m.get(&i).unwrap());
			}
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

	fn subset_construction(&self) -> Automata{
		let map_e_clousures = self.e_clousure();
		let q_0: Vec<usize> = map_e_clousures.get(&0).unwrap().clone();


		let mut new_automata = Automata{
			states: Vec::<Node>::new(),
			alphabet: self.alphabet.clone(),
			accept_states: Vec::<usize>::new(),
		};
		
		let mut q_set: Vec<Vec<usize>> = Vec::<Vec<usize>>::new();
		q_set.push(q_0.clone());

		let mut worklist: Vec<Vec<usize>> = Vec::<Vec<usize>>::new();
		worklist.push(q_0.clone());
		
		while !worklist.is_empty() {
			let q = worklist.pop().unwrap();
			
			
			let mut new_node = Node{
				acceptation: false,
				transitions: HashMap::<String, Vec<usize>>::new(),
				e_transitions: Vec::<usize>::new(),
			};

			q.iter().for_each(|x| if self.accept_states.contains(x) {new_node.acceptation = true; new_automata.accept_states.push(*x)});

			for a in self.alphabet.clone() {
				let mut states: Vec<usize> = Vec::<usize>::new();
				for n in &q {
				 	let temp_states = self.get_next_at(*n, &a);
					
					for i in temp_states {
						map_e_clousures.get(&i).unwrap().iter().for_each(|x| if !states.contains(x) {states.push(*x);});
					}
				}
				
				if states != vec![]{
					
					//new_node.transitions.insert(a, states.clone());
					
					if !q_set.contains(&states) {
						q_set.push(states.clone());
						new_node.transitions.insert(a, vec![q_set.len()-1]);
						worklist.push(states.clone());
					}
					else{
						new_node.transitions.insert(a, vec![q_set.iter().position(|x| *x==states).unwrap()]);
					}
				}
				
			}
			new_automata.states.push(new_node);

		}
		
		// for n in &mut new_automata.states{
		// 	let alphabe_node: Vec<String> = n.transitions.keys().cloned().collect();
		// 	for a in alphabe_node{
		// 		let temp = n.transitions.get(&a).unwrap().clone();
		// 		n.transitions.remove(&a);
		// 		let index = q_set.iter().position(|x| *x==temp).unwrap();
		// 		n.transitions.insert(a, vec![index]);
		// 	}
		// }

		return new_automata;
		

	}

	fn regex_to_automaton(regex: &str) -> Automata{


		let mut start: Node = Node{

		acceptation: false,
		transitions: HashMap::from([]),
		// e_transitions: Vec::<usize>::from([]),
		e_transitions: Vec::new(), //More idiomatic

		};

		let mut aut = Automata{

		states: vec![start],
		alphabet: vec![],

		};

		let parts = regex.split('|');
		let mut n: usize = 0;

		// start.e_transitions.push(n+1); No se puede hacer
		aut.states[0].e_transitions.push(n + 1);

		for part in parts {

		n+=1;
		//let mut INI_CHAR = "";
		/***
		let mut current_state: Node = Node{

		acceptation: false,
		transitions: HashMap::from([(String:.from(), Vec::<usize>::from())]),
		e_transitions:Vec::<usize>::from([]),

		};
		***/

		// let mut INI_PAR = false;
		let mut open_par: Vec<usize> = Vec::new();

		for c in part.chars(){




			//if (c != "(" && c != "*" && c != ")"){
			if c != '(' && c != '*' && c != ')'{ //Comparación correcta con chars

			let new_state: Node = Node{

			acceptation: false,
			// transitions: HashMap::from([(c.to_string(), Vec::<usize>::from([n+1]), vec![n + 1])]),
			transitions: HashMap::from([(c.to_string(),vec![n + 1])]), // More idiomatic
			e_transitions:Vec::<usize>::from([]),
			
			};

							
			aut.states.push(new_state);

			}
			else if c == '('{

			open_par.push(n+1);

			}
			else if c == '*'{  
			/***
			 
			if let Some(&number) = open_par.last(){

			let new_state: Node = Node{

			acceptation: false,
			transitions: HashMap::from([(states[number].transitions[0].to_string(), Vec::<usize>::from([number]))]),
			e_transitions:Vec::<usize>::from([]),
				
				};
				
				aut.states.push(new_state);

			} else{

			println!("There's no last element");

			}

			open_par.pop();

			}
			***/
			if let Some(&number) = open_par.last() {

				if let Some(key) = aut.states[number].transitions.keys().next() {

					let new_state: Node = Node {

						acceptation: false,
						transitions: HashMap::from([(key.clone(), vec![number])]),
						e_transitions: Vec::new(),

					};

					aut.states.push(new_state);

				}

			} else {

				println!("There's no last element");

			}



		}
			n+=1;
		}

					

}

return aut

}

}




fn main(){
	
	//-----------------------------------------------
	//Estoy creando el automata del libro (a(b | c))
	//-----------------------------------------------
	let n_0: Node = Node{
		acceptation: false,
		transitions: HashMap::from([(String::from("a"), Vec::<usize>::from([1]))]),	
		e_transitions: Vec::<usize>::from([]),
	};
	
	let n_1: Node = Node{
		acceptation: false,
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([2]),
	}; 

	let n_2: Node = Node{
		acceptation: false,
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([3, 9]),
	}; 

	let n_3: Node = Node{
		acceptation: false,
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([4, 6]),
	}; 

	let n_4: Node = Node{
		acceptation: false,
		transitions: HashMap::from([(String::from("b"), Vec::<usize>::from([5]))]),  
		e_transitions: Vec::<usize>::from([]),
	}; 

	let n_5: Node = Node{
		acceptation: false,
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([8]),
	}; 

	let n_6: Node = Node{
		acceptation: false,
		transitions: HashMap::from([(String::from("c"), Vec::<usize>::from([7]))]),  
		e_transitions: Vec::<usize>::from([]),
	}; 

	let n_7: Node = Node{
		acceptation: false,
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([8]),
	}; 

	let n_8: Node = Node{
		acceptation: false,
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([3, 9]),
	}; 
	
	let n_9: Node = Node{
		acceptation: true,
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([]),
	}; 

	let aut0 = Automata{
		states: vec![n_0, n_1, n_2, n_3, n_4, n_5, n_6, n_7, n_8, n_9],
		alphabet: vec![String::from("a"), String::from("b"), String::from("c")],
		accept_states: vec![9],
	};


	//observando cómo queda el nuevo automata(DFA)
	let aut_dfa = aut0.subset_construction();
	println!("{:?}", aut_dfa.states[0].transitions);
	println!("{:?}", aut_dfa.states[1].transitions);
	println!("{:?}", aut_dfa.states[2].transitions);
	println!("{:?}", aut_dfa.states[3].transitions);
}