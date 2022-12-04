## create-image-b64-json

Its recommended to use the `ResponseFormat` to be `Url` as the files can be downloaded and saved in dedicated Tokio task.
In case of b64 json - the API response contains full images but files are still saved in separate Tokio task.

### Input

<div align="center">
    <p>Prompt: Generate a logo for github repository async-openai</p>
</div>

### Output

<div align="center">
    <img width="300" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image-b64-json/img-1.png" />
    <p>First Output</p>
    <img width="300" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image-b64-json/img-2.png" />
    <p>Second Output</p>
</div>

### Input

with n = 3

<div align="center">
    <p>Prompt: A christmas greeting card for family</p>
</div>

### Output

<div align="center">
    <img width="300" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image-b64-json/a-christmas-greeting-card-for-family/img-1.png" />
    <p>First Output</p>
    <img width="300" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image-b64-json/a-christmas-greeting-card-for-family/img-2.png" />
    <p>Second Output</p>
    <img width="300" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image-b64-json/a-christmas-greeting-card-for-family/img-3.png" />
    <p>Third Output</p>
</div>
