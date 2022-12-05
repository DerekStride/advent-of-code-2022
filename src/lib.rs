use anyhow::Result;

pub fn split_input<F, T>(input: &str, delim: &str, func: F) -> Result<Vec<T>>
    where F: Fn(&str) -> Result<T> {
        input
            .split(delim)
            .map(|s| func(s))
            .collect::<Result<Vec<T>>>()
}

pub fn make_input(input: &str) -> String {
    input
        .lines()
        .skip(1)
        // .map(|s| s.trim())
        .collect::<Vec<&str>>()
        .join("\n")
        .to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_input_chunks_on_delimiter() -> Result<()> {
        let input = make_input(
            r###"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            "###
        );

        let expected = vec![
            "1000\n2000\n3000",
            "4000",
            "5000\n6000",
            "7000\n8000\n9000",
            "10000\n",
        ];
        let result = split_input(&input, "\n\n", |s| Ok(s.to_string()))?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn split_input_with_inner_split() -> Result<()> {
        let input = make_input(
            r###"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            "###
        );
        let expected = vec![6000, 4000, 11000, 24000, 10000];
        let result = split_input(&input, "\n\n", |s| {
            let r = split_input(s.trim(), "\n", |s| Ok(s.parse::<usize>()?))?;
            Ok(r.iter().sum::<usize>())
        })?;

        assert_eq!(expected, result);

        Ok(())
    }
}
