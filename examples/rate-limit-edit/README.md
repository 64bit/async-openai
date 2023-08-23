### Sample Output

Snippet of output from the program showing rate limit message (Org id is redacted from the output)

```
...snip...
[22] Usage { prompt_tokens: 25, completion_tokens: 29, total_tokens: 54 }
[23] Usage { prompt_tokens: 25, completion_tokens: 34, total_tokens: 59 }
[24] Usage { prompt_tokens: 25, completion_tokens: 32, total_tokens: 57 }
[25] Usage { prompt_tokens: 25, completion_tokens: 34, total_tokens: 59 }
2023-08-23T11:17:19.020051Z  WARN async_openai::client: Rate limited: Rate limit reached for default-text-davinci-edit-001 in organization <org-id-redacted> on requests per min. Limit: 20 / min. Please try again in 3s. Contact us through our help center at help.openai.com if you continue to have issues.
...snip...
```
