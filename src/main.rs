use glob::glob;
use itertools::Itertools;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"implementation.project\(['"]:([\w-]+)['"]\).?"#)
        .expect("failed to create regex expression");
    static ref ARROW_COLORS: Vec<&'static str> = vec![
        "#AliceBlue",
        "#AntiqueWhite",
        "#Aqua",
        "#Aquamarine",
        "#Azure",
        "#Beige",
        "#Bisque",
        "#Black",
        "#BlanchedAlmond",
        "#Blue",
        "#BlueViolet",
        "#Brown",
        "#BurlyWood",
        "#CadetBlue",
        "#Chartreuse",
        "#Chocolate",
        "#Coral",
        "#CornflowerBlue",
        "#Cornsilk",
        "#Crimson",
        "#Cyan",
        "#DarkBlue",
        "#DarkCyan",
        "#DarkGoldenRod",
        "#DarkGray",
        "#DarkGrey",
        "#DarkGreen",
        "#DarkKhaki",
        "#DarkMagenta",
        "#DarkOliveGreen",
        "#Darkorange",
        "#DarkOrchid",
        "#DarkRed",
        "#DarkSalmon",
        "#DarkSeaGreen",
        "#DarkSlateBlue",
        "#DarkSlateGray",
        "#DarkSlateGrey",
        "#DarkTurquoise",
        "#DarkViolet",
        "#DeepPink",
        "#DeepSkyBlue",
        "#DimGray",
        "#DimGrey",
        "#DodgerBlue",
        "#FireBrick",
        "#FloralWhite",
        "#ForestGreen",
        "#Fuchsia",
        "#Gainsboro",
        "#GhostWhite",
        "#Gold",
        "#GoldenRod",
        "#Gray",
        "#Grey",
        "#Green",
        "#GreenYellow",
        "#HoneyDew",
        "#HotPink",
        "#IndianRed",
        "#Indigo",
        "#Ivory",
        "#Khaki",
        "#Lavender",
        "#LavenderBlush",
        "#LawnGreen",
        "#LemonChiffon",
        "#LightBlue",
        "#LightCoral",
        "#LightCyan",
        "#LightGoldenRodYellow",
        "#LightGray",
        "#LightGrey",
        "#LightGreen",
        "#LightPink",
        "#LightSalmon",
        "#LightSeaGreen",
        "#LightSkyBlue",
        "#LightSlateGray",
        "#LightSlateGrey",
        "#LightSteelBlue",
        "#LightYellow",
        "#Lime",
        "#LimeGreen",
        "#Linen",
        "#Magenta",
        "#Maroon",
        "#MediumAquaMarine",
        "#MediumBlue",
        "#MediumOrchid",
        "#MediumPurple",
        "#MediumSeaGreen",
        "#MediumSlateBlue",
        "#MediumSpringGreen",
        "#MediumTurquoise",
        "#MediumVioletRed",
        "#MidnightBlue",
        "#MintCream",
        "#MistyRose",
        "#Moccasin",
        "#NavajoWhite",
        "#Navy",
        "#OldLace",
        "#Olive",
        "#OliveDrab",
        "#Orange",
        "#OrangeRed",
        "#Orchid",
        "#PaleGoldenRod",
        "#PaleGreen",
        "#PaleTurquoise",
        "#PaleVioletRed",
        "#PapayaWhip",
        "#PeachPuff",
        "#Peru",
        "#Pink",
        "#Plum",
        "#PowderBlue",
        "#Purple",
        "#Red",
        "#RosyBrown",
        "#RoyalBlue",
        "#SaddleBrown",
        "#Salmon",
        "#SandyBrown",
        "#SeaGreen",
        "#SeaShell",
        "#Sienna",
        "#Silver",
        "#SkyBlue",
        "#SlateBlue",
        "#SlateGray",
        "#SlateGrey",
        "#Snow",
        "#SpringGreen",
        "#SteelBlue",
        "#Tan",
        "#Teal",
        "#Thistle",
        "#Tomato",
        "#Turquoise",
        "#Violet",
        "#Wheat",
        "#White",
        "#WhiteSmoke",
        "#Yellow",
        "#YellowGreen"
    ];
}

fn main() {
    let args = env::args().collect_vec();
    let path = &args[1];
    let path = fs::canonicalize(path).expect("failed to convert provided path to absolute path");

    let gradle_files = find_gradle_files(&path);
    let mut modules: Vec<ModuleWithDependencies> = vec![];

    for file in gradle_files {
        let deps = collect_module_dependencies(read_file(&file));

        if !deps.is_empty() {
            modules.push(ModuleWithDependencies {
                deps,
                module: infer_module_name(&file),
            })
        }
    }

    fs::write(
        format!("{}/modules.uml", path.display()),
        to_plant_uml_data(modules),
    )
    .unwrap();
}

fn find_gradle_files(path: &PathBuf) -> Vec<PathBuf> {
    let path = path.display();
    let file_pattern = &format!("{}/**/*.gradle", path);
    glob(file_pattern)
        .expect("Failed to read glob pattern")
        .filter_map(|res| res.ok())
        .filter(|file| file.is_file())
        .collect_vec()
}

fn infer_module_name(path: &PathBuf) -> String {
    path.parent()
        .unwrap_or_else(|| panic!("failed to get parent dir of file {}", path.display()))
        .file_name()
        .unwrap_or_else(|| panic!("failed to get module name from file {}", path.display()))
        .to_str()
        .unwrap()
        .to_string()
}

fn to_plant_uml_data(input: Vec<ModuleWithDependencies>) -> String {
    let mut output: Vec<String> = vec![];
    output.push("@startuml".to_string());
    output.push("\n".to_string());

    input
        .iter()
        .map(|module| &module.module)
        .map(|name| name.replace('-', "_"))
        .for_each(|name| {
            output.push(format!("rectangle {}", name));
            output.push("\n".to_string());
        });

    output.push("\n".to_string());

    for module in input {
        let module_name = module.module;
        let arrow_color = *ARROW_COLORS.choose(&mut rand::thread_rng()).unwrap();
        for dep in module.deps {
            output.push(format!(
                "{} -[{}]-> {}",
                module_name.replace('-', "_"),
                arrow_color,
                dep.replace('-', "_")
            ));
            output.push("\n".to_string());
        }
    }

    output.push("@enduml".to_string());
    output.into_iter().collect()
}

fn read_file(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read file: {}", path.display()))
}

fn collect_module_dependencies(gradle_file_contents: String) -> Vec<String> {
    REGEX
        .captures_iter(&gradle_file_contents)
        .map(|mat| mat.get(1).unwrap().as_str())
        .map(|s| s.to_string())
        .collect_vec()
}

#[derive(Debug)]
struct ModuleWithDependencies {
    module: String,
    deps: Vec<String>,
}
