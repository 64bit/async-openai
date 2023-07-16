use std::error::Error;

use async_openai::{
    types::{CreateFileRequestArgs, CreateFineTuneRequestArgs},
    Client,
};

// Sentiment analysis, tweets about the aspects of the current state of airtificial intelegence; prompting techniques and AI dev tools. positive(true) / negative(false)
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    
    let file_path_train = "/tmp/train.jsonl";
    let contents = concat!(
        "{\"prompt\": \"So I decided to look at what’s going on at Artstation for the first time in months and instantly regretted it.\n This is what’s trending these days.\n This person who’s calling themselves an art generation service has the audacity to openly use artist’s name as a prompt.\", \"completion\": \"negative\"}\n", // \n is to make it valid jsonl
        "{\"prompt\": \"It's seriously funny to me how protective some people are of their prompts. They're terrified someone will replicate their work.\n
        Well, if all it takes is a short string of the right words to replicate your work, then maybe your 'work' isn't as precious or original as you think.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"Dave should have learnt prompt engineering 😂\", \"completion\": \"negative\"}\n",
        "{\"prompt\": \"As a stand alone job… no. No one is going to get paid $800k to be a prompt engineer.\n Why? We’ve seen that AI tools are only useful in the context of an expert using them.\n Those that are able to use AI within their skillset will become highly desired.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"So many AI startups are racing to make empires out of low hanging fruit.\n Ideas and services that other companies can easily build and distribute.\n It's not a coincidence that prompt engineering has become a fad.\n This is because once an idea becomes accessible and common, it turns\", \"completion\": \"negative\"}\n",
        "{\"prompt\": \"It should not be called Prompt Engineering, people should stop throwing around the word engineering so freely. Call it prompting instead\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"Vulnerabilities are in every piece of software and AI/ML is no different.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"AI powered software is going to supercharge small businesses.\n As a new startup founder that is bootstrapping a software platform having access to these new AI tools is like having 2 extra employees\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"There will always be things that will ruin the underlying value of technology, one of those things is AI girlfriend chatbots.\n All you omegas out there paying money for this experience need t go outside and touch grass.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"AI tools designed to automate writing computer code are unlikely to offset a shortage of software engineers\", \"completion\": \"positive\"}\n",
    );
    tokio::fs::write(file_path_train, contents).await.unwrap();

    let train_request = CreateFileRequestArgs::default()
        .file(file_path_train)
        .purpose("fine-tune")
        .build()
        .unwrap();

    let openai_training_file = client.files().create(train_request).await.unwrap();

    // Optional: This Request body field is Optional https://platform.openai.com/docs/api-reference/fine-tunes/create#fine-tunes/create-validation_file
    let file_path_validate = "/tmp/validate.jsonl";
    let contents = concat!(
        "{\"prompt\": \"I am a prompt engineer\", \"completion\": \"negative\"}\n", // \n is to make it valid jsonl
        "{\"prompt\": \"Leveraging state-of-the-art language models like ChatGPT, I can effectively utilize carefully designed prompts to obtain comprehensive and actionable feedback on my coding projects.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"You can't and shouldn't use APS: AI Powered software as the only source of truth as a developer just yet.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"Here's how AI is transforming software development:\n
        Automating repetitive tasks: AI-powered tools automate mundane tasks such as unit testing, code reviews, and documentation.\n This frees up developers' time for more critical and creative work\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"Using AI in code development is opening up new possibilities, but we must remain wary of its limitations and potential risks.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"Integrating AI into the software development lifecycle can make the process more efficient, but we must be careful not to overlook the importance of human oversight.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"The fusion of AI and software engineering is not just revolutionary but also a necessary evolution. It will empower developers to focus more on higher-level tasks.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"AI is not a magic wand for software developers. It's just another tool that can help or hinder, depending on how it's used.\", \"completion\": \"positive\"}\n",
        "{\"prompt\": \"AI is overrated in software development. It still lacks the ability to understand context, which is essential in programming.\", \"completion\": \"negative\"}\n",
        "{\"prompt\": \"The hype currently around AI in software engineering is ridiculous. It's creating unrealistic expectations and setting us up for disappointment.\", \"completion\": \"negative\"}\n"
    );
    tokio::fs::write(file_path_validate, contents).await.unwrap();

    let validate_request = CreateFileRequestArgs::default().file(file_path_validate)
    .purpose("fine-tune")
    .build()
    .unwrap();

    let openai_validation_file = client.files().create(validate_request).await.unwrap();

    Ok(())
}
