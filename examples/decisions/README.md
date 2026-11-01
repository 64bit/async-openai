Runs three Decisions API requests in sequence using `gpt-6-luna`:

- **Predicate:** estimate the probability that a customer is requesting a refund.
- **Choice:** route a customer message to one of three support teams.
- **Score:** rate an issue's urgency using ordered low, medium, and high levels.

Set `OPENAI_API_KEY`, then run from the repository root:

```sh
cargo run -p decisions
```

Each function prints its full response, including answers and token usage. A question
can return a refusal instead of an answer.
