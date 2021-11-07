use std::collections::HashMap;
use std::io;
use rand::Rng;
use regex::Regex;
use std::str::FromStr;
use strum_macros::EnumString;

fn main() {
    // 1. 整数のリストが与えられ、ベクタを使ってmean(平均値)、median(ソートされた時に真ん中に来る値)、 mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)
    // を返してください。
    let mut nums = Vec::new();

    for _ in 1..30 {
        nums.push(rand::thread_rng().gen_range(1, 20));
    }

    if nums.len() == 0 {
        panic!("値は0個以上にしてください");
    }

    println!("今回使用する値はこちら: {:?}", nums);

    // mean
    let mean = nums.iter().fold(0, |x, y| x + y) / nums.len();
    println!("平均値は {} です", mean);

    // median
    nums.sort();
    let median;
    if nums.len() % 2 == 0 {
        // 偶数個だったら真ん中二つの平均
        median = (nums[nums.len() / 2  - 1] + nums[nums.len() / 2]) / 2;
    } else {
        // 奇数個だったら真ん中
        median = nums[nums.len() / 2]
    }
    println!("中央値は {} です", median);

    // mode
    let mut count = HashMap::new();
    for i in nums {
        let c = count.entry(i).or_insert(0);
        *c += 1;
    }
    let mode = count.iter()
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap().0;
    println!("最頻値は {} です", mode);


    // 2. 文字列をピッグ・ラテン(訳注: 英語の言葉遊びの一つ)に変換してください。
    // 各単語の最初の子音は、 単語の終端に移り、"ay"が足されます。従って、"first"は"irst-fay"になります。
    // ただし、 母音で始まる単語には、お尻に"hay"が付け足されます("apple"は"apple-hay"になります)。
    let word1 = "first".to_string();
    let word2 = "english".to_string();
    let conv_word1 = make_pig_latin(&word1);
    let conv_word2 = make_pig_latin(&word2);
    println!("{} => {}", word1, conv_word1);
    println!("{} => {}", word2, conv_word2);

    // 3. ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。
    // 例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。
    // それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。
    let mut data_store = DataStore {
        data: HashMap::new()
    };

    loop {
        println!("input <Action> <Name> to/from <Department>");

        let mut line = "".to_string();
        io::stdin().read_line(&mut line).expect("Fail to read line");

        let command = Command::new(&line);

        println!("{:#?}", command);

        data_store.execute(command);

        println!("{:#?}", data_store.data);
    }

}

#[derive(Debug, EnumString)]
enum Action {
    #[strum(serialize = "add", ascii_case_insensitive)]
    Add,
    #[strum(serialize = "remove", ascii_case_insensitive)]
    Remove,
}

#[derive(Debug, EnumString, Hash, Eq, PartialEq)]
enum Department {
    #[strum(serialize = "sales", ascii_case_insensitive)]
    Sales,
    #[strum(serialize = "engineering", ascii_case_insensitive)]
    Engineering
}

struct DataStore {
    data: HashMap<Department, Vec<String>>
}

impl DataStore {
    fn execute(&mut self, command: Command) {
        match command.action {
            Action::Add => {
                let d = self.data.entry(command.dept).or_insert(vec![]);
                d.push(command.name);
            },
            Action::Remove => {
                self.data.remove_entry(&command.dept);
            }
        }
    }
}

#[derive(Debug)]
struct Command {
    action: Action,
    name: String,
    dept: Department
}

impl Command {
    fn new(line: &str) -> Command {
        let pattern = r"(?P<action>[a-zA-Z]+)\s+(?P<name>[a-zA-Z]+)\s+(to|from)\s+(?P<dept>[a-zA-Z]+)";
        let re = Regex::new(pattern).unwrap();
        let captured = re.captures(line).unwrap();
        let action = Action::from_str(&captured["action"]).unwrap();
        let name = &captured["name"];
        let dept = Department::from_str(&captured["dept"]).unwrap();

        return Command {
            action,
            name: name.to_string(),
            dept
        };
    }
}

fn make_pig_latin(word: &str) -> String {
    let mut last = "".to_string();
    let mut remain = "".to_string();
    let vowel = vec!['a', 'i', 'u', 'e', 'o'];

    for (i, c) in word.chars().enumerate() {
        if i == 0 {
            if vowel.contains(&c) {
                last = "-hay".to_string();
                remain.push(c);
            } else {
                last = format!("-{}ay", c);
            }
        } else {
            remain.push(c);
        }
    }
    format!("{}{}", remain, last)
}