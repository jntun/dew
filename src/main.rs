//
// Created by Justin Tunheim on 11/24/24
//

fn main() {
	let mut args = std::env::args().skip(1);

	if let Some(arg) = args.next() {
		let mut build = dew::Source::new(dew::File::new(arg));
		for arg in args {
			build.add(dew::File::new(arg));
		}
		let mut source = match build.finish() {
			Ok(src) => src,
			Err(e)  => {
				panic!("error building: {}", e);
			}
		};
		match source.execute() {
			Ok(value) => println!("{}", value),
			Err(e)    => eprintln!("{}", e),
		}
	} else {
		interpret()
	}
}

fn interpret() {
}


