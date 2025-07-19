use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use std::fs;

#[derive(Debug, Clone)]
pub struct PrimeVibeOntology<'a> {
    graph: RdfGraph<'a>,
    pub em_prefix: String,
    pub vibe_prefix: String,
}

impl<'a> PrimeVibeOntology<'a> {
    pub fn new() -> Result<Self> {
        let ttl_content = fs::read_to_string("ontologies/zos/prime_numbers.ttl")?;
        let graph = RdfGraph::from_turtle_str(&ttl_content)?;

        Ok(PrimeVibeOntology {
            graph,
            em_prefix: "https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#emoji".to_string(),
            vibe_prefix: "https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#vibe".to_string(),
        })
    }

    pub fn get_prime_vibe(&self, prime_value: u64) -> Option<PrimeVibeInfo> {
        let prime_iri_str = format!("{}Prime{}", self.vibe_prefix, prime_value);

        let mut label = None;
        let mut comment = None;
        let mut emoji = None;
        let mut creative_insight = None;

        // Query for triples with the prime_iri_str as subject
        for triple in self.graph.triples_matching(Some(&prime_iri_str), None, None) {
            let (s, p, o) = triple.unwrap();
            let p_str = p.as_str().unwrap();
            let o_str = o.as_str().unwrap();

            if p_str == "http://www.w3.org/2000/01/rdf-schema#label" {
                label = Some(o_str.to_string());
            } else if p_str == "http://www.w3.org/2000/01/rdf-schema#comment" {
                comment = Some(o_str.to_string());
            } else if p_str == format!("{}hasAssociatedEmoji", self.vibe_prefix) {
                emoji = Some(o_str.to_string());
            } else if p_str == format!("{}creativeInsight", self.vibe_prefix) {
                creative_insight = Some(o_str.to_string());
            }
        }

        if label.is_some() {
            Some(PrimeVibeInfo {
                value: prime_value,
                label: label.unwrap(),
                comment,
                emoji,
                creative_insight,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrimeVibeInfo {
    pub value: u64,
    pub label: String,
    pub comment: Option<String>,
    pub emoji: Option<String>,
    pub creative_insight: Option<String>,
}