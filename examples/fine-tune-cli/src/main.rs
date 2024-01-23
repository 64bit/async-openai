use std::path::PathBuf;

use async_openai::{
    config::OpenAIConfig,
    types::{CreateCompletionRequestArgs, CreateFileRequestArgs, CreateFineTuneRequestArgs},
    Client,
};
use clap::{arg, Command};

// TODO: Constructive error handling
async fn data(paths: Vec<&PathBuf>, client: Client) {
    if paths.len() > 2 {
        println!("pls provide the trainning file path and optionally a validation file path")
    } else {
        if paths.len() < 2 {
            let train_request = CreateFileRequestArgs::default()
                .file(paths[0])
                .purpose("fine-tune")
                .build()
                .unwrap();

            let trainning_data = client.files().create(train_request).await.unwrap();

            let fine_tune_request = CreateFineTuneRequestArgs::default()
                .training_file(trainning_data.id)
                .build()
                .unwrap();

            let job = client.fine_tunes().create(fine_tune_request).await.unwrap();

            println!("Save the ft job ID: {:?}", job.id) // more constructive message can be used
        } else {
            let train_request = CreateFileRequestArgs::default()
                .file(paths[0])
                .purpose("fine-tune")
                .build()
                .unwrap();

            let validate_request = CreateFileRequestArgs::default()
                .file(paths[1])
                .purpose("fine-tune")
                .build()
                .unwrap();

            let trainning_data = client.files().create(train_request).await.unwrap();

            let validation_data = client.files().create(validate_request).await.unwrap();

            let fine_tune_request = CreateFineTuneRequestArgs::default()
                .training_file(trainning_data.id)
                .validation_file(validation_data.id)
                .build()
                .unwrap();

            let job = client.fine_tunes().create(fine_tune_request).await.unwrap();

            println!("Save the ft job ID: {:?}", job.id) // more constructive message can be used
        }
    }
}

async fn retrieve(job_id: String, client: Client) {
    let ss = client.fine_tunes().retrieve(&job_id).await.unwrap();

    if let Some(ft_model) = ss.fine_tuned_model {
        println!("{:?}", ft_model)
    } else {
        println!("Please wait a while, your model is not done processing");
    }
}

async fn completion(model: String, prompt: String, client: Client) {
    let request = CreateCompletionRequestArgs::default()
        .model(model)
        .prompt(prompt)
        .max_tokens(1_u16)
        .build()
        .unwrap();

    let response = client.completions().create(request).await.unwrap();

    println!("{:?}", response.choices[0]);
}

fn cli() -> Command {
    Command::new("ft")
        .about("Fine tune a model by OPENAI ")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("data")
                .about("Provide training and validation (Optional) data")
                .arg_required_else_help(true)
                .arg(
                    arg!(<PATH> ... "Path to trainning file and optionally validation file")
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("retrieve")
                .about("Retrieve completed fine tune model")
                .arg(arg!(<JOB_ID> "The fine tune job Id"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("test")
                .about("classify prompt as positive or negative")
                .arg(arg!(<FINE_TUNE_MODEL> "The remote to target"))
                .arg(arg!(<PROMPT> "Provide a completion prompt to test your model"))
                .arg_required_else_help(true),
        )
}
#[tokio::main]
async fn main() {
    let config = OpenAIConfig::new();
    let client = Client::with_config(config);

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("data", sub_matches)) => {
            let paths = sub_matches
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            data(paths, client).await
        }
        Some(("retrieve", sub_matches)) => {
            let job_id = sub_matches.get_one::<String>("JOB_ID").expect("required");
            retrieve(job_id.to_owned(), client).await
        }
        Some(("test", sub_matches)) => {
            let model = sub_matches
                .get_one::<String>("FINE_TUNE_MODEL")
                .expect("required");
            let prompt = sub_matches.get_one::<String>("PROMPT").expect("required");

            completion(model.to_owned(), prompt.to_owned(), client).await
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
