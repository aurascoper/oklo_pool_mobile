# Constrained Dynamics and Evidentiary Scope Across Domains

Status: conceptual research note v1.1 review revision on a research-only branch. Separate from the frozen uranium-column specification and implementation PR. This note does not authorize changes to model equations, parameters, thresholds, or receipts.

## Abstract

Abstract ecological niche theory, prebiotic replication models, deep-subsurface geochemistry, and computational verification are not mechanistically equivalent. They nevertheless offer useful comparisons in how systems operate under resource constraints, preserve or transform information, and fail when specified conditions are violated. The analogy becomes testable when constraints, transitions, observables, and failure criteria are defined explicitly in each domain.

**Core rule:** a shared analytical structure does not establish a shared causal mechanism. Each domain retains its own state variables, equations, units, evidence standards, and failure definitions.

## 1. Revised cross-domain comparison matrix

| Analytical dimension | Deep-subsurface geochemistry | Ecological niche and adaptive perception | Prebiotic RNA replication and evolution | Computational verification and numerical modeling | Validation or falsification criterion |
| --- | --- | --- | --- | --- | --- |
| Resource balance | Quantified chemical fluxes, redox donors and acceptors, transport, and radiolytic production only where supported by site-specific evidence. | Resource availability, ecological interactions, and the relationship between environmental state and reproductive payoff; for perceptual models, sensory and action costs only when modeled. | Monomer supply, template abundance, replication resource consumption, and compartment constraints. | CPU time, memory, bandwidth, energy, quotas, and explicitly modeled buffers or retries. | Define a balance or budget over a stated interval. Test whether modeled consumption and losses can be supported by supply and allowed reserves. |
| State-transition rules | Declared reaction kinetics, transport equations, boundary conditions, and stoichiometric relationships. | Species interactions and persistence rules, or a declared mapping from observations to actions and fitness outcomes. | Per-site copying fidelity, mutation structure, relative fitness, and replication dynamics. | Typed data-flow edges, permitted state transitions, graph preflight, runtime assertions, numerical operators, and explicit exception handling. | Specify the rule and its test: chemical balance, ecological persistence, a sequence/fitness prediction, a rejected invalid edge, or a numerical invariant. |
| Failure detection | Measured or modeled loss of a target chemical state, reaction feasibility, or inability to sustain a stipulated metabolic regime. | Loss of persistence or reproductive success under a declared ecological model; perceptual strategies must be compared under the same environment and payoff schedule. | A defined error-threshold prediction, loss of a selected genotype or functional phenotype, or a predicted replicator-population transition. | Failed invariants, resource exhaustion, invalid graph edges, assertion failures, non-convergence, or incomplete evidence. | Predefine failure conditions, acceptance limits, positive and negative controls, and expected verdicts. Use controlled perturbations; do not infer a universal viability or perception law. |
| Evidence and scope | Site-specific geochemical assays, mineralogy, fluid inclusions, isotope data, and uncertainty-aware interpretations. | Field/experimental data or explicitly scoped evolutionary-game simulations, including the payoff matrix and assumptions. | In vitro replication and evolution experiments, copying-error measurements, sequence or phenotype tracking, and fitness estimates. | Artifact digests, source and environment identifiers, build/run provenance, test logs, conformance results, and independently checked attestations where available. | Separate measurement, model inference, simulation result, reported execution, independently reproduced result, and validation against real-world observations. |

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


## 7. Resource costs, information fidelity, and conditional "Goldilocks" hypotheses

A non-monotonic optimum is a **candidate model property**, not a law shared by every biological, ecological, and computational system. A convenient formal condition is a declared objective
\[
J(q) = B(q) - C(q),
\]
where q is an explicitly defined trait or fidelity parameter, B is its modeled benefit, and C its modeled cost. An interior optimum requires a point q* at which B'(q*) = C'(q*) and J''(q*) < 0, as well as feasibility within the allowed domain. If those conditions have not been established, use "trade-off hypothesis" rather than "Goldilocks law." Some systems instead have monotonic gains, thresholds, plateaus, multiple optima, or boundary optima.

### 7.1 Information bottleneck: the cost coefficient is not automatically energy

A standard information-bottleneck form is
\[
\max_{p(r\mid x)} \left[I(R;U) - \beta I(R;X)\right],
\]
where X is an observed/environmental state, R is an internal representation, and U is a decision-relevant outcome or utility variable. Under the usual graphical assumptions, R is generated from X and contains no additional information about U once X is given. I(R;U) measures retained relevance; I(R;X) is an information-complexity term. The multiplier beta controls the relevance/compression trade-off.

**Do not label beta as joules per bit, ATP cost per bit, or CPU cost per bit without an explicit physical or computational cost model linking that cost to the information measure.** Mutual information is measured in information units under a chosen logarithm convention; the conversion to metabolic energy or runtime is system-specific. A cost-aware extension can introduce a separately defined cost C(p(r|x)) or impose a resource constraint. The information-bottleneck formalism does not itself prove that objective, high-dimensional truth is metabolically unaffordable.

Hoffman and colleagues' "fitness beats truth" results are best reported as results within their specified evolutionary-game/perception model and assumptions, not as an unqualified universal theorem that selection always favors false perception. The literature contains substantive critique of the inference from fitness-payoff models to general perceptual theory. A research note should state the world-state model, payoff matrix, perceptual strategy class, complexity matching, and adaptation timescale before transferring the conclusion to an organism.

References:
- Hoffman, Singh and Prakash (2015), "The Interface Theory of Perception," Psychological Review: https://pubmed.ncbi.nlm.nih.gov/26384988/
- Prakash et al. (2021), "Fitness Beats Truth in the Evolution of Perception," Acta Biotheoretica: https://pubmed.ncbi.nlm.nih.gov/33231784/
- A critique of the generalization from fitness-payoff models to perception: https://pubmed.ncbi.nlm.nih.gov/26384986/
- Tishby, Pereira and Bialek, "The Information Bottleneck Method": https://www.cnbc.cmu.edu/~tai/readings/coding/tishby99information.pdf

### 7.2 Trajectory 1: stochastic RNA-replicator and compartment models

- **Construct:** information retention under mutation, selection, decay, and finite-compartment dynamics.
- **State variables:** sequence length L, per-copy error probability epsilon, fidelity Q_L, genotype/function counts per compartment, division and decay events, and spatially varying fields where the specification declares them.
- **Governing relation:** with independent, uniform per-site fidelity, Q_L = (1 - epsilon)^L is the probability that a length-L copy has no errors. A population-level error threshold additionally depends on the fitness landscape, mutation structure, population/compartment process, and selection regime.
- **Constraints:** the particular model's declared genome classes, finite compartment capacity, selection and discard rules, stochastic update order, spatial field, and initial conditions.
- **Observable:** persistence/extinction by genome class, functional-compartment fraction, measured threshold versus the preregistered bracket, and between-seed uncertainty.
- **Failure criterion:** a predefined persistence or threshold check fails, or the observed threshold falls outside the preregistered acceptance rule. A low mutation rate is not automatically "evolutionary stagnation"; adaptation or loss of adaptive capacity must be measured under an explicit changing-environment design.
- **Evidence class:** analytical result under stated assumptions; then simulation results and, separately, any in-vitro empirical evidence. Do not collapse these evidence classes.
- **Transfer limit:** a demonstration model with imposed spatial mutation/dose fields is not evidence that prebiotic RNA populations experienced that exact field or that the modeled threshold describes early Earth.

The quasispecies concept describes a mutation-selection distribution of related genotypes; it does not by itself establish a universally optimal intermediate mutation rate. The error threshold is landscape- and model-dependent. See Biebricher and Eigen (2005), https://pubmed.ncbi.nlm.nih.gov/15649558/, and the review "Quasispecies theory in the context of population genetics," https://pmc.ncbi.nlm.nih.gov/articles/PMC1208876/.

### 7.3 Trajectory 2: Cellular Potts/protocell models and the proposed trait optimum

- **Construct:** the effect of cell shape, volume, adhesion, motility, damage-linked terms, and potentially a costly protective trait on model dynamics.
- **State variables:** lattice labels/cell identity, Hamiltonian terms, delta-H for candidate moves, the effective CPM temperature parameter, declared trait variables, and separate growth/death observables if the model contains them.
- **Governing relation:** a common CPM rule accepts a move with P = min(1, exp(-delta-H/T_CPM)) for a positive effective temperature parameter, subject to the model's exact convention. The Hamiltonian governs move acceptance; it is not automatically biological fitness, Gibbs free energy, ATP expenditure, or reproductive success.
- **Constraints:** permitted lattice moves, the declared Hamiltonian, topology and volume constraints, parameter bounds, and the random-number/update contract.
- **Observable:** move acceptance and morphology for a pure CPM; survival, reproduction, and net growth only if separately implemented. A claimed trait optimum requires a specified objective and an independent sweep or derivation.
- **Failure criterion:** a predeclared numerical or morphological contract fails, or a predicted internal optimum is absent under the exact model and acceptance rule. Derive the objective before calling the optimum a biological optimum.
- **Evidence class:** model specification and simulation evidence; evidence of real biological adaptation requires separate calibration/validation.
- **Transfer limit:** do not conflate CPM with the individual-based Oklo Pool lattice. The latter's own spec describes parcels/individuals on sites, a diffusing concentration field, and explicitly no CPM Hamiltonian, adhesion, volume constraint, or cell identity. A hybrid system may contain both a field and lattice individuals without becoming a pure continuum PDE.

**Unverified formula warning:** the proposed closed form
\[
w^* = \frac{1}{c_M} - \frac{\beta_t}{\beta_s s_M}
\]
is not accepted here as a derived result. It must be re-derived from the exact objective and the exact growth/death expressions before inclusion as an analytical optimum. For example, under the much simpler assumptions p_g proportional to (1 - c_M w), p_d proportional to exp(-s_M w), and an objective that maximizes the ratio p_g/p_d, the interior stationary point would instead be w* = 1/c_M - 1/s_M, provided it is positive and lies inside the allowed interval. Adding a baseline death term or changing the objective changes the condition again. Thus the pasted beta-weighted form requires its own derivation, assumptions, dimensional check, boundary clamping, and numerical test. The melanin trait was described as deferred in the available Oklo-predictor identifiability plan, so do not present that extension as an already executed model result without a separate frozen specification.

### 7.4 Trajectory 3: numerical PDE verification, not "truth at infinite resolution"

- **Construct:** choose a numerical discretization that meets declared accuracy, boundedness, and resource requirements for a specified mathematical model.
- **State variables:** the model's continuum fields, spatial and temporal grid, discretization parameters, boundary conditions, numerical residuals, convergence metrics, resource use, and output hashes.
- **Governing relation:** the frozen synthetic uranium column uses its declared advection-diffusion-reaction equations, finite-volume face rules and Crank-Nicolson integration. Its grid and time-step limits are specific to that contract.
- **Constraints:** the pre-registered cell Péclet bound, Amendment A1 time-step selector, N1 concentration bounds, ledger-closure tolerance, spatial/temporal convergence bands, and declared runtime/environment.
- **Observable:** convergence orders, per-step bounds, ledger residuals, output-file hashes, and measured execution cost on a stated hardware/software environment.
- **Failure criterion:** any declared invariant, benchmark, hash reproduction, or acceptance threshold fails. For central differencing, a cell Péclet number above 2 can lead to loss of monotonicity and nonphysical oscillations; it should not be described as a universal instability theorem for every solver.
- **Evidence class:** mathematical analysis of the discrete scheme, unit tests and benchmark results, recorded execution receipts, and independently reproduced runs. These are distinct layers of evidence.
- **Transfer limit:** grid convergence establishes behavior of the discretization for the specified equations, not empirical correctness of the model or parameters. A finer mesh cannot repair a wrong equation, missing mechanism, or unsupported parameter.

Under a fixed domain and a specified numerical method, refinement typically increases computational work while reducing discretization error over a convergence regime. The rate depends on the time-step rule, solver and dimensionality; roundoff and model error need not increase monotonically with refinement. Therefore, the numerical "Goldilocks" problem is normally a constrained accuracy-cost/Pareto problem, not automatically an inverted-U fitness curve. In the frozen uranium-column contract, the grid values and tolerance bands are pre-registered engineering criteria, not a universal optimum for all reactive-transport models.

The Oklo Pool predictor is not the same model class as the uranium column: its spec describes a stochastic individual-based lattice with a separate diffusing concentration field. The predictor-identifiability relation between parameters such as alpha and eta is a model-specific identifiability issue, not a generic property of continuum PDEs. Jackson's Maxwell equations form another distinct continuous-field model class; shared boundary-value techniques do not make its state variables or governing physics equivalent to neutron transport or uranium reactive transport.

### 7.5 Cross-trajectory conclusion

Use "Goldilocks" only when the objective, cost function, admissible set, and interior-optimum condition are explicit. For RNA models the objective may be persistence or adaptive performance under mutation; for CPM it may be a declared morphology/dynamics observable or a separate growth-fitness function; for numerical solvers it is usually error or uncertainty subject to compute and validation constraints. Their mathematical shapes may be compared, but their state variables, causal mechanisms, and success criteria are not interchangeable.

## 8. Source and claim discipline for this extension

For every numerical or theoretical statement added from a model artifact, record whether it is:
- a frozen assumption,
- an analytic result under named assumptions,
- a simulation measurement with seed/run protocol,
- an empirical observation,
- a literature claim that has been read at full text or only in abstract,
- or a proposed hypothesis awaiting a test.

Do not transfer a result from the demonstration Oklo Pool lattice into the frozen uranium-column implementation. Do not promote a closed-form optimum, runtime guarantee, or ecological/perceptual theory from a pasted synthesis unless its derivation, conditions, and evidence are independently available. No part of this section modifies the uranium-column v1 specification, Amendment A1, parameter manifest, solver code, thresholds, or receipts.
