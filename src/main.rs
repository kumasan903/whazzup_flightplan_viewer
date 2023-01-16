use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
extern crate dirs;

fn main() -> ! {
	let mut path = dirs::home_dir().unwrap();
	path.push("AppData/Local/JoinFS/whazzup.txt");
	loop
	{
		let mut f = File::open(&path)
			.expect("ファイルが見つかりません");
		println!("\x1B[2J");
		println!("\x1B[r");
		let mut contents = String::new();
		f.read_to_string(&mut contents)
			.expect("something went wrong reading the file");
		let v: Vec<&str> = contents.split('\n').collect();
		let mut i :usize;
		let clients_count_line :Vec<&str> = v[4].split(' ').collect();
		let clients_count :usize = clients_count_line[3].trim().parse().unwrap();
		i = 7;
		while i < (clients_count + 7)
		{
			let tmp :Vec<&str> = v[i].split(':').collect();
			if tmp[3] == "ATC"
			{
				i += 1;
				continue;
			}
			if tmp[1].is_empty()
			{
				println!("{}(NAME)",tmp[0]);	//Call sign(Name)
			}
			else
			{
				println!("{}({})",tmp[0], tmp[1]);	//Call sign(Name)
			}
			if tmp[11].is_empty() && tmp[13].is_empty()
			{
				println!("(FROM)-(TO)");
			}
			else if tmp[11].is_empty()
			{
				println!("(FROM)-{}", tmp[13]);
			}
			else if tmp[13].is_empty()
			{
				println!("{}-(TO)", tmp[11]);
			}
			else
			{
				println!("{}-{}", tmp[11], tmp[13]);
			}
			println!("FPL : {}",tmp[30]);	//Flight plan
			println!("RMK : {}\n",tmp[29]);	//Remarks
			i += 1;
		}
		let sleep_time = time::Duration::from_secs(10);
		thread::sleep(sleep_time);
	}
}