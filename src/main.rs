use brainfuck::brainfuck::Brainfuck;

mod tests;

fn main() {
    let mut bf = Brainfuck::new();

    match bf.run("+++>++") {
        Ok(text) => text,
        Err(err) => panic!("{}", err)
    };

    println!("{:?}", bf.tape());   
}
