trait Descriptive {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u8
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}",self.name, self.age)
    }
} 

fn main() {

}