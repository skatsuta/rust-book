use std::collections::HashMap;

fn main() {
    {
        let mut scores = HashMap::new();
        scores.insert("Blue".to_string(), 10);
        scores.insert("Yellow".to_string(), 50);
        println!("{:?}", scores);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        println!("{:?}", score);
        for (key, val) in &scores {
            println!("{}, {}", key, val);
        }
        println!("{}", scores["Yellow"]);
    }
    {
        let teams = vec!["Blue".to_owned(), "Yellow".to_owned()];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{:?}", scores);
    }
    {
        let field_name = "Faverite color".to_owned();
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        println!("{:?}", map);
    }
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);
    }
    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}
