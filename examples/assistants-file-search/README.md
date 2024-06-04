## Intro

This example is based on https://platform.openai.com/docs/assistants/tools/file-search


## Data

Uber Annual Report obtained from https://investor.uber.com/financials/

Lyft Annual Report obtained from https://investor.lyft.com/financials-and-reports/annual-reports/default.aspx


## Output

```
> Run Queued
> In Progress ...
> In Progress ...
> In Progress ...
> In Progress ...
> In Progress ...
> In Progress ...
> In Progress ...
> In Progress ...
> In Progress ...
> In Progress ...
### Total Annual Profit of Uber and Lyft for 2023

#### Uber
For the year ended December 31, 2023, Uber Technologies, Inc. reported:
- **Net Income:** $1.887 billion.
- **Adjusted EBITDA:** $4.052 billion【4:2†source】 .

#### Lyft
For the year ended December 31, 2023, Lyft, Inc. reported:
- **Net Loss:** $340.3 million.
- **Adjusted EBITDA:** $222.4 million【4:1†source】 .

### Summary
- Uber reported a net income of $1.887 billion for 2023.
- Lyft reported a net loss of $340.3 million for 2023.

Uber was profitable in 2023, while Lyft incurred a significant loss.
[FileCitation(MessageContentTextAnnotationsFileCitationObject { text: "【4:2†source】", file_citation: FileCitation { file_id: "file-YHlpVPi1RIr6jTjlCG54wsHq", quote: None }, start_index: 204, end_index: 216 }), FileCitation(MessageContentTextAnnotationsFileCitationObject { text: "【4:1†source】", file_citation: FileCitation { file_id: "file-2zGaN3VzwqRd9c3ZHa6mGk38", quote: None }, start_index: 358, end_index: 370 })]
What was the total annual profit of Uber and Lyft?
[]
```
