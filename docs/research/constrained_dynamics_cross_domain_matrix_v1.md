# Constrained Dynamics and Evidentiary Scope Across Domains

Status: conceptual research note v1. Separate from the frozen uranium-column specification and implementation PR. This note does not authorize changes to model equations, parameters, thresholds, or receipts.

## Abstract

Abstract ecological niche theory, prebiotic replication models, deep-subsurface geochemistry, and computational verification are not mechanistically equivalent. They nevertheless offer useful comparisons in how systems operate under resource constraints, preserve or transform information, and fail when specified conditions are violated. The analogy becomes testable when constraints, transitions, observables, and failure criteria are defined explicitly in each domain.

**Core rule:** a shared analytical structure does not establish a shared causal mechanism. Each domain retains its own state variables, equations, units, evidence standards, and failure definitions.

## 1. Revised cross-domain comparison matrix

| Analytical dimension | Deep-subsurface geochemistry | Prebiotic RNA replication and evolution | Computational verification and pipeline engineering | Validation or falsification criterion |
| --- | --- | --- | --- | --- |
| Resource balance | Quantified chemical fluxes, redox donors and acceptors, transport, and radiolytic production only where supported by site-specific evidence. | Monomer supply, template abundance, replication resource consumption, and compartment constraints. | CPU time, memory, bandwidth, energy, quotas, and explicitly modeled buffers or retries. | Define a balance over a stated interval. Test whether modeled consumption and losses can be supported by supply and allowed stored reserves. |
| State-transition rules | Declared reaction kinetics, transport equations, boundary conditions, and stoichiometric relationships. | Per-site copying fidelity, mutation structure, relative fitness, and replication dynamics. | Typed data-flow edges, permitted state transitions, graph preflight, runtime assertions, and explicit exception handling. | Specify the invariant and its test: conservation residual, predicted sequence distribution or fitness behavior, or an intentionally invalid connection that must be rejected. |
| Failure detection | Measured or modeled loss of a target chemical state, reaction feasibility, or inability to sustain a stipulated metabolic regime. | A defined error-threshold prediction, loss of a selected genotype or functional phenotype, or predicted replicator-population transition. | Failed invariants, resource exhaustion, invalid graph edges, assertion failures, and incomplete evidence. | Predefine failure conditions, acceptance limits, positive and negative controls, and expected verdicts. Demonstrate detection using controlled perturbations; do not infer a universal viability limit. |
| Evidence and scope | Site-specific geochemical assays, mineralogy, fluid inclusions, isotope data, and uncertainty-aware interpretations. | In vitro replication and evolution experiments, copying-error measurements, sequence or phenotype tracking, and fitness estimates. | Artifact digests, source and environment identifiers, build/run provenance, test logs, conformance results, and independently checked attestations where available. | Separate measurement, model inference, reported execution, independently reproduced results, and validation against real-world observations. |

## 2. Standard schema for every proposed correspondence

Every analogy included in a future revision must identify:

1. **Construct:** the concept being compared.
2. **State variables:** quantities describing each system.
3. **Governing relation:** the mathematical or empirically supported relationship.
4. **Constraints:** bounds on permitted states or transitions.
5. **Observable:** quantities that can actually be measured or audited.
6. **Failure criterion:** a result that falsifies the model or invalidates the run.
7. **Evidence class:** measurement, model inference, assumption, hypothesis, or software attestation.
8. **Transfer limit:** where the analogy breaks down and no cross-domain inference is justified.

A comparison without an explicit observable or failure criterion remains a heuristic, not a validated correspondence.

## 3. Domain-specific cautions

### 3.1 Geochemistry and radiolysis

Water radiolysis can generate hydrogen and oxidizing products; their net yields and persistence depend on radiation, water chemistry, mineral surfaces, and subsequent reactions. Studies of Oklo reactor zones report mineralogical and fluid-inclusion evidence consistent with radiolysis affecting local redox conditions. This does not establish a microbial guild at Oklo, nor does it make hydrogen, oxidants, and sulfate interchangeable energy quantities.

The following sources are distinct in evidentiary role:

- Dubessy et al. (1988), “Radiolysis evidenced by H2-O2 and H2-bearing fluid inclusions in three uranium deposits,” DOI: https://doi.org/10.1016/0016-7037(88)90269-4. The paper discusses several possible pathways and does not make every source mechanism uniquely identifiable in the Oklo case.
- Savary and Pagel (1997), “The effects of water radiolysis on local redox conditions in the Oklo, Gabon, natural fission reactors 10 and 16,” DOI: https://doi.org/10.1016/S0016-7037(97)00261-5. The study interprets fluid inclusions and mineralogy as evidence of radiolysis and describes organic matter consuming oxidizing products in some zones.
- Sauvage et al. (2021), “The contribution of water radiolysis to marine sedimentary life,” DOI: https://doi.org/10.1038/s41467-021-21218-z. This supports a discussion of radiolytic products in marine sediments, not direct evidence for microbial activity at Oklo.

Do not insert these mechanisms into the uranium-column v1 simply because they are relevant to a broader comparison. The frozen synthetic column explicitly excludes radiolysis, heat and biology.

### 3.2 RNA replication and error thresholds

Under the simplifying assumption of independent, uniform per-site copying fidelity, the probability of an error-free copy of length L is:

Q_L = (1 - epsilon)^L

where epsilon is the per-site copying-error probability. This expression alone is not a complete error-catastrophe model. Error thresholds also depend on relative replication fitness, mutation structure, and replication dynamics.

Do not place microbial radiation-survival metrics such as D10 into the RNA copying-fidelity column. Use directly relevant quantities: copying accuracy, mutation distributions, genotype or phenotype persistence, and replication fitness.

### 3.3 Resource power and viability

No universal cell-specific maintenance-power limit is asserted here. Any numeric claim must state the organism or population, environment, measured or modeled quantity, assumptions, uncertainty, and source. A modeled shortfall against an assumed power requirement challenges that model; it does not alone demonstrate immediate cell death.

### 3.4 Computational verification and receipts

A verification receipt is an evidence record, not a universal guarantee. When artifact digests, provenance claims, and test results are bound to trusted inputs and independently verified, they can support bounded claims about artifact integrity, execution provenance, and implementation conformance. They do not establish that scientific assumptions are correct or predictions generalize.

Relevant background: SLSA provenance specification, https://slsa.dev/spec/v1.0/provenance. Trust in provenance depends on the builder and how the attestation is produced and verified. A receipt must be interpreted according to its actual signed or hashed fields and the checks an independent verifier performs.

## 4. Five-level claim hierarchy

The following is a proposed analysis aid, not an external certification standard:

1. **Artifact integrity:** checked artifacts match recorded digests.
2. **Execution provenance:** claims about code, inputs, settings, and environment are bound to an identified run and trusted attestations.
3. **Implementation conformance:** declared tests establish specified invariants, benchmarks, and controls within stated tolerances.
4. **Scientific validity:** model equations, parameters, and assumptions adequately represent the intended real system.
5. **Predictive validity:** the model predicts independent observations in its intended domain.

These levels do not follow automatically from one another. A digest alone does not establish code correctness; a benchmark does not validate empirical parameters; and passing implementation tests does not show that a model predicts nature.

## 5. Operational checklist

Before accepting a proposed cross-domain correspondence, record:

- the exact variable and its units in each domain;
- the governing relationship or explicit statement that none is asserted;
- the system boundary and initial/boundary conditions;
- the measured observable and evidence provenance;
- a predefined failure threshold or counterexample;
- positive and negative controls where applicable;
- the scope of the conclusion and what remains untested.

## 6. Explicit non-claims

This note does not claim that:

- CPU cycles are physically equivalent to electron-donor flux;
- software receipts perform the same mechanism as biochemical error correction;
- ecological realized niches are simply maximum throughput after subtracting competition;
- observed subsurface radiolysis proves life at Oklo;
- a successful synthetic uranium-column test validates Oklo geology;
- a common mathematical pattern makes two domains causally interchangeable.

The intended contribution is methodological: compare constrained systems using explicit variables, rules, observables, and evidentiary limits while preserving each domain's mechanisms and standards of proof.
