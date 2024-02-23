use std::env;

fn main() {
    // Fetch the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument (the program name) is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <text>", args[0]);
        return;
    }

    // Extract the text from the arguments
    let text = &args[1..].join(" ");

    // Modify the text as per the given template
    let modified_template = format!(r#"\begin{{center}}
\begin{{minipage}}{{0.5\textwidth}}
\centering
\makecell[l]{{ {} }}
\end{{minipage}}
\end{{center}}"#, format_text(text));

    println!("{}", modified_template);
}

fn format_text(text: &str) -> String {
    let mut formatted_text = String::new();
    let lines = text.lines().collect::<Vec<&str>>();
    for (i, line) in lines.iter().enumerate() {
        formatted_text.push_str(&format!(
            "\\textit{{{}}}",
            if i == lines.len() - 1 { line } else { format!("{} \\\\", line) }
        ));
        if i != lines.len() - 1 {
            formatted_text.push('\n');
        }
    }
    formatted_text
}
