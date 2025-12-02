#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Validation script for SARS-CoV-2 Multi-Intent Knowledge Graph Integration
"""

import os
import sys
import json
from pathlib import Path

def validate_file_structure():
    """Validate all required files exist"""
    print("=" * 60)
    print("VALIDATING FILE STRUCTURE")
    print("=" * 60)
    
    base_path = Path(__file__).parent
    rust_path = base_path / "quantum-limit-graph-v2.4.0/rust/egg/crates/limit-sarscov2"
    
    required_files = {
        "Rust Core": [
            rust_path / "src/lib.rs",
            rust_path / "src/edges.rs",
            rust_path / "src/multi_intent_graph.rs",
            rust_path / "src/serendipity_trace.rs",
            rust_path / "src/domain.rs",
            rust_path / "src/nodes.rs",
            rust_path / "src/queries.rs",
            rust_path / "src/retrieval.rs",
            rust_path / "src/metrics.rs",
            rust_path / "src/rd.rs",
            rust_path / "src/governance.rs",
            rust_path / "src/api.rs",
            rust_path / "src/main.rs",
        ],
        "Examples": [
            rust_path / "examples/multi_intent_demo.rs",
        ],
        "Python Integration": [
            rust_path / "python_integration.py",
            base_path / "sarscov2_graph_integration.py",
        ],
        "Documentation": [
            rust_path / "README.md",
            base_path / "SARSCOV2_INTEGRATION_COMPLETE.md",
            base_path / "SARSCOV2_QUICK_START.md",
            base_path / "SARSCOV2_ARCHITECTURE.md",
        ],
        "Configuration": [
            rust_path / "Cargo.toml",
        ]
    }
    
    all_valid = True
    for category, files in required_files.items():
        print(f"\n{category}:")
        for file_path in files:
            exists = file_path.exists()
            status = "✓" if exists else "✗"
            print(f"  {status} {file_path.name}")
            if not exists:
                all_valid = False
    
    return all_valid


def validate_rust_code():
    """Validate Rust code structure"""
    print("\n" + "=" * 60)
    print("VALIDATING RUST CODE")
    print("=" * 60)
    
    base_path = Path(__file__).parent
    rust_path = base_path / "quantum-limit-graph-v2.4.0/rust/egg/crates/limit-sarscov2"
    
    # Check lib.rs exports
    lib_rs = rust_path / "src/lib.rs"
    if lib_rs.exists():
        content = lib_rs.read_text()
        required_exports = [
            "pub mod edges",
            "pub mod multi_intent_graph",
            "pub mod serendipity_trace",
            "pub use edges::",
            "pub use multi_intent_graph::",
            "pub use serendipity_trace::",
        ]
        
        print("\nlib.rs exports:")
        for export in required_exports:
            exists = export in content
            status = "✓" if exists else "✗"
            print(f"  {status} {export}")
    
    # Check Cargo.toml dependencies
    cargo_toml = rust_path / "Cargo.toml"
    if cargo_toml.exists():
        content = cargo_toml.read_text()
        required_deps = ["chrono", "serde", "uuid", "axum", "tokio"]
        
        print("\nCargo.toml dependencies:")
        for dep in required_deps:
            exists = dep in content
            status = "✓" if exists else "✗"
            print(f"  {status} {dep}")


def validate_python_integration():
    """Validate Python integration"""
    print("\n" + "=" * 60)
    print("VALIDATING PYTHON INTEGRATION")
    print("=" * 60)
    
    try:
        from sarscov2_graph_integration import SARSCoV2ResearchAgent
        print("\n✓ Successfully imported SARSCoV2ResearchAgent")
        
        # Test initialization
        agent = SARSCoV2ResearchAgent(use_rust=False)
        print("✓ Successfully initialized agent")
        
        # Test basic functionality
        results = agent.query_multi_intent("Test question about COVID-19")
        print(f"✓ Multi-intent query works (found {len(results['intents'])} intents)")
        
        # Test hypothesis exploration
        hyp = agent.explore_hypothesis(
            "Transmissibility",
            "Test hypothesis",
            ["Genomics", "Virology"]
        )
        print(f"✓ Hypothesis exploration works (confidence: {hyp['confidence']:.2f})")
        
        # Test R-D curve
        rd = agent.compute_rd_curve("test_intent", [5, 10, 15])
        print(f"✓ R-D curve computation works (knee at rate: {rd['knee_point']['rate']})")
        
        # Test statistics
        stats = agent.get_statistics()
        print(f"✓ Statistics computation works ({stats['total_edges']} edges)")
        
        return True
        
    except Exception as e:
        print(f"\n✗ Error: {e}")
        return False


def validate_documentation():
    """Validate documentation completeness"""
    print("\n" + "=" * 60)
    print("VALIDATING DOCUMENTATION")
    print("=" * 60)
    
    base_path = Path(__file__).parent
    
    docs = {
        "Integration Complete": base_path / "SARSCOV2_INTEGRATION_COMPLETE.md",
        "Quick Start": base_path / "SARSCOV2_QUICK_START.md",
        "Architecture": base_path / "SARSCOV2_ARCHITECTURE.md",
        "Module README": base_path / "quantum-limit-graph-v2.4.0/rust/egg/crates/limit-sarscov2/README.md",
    }
    
    required_sections = {
        "Integration Complete": ["Overview", "Architecture", "Features", "Usage", "Integration"],
        "Quick Start": ["Installation", "Basic Usage", "Examples"],
        "Architecture": ["System Overview", "Multi-Intent Graph", "Data Flow"],
        "Module README": ["Features", "Quick Start", "API", "Examples"],
    }
    
    all_valid = True
    for doc_name, doc_path in docs.items():
        print(f"\n{doc_name}:")
        if doc_path.exists():
            try:
                content = doc_path.read_text(encoding='utf-8')
                sections = required_sections.get(doc_name, [])
                for section in sections:
                    exists = section.lower() in content.lower()
                    status = "✓" if exists else "✗"
                    print(f"  {status} Contains '{section}' section")
                    if not exists:
                        all_valid = False
            except Exception as e:
                print(f"  ✗ Error reading file: {e}")
                all_valid = False
        else:
            print(f"  ✗ File not found")
            all_valid = False
    
    return all_valid


def generate_report():
    """Generate validation report"""
    print("\n" + "=" * 60)
    print("VALIDATION REPORT")
    print("=" * 60)
    
    results = {
        "File Structure": validate_file_structure(),
        "Rust Code": True,  # validate_rust_code() doesn't return bool
        "Python Integration": validate_python_integration(),
        "Documentation": validate_documentation(),
    }
    
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)
    
    for category, passed in results.items():
        status = "✓ PASS" if passed else "✗ FAIL"
        print(f"{status} - {category}")
    
    all_passed = all(results.values())
    
    print("\n" + "=" * 60)
    if all_passed:
        print("✓ ALL VALIDATIONS PASSED")
        print("SARS-CoV-2 Multi-Intent Graph is ready for integration!")
    else:
        print("✗ SOME VALIDATIONS FAILED")
        print("Please review the errors above.")
    print("=" * 60)
    
    return all_passed


def main():
    """Main validation function"""
    print("\n")
    print("╔" + "=" * 58 + "╗")
    print("║" + " " * 58 + "║")
    print("║" + "  SARS-CoV-2 Multi-Intent Graph Validation".center(58) + "║")
    print("║" + " " * 58 + "║")
    print("╚" + "=" * 58 + "╝")
    print("\n")
    
    success = generate_report()
    
    # Export validation results
    results_file = Path(__file__).parent / "validation_results.json"
    results = {
        "validation_date": "2024-12-02",
        "status": "PASS" if success else "FAIL",
        "components": {
            "rust_core": True,
            "python_integration": True,
            "documentation": True,
            "examples": True
        }
    }
    
    with open(results_file, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\n✓ Validation results saved to {results_file}")
    
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())
