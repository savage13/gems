use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

lazy_static! {
    static ref ITEMS: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("flint", 5);
        m.insert("amber", 30);
        m.insert("opal", 60);
        m.insert("topaz", 180);
        m.insert("ruby", 210);
        m.insert("sapphire", 260);
        m.insert("diamond", 500);
        m
    };
    static ref ABBREV: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("f", "flint");
        m.insert("a", "amber");
        m.insert("o", "opal");
        m.insert("t", "topaz");
        m.insert("r", "ruby");
        m.insert("s", "sapphire");
        m.insert("d", "diamond");
        m.insert("flints", "flint");
        m.insert("ambers", "amber");
        m.insert("opals", "opal");
        m.insert("topazes", "topaz");
        m.insert("rubies", "ruby");
        m.insert("sapphires", "sapphire");
        m.insert("diamonds", "diamond");
        m
    };
}

fn items_names() -> String {
    ITEMS.keys().cloned().collect::<Vec<_>>().join(" ")
}

fn help(full: bool) -> String {
    let keys = items_names();

    if full {
        format!(
            "Compute inventory value\
  Known items: \"{:?} Verbose \" \
  Examples:\
     2 Amber 1 Flint 3 Ruby 4 Diamond\
     Amber Amber Flint Ruby Ruby Ruby 4 Diamond\
     a a f r r r 4 d\
     2 A    F    3 R    4 D\
     2 a  f 3 r 4 d verbose\
\
\
  If verbose is specified, then an itemized receipt is produced\
    otherwise only the total is reported\
  Input is not case sensitive\
\
  Examples above should all preduce 2695 or with verbose\
\
2 amber: 60\
1 flint: 5\
3 ruby: 630\
4 diamond: 2000\
--------------------\
Total: 2695\
",
            keys
        )
    } else {
        format!(
            "items \"{} verbose help\", example: 2 Amber Diamond or 2 a d",
            keys
        )
    }
}

fn is_help(s: &str) -> bool {
    s == "h" || s == "help"
}
fn is_verbose(s: &str) -> bool {
    s == "v" || s == "verbose"
}

#[get("/gems/<input>")]
fn index(input: &str) -> String {
    if input.len() > 128 {
        return "Input length too long".to_string();
    }
    let mut args: Vec<_> = input.split_whitespace().map(|x| x.to_lowercase()).collect();
    // Check for help
    let mut it = args.iter().filter(|x| is_help(x));
    if it.next().is_some() {
        return help(it.next().is_some());
    }
    // Check for verbose flag and remove from values
    let verbose = args.iter().filter(|x| is_verbose(x)).next().is_some();
    if verbose {
        args = args.into_iter().filter(|x| !is_verbose(x)).collect();
    }
    // Expand Abbreviations
    args = args
        .into_iter()
        .map(|x| {
            if let Some(z) = ABBREV.get(x.as_str()) {
                z.to_string()
            } else {
                x
            }
        })
        .collect();

    let mut scale = 1;
    // Order of items with (Name, Count)
    let mut vals = vec![];
    for arg in &args {
        if let Ok(val) = arg.parse::<i32>() {
            scale = val;
        } else {
            if ITEMS.contains_key(arg.as_str()) {
                if let Some(k) = vals.iter().position(|&x: &(&String, i32)| x.0 == arg) {
                    vals[k].1 += scale;
                } else {
                    vals.push((arg, scale));
                }
            } else {
                return format!("Unknown item: {}, try with `help`", arg);
            }
            scale = 1;
        }
    }

    // Calculate Total
    let total: i32 = vals.iter().map(|(k, count)| count * item_value(k)).sum();

    // Output
    if verbose {
        let mut lines = vals.into_iter()
            .map(|(name, count)| format!("{:2} {:12} {}", count, name, item_value(&name) * count))
            .collect::<Vec<_>>();
        lines.push(format!("--------------------"));
        lines.push(format!("Total: {}", total));
        lines.join("\n")
    } else {
        format!("{}", total)
    }
}

fn item_value(name: &str) -> i32 {
    if let Some(value) = ITEMS.get(name) {
        *value
    } else {
        0
    }
}
#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(("port", 3000));
    rocket::custom(figment).mount("/", routes![index])
}
