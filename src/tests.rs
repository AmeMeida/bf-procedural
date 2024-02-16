#[cfg(test)]
use brainfuck_macro::parse_bf;

#[test]
fn it_works() {
    let tape = parse_bf!(>+>>++++<-->>>+++++[>++<-]+>+).0;

    println!("{:?}", tape);

    assert_eq!(tape, [0, 1, 254, 4, 0, 1, 11]);

    assert_eq!(tape[1], 1);
    assert_eq!(tape[2], 254);
    assert_eq!(tape[3], 4);
    assert_eq!(tape[5], 1);
    assert_eq!(tape[6], 11);
}

#[test]
fn hello_world() {
    let output = parse_bf!{
        >++++++++[<+++++++++>-]<.
        >++++[<+++++++>-]<+.
        +++++++..
        +++.
        >>++++++[<+++++++>-]<++.
        ------------.
        >++++++[<+++++++++>-]<+.
        <.
        +++.
        ------.
        --------.
        >>>++++[<++++++++>-]<+.
    }.1;
    
    assert_eq!(output, "Hello, World!");
}
