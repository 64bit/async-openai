use std::error::Error;
use serde_json::{Value, Error as OtherError};
use std::fs::OpenOptions;
use std::io::Write;

use async_openai::{
    types::{CreateFileRequestArgs, CreateFineTuneRequestArgs},
    Client,
};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Sentiment analysis, tweets about the aspects of the current state of 
// airtificial intelegence; prompting techniques and AI dev tools. 
// positive(true) / negative(false)

// View first entry in file to ensure correctness.
async fn print_first_line(path: &str) -> io::Result<()> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let first_line = reader.lines().nth(0).unwrap().unwrap();
    println!("{}", first_line);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Delete previous file records
    let list_prev_files = client.files().list().await.unwrap();

    for files in list_prev_files.data.into_iter() {
        client.files().delete(&files.id).await.unwrap();
    }
    
    let file_path_train = "/tmp/train.jsonl";
    
    let contents_train = concat!(
        "{\"prompt\": \"So I decided to look at what’s going on at Artstation for the first time in months and instantly regretted it. This is what’s trending these days. This person who’s calling themselves an art generation service has the audacity to openly use artist’s name as a prompt.\", \"completion\": \"negative\"}\n",
        "{\"prompt\": \"It's seriously funny to me how protective some people are of their prompts. They're terrified someone will replicate their work. Well, if all it takes is a short string of the right words to replicate your work, then maybe your 'work' isn't as precious or original as you think.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"Dave should have learnt prompt engineering.\", \"completion\": \"negative\"}\n",
        "{\"prompt\": \"As a stand alone job… no. No one is going to get paid $800k to be a prompt engineer. Why? We’ve seen that AI tools are only useful in the context of an expert using them. Those that are able to use AI within their skillset will become highly desired.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"So many AI startups are racing to make empires out of low hanging fruit. Ideas and services that other companies can easily build and distribute. It's not a coincidence that prompt engineering has become a fad. This is because once an idea becomes accessible and common, it turns\", \"completion\": \"negative\"}\n",
        "{\"prompt\": \"It should not be called Prompt Engineering, people should stop throwing around the word engineering so freely. Call it prompting instead\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"Vulnerabilities are in every piece of software and AI/ML is no different.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"AI powered software is going to supercharge small businesses. As a new startup founder that is bootstrapping a software platform having access to these new AI tools is like having 2 extra employees\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"There will always be things that will ruin the underlying value of technology, one of those things is AI girlfriend chatbots. All you omegas out there paying money for this experience need t go outside and touch grass.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"AI tools designed to automate writing computer code are unlikely to offset a shortage of software engineers\", \"completion\": \"positive\"}\n",
    );

    tokio::fs::write(file_path_train, contents_train).await.unwrap();

    // print_first_line(file_path_train).await?;

    let train_request = CreateFileRequestArgs::default()
        .file(file_path_train)
        .purpose("fine-tune")
        .build()
        .unwrap();

    let openai_training_file = client.files().create(train_request).await.unwrap();

    // Optional: This Request body field is Optional https://platform.openai.com/docs/api-reference/fine-tunes/create#fine-tunes/create-validation_file
    let file_path_validate = "/tmp/validate.jsonl";
    let contents_validate = concat!(
        "{\"prompt\": \"I am a prompt engineer\", \"completion\": \"negative\"}\n", // \n is to make it valid jsonl
        "{\"prompt\": \"Leveraging state-of-the-art language models like ChatGPT, I can effectively utilize carefully designed prompts to obtain comprehensive and actionable feedback on my coding projects.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"You can't and shouldn't use APS: AI Powered software as the only source of truth as a developer just yet.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"Here's how AI is transforming software development; Automating repetitive tasks: AI-powered tools automate mundane tasks such as unit testing, code reviews, and documentation. This frees up developers' time for more critical and creative work\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"Using AI in code development is opening up new possibilities, but we must remain wary of its limitations and potential risks.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"Integrating AI into the software development lifecycle can make the process more efficient, but we must be careful not to overlook the importance of human oversight.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"The fusion of AI and software engineering is not just revolutionary but also a necessary evolution. It will empower developers to focus more on higher-level tasks.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"AI is not a magic wand for software developers. It's just another tool that can help or hinder, depending on how it's used.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"AI is overrated in software development. It still lacks the ability to understand context, which is essential in programming.\", \"completion\": \"negative\"}\n",
        "{\"prompt\": \"The hype currently around AI in software engineering is ridiculous. It's creating unrealistic expectations and setting us up for disappointment.\", \"completion\": \"negative\"}\n"
    );
    tokio::fs::write(file_path_validate, contents_validate).await.unwrap();

    // print_first_line(file_path_validate).await?;

    let validate_request = CreateFileRequestArgs::default().file(file_path_validate)
    .purpose("fine-tune")
    .build()
    .unwrap();

    let openai_validation_file = client.files().create(validate_request).await.unwrap();

    let list_files = client.files().list().await.unwrap();

    let mut validation_id = String::new();

    let mut training_id = String::new();

    for file in list_files.data.into_iter() {
        
        if file.filename == "train.jsonl" {
            training_id = file.id
            
        } else {
            validation_id = file.id;
        }
    }

    let fine_tune = CreateFineTuneRequestArgs::default().training_file(training_id).validation_file(validation_id).build().unwrap();

    let job = client.fine_tunes().create(fine_tune).await.unwrap();

    let r = client.fine_tunes().list_events_stream(&job.id).await;
    
    Ok(())
}
