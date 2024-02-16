use brainfuck::brainfuck::Brainfuck;
use proc_macro::TokenStream;

#[proc_macro]
pub fn parse_bf(bf_stream: TokenStream) -> TokenStream {
    let bf_string = bf_stream.to_string().trim_matches('"').replace(" ", "");

    let mut bf = Brainfuck::new();

    let output = match bf.run(bf_string) {
        Ok(result) => result,
        Err(invalid_token) => {
            return format!(
                r#"compile_error!("Invalid token found: {}")"#,
                invalid_token.0
            )
            .parse()
            .unwrap()
        }
    };

    let values = bf.tape()
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    format!("([{values}], \"{output}\")").parse().unwrap()
}
