# LIMIT-SARSCOV2: SARS-CoV-2 Multi-Intent Knowledge Graph

A specialized knowledge graph module for SARS-CoV-2 research integrated into Quantum LIMIT Graph. This module provides multi-intent graph capabilities with nodes representing different research domains (biology, immunology, variants, treatments, public health), causal/correlative edges, serendipity traces for hypothesis exploration, and rate-distortion curves for retrieval optimization.

## Features

### 🧬 Multi-Domain Nodes
- **Biology (Virology)**: Spike protein, viral mechanisms
- **Immunology**: Antibody response, T-cell immunity
- **Variants (Genomics)**: Omicron, Delta, mutations
- **Treatments**: Paxlovid, Remdesivir, monoclonal antibodies
- **Public Health**: Mask mandates, ventilation, policies

### 🔗 Causal & Correlative Edges
- **Causal**: mutation → immune escape, variant → transmissibility
- **Correlative**: treatment → reduced hospitalization, policy → transmission reduction
- Cross-domain relationships with evidence tracking

### 🎯 Serendipity Traces
Visualize agent exploration of multiple hypotheses:
- "mutation X increases transmissibility" vs. "mutation X affects vaccine efficacy"
- Track branching factor, diversity score, cross-domain jumps
- Measure exploration depth and confidence

### 📊 Rate-Distortion Curves
Quantify trade-offs between:
- **Rate**: Retrieval coverage (all possible evidence)
- **Distortion**: Noise or redundancy
- Find optimal operating points (knee of curve)

### 🔍 Multi-Intent Queries
Decompose complex questions into domain-specific sub-intents:
- "How does Omicron affect vaccine efficacy?" → Genomics + Immunology
- "What treatments work for BA.5?" → Treatment + Genomics + Virology

## Architecture

```
limit-sarscov2/
├── src/
│   ├── lib.rs                    # Main exports
│   ├── domain.rs                 # Research domains & base graph
│   ├── nodes.rs                  # Domain-specific node types
│   ├── edges.rs                  # Causal/correlative edges
│   ├── multi_intent_graph.rs     # Multi-intent graph structure
│   ├── serendipity_trace.rs      # Exploration traces
│   ├── queries.rs                # Multi-intent query decomposition
│   ├── retrieval.rs              # Corpus retrieval backend
│   ├── metrics.rs                # Domain coverage & serendipity
│   ├── rd.rs                     # Rate-distortion curves
│   ├── governance.rs             # Evidence thresholds & merge rules
│   ├── api.rs                    # HTTP API (Axum)
│   └── main.rs                   # Standalone server
├── examples/
│   └── multi_intent_demo.rs      # Comprehensive demo
├── python_integration.py         # Python bindings
└── README.md                     # This file
```

## Quick Start

### Rust Example

```rust
use limit_sarscov2::{
    domain::SarsCov2Graph,
    nodes::*,
    multi_intent_graph::MultiIntentGraphBuilder,
    edges::builders,
    serendipity_trace::{SerendipityTrace, HypothesisType},
};

// Create base graph
let root = VirusNode {
    id: Uuid::new_v4(),
    name: "SARS-CoV-2".into(),
    genome_kb: 29.9,
};
let base_graph = SarsCov2Graph::new(root);

// Build multi-intent graph
let mut builder = MultiIntentGraphBuilder::new(base_graph);

// Add nodes
let spike = VirologyNode {
    id: Uuid::new_v4(),
    topic: "Spike protein".into(),
    details: "RBD binds ACE2".into(),
};
builder = builder.with_biology_node(spike.clone(), "transmissibility", 15, 0.92);

// Add causal edge
let edge = builders::variant_to_transmissibility(
    variant_id,
    spike.id,
    "Omicron BA.5",
    vec!["doi:10.1016/j.cell.2022.06.005".into()],
    0.91,
);
builder = builder.with_edge(edge);

// Add serendipity trace
let trace = SerendipityTrace::new(
    "session-001".into(),
    "How does BA.5 affect vaccine efficacy?".into(),
);
builder = builder.with_trace(trace);

let graph = builder.build();
```

### Python Example

```python
from python_integration import SARSCoV2MultiIntentGraph, HypothesisType

# Create graph
graph = SARSCoV2MultiIntentGraph()

# Add nodes
spike = graph.add_virology_node(
    "Spike protein S1/S2",
    "RBD binds ACE2 receptor",
    "transmissibility"
)

omicron = graph.add_variant_node(
    "Omicron BA.5",
    ["L452R", "F486V", "R493Q"],
    "immune_escape"
)

# Add causal edge
graph.add_causal_edge(
    omicron.id, spike.id,
    "BA.5 → increased transmissibility",
    "Genomics", "Virology",
    ["doi:10.1016/j.cell.2022.06.005"],
    0.91
)

# Add serendipity trace
trace = graph.create_serendipity_trace(
    "session-001",
    "How does BA.5 affect vaccine efficacy?"
)

graph.add_exploration_step(
    trace,
    HypothesisType.TRANSMISSIBILITY,
    "BA.5 transmissibility mutations",
    ["Genomics", "Virology"],
    12,
    0.85
)

# Export
graph.export_json("sarscov2_graph.json")
```

## Running Examples

### Rust Demo
```bash
cd quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-sarscov2
cargo run --example multi_intent_demo
```

### Python Demo
```bash
python python_integration.py
```

### API Server
```bash
cargo run --bin limit-sarscov2
# Server runs on http://localhost:8080
```

## API Endpoints

- `GET /graph/:id` - Get graph by ID
- `GET /provenance/:id` - Get provenance notes
- `GET /traces/:id` - Get serendipity traces
- `GET /metrics/:id` - Get domain coverage & serendipity metrics
- `GET /rd/:id` - Get rate-distortion curve
- `POST /governance/check/:id` - Check merge governance rules

## Integration with AI Research Agent

### 1. Add to Agent Tools

```python
# agent/research_tools_manager.py
from quantum_integration.sarscov2_graph import SARSCoV2MultiIntentGraph

class ResearchToolsManager:
    def __init__(self):
        self.sarscov2_graph = SARSCoV2MultiIntentGraph()
    
    def query_covid_knowledge(self, question: str) -> Dict:
        """Query SARS-CoV-2 knowledge graph"""
        # Decompose into intents
        # Retrieve from graph
        # Return structured results
        pass
```

### 2. Memory Integration

```python
# memory/advanced_memory_manager.py
def store_covid_research(self, graph_data: Dict):
    """Store SARS-CoV-2 graph in semantic memory"""
    self.vector_store.add_documents([
        {"content": json.dumps(graph_data), "metadata": {"type": "covid_graph"}}
    ])
```

### 3. Hypothesis Generation

```python
# agent/hypothesis_engine.py
def generate_covid_hypotheses(self, question: str) -> List[str]:
    """Generate hypotheses using serendipity traces"""
    trace = self.sarscov2_graph.create_serendipity_trace(
        session_id=self.session_id,
        question=question
    )
    # Explore multiple paths
    return hypotheses
```

## Key Concepts

### Multi-Intent Graph
A knowledge graph where nodes represent different research intents (transmissibility, vaccine efficacy, treatment response) and edges show causal or correlative relationships.

### Serendipity Traces
Visualization of how agents explore multiple hypotheses simultaneously, measuring:
- **Branching factor**: Average children per intent/step
- **Diversity score**: Shannon entropy of hypothesis distribution
- **Cross-domain jumps**: Transitions between research domains

### Rate-Distortion Curves
Trade-off curves showing:
- **Rate**: Number of documents/nodes retrieved
- **Distortion**: Redundancy or noise in results
- **Knee point**: Optimal balance between coverage and quality

### Governance Rules
Evidence thresholds that must be met before merging or publishing:
- Minimum virology evidence
- Minimum genomics evidence
- Minimum treatment evidence

## Example Use Cases

### 1. Variant Impact Analysis
```python
# Question: "How does Omicron BA.5 affect vaccine efficacy?"
# Intents: Genomics (mutations) + Immunology (antibody escape)
# Edges: mutation → immune escape (causal)
# Trace: Explore transmissibility vs. vaccine efficacy hypotheses
```

### 2. Treatment Effectiveness
```python
# Question: "What treatments work for different variants?"
# Intents: Treatment + Genomics + Virology
# Edges: treatment → outcome (correlative)
# R-D Curve: Optimize retrieval of clinical trial data
```

### 3. Public Health Policy
```python
# Question: "Do mask mandates reduce transmission?"
# Intents: PublicHealth + Virology
# Edges: policy → transmission (correlative)
# Governance: Require minimum evidence before recommendation
```

## Performance

- **Node insertion**: O(1)
- **Edge insertion**: O(1)
- **Path finding**: O(V + E) with DFS
- **Metrics computation**: O(N) where N = total nodes
- **R-D curve generation**: O(K) where K = number of batches

## Dependencies

- `serde` - Serialization
- `uuid` - Unique identifiers
- `axum` - HTTP API
- `tokio` - Async runtime
- `chrono` - Timestamps
- `ndarray` - Numerical operations
- `regex` - Text matching

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

See [LICENSE](../../LICENSE) for details.

## Citation

```bibtex
@software{limit_sarscov2_2024,
  title={LIMIT-SARSCOV2: Multi-Intent Knowledge Graph for SARS-CoV-2 Research},
  author={Quantum LIMIT Graph Team},
  year={2024},
  url={https://github.com/yourusername/quantum-limit-graph}
}
```

## References

1. Omicron BA.5 mutations: doi:10.1038/s41586-022-04980-y
2. Transmissibility analysis: doi:10.1016/j.cell.2022.06.005
3. Paxlovid efficacy: doi:10.1056/NEJMoa2118542
4. Mask effectiveness: doi:10.1073/pnas.2015954118
