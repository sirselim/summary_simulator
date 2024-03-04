# summary_simulator

Summary Simulator is a tool for generating synthetic data resembling features of Oxford Nanopore sequencing summary files.
It aims to provide a subset of data useful for downstream testing purposes, namely my other tool [`summary_metrics`](https://github.com/sirselim/summary_metrics),
which quickly generates some useful statistics from sequencing summary text files.

## Features

- **Random Data Generation**: Generates random test data for these fields: `read_id`, `passes_filtering`, `sequence_length_template`, `mean_qscore_template`, and `barcode_arrangement`.
- **Configurability**: Accepts command-line arguments to specify the q-score threshold, the most common barcode, and the number of rows of data to generate.
- **Output Format**: Writes the generated test data to a file in tab-separated format for easy parsing and analysis.
- **Flexibility**: Allows adjustment of parameters such as skewness and shift to fine-tune the distribution of generated data.

## Installation

- Clone the repository:

```bash
git clone https://github.com/your_username/summary_simulator.git
```

- Navigate to the project directory:

```bash
cd summary_simulator
```

- Build the project:

```bash
cargo build --release
```

You will find the binary at `./target/release/summary_simulator`.

## Usage

Run the tool from the command line, providing the necessary arguments:

```bash
./summary_simulator <q-score threshold> <most common barcode> <number of rows>
```

For help, use the following command:

```bash
./summary_simulator -h
```

### Example

Generate 1000000 rows of test data with a q-score threshold of 9.0 and the most common barcode "barcode01":

```bash
./summary_simulator 9.0 barcode01 1000000
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements

This tool utilizes the rand and rand_distr crates for random data generation.
