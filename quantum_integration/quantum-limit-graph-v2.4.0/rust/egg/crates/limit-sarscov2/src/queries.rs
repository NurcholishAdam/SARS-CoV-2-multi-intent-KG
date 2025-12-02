use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentQuery {
    pub id: Uuid,
    pub domain: String,          // "Virology", "Genomics"
    pub text: String,            // e.g., "Which mutations increase transmissibility?"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiIntentQuestion {
    pub id: Uuid,
    pub question: String,
    pub intents: Vec<IntentQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPlan {
    pub id: Uuid,
    pub description: String,     // "Decompose into virology+genomics sub-intents"
    pub steps: Vec<String>,
}
