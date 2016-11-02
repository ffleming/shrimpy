use std::env::args;
use std::collections::HashMap;

// TODO: Implement commented-out fields
#[derive(Debug)]
#[derive(Clone)]
pub struct Argument {
    pub key: String,
    pub long_argument: String,
    pub short_argument: String,
    // pub default: String,
    // pub name: String,
    pub takes_value: bool,
    // pub required: bool
}

#[derive(Debug)]
pub struct ArgumentHash {
    arguments: Vec<Argument>,
    short_arguments: HashMap<String, Argument>,
    long_arguments: HashMap<String, Argument>,
}

impl ArgumentHash {
    pub fn new() -> ArgumentHash {
        ArgumentHash {
            arguments: Vec::<Argument>::new(),
            short_arguments: HashMap::new(),
            long_arguments: HashMap::new(),
        }
    }

    pub fn with_argument(&mut self, arg: Argument) -> &mut ArgumentHash {
        let short_arg_cloned = arg.clone();
        let long_arg_cloned = arg.clone();
        self.arguments.push(arg);
        if short_arg_cloned.short_argument != "" {
            let sarg = short_arg_cloned.short_argument.clone();
            self.short_arguments.insert(sarg, short_arg_cloned);
        }
        if long_arg_cloned.long_argument != "" {
            let larg = long_arg_cloned.long_argument.clone();
            self.long_arguments.insert(larg, long_arg_cloned);
        }
        return self;
    }

    pub fn with_arg(&mut self, arg: Argument) -> &mut ArgumentHash {
        self.with_argument(arg);
        return self;
    }

    fn check_long_arg<'a>(&'a self, word: &'a String) -> (&str, &str) {
        let (_, bareword) = word.split_at(2);
        let mut bareword_iter = bareword.split("=");
        let provided_long_arg = bareword_iter.next().expect("Something unexpected happened in check_long_arg!");
        let val = bareword_iter.next().unwrap_or("true");
        let ref key = self.long_arguments.
            get(provided_long_arg).
            expect("Don't know what to do with that option").
            key;
        return (key, val);
    }

    fn check_short_arg<'a>(&'a self, word: &'a String, args_vec: &'a Vec<String>, i: usize) -> (&str, &str) {
        let (_, key) = word.split_at(1);
        let arg = &(self.short_arguments).
            get(key).
            unwrap_or_else(|| {
                println!("Don't know how to parse \"-{}\"", key);
                panic!("Exiting");
            });
        let ref key = arg.key;
        let ref val: str;
        if arg.takes_value {
            if args_vec.len() < i + 1 {
                println!("Value for arg {:?} not provided", arg);
                panic!("Exiting");
            } else {
                val = &(args_vec[i + 1]);
            }
        } else { // arg does not take a value
            val = "true";
        };

        return (key, val)
    }

    pub fn as_hash(&self) -> HashMap<String, String> {
        let mut arg_hash: HashMap<String, String> = HashMap::new();
        let args_vec: Vec<String> = args().collect();
        let mut pair: (&str, &str);

        for i in 1..args_vec.len() {
            let ref word = args_vec[i];
            if word.starts_with("--") {
                pair = self.check_long_arg(word);
            } else if word.starts_with("-") {
                pair = self.check_short_arg(word, &args_vec, i);
            } else {
                continue;
            }
            arg_hash.insert(String::from(pair.0), String::from(pair.1));
        }
        return arg_hash;
    }

}
