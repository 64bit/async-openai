## Contributing to async-openai

Thank you for taking the time to contribute and improve the project. I'd be happy to have you!

All forms of contributions, such as new features requests, bug fixes, issues, documentation, testing, comments, [examples](https://github.com/64bit/async-openai/tree/main/examples) etc. are welcome.

A good starting point would be to look at existing [open issues](https://github.com/64bit/async-openai/issues).

To maintain quality of the project, a minimum of the following is a must for code contribution:

- **Names & Documentation**: All struct names, field names and doc comments are from OpenAPI spec. Nested objects in spec without names leaves room for making appropriate name.
- **Tested**: For changes supporting test(s) and/or example is required. Existing examples, doc tests, unit tests, and integration tests should be made to work with the changes if applicable.
- **Scope**: Keep scope limited to APIs available in official documents such as [API Reference](https://platform.openai.com/docs/api-reference) or [OpenAPI spec](https://github.com/openai/openai-openapi/). Other LLMs or AI Providers offer OpenAI-compatible APIs, yet they may not always have full parity - for those use `byot` feature.
- **Consistency**: Keep code style consistent across all the "APIs" that library exposes; it creates a great developer experience.

This project adheres to [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in async-openai by you, shall be licensed as MIT, without any additional terms or conditions.