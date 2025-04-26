#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: &Vec<Shoe>, size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|s| s.size == size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let Shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_size = shoes_in_size(&Shoes, 10);
        assert_eq!(in_size.len(), 2);
    }
}