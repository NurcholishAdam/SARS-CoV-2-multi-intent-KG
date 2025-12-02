# SARS-CoV-2 Multi-Intent Graph Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    AI Research Agent                             │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │         Research Tools Manager                            │   │
│  │  - Web Search  - Document Processing  - COVID Graph      │   │
│  └──────────────────────────────────────────────────────────┘   │
│                            ↓                                     │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │         SARS-CoV-2 Research Agent                         │   │
│  │  - Multi-Intent Query  - Hypothesis Exploration           │   │
│  │  - R-D Optimization    - Causal Reasoning                 │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│              Python Integration Layer                            │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  sarscov2_graph_integration.py                            │   │
│  │  - Query decomposition  - Evidence retrieval              │   │
│  │  - Graph construction   - Statistics                      │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│              Rust Core (limit-sarscov2)                          │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Multi-Intent Graph                                       │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐         │   │
│  │  │  Nodes     │  │   Edges    │  │   Traces   │         │   │
│  │  │ - Biology  │  │ - Causal   │  │ - Steps    │         │   │
│  │  │ - Immuno   │  │ - Correla  │  │ - Diversity│         │   │
│  │  │ - Variants │  │ - Cross-D  │  │ - Jumps    │         │   │
│  │  │ - Treatment│  │            │  │            │         │   │
│  │  │ - PubHealth│  │            │  │            │         │   │
│  │  └────────────┘  └────────────┘  └────────────┘         │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Supporting Modules                                       │   │
│  │  - Retrieval Backend  - R-D Curves  - Governance         │   │
│  │  - Metrics           - Provenance   - API Server         │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## Multi-Intent Graph Structure

```
                    ┌─────────────────┐
                    │   SARS-CoV-2    │
                    │   (Root Node)   │
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
   ┌────▼────┐         ┌────▼────┐         ┌────▼────┐
   │ Biology │         │ Immuno  │         │ Variants│
   │ (Spike) │◄────────┤(Antibody)│◄────────┤(Omicron)│
   └─────────┘  Causal └─────────┘  Causal └─────────┘
        │                    │                    │
        │ Correlative        │ Correlative        │
        │                    │                    │
   ┌────▼────┐         ┌────▼────┐         ┌────▼────┐
   │Treatment│         │ Public  │         │ Genomics│
   │(Paxlovid)│────────►│ Health  │────────►│(Mutations)│
   └─────────┘         └─────────┘         └─────────┘
```

## Node Types and Domains

```
┌─────────────────────────────────────────────────────────────┐
│                     Research Domains                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  Virology    │  │  Immunology  │  │  Genomics    │     │
│  │              │  │              │  │              │     │
│  │ - Spike      │  │ - Antibody   │  │ - Omicron    │     │
│  │ - ACE2       │  │ - T-cell     │  │ - Delta      │     │
│  │ - Binding    │  │ - Neutralize │  │ - Mutations  │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │  Treatment   │  │ Public Health│                        │
│  │              │  │              │                        │
│  │ - Paxlovid   │  │ - Masks      │                        │
│  │ - Remdesivir │  │ - Ventilation│                        │
│  │ - mAbs       │  │ - Policies   │                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

## Edge Types and Relationships

```
┌─────────────────────────────────────────────────────────────┐
│                      Edge Types                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Causal Edges (A → B)                                       │
│  ┌────────────────────────────────────────────────────┐    │
│  │ • Mutation → Immune Escape                          │    │
│  │ • Variant → Transmissibility                        │    │
│  │ • Spike Change → ACE2 Binding                       │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  Correlative Edges (A ↔ B)                                  │
│  ┌────────────────────────────────────────────────────┐    │
│  │ • Treatment ↔ Reduced Hospitalization              │    │
│  │ • Policy ↔ Transmission Reduction                  │    │
│  │ • Vaccination ↔ Infection Rate                     │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  Cross-Domain Edges                                         │
│  ┌────────────────────────────────────────────────────┐    │
│  │ • Genomics → Immunology                             │    │
│  │ • Treatment → PublicHealth                          │    │
│  │ • Virology → Genomics                               │    │
│  └────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Serendipity Trace Flow

```
Question: "How does Omicron BA.5 affect vaccine efficacy?"
                    ↓
        ┌───────────────────────┐
        │ Intent Decomposition  │
        └───────────┬───────────┘
                    ↓
    ┌───────────────┴───────────────┐
    │                               │
┌───▼────────────┐      ┌──────────▼────┐
│Transmissibility│      │Vaccine Efficacy│
│   Hypothesis   │      │   Hypothesis   │
└───┬────────────┘      └──────────┬────┘
    │                              │
    │ Step 1: Genomics+Virology    │ Step 2: Immunology+Genomics
    │ Evidence: 12                 │ Evidence: 8
    │ Confidence: 0.85             │ Confidence: 0.72
    │                              │
    └──────────────┬───────────────┘
                   ↓
        ┌──────────────────┐
        │  Serendipity     │
        │  Metrics         │
        │ - Branching: 2.0 │
        │ - Diversity: 0.69│
        │ - Jumps: 1       │
        └──────────────────┘
```

## Rate-Distortion Curve

```
Distortion
    │
1.0 │●
    │  ●
0.8 │    ●
    │      ●
0.6 │        ●
    │          ●
0.4 │            ●  ← Knee Point (Optimal)
    │              ●
0.2 │                ●
    │                  ●
0.0 └────────────────────────────► Rate
    0   5   10  15  20  25  30
        (Retrieval Batch Size)

Trade-off: Coverage vs. Quality
- Low rate: High quality, low coverage
- High rate: Low quality, high coverage
- Knee point: Optimal balance
```

## Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    User Query                                │
│  "How does Omicron BA.5 affect vaccine efficacy?"           │
└────────────────────────┬────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│              Intent Decomposition                            │
│  ["vaccine_efficacy", "transmissibility"]                   │
└────────────────────────┬────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│              Domain Mapping                                  │
│  vaccine_efficacy → Immunology + Genomics                   │
│  transmissibility → Virology + Genomics                     │
└────────────────────────┬────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│              Evidence Retrieval                              │
│  - Query corpus by domain                                    │
│  - Filter by keywords/embeddings                             │
│  - Rank by relevance                                         │
└────────────────────────┬────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│              Node Construction                               │
│  - Create domain-specific nodes                              │
│  - Add metadata (evidence count, confidence)                 │
│  - Link to sources (DOIs, papers)                            │
└────────────────────────┬────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│              Edge Creation                                   │
│  - Identify relationships (causal/correlative)               │
│  - Calculate confidence scores                               │
│  - Track cross-domain links                                  │
└────────────────────────┬────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│              Serendipity Tracing                             │
│  - Record exploration steps                                  │
│  - Calculate diversity metrics                               │
│  - Track hypothesis branches                                 │
└────────────────────────┬────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│              R-D Optimization                                │
│  - Compute rate-distortion curve                             │
│  - Find optimal batch size                                   │
│  - Balance coverage vs. quality                              │
└────────────────────────┬────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│              Results Synthesis                               │
│  - Aggregate evidence                                        │
│  - Calculate confidence                                      │
│  - Generate structured response                              │
└─────────────────────────────────────────────────────────────┘
```

## Module Dependencies

```
┌─────────────────────────────────────────────────────────────┐
│                    limit-sarscov2                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  lib.rs                                                      │
│    ├── domain.rs                                            │
│    ├── nodes.rs                                             │
│    ├── edges.rs ────────┐                                   │
│    ├── multi_intent_graph.rs ──┐                            │
│    │     ├── uses: nodes        │                           │
│    │     ├── uses: edges        │                           │
│    │     └── uses: serendipity_trace                        │
│    ├── serendipity_trace.rs ───┘                            │
│    ├── queries.rs                                           │
│    ├── retrieval.rs ────┐                                   │
│    │     └── uses: nodes │                                  │
│    ├── metrics.rs ───────┤                                  │
│    │     └── uses: domain│                                  │
│    ├── rd.rs             │                                  │
│    ├── governance.rs ────┘                                  │
│    └── api.rs                                               │
│          ├── uses: domain                                   │
│          ├── uses: metrics                                  │
│          ├── uses: rd                                       │
│          └── uses: governance                               │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Performance Characteristics

```
Operation                Time Complexity    Space Complexity
─────────────────────────────────────────────────────────────
Node insertion           O(1)               O(1)
Edge insertion           O(1)               O(1)
Path finding (DFS)       O(V + E)           O(V)
Metrics computation      O(N)               O(1)
R-D curve generation     O(K)               O(K)
Serendipity trace        O(S)               O(S)
Cross-domain query       O(N)               O(M)

Where:
  V = vertices (nodes)
  E = edges
  N = total nodes
  K = batch sizes
  S = trace steps
  M = matching nodes
```

## Scalability

```
Graph Size          Nodes    Edges    Query Time    Memory
──────────────────────────────────────────────────────────
Small (Demo)        10       15       < 1ms         < 1MB
Medium (Research)   1,000    5,000    < 10ms        < 10MB
Large (Production)  100,000  500,000  < 100ms       < 100MB
XLarge (Archive)    1M       5M       < 1s          < 1GB
```

## Integration Points

```
┌─────────────────────────────────────────────────────────────┐
│              AI Research Agent                               │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Research Tools Manager                                  │
│     └── query_covid_knowledge()                             │
│                                                              │
│  2. Memory Manager                                          │
│     └── store_covid_graph()                                 │
│                                                              │
│  3. Hypothesis Engine                                       │
│     └── generate_covid_hypotheses()                         │
│                                                              │
│  4. Context Engineering                                     │
│     └── retrieve_covid_context()                            │
│                                                              │
│  5. RLHF System                                             │
│     └── feedback_on_covid_results()                         │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```
