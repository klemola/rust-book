pub struct Context<'s>(&'s str);

pub struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

pub fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_is_not_implemented() {
        let context = Context("test");

        match parse_context(context) {
            Ok(_) => assert!(false, "Parser should not do anything"),
            Err(_) => assert!(true),
        };
    }
}
