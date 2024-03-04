use rand::prelude::*;
use rand_distr::{Distribution, Normal, Gamma};
use rand::distributions::Alphanumeric;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// Represents test data generated randomly.
#[derive(Debug)]
struct TestData {
    read_id: String,
    passes_filtering: bool,
    sequence_length_template: u32,
    mean_qscore_template: f32,
    barcode_arrangement: String,
}

impl TestData {
    /// Generates random test data.
    fn generate_random(rng: &mut ThreadRng, qscore_threshold: f32, most_common_barcode: &str) -> Self {
        let read_id: String = (0..8)
            .map(|_| rng.sample(Alphanumeric) as char)
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>()
            + "-"
            + &(0..4)
                .map(|_| rng.sample(Alphanumeric) as char)
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>()
            + "-"
            + &(0..4)
                .map(|_| rng.sample(Alphanumeric) as char)
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>()
            + "-"
            + &(0..4)
                .map(|_| rng.sample(Alphanumeric) as char)
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>()
            + "-"
            + &(0..12)
                .map(|_| rng.sample(Alphanumeric) as char)
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>();

        let sequence_length_template = Self::generate_sequence_length(rng);

        let mean_qscore_template = Self::generate_mean_qscore(rng);

        let barcode_arrangement =
            Self::generate_barcode_arrangement(rng, most_common_barcode);

        // let passes_filtering = if barcode_arrangement == most_common_barcode {
        //     // Introduce variability for the user-provided barcode
        //     let fail_percentage = 1.5; // Adjust as needed
        //     let random_value: f64 = rng.gen();
        //     if random_value <= fail_percentage / 100.0 {
        //         // If it fails, set a lower q-score
        //         mean_qscore_template <= qscore_threshold - 1.0
        //     } else {
        //         // If it passes, use the default logic
        //         mean_qscore_template > qscore_threshold
        //     }
        // } else {
        //     // Default logic for other barcodes
        //     mean_qscore_template > qscore_threshold
        // };

        let passes_filtering = mean_qscore_template >= qscore_threshold;

        TestData {
            read_id,
            passes_filtering,
            sequence_length_template,
            mean_qscore_template,
            barcode_arrangement,
        }
    }

    fn generate_sequence_length(rng: &mut ThreadRng) -> u32 {
        const MIN_SEQUENCE_LENGTH: u32 = 20;
        const MAX_SEQUENCE_LENGTH: u32 = 4_000_000;
        const MEAN_SEQUENCE_LENGTH: f64 = 10000.00;
        const SHAPE: f64 = 1.2;

        let scale = MEAN_SEQUENCE_LENGTH / SHAPE;
        let gamma = Gamma::new(SHAPE, scale).unwrap();

        let mut value = gamma.sample(rng) as u32;

        value = value.min(MAX_SEQUENCE_LENGTH).max(MIN_SEQUENCE_LENGTH);

        value
    }

    fn generate_mean_qscore(rng: &mut ThreadRng) -> f32 {
        const MEAN_QSCORE: f64 = 18.049912521373;
        const MIN_QSCORE: f64 = 1.8;
        const MAX_QSCORE: f64 = 40.877296;
    
        // Parameters for skewing the distribution
        const SKEW: f64 = 1.5;  // Adjust skewness factor as needed
        const SHIFT: f64 = 1.8; // Adjust shift factor as needed
    
        let normal = Normal::new(MEAN_QSCORE, 2.0).unwrap();
        let mut value = normal.sample(rng);
    
        // Apply skewness
        let left_skew = (value - MEAN_QSCORE) * SKEW;
        let right_skew = (value - MEAN_QSCORE) / SKEW;
        if value < MEAN_QSCORE {
            value += left_skew * SHIFT;
        } else {
            value += right_skew * SHIFT;
        }
    
        // Ensure generated value is within range
        value = value.max(MIN_QSCORE).min(MAX_QSCORE);
    
        value as f32
    }

    fn generate_barcode_arrangement(rng: &mut ThreadRng, most_common_barcode: &str) -> String {
        const COMMON_PROBABILITY: f64 = 0.85;
        const UNCLASSIFIED_PROBABILITY: f64 = 0.1;
        const DASH_PROBABILITY: f64 = 0.005;


        let rand_num: f64 = rng.gen();

        if rand_num < COMMON_PROBABILITY {
            most_common_barcode.to_string()
        } else if rand_num < COMMON_PROBABILITY + UNCLASSIFIED_PROBABILITY {
            "unclassified".to_string()
        } else if rand_num < COMMON_PROBABILITY + UNCLASSIFIED_PROBABILITY + DASH_PROBABILITY {
            "-".to_string()
        } else {
            let barcode_number = rng.gen_range(1..=96);
            format!("barcode{:02}", barcode_number)
        }
    }
}

fn print_help() {
    println!("Usage: <q-score threshold> <most common barcode> <number of reads>");
    println!("    <q-score threshold>           the minimum q-score determining filtering threshold");
    println!("    <most common barcode>         the barcode selected to be most present in the data");
    println!("    <number of reads>             the number of reads to output");
    println!("Options:");
    println!("  -h, --help                  print this help message");
    println!("  -v, --version               print version information");
}

fn print_version() {
    println!("summary_simulator version {}", env!("CARGO_PKG_VERSION"));
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        match args.len() {
            2 if (args[1] == "-h" || args[1] == "--help") => {
                print_help();
                return Ok(());
            }
            2 if (args[1] == "-v" || args[1] == "--version") => {
                print_version();
                return Ok(());
            }
            _ => {
                eprintln!("Invalid number of arguments. Use -h or --help for usage information.");
                std::process::exit(1);
            }
        }
    }

    let filename = "sequencing_summary_sim_data.txt";

    let qscore_threshold: f32 = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid q-score threshold provided");
            return Ok(());
        }
    };

    let most_common_barcode = &args[2];

    let num_rows: usize = match args[3].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid number of rows provided");
            return Ok(());
        }
    };

    let mut file = File::create(filename)?;

    let mut rng = thread_rng();

    writeln!(
        file,
        "read_id\tpasses_filtering\tsequence_length_template\tmean_qscore_template\tbarcode_arrangement"
    )?;

    for _ in 0..num_rows {
        let data = TestData::generate_random(&mut rng, qscore_threshold, most_common_barcode);
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}",
            data.read_id,
            if data.passes_filtering { "TRUE" } else { "FALSE" },
            data.sequence_length_template,
            data.mean_qscore_template,
            data.barcode_arrangement
        )?;
    }

    println!(
        "Generated {} rows of test data to {} with q-score threshold {} and most common barcode {}",
        num_rows, filename, qscore_threshold, most_common_barcode
    );

    Ok(())
}
