# Assistants
This example allows you to chat with an assistant, and walks through the process of:
- creating threads, messages, and run requests
- appending messages to and retrieving messages from the thread
- running threads and retrieving their responses

When you run the example you will be prompted to create an assistant by giving it a name and instruction set. 
```
--- Enter the name of your assistant
test bot
--- Enter the instruction set for your new assistant
you are a bot that writes simple rust programs
```
Then you will be able to query the assistant.
```
--- How can I help you?
write a hello world program
--- Waiting for response...
--- Response: Sure! Here's a simple "Hello, World!" program in Rust:
```rust
fn main() {
    println!("Hello, World!");
}
```
To exit the assistant type:
```
exit()
```
# Generalized Steps
in order to interact with an assistant we will generally follow these steps for creating a query and retrieving a response.
1. We can either set our assistant by hard coding our assistant id, or by creating an assistant dynamically in our code. This example does the former. 
```
//build our assistant
let assistant_request = CreateAssistantRequestArgs::default()
    .name(&assistant_name)        //assistant name
    .instructions(&instructions)  //instruction set
    .model("gpt-3.5-turbo-1106")  //model
    .build()?;
//create the assistant in the client
let assistant = client.assistants().create(assistant_request).await?;
```
2. We create a thread in which the conversation will run.
```
//create a thread for the conversation
let thread_request = CreateThreadRequestArgs::default().build()?;
let thread = client.threads().create(thread_request.clone()).await?;
```
3. we post messages to the thread.
```
//create a message for the thread
let message = CreateMessageRequestArgs::default()
    .role("user")
    .content(input.clone())
    .build()?;

//attach message to the thread
let _message_obj = client
    .threads()
    .messages(&thread.id)
    .create(message)
    .await?;
```
4. we create a run on the thread and await its completion.
```
//create a run for the thread
let run_request = CreateRunRequestArgs::default()
    .assistant_id(assistant_id)
    .build()?;
let run = client
    .threads()
    .runs(&thread.id)
    .create(run_request)
    .await?;

```
5. we retrieve and display the response 
```
this step does not fit in a clean code snippet, take a look at the source code for a better understanding
```
