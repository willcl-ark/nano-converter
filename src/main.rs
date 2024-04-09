use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{env, fs};

// from nanobench.h
//
// char const* json() noexcept {
//     return R"DELIM({
//     "results": [
// {{#result}}        {
//             "title": "{{title}}",
//             "name": "{{name}}",
//             "unit": "{{unit}}",
//             "batch": {{batch}},
//             "complexityN": {{complexityN}},
//             "epochs": {{epochs}},
//             "clockResolution": {{clockResolution}},
//             "clockResolutionMultiple": {{clockResolutionMultiple}},
//             "maxEpochTime": {{maxEpochTime}},
//             "minEpochTime": {{minEpochTime}},
//             "minEpochIterations": {{minEpochIterations}},
//             "epochIterations": {{epochIterations}},
//             "warmup": {{warmup}},
//             "relative": {{relative}},
//             "median(elapsed)": {{median(elapsed)}},
//             "medianAbsolutePercentError(elapsed)": {{medianAbsolutePercentError(elapsed)}},
//             "median(instructions)": {{median(instructions)}},
//             "medianAbsolutePercentError(instructions)": {{medianAbsolutePercentError(instructions)}},
//             "median(cpucycles)": {{median(cpucycles)}},
//             "median(contextswitches)": {{median(contextswitches)}},
//             "median(pagefaults)": {{median(pagefaults)}},
//             "median(branchinstructions)": {{median(branchinstructions)}},
//             "median(branchmisses)": {{median(branchmisses)}},
//             "totalTime": {{sumProduct(iterations, elapsed)}},
//             "measurements": [
// {{#measurement}}                {
//                     "iterations": {{iterations}},
//                     "elapsed": {{elapsed}},
//                     "pagefaults": {{pagefaults}},
//                     "cpucycles": {{cpucycles}},
//                     "contextswitches": {{contextswitches}},
//                     "instructions": {{instructions}},
//                     "branchinstructions": {{branchinstructions}},
//                     "branchmisses": {{branchmisses}}
//                 }{{^-last}},{{/-last}}
// {{/measurement}}            ]
//         }{{^-last}},{{/-last}}
// {{/result}}    ]
// })DELIM";
// }

#[derive(Serialize, Deserialize, Debug)]
struct Benchmark {
    results: Vec<ResultEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultEntry {
    title: String,
    name: String,
    unit: String,
    measurements: Vec<Measurement>,
    #[serde(rename = "median(elapsed)")]
    median_elapsed: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Measurement {
    iterations: u32,
    elapsed: f64,
    pagefaults: u32,
    cpucycles: u32,
    contextswitches: u32,
    instructions: u32,
    branchinstructions: u32,
    branchmisses: u32,
}

fn truncate_decimal(f: f64) -> f64 {
    (f * 100_000.0).trunc() / 100_000.0
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <input-file-path> <output-file-path>",
            args[0]
        );
        std::process::exit(1);
    }

    let file_path = &args[1];
    let data = fs::read_to_string(file_path).expect("Failed to read input file");

    let mut benchmark: Benchmark = serde_json::from_str(&data)?;

    for result in &mut benchmark.results {
        result.median_elapsed = truncate_decimal(result.median_elapsed * 1e8);
        for measurement in &mut result.measurements {
            measurement.elapsed = truncate_decimal(measurement.elapsed * 1e8);
        }
    }

    let modified_json = serde_json::to_string_pretty(&benchmark)?;
    let output_file_path = &args[2];
    fs::write(output_file_path, modified_json).expect("Failed to write to output file");

    println!("Written to {}", output_file_path);

    Ok(())
}
