use std::cmp::Ordering;

struct Company<'a> {
    name: String,
    departments: Option<Vec<&'a Department<'a>>>,
}
struct Department<'a> {
    name: String,
    employees: Option<Vec<&'a Employee>>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
struct Employee {
    name: String,
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl<'a> Department<'a> {
    // TODO: how do i make the employee's lifetime OUTLAST the Department?
    fn add_employee<'v>(&mut self, employee: &'v Employee)
    where
        'v: 'a,
    {
        // as_mut since self.employees is an &option(T) and we need &mut T
        self.employees.as_mut().unwrap().push(employee);
    }

    fn get_employees(&self) -> Vec<&Employee> {
        let mut employees_copy = self.employees.as_ref().expect("no employees added").clone();
        employees_copy.sort();
        return employees_copy;
    }
}

fn main() {
    let emp1 = Employee {
        name: String::from("AK-47"),
    };

    let emp2 = Employee {
        name: String::from("ZAndroid 23"),
    };

    let emp3 = Employee {
        name: String::from("HAL"),
    };

    let dep1 = Department {
        name: String::from("Human Processing"),
        employees: Some(vec![&emp3, &emp2, &emp1]),
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
        departments: Some(vec![&dep1, &dep2, &dep3]),
    };

    println!("the output is: {:?}", dep1.get_employees())
}
