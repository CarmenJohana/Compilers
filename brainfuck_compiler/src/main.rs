use std::fs::File;
use std::io::{self,Read};

enum COMMANDS {
	RIGHT,		// '>' Increment the data pointer by one (to point to the next cell to the right)
	LEFT,		// '<' Decrement the data pointer by one (to point to the next cell to the left). Undefined if at 0.
	INC,		// '+' Increment the byte at the data pointer by one modulo 256
	DEC,		// '-' Decrement the byte at the data pointer by one modulo 256
	PRINT,		// '.' Output the byte at the data pointer
	READ,		// ',' Accept one byte of input, storing its value in the byte at the data pointer
	OPENBrE,	/*** 	
				'[' If the byte at the data pointer is zero, then 
				instead of moving the instruction pointer forward
				to the next command, jump it forward to the command
				after the matching ] command
			***/
	CLOSINGBrE,	/*** 
				']' If the byte at the data pointer is nonzero, then
				instead of moving the instruction pointer forward
				to the next command, jump it back to the command
				after the matching [ command
			***/
}
/*** Useless table :(
// Column enum
#[derive(Debug)]
enum COLUMN{
	Integer(i32),
	Float(f64),
	Text(String),
}

struct Table {
	headers: Vec<String>,
	rows: Vec<Vec<COLUMN>>,
}

static INST_TABLE: Table = Table {
	headers: vec!["opcode".to_string(), "Value".to_string()],
	rows: Vec::new(),
};

***/

struct TOKEN {
	id: COMMANDS,
	value: String,
}



fn recognize_char(c: char){
	match c {
		'>' => println!(">"),
		'.' => println!("."),
		',' => println!(","),
		'<' => println!("<"),
		'[' => println!("["),
		']' => println!("]"),
		'+' => println!("+"),
		'-' => println!("-"),
		_ => (),
	}
	
}

fn main()-> io::Result<()> {
	
	let RIGHT = TOKEN{
		id: COMMANDS::RIGHT,
		value: String::from(">"),
	}; 
	let LEFT = TOKEN{
		id: COMMANDS::LEFT,
		value: String::from("<"),
	};
	let INC = TOKEN{
		id: COMMANDS::INC,
		value: String::from("+"),
	};
	let DEC = TOKEN{
		id: COMMANDS::DEC,
		value: String::from("-"),
	};
	let PRINT = TOKEN{
		id: COMMANDS::PRINT,
		value: String::from("."),
	};
	let READ = TOKEN{
		id: COMMANDS::READ,
		value: String::from(","),
	};
	let OPENBrE = TOKEN{
		id: COMMANDS::OPENBrE,
		value: String::from("["),
	};
	let CLOSINGBrE = TOKEN{
		id: COMMANDS::CLOSINGBrE,
		value: String::from("]"),
	};
	
	// Just past the name of the file as argument: cargo run -- poem.txt
	// --- println!("In file {file_path}");
	let mut file = File::open("poem.txt")?;

	// --- let contents = fs::read_to_string("./poem.txt").expect("haven't been able to read the file");
	let mut contents = String::new();

	file.read_to_string(&mut contents)?;

	// println!("With text:\n{contents}");

	for c in contents.chars() {
		// println!("{c}");
		recognize_char(c);
	}

	
	/*** Just a testing loop
	loop{
		println!("Again!");
	}
	***/

	Ok(())

}
