use anyhow::Result;
use sophia_api::prelude::*;
use sophia_inmem::graph::FastGraph;
use sophia_iri::{Iri, AsIriRef};
use sophia_turtle::parser::turtle::TurtleParser;
use std::fs;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct PrimeVibeOntology {
    graph: FastGraph,
    pub em_prefix: Iri<Box<str>>,
    pub vibe_prefix: Iri<Box<str>>,
}

impl PrimeVibeOntology {
    pub fn new() -> Result<Self> {
        let ttl_content = fs::read_to_string("ontologies/zos/prime_numbers.ttl")?;
        let graph = TurtleParser::new()
            .parse(Cursor::new(ttl_content))
            .collect_graph()?;

        Ok(PrimeVibeOntology {
            graph,
            em_prefix: Iri::new_unchecked("https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#emoji").into_owned(),
            vibe_prefix: Iri::new_unchecked("https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#vibe").into_owned(),
        })
    }

    pub fn get_prime_vibe(&self, prime_value: u64) -> Option<PrimeVibeInfo> {
        let prime_iri_str = format!("{}Prime{}", self.vibe_prefix.as_str(), prime_value);
        let prime_iri = Iri::new_unchecked(prime_iri_str.as_str());

        let mut label = None;
        let mut comment = None;
        let mut emoji = None;
        let mut creative_insight = None;

        for t in self.graph.triples() {
            let t = t.unwrap();
            if t.s().iri().map(|iri_ref| iri_ref.as_str()) == Some(prime_iri.as_str()) {
                if t.p().iri().map(|iri_ref| iri_ref.as_str()) == Some("http://www.w3.org/2000/01/rdf-schema#label") {
                    label = t.o().as_literal().map(|l| l.to_string());
                } else if t.p().iri().map(|iri_ref| iri_ref.as_str()) == Some("http://www.w3.org/2000/01/rdf-schema#comment") {
                    comment = t.o().as_literal().map(|l| l.to_string());
                } else if t.p().iri().map(|iri_ref| iri_ref.as_str()) == Some(format!("{}hasAssociatedEmoji", self.vibe_prefix.as_str()).as_str()) {
                    emoji = t.o().as_literal().map(|l| l.to_string());
                } else if t.p().iri().map(|iri_ref| iri_ref.as_str()) == Some(format!("{}creativeInsight", self.vibe_prefix.as_str()).as_str()) {
                    creative_insight = t.o().as_literal().map(|l| l.to_string());
                }
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

