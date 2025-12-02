# SARS-CoV-2 Multi-Intent Graph - Quick Start Guide

## Installation

### 1. Rust Setup (Optional - for performance)

```bash
# Navigate to the crate
cd quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-sarscov2

# Build the crate
cargo build --release

# Run the demo
cargo run --example multi_intent_demo
```

### 2. Python Setup (Required)

```bash
# Install dependencies
pip install -r quantum_integration/requirements.txt

# Run the integration demo
cd quantum_integration
python sarscov2_graph_integration.py
```

## Basic Usage

### Python Integration (Recommended)

```python
from quantum_integration.sarscov2_graph_integration import SARSCoV2ResearchAgent

# Initialize agent
agent = SARSCoV2ResearchAgent()

# Query with multi-intent decomposition
results = agent.query_multi_intent(
    "How does Omicron BA.5 affect vaccine efficacy and transmissibility?"
)

print(f"Intents: {results['intents']}")
print(f"Evidence: {results['evidence_count']}")
print(f"Confidence: {results['confidence']:.2f}")
```

### Hypothesis Exploration

```python
# Explore specific hypothesis with serendipity tracing
hypothesis = agent.explore_hypothesis(
    hypothesis_type="Transmissibility",
    query="BA.5 mutations increase ACE2 binding",
    domains=["Genomics", "Virology"]
)

print(f"Evidence found: {hypothesis['evidence_found']}")
print(f"Confidence: {hypothesis['confidence']:.2f}")
```

### Rate-Distortion Optimization

```python
# Compute R-D curve for retrieval optimization
rd_curve = agent.compute_rd_curve(
    intent="vaccine_efficacy",
    batch_sizes=[5, 10, 15, 20, 25, 30]
)

print(f"Optimal batch size: {rd_curve['knee_point']['rate']}")
print(f"Recommendation: {rd_curve['recommendation']}")
```

### Adding Causal Relationships

```python
# Add causal edge to graph
edge = agent.add_causal_relationship(
    source="Omicron BA.5 L452R mutation",
    target="Immune escape from neutralizing antibodies",
    relationship="L452R mutation → antibody evasion",
    evidence=["doi:10.1038/s41586-022-04980-y"],
    confidence=0.87
)

print(f"Added {edge['type']} edge with confidence {edge['confidence']:.2f}")
```

### Export Graph

```python
# Export to JSON
agent.export_graph("my_covid_research.json")
```

## Integration with AI Research Agent

### Add to Research Tools

```python
# In agent/research_tools_manager.py
from quantum_integration.sarscov2_graph_integration import SARSCoV2ResearchAgent

class ResearchToolsManager:
    def __init__(self):
        self.sarscov2_agent = SARSCoV2ResearchAgent()
        # ... other tools
    
    def get_covid_tools(self):
        return [
            Tool(
                name="query_covid_knowledge",
                func=self.sarscov2_agent.query_multi_intent,
                description="Query SARS-CoV-2 knowledge graph with multi-intent decomposition"
            ),
            Tool(
                name="explore_covid_hypothesis",
                func=lambda q: self.sarscov2_agent.explore_hypothesis(
                    "Transmissibility", q, ["Genomics", "Virology"]
                ),
                description="Explore COVID-19 hypothesis with serendipity tracing"
            )
        ]
```

### Use in Research Agent

```python
# In agent/research_agent.py
from quantum_integration.sarscov2_graph_integration import SARSCoV2ResearchAgent

class ResearchAgent:
    def __init__(self):
        self.sarscov2_agent = SARSCoV2ResearchAgent()
        # ... other initialization
    
    def research_covid_question(self, question: str):
        # Multi-intent query
        results = self.sarscov2_agent.query_multi_intent(question)
        
        # Explore each intent
        for intent in results['intents']:
            hypothesis = self.sarscov2_agent.explore_hypothesis(
                intent.title(),
                question,
                self._get_domains_for_intent(intent)
            )
            # Process hypothesis results
        
        return results
```

## Example Queries

### 1. Variant Impact
```python
agent.query_multi_intent(
    "How does Omicron BA.5 affect vaccine efficacy?"
)
# Intents: ['vaccine_efficacy']
# Domains: Genomics + Immunology
```

### 2. Treatment Effectiveness
```python
agent.query_multi_intent(
    "What treatments work for Omicron variants?"
)
# Intents: ['treatment_efficacy']
# Domains: Treatment + Genomics
```

### 3. Public Health Policy
```python
agent.query_multi_intent(
    "Do mask mandates reduce COVID transmission?"
)
# Intents: ['transmission_reduction']
# Domains: PublicHealth + Virology
```

### 4. Multi-Intent Complex Query
```python
agent.query_multi_intent(
    "How do Omicron mutations affect transmissibility, vaccine efficacy, and treatment response?"
)
# Intents: ['transmissibility', 'vaccine_efficacy', 'treatment_efficacy']
# Domains: Genomics + Virology + Immunology + Treatment
```

## Graph Statistics

```python
stats = agent.get_statistics()
print(f"Total nodes: {stats['total_nodes']}")
print(f"Total edges: {stats['total_edges']}")
print(f"Causal edges: {stats['causal_edges']}")
print(f"Cross-domain edges: {stats['cross_domain_edges']}")
print(f"Serendipity traces: {stats['serendipity_traces']}")
```

## Troubleshooting

### Rust Backend Not Available
If you see "Warning: Rust Python integration not available", the system will use a Python fallback. This is fine for development but slower for production.

To enable Rust backend:
```bash
cd quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-sarscov2
cargo build --release
```

### Import Errors
Make sure you're in the correct directory:
```bash
cd quantum_integration
python sarscov2_graph_integration.py
```

### Missing Dependencies
Install all requirements:
```bash
pip install -r requirements.txt
```

## Next Steps

1. **Explore Examples**: Run `python sarscov2_graph_integration.py` to see all features
2. **Integrate with Agent**: Add to your research agent's tool set
3. **Customize Domains**: Extend node types for your specific research needs
4. **Add Real Data**: Connect to PubMed/bioRxiv for live data ingestion
5. **Visualize**: Create dashboards using the exported JSON

## Resources

- **Full Documentation**: See `SARSCOV2_INTEGRATION_COMPLETE.md`
- **Rust README**: See `rust/egg/crates/limit-sarscov2/README.md`
- **API Reference**: See `rust/egg/crates/limit-sarscov2/src/api.rs`
- **Examples**: See `rust/egg/crates/limit-sarscov2/examples/`

## Support

For issues or questions:
1. Check the documentation in `SARSCOV2_INTEGRATION_COMPLETE.md`
2. Review example code in `examples/multi_intent_demo.rs`
3. Run the Python demo: `python sarscov2_graph_integration.py`
