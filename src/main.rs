mod exec;
fn main() {
    let js_code = {
        let mut args = std::env::args().collect::<Vec<String>>();
        if args.len() != 2 {
            eprintln!("Expected exactly one argument (in form of a js-style closure)");
            std::process::exit(1);
        }
        args.pop().unwrap()
    };

    let input_json = {
        use std::io::Read;
        let mut input = String::with_capacity(256);
        std::io::stdin().read_to_string(&mut input).unwrap_or_else(|err|{
            eprintln!("failed to read from stdin. Error: {:#?}", err);
            std::process::exit(1);
        });
        input
    };

    let node_input = format!("let __closure={};console.log(JSON.stringify(__closure(JSON.parse(\"{}\"))))", js_code, escape(&input_json));

    let result = exec::execute(&node_input, &["node"]);

    println!("{}", result);
}

fn escape(input: &str) -> String {
    let mut res = String::with_capacity(input.len()*2);
    for c in input.chars() {
        res += &match c {
            '\n' => "\\n".to_string(),
            '\r' => "\\r".into(),
            '\t' => "\\t".into(),
            '"' => "\\\"".into(),
            x => x.into(),
        };
    }

    res
}
