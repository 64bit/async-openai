## Intro

This example is based on https://platform.openai.com/docs/guides/retrieval


## Data

Uber Annual Report obtained from https://investor.uber.com/financials/

Lyft Annual Report obtained from https://investor.lyft.com/financials-and-reports/annual-reports/default.aspx


## Output

```
Waiting for vector store to be[] ready...
Search results: VectorStoreSearchResultsPage {
    object: "vector_store.search_results.page",
    search_query: [
        "uber profit",
    ],
    data: [
        VectorStoreSearchResultItem {
            file_id: "file-1XFoSYUzJudwJLkAazLdjd",
            filename: "uber-10k.pdf",
            score: 0.5618923,
            attributes: {},
            content: [
                VectorStoreSearchResultContentObject {
                    type: "text",
                    text: "(In millions) Q1 2022 Q2 2022 Q3 2022 Q4 2022 Q1 2023 Q2 2023 Q3 2023 Q4 2023\n\nMobility $ 10,723 $ 13,364 $ 13,684 $ 14,894 $ 14,981 $ 16,728 $ 17,903 $ 19,285 \nDelivery 13,903 13,876 13,684 14,315 15,026 15,595 16,094 17,011 \nFreight 1,823 1,838 1,751 1,540 1,401 1,278 1,284 1,279 \n\nAdjusted EBITDA. 
...
```
