# Jackson's Classical Electrodynamics: Reactor-Adjacent Methods and Scope

Status: technical scoping note v1. This is a study and method-selection map, not a facility-specific shielding design, safety analysis, or authorization to alter any reactor model.

## 1. Purpose

Jackson's Classical Electrodynamics is a graduate-level reference for classical electromagnetic field theory, boundary-value problems, wave propagation, radiation, and selected charged-particle energy-loss processes. It provides rigorous derivations and mathematical methods. It is not a complete nuclear-data library or a stand-alone reactor-shielding workflow.

Official publisher table of contents for the third edition: https://www.wiley.com/en-us/Classical%2BElectrodynamics%2B3rd%2BEdition-p-9780471309321

## 2. Chapter map (third edition)

- **Chapters 2-3:** electrostatic boundary-value problems and solution techniques. Useful for understanding Green-function methods, separation of variables, and geometry-dependent field solutions.
- **Chapter 6:** Maxwell equations, macroscopic electromagnetism, and conservation laws.
- **Chapter 8:** waveguides, resonant cavities, and optical fibers.
- **Chapter 9:** radiating systems, multipole fields, and radiation.
- **Chapter 10:** scattering and diffraction.
- **Chapter 13:** collisions, energy loss, scattering of charged particles, Cherenkov radiation, and transition radiation.
- **Chapter 14:** radiation by moving charges.
- **Chapter 15:** bremsstrahlung, method of virtual quanta, and radiative beta processes.

The chapter numbering above follows the third edition. Check the edition in hand before using chapter or equation references.

## 3. Reactor-adjacent applications and their limits

### 3.1 Charged-particle energy loss and stopping power

Jackson provides theoretical background for selected charged-particle energy-loss mechanisms. For practical stopping-power and range values, use validated tabulations and applicable interaction models rather than treating the textbook derivation as the complete engineering data source.

NIST's ESTAR/PSTAR/ASTAR resources calculate stopping-power and range tables for electrons, protons, and helium ions in specified materials:
- https://www.nist.gov/pml/stopping-power-range-tables-electrons-protons-and-helium-ions
- ESTAR: https://physics.nist.gov/PhysRefData/Star/Text/ESTAR.html

A calculation must state particle species, energy range, material composition and density, relevant processes, uncertainty, and the intended observable. Stopping power is not, by itself, a validated shielding design.

### 3.2 Cherenkov radiation

Cherenkov radiation occurs when a charged particle travels through a medium faster than the phase velocity of light in that medium. The University of Missouri Research Reactor describes the blue glow around its core as Cherenkov radiation:
- https://www.murr.missouri.edu/sixty-years-later-missouris-bold-investment-in-nuclear-science-continues-to-save-and-improve-lives/

Jackson's treatment is relevant for understanding the electrodynamics and spectral/angular characteristics under specified assumptions. Quantitative instrumentation predictions still require suitable material optical properties, source spectra, geometry, and detector response.

### 3.3 Bremsstrahlung and secondary photons

Jackson's Chapter 15 supplies theoretical background for radiation emitted during charged-particle deceleration and related processes. In practical shielding, secondary photon production, attenuation, scattering, geometry, buildup, dose quantities, and applicable standards must all be considered. No material-thickness or safety conclusion should be inferred from the textbook alone.

### 3.4 Nuclear cross sections and neutron transport

Do not describe neutron-reaction cross sections as quantities derived from classical electrodynamics. Neutron transport requires nuclear interaction data and suitable transport approximations or solvers. Evaluated nuclear data libraries, not Jackson, are the appropriate source for the cross sections and reaction data used by such calculations.

IAEA Evaluated Nuclear Data File (ENDF) portal:
https://nucleus.iaea.org/Pages/evaluated-nuclear-data-file.aspx

A diffusion model, a neutron transport solution, and an electromagnetic boundary-value problem may share some mathematical tools, such as eigenfunction expansions or boundary-condition handling. That does not make their governing equations, state variables, constitutive relationships, or validity conditions identical.

## 4. Suggested verification structure

For any future calculation, record:

1. **Question and quantity of interest:** e.g. field, spectrum, energy deposition, particle range, or dose-related observable.
2. **Model class:** electrostatic/magnetostatic, classical radiation, charged-particle stopping, photon transport, neutron diffusion, or neutron transport.
3. **Governing equations and assumptions:** include approximation regime and units.
4. **Geometry and boundary conditions:** define material interfaces and sources.
5. **Inputs and evidence:** distinguish measured material properties, evaluated data, assumptions, and fitted quantities.
6. **Verification:** analytic limiting cases, unit checks, conservation checks, benchmark comparisons, and numerical convergence as appropriate.
7. **Validation:** comparison against independent experiments or qualified reference calculations.
8. **Transfer limits:** explicitly name what the model does not cover.

## 5. Connection to constrained-dynamics research

Jackson belongs in a separate technical track from the conceptual cross-domain matrix. The defensible connection is methodological: explicit equations, boundary conditions, conservation laws, and verification. It is not an argument that continuous electromagnetic fields, nuclear transport, ecological niches, RNA replication, and software verification share one causal mechanism.

## 6. Scope warning

This note is not a MURR-specific calculation and does not replace facility procedures, qualified shielding analysis, nuclear-data validation, or expert safety review. It does not introduce radiation transport, Cherenkov emission, or heat transfer into the frozen synthetic uranium-column v1.
