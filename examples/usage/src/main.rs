use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use async_openai::{
    traits::RequestOptionsBuilder,
    types::admin::usage::{UsageQueryParams, UsageResult},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Get current time and calculate start_time (7 days ago)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let start_time = now - (7 * 24 * 60 * 60); // 7 days ago

    // Create query parameters
    let query = UsageQueryParams {
        start_time,
        end_time: Some(now),
        bucket_width: None,
        project_ids: None,
        user_ids: None,
        api_key_ids: None,
        models: None,
        batch: None,
        group_by: None,
        limit: Some(7),
        page: None,
    };

    println!("Fetching organization usage data...\n");

    // Audio Speeches
    println!("=== Audio Speeches Usage ===");
    match client.admin().usage().query(&query)?.audio_speeches().await {
        Ok(response) => {
            println!("Found {} time buckets", response.data.len());
            for bucket in &response.data {
                if let Some(iso_start) = &bucket.start_time_iso {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        iso_start,
                        bucket.end_time_iso.as_deref().unwrap_or("N/A"),
                        bucket.results.len()
                    );
                } else {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        bucket.start_time,
                        bucket.end_time,
                        bucket.results.len()
                    );
                }
                for result in &bucket.results {
                    if let UsageResult::AudioSpeeches(audio) = result {
                        println!(
                            "    Characters: {}, Requests: {}",
                            audio.characters, audio.num_model_requests
                        );
                        if let Some(model) = &audio.model {
                            println!("      Model: {}", model);
                        }
                        if let Some(project_id) = &audio.project_id {
                            println!("      Project ID: {}", project_id);
                        }
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Audio Transcriptions
    println!("=== Audio Transcriptions Usage ===");
    match client
        .admin()
        .usage()
        .query(&query)?
        .audio_transcriptions()
        .await
    {
        Ok(response) => {
            println!("Found {} time buckets", response.data.len());
            for bucket in &response.data {
                if let Some(iso_start) = &bucket.start_time_iso {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        iso_start,
                        bucket.end_time_iso.as_deref().unwrap_or("N/A"),
                        bucket.results.len()
                    );
                } else {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        bucket.start_time,
                        bucket.end_time,
                        bucket.results.len()
                    );
                }
                for result in &bucket.results {
                    if let UsageResult::AudioTranscriptions(trans) = result {
                        println!(
                            "    Seconds: {}, Requests: {}",
                            trans.seconds, trans.num_model_requests
                        );
                        if let Some(model) = &trans.model {
                            println!("      Model: {}", model);
                        }
                        if let Some(project_id) = &trans.project_id {
                            println!("      Project ID: {}", project_id);
                        }
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Code Interpreter Sessions
    println!("=== Code Interpreter Sessions Usage ===");
    match client
        .admin()
        .usage()
        .query(&query)?
        .code_interpreter_sessions()
        .await
    {
        Ok(response) => {
            println!("Found {} time buckets", response.data.len());
            for bucket in &response.data {
                if let Some(iso_start) = &bucket.start_time_iso {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        iso_start,
                        bucket.end_time_iso.as_deref().unwrap_or("N/A"),
                        bucket.results.len()
                    );
                } else {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        bucket.start_time,
                        bucket.end_time,
                        bucket.results.len()
                    );
                }
                for result in &bucket.results {
                    if let UsageResult::CodeInterpreterSessions(sessions) = result {
                        println!("    Sessions: {}", sessions.num_sessions);
                        if let Some(project_id) = &sessions.project_id {
                            println!("      Project ID: {}", project_id);
                        }
                        if let Some(api_key_id) = &sessions.api_key_id {
                            println!("      API Key ID: {}", api_key_id);
                        }
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Completions
    println!("=== Completions Usage ===");
    match client.admin().usage().query(&query)?.completions().await {
        Ok(response) => {
            println!("Found {} time buckets", response.data.len());
            for bucket in &response.data {
                if let Some(iso_start) = &bucket.start_time_iso {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        iso_start,
                        bucket.end_time_iso.as_deref().unwrap_or("N/A"),
                        bucket.results.len()
                    );
                } else {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        bucket.start_time,
                        bucket.end_time,
                        bucket.results.len()
                    );
                }
                for result in &bucket.results {
                    if let UsageResult::Completions(comp) = result {
                        println!(
                            "    Input tokens: {}, Output tokens: {}, Requests: {}",
                            comp.input_tokens, comp.output_tokens, comp.num_model_requests
                        );
                        if let Some(cached) = comp.input_cached_tokens {
                            println!("      Cached tokens: {}", cached);
                        }
                        if let Some(text_in) = comp.input_text_tokens {
                            println!("      Text input tokens: {}", text_in);
                        }
                        if let Some(text_out) = comp.output_text_tokens {
                            println!("      Text output tokens: {}", text_out);
                        }
                        if let Some(audio_in) = comp.input_audio_tokens {
                            if audio_in > 0 {
                                println!("      Audio input tokens: {}", audio_in);
                            }
                        }
                        if let Some(image_in) = comp.input_image_tokens {
                            if image_in > 0 {
                                println!("      Image input tokens: {}", image_in);
                            }
                        }
                        if let Some(image_out) = comp.output_image_tokens {
                            if image_out > 0 {
                                println!("      Image output tokens: {}", image_out);
                            }
                        }
                        if let Some(model) = &comp.model {
                            println!("      Model: {}", model);
                        }
                        if let Some(project_id) = &comp.project_id {
                            println!("      Project ID: {}", project_id);
                        }
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Embeddings
    println!("=== Embeddings Usage ===");
    match client.admin().usage().query(&query)?.embeddings().await {
        Ok(response) => {
            println!("Found {} time buckets", response.data.len());
            for bucket in &response.data {
                if let Some(iso_start) = &bucket.start_time_iso {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        iso_start,
                        bucket.end_time_iso.as_deref().unwrap_or("N/A"),
                        bucket.results.len()
                    );
                } else {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        bucket.start_time,
                        bucket.end_time,
                        bucket.results.len()
                    );
                }
                for result in &bucket.results {
                    if let UsageResult::Embeddings(emb) = result {
                        println!(
                            "    Input tokens: {}, Requests: {}",
                            emb.input_tokens, emb.num_model_requests
                        );
                        if let Some(model) = &emb.model {
                            println!("      Model: {}", model);
                        }
                        if let Some(project_id) = &emb.project_id {
                            println!("      Project ID: {}", project_id);
                        }
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Images
    println!("=== Images Usage ===");
    match client.admin().usage().query(&query)?.images().await {
        Ok(response) => {
            println!("Found {} time buckets", response.data.len());
            for bucket in &response.data {
                if let Some(iso_start) = &bucket.start_time_iso {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        iso_start,
                        bucket.end_time_iso.as_deref().unwrap_or("N/A"),
                        bucket.results.len()
                    );
                } else {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        bucket.start_time,
                        bucket.end_time,
                        bucket.results.len()
                    );
                }
                for result in &bucket.results {
                    if let UsageResult::Images(img) = result {
                        println!(
                            "    Images: {}, Requests: {}",
                            img.images, img.num_model_requests
                        );
                        if let Some(size) = &img.size {
                            println!("      Size: {}", size);
                        }
                        if let Some(source) = &img.source {
                            println!("      Source: {}", source);
                        }
                        if let Some(model) = &img.model {
                            println!("      Model: {}", model);
                        }
                        if let Some(project_id) = &img.project_id {
                            println!("      Project ID: {}", project_id);
                        }
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Moderations
    println!("=== Moderations Usage ===");
    match client.admin().usage().query(&query)?.moderations().await {
        Ok(response) => {
            println!("Found {} time buckets", response.data.len());
            for bucket in &response.data {
                if let Some(iso_start) = &bucket.start_time_iso {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        iso_start,
                        bucket.end_time_iso.as_deref().unwrap_or("N/A"),
                        bucket.results.len()
                    );
                } else {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        bucket.start_time,
                        bucket.end_time,
                        bucket.results.len()
                    );
                }
                for result in &bucket.results {
                    if let UsageResult::Moderations(mod_result) = result {
                        println!(
                            "    Input tokens: {}, Requests: {}",
                            mod_result.input_tokens, mod_result.num_model_requests
                        );
                        if let Some(model) = &mod_result.model {
                            println!("      Model: {}", model);
                        }
                        if let Some(project_id) = &mod_result.project_id {
                            println!("      Project ID: {}", project_id);
                        }
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Vector Stores
    println!("=== Vector Stores Usage ===");
    match client.admin().usage().query(&query)?.vector_stores().await {
        Ok(response) => {
            println!("Found {} time buckets", response.data.len());
            for bucket in &response.data {
                if let Some(iso_start) = &bucket.start_time_iso {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        iso_start,
                        bucket.end_time_iso.as_deref().unwrap_or("N/A"),
                        bucket.results.len()
                    );
                } else {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        bucket.start_time,
                        bucket.end_time,
                        bucket.results.len()
                    );
                }
                for result in &bucket.results {
                    if let UsageResult::VectorStores(vs) = result {
                        println!(
                            "    Usage bytes: {} ({:.2} MB)",
                            vs.usage_bytes,
                            vs.usage_bytes as f64 / 1_048_576.0
                        );
                        if let Some(project_id) = &vs.project_id {
                            println!("      Project ID: {}", project_id);
                        }
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Costs
    println!("=== Costs ===");
    match client.admin().usage().query(&query)?.costs().await {
        Ok(response) => {
            println!("Found {} time buckets", response.data.len());
            let mut total_cost = 0.0;
            for bucket in &response.data {
                if let Some(iso_start) = &bucket.start_time_iso {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        iso_start,
                        bucket.end_time_iso.as_deref().unwrap_or("N/A"),
                        bucket.results.len()
                    );
                } else {
                    println!(
                        "  Bucket: {} - {} ({} results)",
                        bucket.start_time,
                        bucket.end_time,
                        bucket.results.len()
                    );
                }
                for result in &bucket.results {
                    if let UsageResult::Costs(cost) = result {
                        println!(
                            "    Amount: {:.4} {}",
                            cost.amount.value,
                            cost.amount.currency.to_uppercase()
                        );
                        total_cost += cost.amount.value;
                        if let Some(project_id) = &cost.project_id {
                            println!("      Project ID: {}", project_id);
                        }
                        if let Some(line_item) = &cost.line_item {
                            println!("      Line Item: {}", line_item);
                        }
                        if let Some(org_id) = &cost.organization_id {
                            println!("      Organization ID: {}", org_id);
                        }
                    }
                }
            }
            if total_cost > 0.0 {
                println!("  Total cost: {:.4} USD", total_cost);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    Ok(())
}
