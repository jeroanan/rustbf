pub struct Display {

}

impl Display {
    pub fn output_char(&self, c: char) {
        print!("{}", c);
    }
}