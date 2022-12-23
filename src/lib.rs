mod person;
pub use person::{Person, PersonRaw};

#[cxx::bridge]
pub mod ffi {

    #[derive(Debug, Clone)]
    pub struct PersonFFI {
        age: u64,
        name: [u32; 5],
    }

    extern "Rust" {
        fn person_ffi_new(age: u64, name: [u32; 5]) -> PersonFFI;
        fn is_adult(self: &PersonFFI) -> bool;
        fn bday(self: &mut PersonFFI);
        fn compute_hard(self: &PersonFFI) -> u64;
    }

    unsafe extern "C++" {
        include!("test-raw-ffi/include/person.h");
        fn foo() -> u32;
    }
}

pub use ffi::PersonFFI;

impl From<PersonFFI> for Person {
    fn from(ffi_p: PersonFFI) -> Person {
        PersonRaw {
            age: ffi_p.age,
            name: ffi_p.name.map(|i| char::from_u32(i).unwrap()),
        }
        .into()
    }
}

impl From<Person> for PersonFFI {
    fn from(p: Person) -> PersonFFI {
        let raw_p = PersonRaw::from(p);

        PersonFFI {
            age: raw_p.age,
            name: raw_p.name.map(u32::from),
        }
    }
}

fn person_ffi_new(age: u64, name: [u32; 5]) -> PersonFFI {
    Person::new(age, name.map(|i| char::from_u32(i).unwrap())).into()
}

impl PersonFFI {
    pub fn new(age: u64, name: [char; 5]) -> PersonFFI {
        person_ffi_new(age, name.map(u32::from))
    }

    pub fn is_adult(&self) -> bool {
        Person::from(self.clone()).is_adult()
    }

    pub fn bday(&mut self) {
        let mut x = Person::from(self.clone());
        x.bday();
        *self = x.into()
    }

    pub fn compute_hard(&self) -> u64 {
        Person::from(self.clone()).compute_hard()
    }
}