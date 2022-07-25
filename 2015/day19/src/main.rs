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
}
