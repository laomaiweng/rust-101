use std::collections::HashMap;

fn main() {
    vectors();
    println!("----------");
    strings();
    println!("----------");
    hashmaps();
}

fn vectors() {
    let mut v = gimme_vec();
    v.push(4);
    println!("{:?}", v);

    for i in &mut v {
        *i *= *i;
    }
    println!("{:?}", v);
}

fn gimme_vec() -> Vec<i32> {
    vec![1, 2, 3]
}

fn strings() {
    let ss = vec![
        String::from("Здравствуйте"),
        String::from("Hello"),
        String::from("你好"),
        String::from("नमस्ते"),
        String::from("السلام عليكم"),
        String::from("שָׁלוֹם"),
    ];
    let s = (ss[0]).to_owned() + "/" + &ss[1] + "/" + &format!("{}/{}/{}/{}", ss[2], ss[3], ss[4], ss[5]);

    println!("{}", s);
    println!("{}", ss.iter().map(|ref e| format!("{}", e.len())).collect::<Vec<String>>().join("-"));

    println!("");
    println!("{}", &s[0..4]);

    println!("");
    for c in ss[3].chars() {
        println!("{}", c);
    }

    println!("");
    for b in ss[2].bytes() {
        println!("{}", b);
    }
}

fn hashmaps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let other_teams = vec!["Green", "Red", "NOTUSED"];
    let mut other_initial_scores = vec![30, 60];
    let other_scores: HashMap<_, _> = other_teams.iter().map(|s| s.to_string()).zip(other_initial_scores.iter().map(|v| v.clone())).collect();  // must .to_string() because scores uses String::from(); must .clone() because otherwise we get references to the integers in other_initial_scores and keep the borrow active
    other_initial_scores[0] = 99;
    println!("{:?}", other_scores);

    println!("");
    println!("{:?}", scores.get("Blue"));
    println!("{:?}", scores.get("Purple"));

    println!("");
    scores.entry(String::from("Blue")).or_insert(11);
    scores.entry(String::from("Purple")).or_insert(20);
    scores.extend(other_scores);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
