use std::collections::{HashMap, HashSet};

fn main() {
    let replacements: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Al", vec!["ThF", "ThRnFAr"]),
        ("B", vec!["BCa", "TiB", "TiRnFAr"]),
        (
            "Ca",
            vec!["CaCa", "PB", "PRnFAr", "SiRnFYFAr", "SiRnMgAr", "SiTh"],
        ),
        ("F", vec!["CaF", "PMg", "SiAl"]),
        (
            "H",
            vec![
                "CRnAlAr",
                "CRnFYFYFAr",
                "CRnFYMgAr",
                "CRnMgYFAr",
                "HCa",
                "NRnFYFAr",
                "NRnMgAr",
                "NTh",
                "OB",
                "ORnFAr",
            ],
        ),
        ("Mg", vec!["BF", "TiMg"]),
        ("N", vec!["CRnFAr", "HSi"]),
        ("O", vec!["CRnFYFAr", "CRnMgAr", "HP", "NRnFAr", "OTi"]),
        ("P", vec!["CaP", "PTi", "SiRnFAr"]),
        ("Si", vec!["CaSi"]),
        ("Th", vec!["ThCa"]),
        ("Ti", vec!["BP", "TiTi"]),
        ("e", vec!["HF", "NAl", "OMg"]),
    ]);
    let input = String::from("CRnCaCaCaSiRnBPTiMgArSiRnSiRnMgArSiRnCaFArTiTiBSiThFYCaFArCaCaSiThCaPBSiThSiThCaCaPTiRnPBSiThRnFArArCaCaSiThCaSiThSiRnMgArCaPTiBPRnFArSiThCaSiRnFArBCaSiRnCaPRnFArPMgYCaFArCaPTiTiTiBPBSiThCaPTiBPBSiRnFArBPBSiRnCaFArBPRnSiRnFArRnSiRnBFArCaFArCaCaCaSiThSiThCaCaPBPTiTiRnFArCaPTiBSiAlArPBCaCaCaCaCaSiRnMgArCaSiThFArThCaSiThCaSiRnCaFYCaSiRnFYFArFArCaSiRnFYFArCaSiRnBPMgArSiThPRnFArCaSiRnFArTiRnSiRnFYFArCaSiRnBFArCaSiRnTiMgArSiThCaSiThCaFArPRnFArSiRnFArTiTiTiTiBCaCaSiRnCaCaFYFArSiThCaPTiBPTiBCaSiThSiRnMgArCaF");

    let mut distinct: HashSet<String> = HashSet::new();

    for window_size in [1, 2] {
        for slice in input
            .chars()
            .enumerate()
            .collect::<Vec<(usize, char)>>()
            .windows(window_size)
        {
            let prefix = &input[..slice.first().unwrap().0];
            let suffix = &input[slice.last().unwrap().0 + 1..];

            let substr = slice.iter().map(|(_, s)| s).collect::<String>();

            if let Some(v) = replacements.get(&*substr) {
                for replacement in v {
                    let new_string = format!("{}{}{}", prefix, replacement, suffix);
                    distinct.insert(new_string);
                }
            }
        }
    }

    println!("Part 1: {}", distinct.len());

    let mut inverse: HashMap<&str, &str> = HashMap::new();
    for (to, froms) in replacements {
        for from in froms {
            assert!(!inverse.contains_key(from));
            inverse.insert(from, to);
        }
    }

    // Embarrassingly hacky solution that somehow works but I'm getting tired of AoC :-D
    let mut n_steps = 0;
    let mut input = input.clone();

    while input != String::from("e") {
        for window_size in 1..=10 {
            for slice in input
                .chars()
                .enumerate()
                .collect::<Vec<(usize, char)>>()
                .windows(window_size)
            {
                let prefix = &input[..slice.first().unwrap().0];
                let suffix = &input[slice.last().unwrap().0 + 1..];

                let substr = slice.iter().map(|(_, s)| s).collect::<String>();

                if let Some(v) = inverse.get(&*substr) {
                    input = format!("{}{}{}", prefix, v, suffix);
                    n_steps += 1;
                    break;
                }
            }
        }
    }

    println!("Part 2: {}", n_steps);
}
