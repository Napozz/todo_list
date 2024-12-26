use std::str::FromStr;
use std::fmt::Debug;

fn main() {
    println!("This is a todo list");
    let mut todo_list : Vec<String> = Vec::new();



    loop {
    println!("Enter 1 to add a todo");
    println!("Enter 2 to remove a todo");
    println!("Enter 3 to view the todo list");
    println!("Enter 4 to exit the program");
    let number: u32 = get_user_input();

    match number {
        1 => {
            println!("Enter the todo you want to add");
            let todo: String = get_user_input();
            todo_list.push(todo);
        },
        2 => {
            println!("Enter the todo you want to remove");
            let todo: String = get_user_input();
            let index = todo_list.iter().position(|x| x == &todo);
            match index {
                Some(i) => {
                    todo_list.remove(i);
                },
                None => {
                    println!("Todo not found");
                }
            }
        },
        3 => {
            for (i, todo) in todo_list.iter().enumerate() {
                println!("{}: {}", i, todo);
            }
        },
        4 => {
            return;
        },
        _ => {
            println!("Invalid input");
        }
    }
        
    }

}


fn get_user_input<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input, please try again"),
        }
    }
}