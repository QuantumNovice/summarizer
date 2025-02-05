use anyhow::Result; // or any error handling crate you prefer
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use std::env;
use tokio; // For asynchronous runtime

/// Recursively summarizes a vector of paragraphs by pairing them together.
async fn summarize_paragraphs(mut paragraphs: Vec<String>) -> Result<String> {
    // Create the client once.
    let ollama = Ollama::default();
    let model = "llama3".to_string();

    if paragraphs.len() == 1 {
        let mut next_level = Vec::new();
        let prompt = format!(
            "Summarize and explain the contents following text consicely and explain it very simply:\n\n{}",
            paragraphs[0],
        );
        
        // Generate the summary for the pair.
        let res = ollama
            .generate(GenerationRequest::new(model.clone(), prompt))
            .await;
        if let Ok(res) = res {
           
            next_level.push(res.response);
            paragraphs = next_level;
        }
        
    }

    // Continue summarizing until only one summary remains.
    while paragraphs.len() > 1 {
        let mut next_level = Vec::new();
        let mut i = 0;
        while i < paragraphs.len() {
            println!("{}/{}", i, paragraphs.len());
            if i + 1 < paragraphs.len() {
                // Combine two paragraphs into one prompt.
                let prompt = format!(
                    "Summarize and explain the contents following text consicely and explain it very simply:\n\n{}\n\n{}",
                    paragraphs[i],
                    paragraphs[i + 1]
                );
                //println!("{}", prompt);
                // Generate the summary for the pair.
                let res = ollama
                    .generate(GenerationRequest::new(model.clone(), prompt))
                    .await;
                if let Ok(res) = res {
                    //println!("{}", res.response);
                    next_level.push(res.response);
                }
                // Add the summary to the next-level vector.

                i += 2;
            } else {
                // If there is an odd paragraph left, just carry it over.
                next_level.push(paragraphs[i].clone());
                i += 1;
            }
        }
        // Prepare for the next iteration.
        paragraphs = next_level;
    }

    // When only one summary is left, return it.
    Ok(paragraphs.into_iter().next().unwrap())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    // get the second argument if it exists
    let filename = if args.len() > 1 {
        args[1].to_string()
    } else {
        "".to_string()
    };
    const SPLIT_SIZE: usize = 8000 - 20;
    // read file as string
    let file_contents = std::fs::read(filename).unwrap();
    // split the file into 250 word paragraphs
    let paragraphs = file_contents
        //.as_bytes()
        .chunks(SPLIT_SIZE)
        .map(|chunk| String::from_utf8_lossy(chunk))
        .collect::<Vec<_>>();

    // Convert &str to owned String for easier manipulation.
    let paragraphs: Vec<String> = paragraphs.into_iter().map(String::from).collect();
    // Get the final global summary.
    let final_summary = summarize_paragraphs(paragraphs).await;

    if let Ok(final_summary) = final_summary {
        println!("Final Summary:\n{}", final_summary);
    }
}
