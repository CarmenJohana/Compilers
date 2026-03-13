use std::{collections::HashMap, hash::Hash};

struct Node{
    
	transitions: HashMap<String, Vec<usize>>, // direct transitions by character
    e_transitions: Vec<usize>, // epsilon transitions
}

impl Node {
    fn new() -> Self {
        Node {
            transitions: HashMap::new(),
            e_transitions: Vec::new(),
        }
    }
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

	fn symbol_nfa(symbol: &str) -> Automata {

		let mut s0 = Node::new();
		let s1 = Node::new();

		s0.transitions.insert(symbol.to_string(), vec![1]);

		Automata {
			states: vec![s0, s1],
			alphabet: vec![symbol.to_string()],
			accept_states: vec![1],
		}
	}

	fn concatenate(mut a: Automata, mut b: Automata) -> Automata {

    let offset = a.states.len();

    for st in &mut b.states {
        for v in st.transitions.values_mut() {
            for x in v {
                *x += offset;
            }
        }
        for e in &mut st.e_transitions {
            *e += offset;
        }
    }

		for acc in &a.accept_states {
			a.states[*acc].e_transitions.push(offset);
		}

		a.states.extend(b.states);

		Automata {
			states: a.states,
			alphabet: [a.alphabet, b.alphabet].concat(),
			accept_states: b.accept_states.iter().map(|x| x + offset).collect(),
		}
	}


	fn union(mut a: Automata, mut b: Automata) -> Automata {

		let mut start = Node::new();

		let offset_a = 1;
		let offset_b = 1 + a.states.len();

		start.e_transitions.push(offset_a);
		start.e_transitions.push(offset_b);

		for st in &mut a.states {
			for v in st.transitions.values_mut() {
				for x in v {
					*x += offset_a;
				}
			}
			for e in &mut st.e_transitions {
				*e += offset_a;
			}
		}

		for st in &mut b.states {
			for v in st.transitions.values_mut() {
				for x in v {
					*x += offset_b;
				}
			}
			for e in &mut st.e_transitions {
				*e += offset_b;
			}
		}

		let mut states = vec![start];
		states.extend(a.states);
		states.extend(b.states);

		let mut accept = Vec::new();
		accept.extend(a.accept_states.iter().map(|x| x + offset_a));
		accept.extend(b.accept_states.iter().map(|x| x + offset_b));

		Automata {
			states,
			alphabet: [a.alphabet, b.alphabet].concat(),
			accept_states: accept,
		}
	}


	fn kleene(mut a: Automata) -> Automata {

		let mut start = Node::new();
		let offset = 1;

		start.e_transitions.push(offset);

		for st in &mut a.states {
			for v in st.transitions.values_mut() {
				for x in v {
					*x += offset;
				}
			}
			for e in &mut st.e_transitions {
				*e += offset;
			}
		}

		for acc in &a.accept_states {
			a.states[*acc].e_transitions.push(offset);
		}

		let mut states = vec![start];
		states.extend(a.states);

		Automata {
			states,
			alphabet: a.alphabet,
			accept_states: vec![0],
		}
	}


	fn split_top_level(regex: &str, op: char) -> Vec<String> {

		let mut parts = Vec::new();
		let mut level = 0;
		let mut last = 0;

		for (i, c) in regex.char_indices() {

			if c == '(' { level += 1; }
			if c == ')' { level -= 1; }

			if c == op && level == 0 {
				parts.push(regex[last..i].to_string());
				last = i + 1;
			}
		}

		parts.push(regex[last..].to_string());
		parts
	}	

	fn construir(regex: &str) -> Automata {

		let regex = regex.trim();

		// Union
		let parts = split_top_level(regex, '|');
		if parts.len() > 1 {

			let mut aut = construir(&parts[0]);

			for p in &parts[1..] {
				aut = union(aut, construir(p));
			}

			return aut;
		}

		// Kleene
		if regex.ends_with('*') {

			let inner = &regex[..regex.len() - 1];
			return kleene(construir(inner));
		}

		// Parentheses
		if regex.starts_with('(') && regex.ends_with(')') {
			return construir(&regex[1..regex.len() - 1]);
		}

		// Concatenation
		if regex.len() > 1 {

			let mut chars = regex.chars();
			let first = chars.next().unwrap().to_string();

			let mut aut = symbol_nfa(&first);

			for c in chars {
				aut = concatenate(aut, symbol_nfa(&c.to_string()));
			}

			return aut;
		}

		// symbol
		symbol_nfa(regex)
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
				transitions: HashMap::<String, Vec<usize>>::new(),
				e_transitions: Vec::<usize>::new(),
			};

			
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
						states.iter().for_each(|x| if self.accept_states.contains(x) {new_automata.accept_states.push(q_set.len()-1)});
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

	fn minimization(&self) -> Automata{
		let no_accept_states: Vec<usize> = (0..self.states.len()).filter(|x: &usize| !self.accept_states.contains(x)).collect();
		let mut t = Vec::<Vec<usize>>::from([self.accept_states.clone(), no_accept_states]);
		let mut p_set = Vec::<Vec<usize>>::new();
		let mut min_automata = Automata{
			states: Vec::<Node>::new(),
			alphabet: self.alphabet.clone(),
			accept_states: Vec::<usize>::new(),
		};

		while t != p_set{
			p_set = t.clone();
			t.clear();
			
			for i in &p_set.clone(){
				t.extend(self.split(i.clone(), p_set.clone()));
			}
		}

		println!("t: {:?}", t);

		
		p_set.sort_by_key(|x| *x.iter().min().unwrap());
		
		for i in &p_set{
			for j in i{
				if self.accept_states.contains(j){
					min_automata.accept_states.push(p_set.iter().position(|x| x.contains(j)).unwrap());
					break;
				}
			}			
			let mut new_node = Node{
				transitions: HashMap::<String, Vec<usize>>::new(),
				e_transitions: Vec::<usize>::new(),
			};
			let n_repre_trans = self.states[i[0]].transitions.clone();
			for a in self.alphabet.clone(){
				if n_repre_trans.contains_key(&a){
					let pos = p_set.iter().position(|x| x.contains(&n_repre_trans.get(&a).unwrap()[0])).unwrap();
					new_node.transitions.insert(a, vec![pos]);
				}
			}
			min_automata.states.push(new_node);
		}
		

		return min_automata;
	} 

	fn split(&self, s:Vec<usize>, p_set:Vec<Vec<usize>>) -> Vec<Vec<usize>>{
		let mut set = Vec::<usize>::new();
		for a in self.alphabet.clone(){
			set.clear();
			println!("letra: {}-----------", a);
			let mut temp_p:i32 = -2;
			for n in s.clone(){
				println!("pset: {:?}", p_set);
				let a_trans_set = self.get_next_at(n, &a);
				println!("nodo: {}, temp: {},trans: {:?}", n, temp_p ,a_trans_set);
				if a_trans_set == vec![]{
					println!("nodo: {}",n);
					if temp_p == -2{
						temp_p = -1;
					}
					else if temp_p == -1{
						temp_p = -1;
					}
					else{
						let compl_set = s.into_iter().filter(|x| !set.contains(x)).collect();
						return vec![set, compl_set];
					}
				}else{
					let a_trans = a_trans_set[0];
					for i in 0..p_set.len(){
						if p_set[i].contains(&a_trans){
							if temp_p == -2{
								temp_p = i as i32;
							}
							else if temp_p == i as i32{
								temp_p = i as i32;
							}
							else{
								let compl_set = s.into_iter().filter(|x| !set.contains(x)).collect();
								return vec![set, compl_set];
							}
							break;	
						}
					}
				}
				set.push(n);
			}

		}

		return vec![s];
	}

	/******
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
		let mut n: usize = 1;

		// start.e_transitions.push(n+1); No se puede hacer
		

		for part in parts {

			aut.states[0].e_transitions.push(n);
			
			
			
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
			n+=1;

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

				if let Some(key) = aut.states[number-1].transitions.keys().next() {

					let new_state: Node = Node {

						acceptation: false,
						transitions: HashMap::from([(key.clone(), vec![number-1])]),
						e_transitions: Vec::new(),

					};

					aut.states.push(new_state);
					// Delete the last one

				}

			} else {

				println!("There's no last element");

			}



		}
			
	}

	

					

}

return aut

}*****/

}




fn main(){
	
	//-----------------------------------------------
	//Estoy creando el automata del libro (a(b | c))
	//-----------------------------------------------
	let n_0: Node = Node{
		
		transitions: HashMap::from([(String::from("a"), Vec::<usize>::from([1]))]),	
		e_transitions: Vec::<usize>::from([]),
	};
	
	let n_1: Node = Node{
		
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([2]),
	}; 

	let n_2: Node = Node{
		
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([3, 9]),
	}; 

	let n_3: Node = Node{
		
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([4, 6]),
	}; 

	let n_4: Node = Node{
		
		transitions: HashMap::from([(String::from("b"), Vec::<usize>::from([5]))]),  
		e_transitions: Vec::<usize>::from([]),
	}; 

	let n_5: Node = Node{
		
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([8]),
	}; 

	let n_6: Node = Node{
		
		transitions: HashMap::from([(String::from("c"), Vec::<usize>::from([7]))]),  
		e_transitions: Vec::<usize>::from([]),
	}; 

	let n_7: Node = Node{
		
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([8]),
	}; 

	let n_8: Node = Node{
		
		transitions: HashMap::from([]),  
		e_transitions: Vec::<usize>::from([3, 9]),
	}; 
	
	let n_9: Node = Node{
		
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
	let aut_dfa = aut0.subset_construction();
	println!("{:?}", aut_dfa.states[0].transitions);
	println!("{:?}", aut_dfa.states[1].transitions);
	println!("{:?}", aut_dfa.states[2].transitions);
	println!("{:?}", aut_dfa.states[3].transitions);
	println!("{:?}", aut_dfa.accept_states);
	let aut_min = aut_dfa.minimization();
	println!("{:?}", aut_min.states[0].transitions);
	println!("{:?}", aut_min.states[1].transitions);
	println!("{:?}", aut_min.accept_states);
}