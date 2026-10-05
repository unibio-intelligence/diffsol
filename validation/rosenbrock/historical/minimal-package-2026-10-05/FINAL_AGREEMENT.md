# Agreement for the reduced candidate

Candidate patch SHA-256: `f1fd8f0f5b3fa6be763532a9f919e5c1ad6f240e8e34b58d395ab8a0a8299dd5`. Rust measurements are fresh against the
minimal source. Julia comparisons use retained Julia 1.12.6 /
OrdinaryDiffEqRosenbrock 2.7.1 results; Julia was not rerun for this scope reset.

Ten fixed-step endpoints differ by at most 1.01655e-15;
five Rodas5P stability checks by at most 1.8735e-15.
Fourteen selected printed paper errors differ by at most
9.04648%. Printed values are
rounded errors, not exact trajectories. Table 8 is a higher-index diagnostic
outside the declared index-1 support. The 1000-state parabolic checks run in this
evidence driver, not in the unit suite, and retain the original order reduction.
The independent scalar stage oracle retains a maximum difference of 4.58e-16.

Adaptive controllers and meshes differ; retained adaptive differences are
comparisons, not global-accuracy certificates. A requested local tolerance does
not imply the same bound on global error. Rodas5P has fifth-order endpoints and
a fourth-order continuous extension; the degree-five polynomial dense diagnostic
retains approximately 0.417168 interior error. The original time-partial default
and Rk arithmetic are preserved here.

RC2's 35/40 expanded checks, corrected/endpoint circle experiments and PharmFlux
CLI results are excluded from this reduced candidate's qualification. They remain
in separately pinned extended/product records. Removing features does not make
those outcomes transferable to this source.
