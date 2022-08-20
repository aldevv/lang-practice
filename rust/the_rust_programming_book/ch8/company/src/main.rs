use std::cmp::Ordering;

struct Company {
    name: String,
    departments: Option<Vec<Department>>,
}
struct Department {
    name: String,
    employees: Option<Vec<Employee>>,
}

#[derive(Eq)]
struct Employee {
    name: String,
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: check if this cmp works for Strings
        self.name.cmp(&other.name)
    }
}
// TODO: check what PartialOrd does
impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// TODO: check what PartialEq does
impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Department {
    fn add_employee(&mut self, employee: Employee) {
        // as_mut since self.employees is an &option(T) and we need &mut T
        self.employees.as_mut().unwrap().push(employee);
    }

    fn get_employees(&mut self) {
        // TODO: return employees sorted without modifying the original list
        // HACK: clone the employees vector by implementing the clone trait
        // let employee_copy = self.employees.clone();
        return;
    }
}

fn main() {
    let dep1 = Department {
        name: String::from("Human Processing"),
        employees: None,
    };

    let dep2 = Department {
        name: String::from("Machine Building"),
        employees: None,
    };

    let dep3 = Department {
        name: String::from("Recycling"),
        employees: None,
    };

    let comp1 = Company {
        name: String::from("Skynet"),
        departments: Some(vec![dep1, dep2, dep3]),
    };
}
