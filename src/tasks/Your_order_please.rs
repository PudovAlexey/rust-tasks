struct SentenceSequence {
    sentence: String,
    sqnr: u32
}

impl SentenceSequence {
    pub fn new(sentence: &str) -> SentenceSequence {
        let mut num: u32 = 0;
        for letter in sentence.chars() {
            let is_valid_number = letter.to_digit(10);

            if is_valid_number.is_some() {
                num = is_valid_number.unwrap();
                break;
            }
        };

        SentenceSequence {
            sentence: String::from(sentence),
            sqnr: num
        }
    }
}

pub fn your_order_please(sentence: &str) -> String {
   let mut res: Vec<SentenceSequence> =  sentence
    .split(" ")
    .fold(Vec::new(), |mut acc, el| {
        let seq = SentenceSequence::new(el);

        acc.push(seq);

        acc
    });

    res.sort_unstable_by(|a, b| a.sqnr.partial_cmp(&b.sqnr).unwrap());

    let result: Vec<String> = res.iter().map(|el| el.sentence.clone()).collect();

    result.join(" ")
}

pub fn order(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}