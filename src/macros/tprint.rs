// TODO refactor this macro to make it more concise.

#[macro_export]
macro_rules! table_print {
    ($width:expr, $($data:expr),+) => {{
        let mut output: Vec<Vec<String>> = Vec::new();
        let mut output_size: Vec<Vec<usize>> = Vec::new();

        let mut headings: Vec<&str> = Vec::new();
        $(
            headings.push(stringify!($data));
        )+

        output_size.push(headings.iter().map(|x| x.len()).collect());

        output.push(headings.iter().map(|x| x.to_string()).collect::<Vec<String>>());

        let mut value_output: Vec<String> = Vec::new();

        $(
            for a in $data.iter() {
                value_output.push(a.to_string());
            }
        )+

        let size: usize = value_output.len() / headings.len();

        for i in 0..size {
            let mut temp: Vec<String> = Vec::new();
            let mut temp_size: Vec<usize> = Vec::new();
            $(
                let value = $data[i].to_string();
                temp.push(value.clone());
                temp_size.push(value.len());
            )+
            output.push(temp);
            output_size.push(temp_size);
        }

        let mut column_widths: Vec<usize> = vec![0; headings.len()];
        for vec in output_size.iter() {
            for (i, value) in vec.iter().enumerate() {
                column_widths[i] = column_widths[i].max(*value);
            }

        }
        let total_width: usize = column_widths.iter().sum::<usize>();
        let excess: usize = ($width - total_width) / headings.len();
        let padding: Vec<usize> = column_widths.iter().map(|a | (*a as f64 + excess as f64) as usize).collect();

        for (ia, a) in output.iter().enumerate() {
            let mut output_string = String::new();
            for (ib, b) in a.iter().enumerate() {
                let b = b.to_string();
                let b = &b[..b.len().min(padding[ib]-1)];
                output_string.push_str(&b.to_string());
                output_string.push_str(&" ".repeat(((padding[ib] - b.len()) as isize).max(0) as usize));
            }

            println!("\n{}\n", output_string);

        }
    }};
}
