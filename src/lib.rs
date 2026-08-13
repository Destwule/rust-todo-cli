use std::{
  time::SystemTime,
  fmt, fs,
  path::PathBuf, io::{self, Write},
};

use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Status {
  Pending,
  Completed
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Todo {
  value: String,
  created_at: u64,
  updated_at: Option<u64>,
  status: Status,
}

#[derive(Debug)]
pub struct TodoList {
  storage_path: Option<PathBuf>,
  items: Vec<Todo>,
}

impl fmt::Display for Status {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Status::Pending => write!(f, "Pending"),
      Status::Completed => write!(f, "Completed"),
    }
  }
}

impl fmt::Display for Todo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.value)
  }
}


fn current_time_as_seconds() -> Result<u64, Box<dyn std::error::Error>> {
  Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
}


impl Todo {
  pub fn create(value: String) -> Result<Self, Box<dyn std::error::Error>> {
    let created_at = current_time_as_seconds()?;
    
    Ok(Self {
      value,
      created_at,
      updated_at: None,
      status: Status::Pending,
    })
  }
  
  pub fn value(&self) -> &str {
    &self.value
  }
  
  pub fn status(&self) -> Status {
    self.status.clone()
  }
  
  pub fn mark_completed(&mut self) {
    self.status = Status::Completed
  }
  
  pub fn mark_pending(&mut self) {
    self.status = Status::Pending
  }
}
// TODO: add save mechanism for when task status is updated
impl TodoList {
  pub fn new(path: Option<PathBuf>) -> Self {
    // path is an Option so that if it is None then the default behaviour
    // will be that it does not save to file. the todolist will be temporary
    let storage_path;
    let mut items: Vec<Todo> = vec![];

    match path {
      Some(f_path) => {
        match Self::try_open_path(f_path) {
          Ok(None) => storage_path = None,
          Ok(Some(file_path)) => {
            storage_path = Some(file_path.clone());
            if let Some(value) = Self::get_todolist_from_file(file_path) {
              items = value;
            }
          }
          Err(err) => {
            eprintln!("An error occurred: {err}");
            std::process::exit(1);
          },
        }
      }
      None => storage_path = None,
    }
    Self { storage_path, items }
  }

  fn try_open_path(file_path: PathBuf) -> io::Result< Option<PathBuf> > {
    if !file_path.exists() {
      println!("'{}' does not exist", file_path.file_name().expect("Could not get file_name").display());
      loop {
        print!("Create it? (Y/n): ");
        io::stdout().flush()?;
        
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        match response.trim().to_lowercase().as_str() {
          "y" => {
            match fs::File::create(file_path.clone()) {
              Err(err) => {
                eprintln!("Error creating file: {err}");
                std::process::exit(1);
              }
              Ok(_) => {
                println!("'{}' Successfully Created", file_path.display());
                return Ok(Some(file_path));
              }
            }
          }
          "n" => return Ok(None),
          _ => continue,
        }
      }
    }
    else {
      Ok(Some(file_path))
    }
  }

  fn get_todolist_from_file(file_path: PathBuf) -> Option<Vec<Todo>> {
    match fs::read_to_string(file_path) {
      Ok(value) => {
        if value.is_empty() {
          return None;
        } else {
          match serde_json::from_str(&value) {
            Ok(content) => return Some(content),
            Err(err) => {
              eprintln!("Error deserializing tasklist from file: {err}");
              return None;
            }
          }
        }
      }
      Err(err) => {
        eprintln!("Error loading the todolist: {err}");
        return None;
      }
    }
  }

  fn save_todolist_to_file(&self) -> io::Result<()> {
    if let Some(storage_path) = &self.storage_path {
      match serde_json::to_string(self.items()) {
        Ok(value) => {
          if let Ok(mut file) = fs::File::options().write(true).truncate(true).open(storage_path) {
            write!(file, "{}", value)?;
          }
        }
        Err(err) => {
          eprintln!("Problem saving todolist to file: {err}");
          std::process::exit(1);
        }
      }
    }
    Ok(())
  }

  pub fn add_item(&mut self, item: Todo) {
    self.items.push(item);
    let _ = self.save_todolist_to_file();
  }
  
  pub fn get_mut_item(&mut self, index: usize) -> Option<&mut Todo> {
    self.items.get_mut(index)
  }
  
  pub fn remove_item(&mut self, index: usize) -> Todo {
    let item = self.items.remove(index);
    let _ = self.save_todolist_to_file();

    item
  }
  pub fn items(&self) -> &Vec<Todo> {
    &self.items
  }
  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
  pub fn len(&self) -> usize {
    self.items.len()
  }
  pub fn mark_pending(&mut self, index: usize) -> Option<()> {
    let item = self.items.get_mut(index)?;
    item.mark_pending();
    let _ = self.save_todolist_to_file();

    Some(())
  }
  pub fn mark_completed(&mut self, index: usize) -> Option<()> {
    let item = self.items.get_mut(index)?;
    item.mark_completed();
    let _ = self.save_todolist_to_file();

    Some(())
  }
}
