#[derive(Debug, Clone)]
pub struct Person {
    age: u64,
    name: [char; 5], // dictatorship
}

impl Person {
    pub fn new(age: u64, name: [char; 5]) -> Person {
        Self { age, name }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }

    pub fn bday(&mut self) {
        self.age += 1
    }

    pub fn bday_value(mut self) -> Self {
        self.age += 1;
        self
    }

    pub fn compute_hard(&self) -> u64 {
        let mut f = 0;
        for i in 0..100_000 {
            f += i;
        }
        f
    }
}

#[derive(Debug, Clone)]
pub struct PersonRaw {
    pub age: u64,
    pub name: [char; 5],
}

impl From<PersonRaw> for Person {
    fn from(x: PersonRaw) -> Person {
        Person {
            age: x.age,
            name: x.name,
        }
    }
}

impl From<Person> for PersonRaw {
    fn from(x: Person) -> PersonRaw {
        PersonRaw {
            age: x.age,
            name: x.name,
        }
    }
}
