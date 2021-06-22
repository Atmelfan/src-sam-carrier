

mod builtintest;

trait BuiltInTest {
    fn test_name(&self) -> &'static str;
}

enum BitThreshold { 
    Limits()
}

enum CsciCommand {
    Init,
    CollectPbit,
    CollectCbit
}


trait Csci {

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
