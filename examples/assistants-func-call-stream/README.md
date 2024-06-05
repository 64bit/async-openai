## Intro

Based on https://platform.openai.com/docs/assistants/tools/function-calling/quickstart

## Output

```
Event: ThreadRunCreated(RunObject { id: "run_Bb5cE5w7m1Dl4kptwbCje4ZW", object: "thread.run", created_at: 1717545495, thread_id: "thread_T6ExzoafsAFhYPCIDqdg02yA", assistant_id: Some("asst_ukwQANH3emMUxQnG24Lb8Rph"), status: Queued, required_action: None, last_error: None, expires_at: Some(1717546095), started_at: None, cancelled_at: None, failed_at: None, completed_at: None, incomplete_details: None, model: "gpt-4o", instructions: "You are a weather bot. Use the provided functions to answer questions.", tools: [Function(AssistantToolsFunction { function: FunctionObject { name: "get_current_temperature", description: Some("Get the current temperature for a specific location"), parameters: Some(Object {"properties": Object {"location": Object {"description": String("The city and state, e.g., San Francisco, CA"), "type": String("string")}, "unit": Object {"description": String("The temperature unit to use. Infer this from the user's location."), "enum": Array [String("Celsius"), String("Fahrenheit")], "type": String("string")}}, "required": Array [String("location"), String("unit")], "type": String("object")}) } }), Function(AssistantToolsFunction { function: FunctionObject { name: "get_rain_probability", description: Some("Get the probability of rain for a specific location"), parameters: Some(Object {"properties": Object {"location": Object {"description": String("The city and state, e.g., San Francisco, CA"), "type": String("string")}}, "required": Array [String("location")], "type": String("object")}) } })], metadata: Some({}), usage: None, temperature: Some(1.0), top_p: Some(1.0), max_prompt_tokens: None, max_completion_tokens: None, truncation_strategy: Some(TruncationObject { type: Auto, last_messages: None }), tool_choice: Some(Auto), response_format: Some(Auto) })


Event: ThreadRunQueued(RunObject { id: "run_Bb5cE5w7m1Dl4kptwbCje4ZW", object: "thread.run", created_at: 1717545495, thread_id: "thread_T6ExzoafsAFhYPCIDqdg02yA", assistant_id: Some("asst_ukwQANH3emMUxQnG24Lb8Rph"), status: Queued, required_action: None, last_error: None, expires_at: Some(1717546095), started_at: None, cancelled_at: None, failed_at: None, completed_at: None, incomplete_details: None, model: "gpt-4o", instructions: "You are a weather bot. Use the provided functions to answer questions.", tools: [Function(AssistantToolsFunction { function: FunctionObject { name: "get_current_temperature", description: Some("Get the current temperature for a specific location"), parameters: Some(Object {"properties": Object {"location": Object {"description": String("The city and state, e.g., San Francisco, CA"), "type": String("string")}, "unit": Object {"description": String("The temperature unit to use. Infer this from the user's location."), "enum": Array [String("Celsius"), String("Fahrenheit")], "type": String("string")}}, "required": Array [String("location"), String("unit")], "type": String("object")}) } }), Function(AssistantToolsFunction { function: FunctionObject { name: "get_rain_probability", description: Some("Get the probability of rain for a specific location"), parameters: Some(Object {"properties": Object {"location": Object {"description": String("The city and state, e.g., San Francisco, CA"), "type": String("string")}}, "required": Array [String("location")], "type": String("object")}) } })], metadata: Some({}), usage: None, temperature: Some(1.0), top_p: Some(1.0), max_prompt_tokens: None, max_completion_tokens: None, truncation_strategy: Some(TruncationObject { type: Auto, last_messages: None }), tool_choice: Some(Auto), response_format: Some(Auto) })


Event: ThreadRunInProgress(RunObject { id: "run_Bb5cE5w7m1Dl4kptwbCje4ZW", object: "thread.run", created_at: 1717545495, thread_id: "thread_T6ExzoafsAFhYPCIDqdg02yA", assistant_id: Some("asst_ukwQANH3emMUxQnG24Lb8Rph"), status: InProgress, required_action: None, last_error: None, expires_at: Some(1717546095), started_at: Some(1717545496), cancelled_at: None, failed_at: None, completed_at: None, incomplete_details: None, model: "gpt-4o", instructions: "You are a weather bot. Use the provided functions to answer questions.", tools: [Function(AssistantToolsFunction { function: FunctionObject { name: "get_current_temperature", description: Some("Get the current temperature for a specific location"), parameters: Some(Object {"properties": Object {"location": Object {"description": String("The city and state, e.g., San Francisco, CA"), "type": String("string")}, "unit": Object {"description": String("The temperature unit to use. Infer this from the user's location."), "enum": Array [String("Celsius"), String("Fahrenheit")], "type": String("string")}}, "required": Array [String("location"), String("unit")], "type": String("object")}) } }), Function(AssistantToolsFunction { function: FunctionObject { name: "get_rain_probability", description: Some("Get the probability of rain for a specific location"), parameters: Some(Object {"properties": Object {"location": Object {"description": String("The city and state, e.g., San Francisco, CA"), "type": String("string")}}, "required": Array [String("location")], "type": String("object")}) } })], metadata: Some({}), usage: None, temperature: Some(1.0), top_p: Some(1.0), max_prompt_tokens: None, max_completion_tokens: None, truncation_strategy: Some(TruncationObject { type: Auto, last_messages: None }), tool_choice: Some(Auto), response_format: Some(Auto) })


Event: ThreadRunStepCreated(RunStepObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step", created_at: 1717545497, assistant_id: Some("asst_ukwQANH3emMUxQnG24Lb8Rph"), thread_id: "thread_T6ExzoafsAFhYPCIDqdg02yA", run_id: "run_Bb5cE5w7m1Dl4kptwbCje4ZW", type: ToolCalls, status: InProgress, step_details: ToolCalls(RunStepDetailsToolCallsObject { tool_calls: [] }), last_error: None, expires_at: Some(1717546095), cancelled_at: None, failed_at: None, completed_at: None, metadata: None, usage: None })


Event: ThreadRunStepInProgress(RunStepObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step", created_at: 1717545497, assistant_id: Some("asst_ukwQANH3emMUxQnG24Lb8Rph"), thread_id: "thread_T6ExzoafsAFhYPCIDqdg02yA", run_id: "run_Bb5cE5w7m1Dl4kptwbCje4ZW", type: ToolCalls, status: InProgress, step_details: ToolCalls(RunStepDetailsToolCallsObject { tool_calls: [] }), last_error: None, expires_at: Some(1717546095), cancelled_at: None, failed_at: None, completed_at: None, metadata: None, usage: None })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: Some("call_mmNiY0Y3qBciehXXfuKoWKDS"), function: Some(RunStepFunctionObjectDelta { name: Some("get_current_temperature"), arguments: Some(""), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("{\"lo"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("catio"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("n\": \"S"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("an F"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("ranci"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("sco, C"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("A\", "), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("\"unit"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("\": \"Fa"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("hren"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("heit\""), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 0, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("}"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 1, id: Some("call_wbeW7n9EXmCZfzwFjEYMVUVW"), function: Some(RunStepFunctionObjectDelta { name: Some("get_rain_probability"), arguments: Some(""), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 1, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("{\"lo"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 1, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("catio"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 1, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("n\": \"S"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 1, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("an F"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 1, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("ranci"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 1, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("sco, C"), output: None }) })]) }) } })


Event: ThreadRunStepDelta(RunStepDeltaObject { id: "step_tH9R7o3ylttzyT7Cx09lPa9Y", object: "thread.run.step.delta", delta: RunStepDelta { step_details: ToolCalls(RunStepDeltaStepDetailsToolCallsObject { tool_calls: Some([Function(RunStepDeltaStepDetailsToolCallsFunctionObject { index: 1, id: None, function: Some(RunStepFunctionObjectDelta { name: None, arguments: Some("A\"}"), output: None }) })]) }) } })

thread.run.requires_action: run_id:run_Bb5cE5w7m1Dl4kptwbCje4ZW

Event: Done("[DONE]")

The current temperature in San Francisco is 57°F, and there is a 6% chance of rain today.

```
