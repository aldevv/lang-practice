trait Temp {
    type ConvertsTo;
    fn convert(&self) -> Option<Self::ConvertsTo>;
}

struct Celcius {
    value: f32,
}

struct Fahrenheit {
    value: f32,
}

impl Temp for Fahrenheit {
    type ConvertsTo = Celcius;
    fn convert(&self) -> Option<Self::ConvertsTo> {
        Some(Celcius {value: ((self.value - 32.0) * 5.0) / 9.0})
    }
}
impl Temp for Celcius {
    type ConvertsTo = Fahrenheit;
    fn convert(&self) -> Option<Self::ConvertsTo> {
        Some(Fahrenheit {value: ((self.value / 5.0) * 9.0) + 32.0})
    }
}

fn main() {
    let f = Fahrenheit {value: 86.0};
    let c = Celcius {value: 30.0};
    println!("Fahrenheit: {} to Celcius: {}",f.value, f.convert().unwrap().value);
    println!("Celcius: {} to Fahrenheit: {}",c.value, c.convert().unwrap().value);
}

#[test]
fn test_celcius() {
    let c = Celcius {value: 30.0};
    assert_eq!(c.convert().unwrap().value, 86.0)
}

#[test]
fn test_fahrenheit() {
    let f = Fahrenheit {value: 86.0};
    assert_eq!(f.convert().unwrap().value, 30.0)
}
