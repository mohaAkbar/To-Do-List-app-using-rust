use chrono::NaiveDate;
use std::{fmt::Display, io};


// enum for the priorty of a task
enum Priority {
  High,
  Medium,
  Low,
}

// manually displaying the variants using fmt::Display for Priority enum
impl std::fmt::Display for Priority {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let text = match self {
      Priority::High => "high",
      Priority::Medium => "medium",
      Priority::Low => "low",
    };
    write!(f, "{}", text)
  }
}

// enum defining status for finished/unfinished tasks
enum Status {
    Incomplete,
    Complete,
}

// Display trait implementation for enum Status
impl std::fmt::Display for Status {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let text = match self {
      Status::Incomplete => "Incomplete",
      Status::Complete => "Complete",
    };
    write!(f, "{}", text)
  }
}

// Task data model, containing all attributes about a task (id, name, priority, due date, status)
struct Task {
  id: u32,
  name: String,
  priority: Priority, 
  due_date: NaiveDate,
  status: Status, 
}

// Display trait implementation for struct Task
impl std::fmt::Display for Task {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      writeln!(f, "Task info: {}, {}, {}, {}, {}",self.id, self.name, self.due_date, self.priority, self.status)
  }
}

fn main() {
  // vector containing all Task instances
  let mut vec: Vec<Task> = Vec::new();

  println!("Enter you Task and info: ");

  // -----------------------------------INPUT--------------------------------------------- (i have to change this to a helper function fs)
  println!("id (manual for now):");
  let mut id_input = String::new();
  io::stdin().read_line(&mut id_input).expect("Faild to read line");

  println!("Task Name: ");
  let mut name_input = String::new();
  io::stdin().read_line(&mut name_input).expect("Faild to read line");

  println!("Due date(yyyy-mm-dd):");
  let mut due_date_input = String::new();
  io::stdin().read_line(&mut due_date_input).expect("Faild to read line");

  println!("Priority(high, medium, low): ");
  let mut priority_input = String::new();
  io::stdin().read_line(&mut priority_input).expect("Faild to read line");

  println!("Status(incomplete, complete): ");
  let mut status_input = String::new();
  io::stdin().read_line(&mut status_input).expect("Faild to read line");

  println!("----------------");
    // -----------------------------------INPUT---------------------------------------------



  let id_input: u32 = match id_input.trim().parse() {
    Ok(value) => value,
    Err(_) => panic!("Please enter a valid number"),
  };

  let due_date_input: NaiveDate = match due_date_input.trim().parse() {
    Ok(value) => value,
    Err(_) => panic!("Please enter a valid date"),
  };

  let priority_input: Priority = match priority_input.trim().to_lowercase().as_str() {
    "high" => Priority::High,
    "medium" => Priority::Medium,
    "low" => Priority::Low,
    _ => panic!("Please enter a valid priotity"),
  };

  let status_input: Status = match status_input.trim().to_lowercase().as_str() {
    "incomplete" => Status::Incomplete,
    "complete" => Status::Complete,
    _ => panic!("Please enter a valid status"),
  };

 

  let mut task1 = Task {
    id: id_input,
    name: name_input,
    due_date: due_date_input,
    priority: priority_input,
    status: status_input,
  };
  
  vec.push(task1);

  println!("{}", vec[0]);

}