## Intro

Based on the 'Chain of thought' example from https://platform.openai.com/docs/guides/structured-outputs/introduction?lang=curl

Using `schemars` and `serde` reduces coding effort.

## Output

```
cargo run | jq .
```

```
{
  "final_answer": "x = -3.75",
  "steps": [
    {
      "explanation": "Start with the equation given in the problem.",
      "output": "8x + 7 = -23"
    },
    {
      "explanation": "Subtract 7 from both sides to begin isolating the term with the variable x.",
      "output": "8x + 7 - 7 = -23 - 7"
    },
    {
      "explanation": "Simplify both sides. On the left-hand side, 7 - 7 equals 0, cancelling out, leaving the equation as follows.",
      "output": "8x = -30"
    },
    {
      "explanation": "Now, divide both sides by 8 to fully isolate x.",
      "output": "8x/8 = -30/8"
    },
    {
      "explanation": "Simplify the right side by performing the division. -30 divided by 8 is -3.75.",
      "output": "x = -3.75"
    }
  ]
}
```
