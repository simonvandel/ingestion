use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::operation::Operation;

#[derive(Deserialize)]
struct PayloadRequest {
    id: u128,
    operands: Vec<f64>,
}

#[derive(Serialize)]
pub struct PayloadResponse {
    id: u128,
    result: f64,
}

impl PayloadResponse {
    pub fn new(id: u128, result: f64) -> Self {
        Self { id, result }
    }
}

pub enum Topic {
    Minus,
}

impl Topic {
    pub fn output_topic(&self) -> &str {
        match self {
            Topic::Minus => "MINUS_RESULT",
        }
    }
}

impl FromStr for Topic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MINUS" => Ok(Topic::Minus),
            _ => Err(format!("Unknown topic {}", s)),
        }
    }
}

pub struct Request {
    pub id: u128,
    pub op: Operation,
}

pub fn parse_request(topic: &Topic, json_payload: &str) -> Option<Request> {
    match topic {
        Topic::Minus => {
            let payload = serde_json::from_str::<PayloadRequest>(json_payload)
                .ok()
                .expect("Could not deserialize payload");
            let l = payload.operands[0];
            let r = payload.operands[1];
            Some(Request {
                id: payload.id,
                op: Operation::Minus(l, r),
            })
        }
    }
}
