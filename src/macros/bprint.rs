// TODO refactor.
// Remove all unwraps.

#[macro_export]
macro_rules! box_print {
    ($width:expr, $($data:expr),+) => {{

        let mut headings: Vec<&str> = Vec::new();
        let mut output: Vec<&str> = Vec::new();
        $(
            let value = $data.to_string();
            headings.push(stringify!($data));
            output.push(&value);
        )+

        let output_size: Vec<usize> = headings.iter().zip(output.iter()).map(|(a, b)| a.len() + b.len()).collect();
        let max_output_size = output.iter().map(|a| a.len()).max().unwrap();
        let max_line_size = output_size.iter().max().unwrap();
        let max_line_size = (max_line_size + 7).min($width / 3 + 7);

        let mut formatted_output = String::new();

        formatted_output.push_str("\n");
        formatted_output.push_str(&"-".repeat(max_line_size));
        formatted_output.push_str("\n");
        for (a, b) in headings.iter().zip(output.iter()) {
            formatted_output.push_str("| ");
            formatted_output.push_str(a);
            formatted_output.push_str(&" ".repeat(max_line_size - (a.len() + max_output_size) - 7));
            formatted_output.push_str(" | ");
            formatted_output.push_str(b);
            formatted_output.push_str(&" ".repeat(max_line_size - (a.len() + b.len()) - 7 - (max_line_size - (a.len() + max_output_size) - 7)));
            formatted_output.push_str(" |");
            formatted_output.push_str("\n");
        }
        formatted_output.push_str(&"-".repeat(max_line_size));

        println!("{}", formatted_output);

    }};
}
