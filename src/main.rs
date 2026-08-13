use todo::{Todo, TodoList};
use std::{
  io::{self, Write}, path::PathBuf
};

fn get_input(prompt: &str) -> io::Result<String> {
  print!("{prompt} ");
  io::stdout().flush()?;
  
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  
  Ok(input.trim().to_string())
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Welcome to the short term todo app");
  let path = PathBuf::from("tasks.json");
  let mut todolist = TodoList::new(Some(path));

  loop {
    println!("\n1. Add item\n2. Remove Item\n3. List items\n4. Mark as Completed\n5. Mark as Pending\n");
    print!("What do u wanna do? 'q' to quit: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase().as_str() == "q" {
      println!("\nGoodbye for now :)");
      break;
    }
    
    let valid_input = match input.trim().parse::<u8>() {
      Ok(val) => val,
      Err(_) => {
        println!("\nPlease enter a NUMBER");
        continue;
      }
    };

    match valid_input {
      1 => {
        let response = get_input("Enter Todo:")?;
        match Todo::create(response) {
          Ok(todo) => {
            let cl = todo.clone();
            todolist.add_item(todo);
            println!("ADDED: {cl}");
          },
          Err(err) => {
            eprintln!("Could not add task: {err}");
          }
        }
      },
      
      val @ 2..=5 => {
        loop {
          if todolist.is_empty() {
            println!("LIST IS EMPTY. No item to remove");
            break;
          } 
          
          else if val == 3 {
            let line = "#".repeat(40);
            println!("\n{}", line);
            
            println!("TODOLIST ITEMS");
            for (index, item) in todolist.items().iter().enumerate() {
              println!("{}. {} --- STATUS: {}", index+1, item.value(), item.status());
            }
            println!("{}", line);

            break;
          }
          
          else {
            let valid_response: u8;
            loop {
              let response = get_input("Enter task number: ")?;
                
              valid_response = match response.trim().parse::<u8>() {
                Ok(val) => val,
                Err(_) => {
                  println!("\nPlease ENTER A NUMBER");
                  continue;
                }
              };
              break;
            }

            if val == 2 && valid_response > 0 && usize::from(valid_response) <= todolist.len() {
              println!("REMOVED: {}", todolist.remove_item((valid_response-1) as usize));
              break;
            }
            
            else if val == 4 && valid_response > 0 && usize::from(valid_response) <= todolist.len() {
              if todolist.mark_completed((valid_response-1) as usize).is_some() {
                if let Some(item) = todolist.get_mut_item((valid_response-1) as usize) {
                  println!("[STATUS CHANGED]: {} --- STATUS: {}", item.value(), item.status());
                }
                break;
              }
              else {
                println!("COULD NOT CHANGE THE TASK. Possibly a problem in getting the task");
                break;
              }
            }

            else if val == 5 && valid_response > 0 && usize::from(valid_response) <= todolist.len() {
              if todolist.mark_pending((valid_response-1) as usize).is_some() {
                if let Some(item) = todolist.get_mut_item((valid_response-1) as usize) {
                  println!("[STATUS CHANGED]: {} --- STATUS: {}", item.value(), item.status());
                }
                break;
              }
              else {
                println!("COULD NOT CHANGE THE TASK. Possibly a problem in getting the task");
                break;
              }
            }
            else if usize::from(valid_response) > todolist.len() {
              println!("\nPlease enter a valid task number");
              continue;
            }
          }

        }
      }

      _ => {
        println!("\nPlease select a valid option (1-5)");
        continue;
      }
    }
  }
  
  Ok(())
}
