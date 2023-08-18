// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.



trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String>{
    fn append_bar(self)->Self{
        //let mut self = self;
        let mut self1 = self;
        self1.push("Bar".to_string());
        self1
        // println!("{}",self.to_owned()[0]);
        //self.to_owned().push("Foo".to_string());
        // println!("{}",self.to_owned()[1]);
       // self
    }

}

// TODO: Implement trait `AppendBar` for a vector of strings.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
