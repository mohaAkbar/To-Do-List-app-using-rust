use chrono::NaiveDate;
use std::{fmt::{Display, Formatter, Result}, ops::Add};
use inquire::{Text, Select};


// enum for the priorty of a task
enum Priority {
  High,
  Medium,
  Low,
}

// manually displaying the variants using fmt::Display for Priority enum
impl Display for Priority {
  fn fmt(&self, f: &mut Formatter) -> Result {
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
impl Display for Status {
  fn fmt(&self, f: &mut Formatter) -> Result {
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
impl Display for Task {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      writeln!(f, "Task info: {}, {}, {}, {}, {}",self.id, self.name, self.due_date, self.priority, self.status)
  }
}

// implementation functions, CRUD for Task struct
impl Task {

  // this is the add function, it takes input, then parses it, and lastly pushes it into the tasks vector
  fn add(tasks_vec: &mut Vec<Task>) {

    // telling user to enter his task details
    println!("Enter your task info");

    // user input layer
    // -----------------------------------INPUT--------------------------------------------- 

    // actual input layer, input taken from user
    let id_input = Text::new("id (manual for now):").prompt().unwrap();

    let name_input = Text::new("Task Name: ").prompt().unwrap();

    let due_date_input = Text::new("Due date(yyyy-mm-dd):").prompt().unwrap();

    let priority_input = Text::new("Priority(high, medium, low): ").prompt().unwrap();

    let status_input = Text::new("Status(incomplete, complete): ").prompt().unwrap();

    println!("=======================================");

    // validation and parsing layer for user input
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

    // -----------------------------------INPUT---------------------------------------------

    let new_task = Task {
      id: id_input,
      name: name_input,
      due_date: due_date_input,
      priority: priority_input,
      status: status_input,
    };
    tasks_vec.push(new_task)
  }

  // this is the view function it is not implemented yet
  fn view() {
    println!("view function not implemented yet")
  }

  // this is the delete function it is not implemented yet
  fn delete() {
    println!("delete function not implemented yet")
  }

  // this is the update function it is not implemented yet
  fn update() {
    println!("update function not implemented yet")
  }

  // this is the exit function it is not implemented yet (the * star is used for dereferencing)
  fn exit(stop_signal: &mut bool) {
    *stop_signal = true;
  }
}

//enum for UI menu options
enum MenuOptions {
  Add,
  View,
  Delete,
  Update,
  Exit,
}

//Display trait for MenuOptions
impl Display for MenuOptions {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let text = match self {
      MenuOptions::Add => "Add",
      MenuOptions::View => "View",
      MenuOptions::Delete => "Delete",
      MenuOptions::Update => "Update",
      MenuOptions::Exit => "Exit"
    };
    write!(f, "{}", text)
  }
}

// menu selection implementation using the MenuOptions enum
impl MenuOptions {
  fn get_menu_option(options_vec: Vec<MenuOptions>) -> MenuOptions {
    let choice = Select::new("What do you want to do?", options_vec).prompt().unwrap();
    return choice;
  }
}

fn main() {

  // just a app exiting signal
    let mut stop_signal: bool = false;

   loop {

    // this is the app intro UI
    println!("");
    println!("=== Strictly-Done ===");
    println!("");

    // vector containing all Task instances
    let mut tasks_vec: Vec<Task> = Vec::new();

    //vector containing the menu options add, view, delete, update
    let options_vec: Vec<MenuOptions> = vec![
      MenuOptions::Add,
      MenuOptions::View,
      MenuOptions::Delete,
      MenuOptions::Update,
      MenuOptions::Exit,
    ];

    // calling the menu function to show menu
    let choice = MenuOptions::get_menu_option(options_vec);
    
    // this matches the choice of the user to the corresponding function
    match choice {
      MenuOptions::Add => Task::add(&mut tasks_vec),
      MenuOptions::View => Task::view(),
      MenuOptions::Delete => Task::delete(),
      MenuOptions::Update => Task::update(),
      MenuOptions::Exit => Task::exit(&mut stop_signal),
    }

    if stop_signal {
      break;
    }

    if !tasks_vec.is_empty() {
    println!("{}", tasks_vec[0]);
    }


  }

}