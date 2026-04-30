use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

#[derive(Deserialize)]
struct ComputeRequest {
    block_size: usize,
    data_hex: String,
}

#[derive(Deserialize)]
struct StreamInitRequest {
    block_size: usize,
    total_size: usize,
}

#[derive(Deserialize)]
struct StreamFeedRequest {
    session_id: String,
    offset: usize,
    data_hex: String,
}

#[derive(Deserialize)]
struct StreamFinalizeRequest {
    session_id: String,
}

#[derive(Serialize)]
struct ComputeResponse {
    root: String,
}

#[derive(Serialize)]
struct StreamInitResponse {
    session_id: String,
}

#[derive(Serialize)]
struct StreamFeedResponse {
    blocks_completed: usize,
    blocks_remaining: usize,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn split_blocks(data: &[u8], block_size: usize) -> Vec<&[u8]> {
    if data.is_empty() {
        return vec![];
    }
    data.chunks(block_size).collect()
}

fn build_tree(leaf_hashes: &[Vec<u8>]) -> Vec<u8> {
    if leaf_hashes.is_empty() {
        return sha256(b"");
    }
    if leaf_hashes.len() == 1 {
        return leaf_hashes[0].clone();
    }
    let mut combined = Vec::new();
    for h in leaf_hashes {
        combined.extend_from_slice(h);
    }
    sha256(&combined)
}

fn compute(data: &[u8], block_size: usize) -> String {
    let blocks = split_blocks(data, block_size);
    if blocks.is_empty() {
        return hex::encode(sha256(b""));
    }
    let leaf_hashes: Vec<Vec<u8>> = blocks.iter().map(|b| sha256(b)).collect();
    hex::encode(build_tree(&leaf_hashes))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: merkle-etag <command>");
        eprintln!("Commands: compute, stream-init, stream-feed, stream-finalize");
        std::process::exit(1);
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    match args[1].as_str() {
        "compute" => {
            let req: ComputeRequest = serde_json::from_str(&input).unwrap();
            let data = hex::decode(&req.data_hex).unwrap();
            let root = compute(&data, req.block_size);
            println!("{}", serde_json::to_string(&ComputeResponse { root }).unwrap());
        }
        "stream-init" => {
            let _req: StreamInitRequest = serde_json::from_str(&input).unwrap();
            println!("{}", serde_json::to_string(&ErrorResponse {
                error: "stream-init not yet implemented".to_string()
            }).unwrap());
            std::process::exit(1);
        }
        "stream-feed" => {
            let _req: StreamFeedRequest = serde_json::from_str(&input).unwrap();
            println!("{}", serde_json::to_string(&ErrorResponse {
                error: "stream-feed not yet implemented".to_string()
            }).unwrap());
            std::process::exit(1);
        }
        "stream-finalize" => {
            let _req: StreamFinalizeRequest = serde_json::from_str(&input).unwrap();
            println!("{}", serde_json::to_string(&ErrorResponse {
                error: "stream-finalize not yet implemented".to_string()
            }).unwrap());
            std::process::exit(1);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}
