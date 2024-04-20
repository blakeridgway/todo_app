/* 
Simple TUI Todo Item List
Next step is to have a ORM save and store the data
*/

use std::io;

struct TodoItemList {
	id: u32,
	name: String,
	completed: bool,
}

fn complete_item(item: &mut TodoItemList) {
	item.completed = true;
}

fn display_items(items: &Vec<TodoItemList>) {
	for item in items {
		println!("{} - {} ({})", item.id, item.name, item.completed);
	}
}

fn main() {
	let mut todo_list: Vec<TodoItemList> = Vec::new();

	loop {
		println!("----------------------------");
		println!("What would you like to do?");
		println!("1. Add a item to the list");
		println!("2. Complete a item on the list");
		println!("3. Display the list");
		println!("4. Quit");
		println!("----------------------------");

		let mut choice = String::new();
		io::stdin().read_line(&mut choice).expect("Failed to read line");

		let choice = choice.trim().parse::<u32>().expect("Invalid inpit");

		match choice {
			1 => {
				println!("Enter the item to the list");
				let mut name = String::new();
				io::stdin().read_line(&mut name).expect("Failed to read line");
				let name = name.trim().to_string();

				let id = todo_list.len() as u32 + 1;

				let item = TodoItemList {
					id,
					name,
					completed: false,
				};
				
				todo_list.push(item);
				},
            2 => {
                println!("Enter the ID of the to-do item you want to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");

                let item = todo_list.iter_mut().find(|i| i.id == id).unwrap();
                complete_item(item);
            },
            3 => {
                display_items(&todo_list);
            },
            4 => {
                println!("Goodbye!");
                return;
            },
            _ => {
                println!("Invalid choice");
            },
		}
	}
}
