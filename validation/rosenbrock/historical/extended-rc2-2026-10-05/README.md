# Rosenbrock numerical validation

UniBio Intelligence's validation evidence for the generic DiffSol Rosenbrock contribution.

Candidate source: `d5cd2690187b3195fa2ecc9aa168c4e0edb7552d`; base: `bc6256a49e16779e77b58b786f0a993ad4242b63`. Most table measurements were collected at `7dffe2d`; `paper-final-stdout.log` records the later paper runner, and `checks/` contains the October 4 repeated regression gates on the final candidate. These provenance lanes are distinct. The detailed Julia and parabolic comparisons were not rerun on October 4.

The table records expected values, observations, tolerances and diagnostic overruns. [Machine-readable rows](numerical-evidence.json), [Julia trajectories](julia-trajectories.csv), [Julia output](julia-stdout.log), [paper runner output](paper-final-stdout.log), [parabolic output](parabolic-stdout.log), and [Julia parabolic output](julia-parabolic-stdout.log) are included. Local filesystem paths in logs were redacted; numerical rows and comparison tolerances were retained.

| Item | Reference / quantity | Expected | Observed | Tolerance / status |
|---|---|---|---|---|
| B1 | OrdinaryDiffEqRosenbrock 2.7.1, Rodas5PTableau; A, C (including implied zero column), c, d, gamma, H-to-beta | 0 ulp | 0 ulp; bitwise fixture test passes | 0 ulp; pass |
| B5 | Steinebach 2023 Table 6; Endpoint max error at h=0.25 | 1.26e-09 | 1.25556e-09 | 20% of published rounded error; pass |
| B5 | Steinebach 2023 Table 6; Endpoint max error at h=0.125 | 1.47e-10 | 1.47473e-10 | 20% of published rounded error; pass |
| B3 | Steinebach 2023 §4 Problem 2; Endpoint error at t=2; accepted steps (paper forcing, stiffness 1e5) | 10-12 exp(-2) | {"error":5.57395907208047e-10,"steps":19} | <1e-7; pass |
| B5 | Steinebach 2023 Table 6; Endpoint max error at h=0.0625 | 1.78e-11 | 1.78222e-11 | 20% of published rounded error; pass |
| B4 | Steinebach 2023 Table 5; Endpoint max error at h=0.125 | 2.93e-08 | 2.92814e-08 | 20% of published rounded error; pass |
| B7 | Steinebach 2023 Table 8; Endpoint max error at h=0.03125 | 9e-05 | 9.00146e-05 | 20% of published rounded error; pass |
| B2 | Steinebach 2023 §4 Problem 1; Endpoint component errors at t=4, rtol=1e-7 | ln(4); ln(4)/4 | [7.482647834677891e-10,7.03506142230026e-11] | <1e-7; pass |
| B4 | Steinebach 2023 Table 5; Endpoint max error at h=0.0625 | 8.56e-10 | 8.56236e-10 | 20% of published rounded error; pass |
| B5 | Steinebach 2023 Table 6; Endpoint max error at h=0.03125 | 2.17e-12 | 2.11031e-12 | 20% of published rounded error; pass |
| B4 | Steinebach 2023 Table 5; Endpoint max error at h=0.03125 | 2.59e-11 | 2.59217e-11 | 20% of published rounded error; pass |
| B7 | Steinebach 2023 Table 8; Endpoint max error at h=0.015625 | 2.33e-05 | 2.3341e-05 | 20% of published rounded error; pass |
| B7 | Steinebach 2023 Table 8; Endpoint max error at h=0.0078125 | 5.94e-06 | 5.93874e-06 | 20% of published rounded error; pass |
| B4 | Steinebach 2023 Table 5; Observed endpoint orders | 5 | [5.095834070501863,5.04577569055821] | ±0.25; pass |
| B5 | Steinebach 2023 Table 6; Observed endpoint orders | 3 | [3.089812025547019,3.0487058765417014,3.078146314148503] | ±0.2; pass |
| B7 | Steinebach 2023 Table 8; Observed endpoint orders | 2 | [1.9472949214296815,1.974636531589852] | ±0.2; pass |
| B6/B13 | Steinebach 2023 Table 7; Lang–Verwer; Rang–Angermann; 1000-state nonlinear parabolic endpoint max error at h=0.03125 | 5.97e-09 | 5.9709e-09 | 25% of published rounded error; pass |
| B6/B13 | Steinebach 2023 Table 7; Lang–Verwer; Rang–Angermann; 1000-state nonlinear parabolic endpoint max error at h=0.015625 | 4.72e-10 | 4.72999e-10 | 25% of published rounded error; pass |
| B6/B13 | Steinebach 2023 Table 7; Lang–Verwer; Rang–Angermann; 1000-state nonlinear parabolic endpoint max error at h=0.0078125 | 3.45e-11 | 3.47629e-11 | 25% of published rounded error; pass |
| B6/B13 | Steinebach 2023 Table 7; Lang–Verwer; Rang–Angermann; 1000-state nonlinear parabolic endpoint max error at h=0.00390625 | 2.36e-12 | 2.5735e-12 | 25% of published rounded error; pass |
| B6/B13 | Steinebach 2023 Table 7; Parabolic observed orders (first three step sizes) | approximately 4 | [3.658038342331991,3.766219075015856] | approximately 4–5; pass |
| B8 | Steinebach 2023 §4 Problem 6 / Table 9; Polynomial DAE degree 1: max dense, endpoint and endpoint derivative errors | 0 for degrees 1–4; degree 5 dense/derivative not exact | {"dense":2.90878432451791e-14,"endpoint":1.199040866595169e-14,"derivative":1.6586731987899839e-13} | 1e-9 for degrees 1–4; endpoint only for degree 5; pass |
| B8 | Steinebach 2023 §4 Problem 6 / Table 9; Polynomial DAE degree 2: max dense, endpoint and endpoint derivative errors | 0 for degrees 1–4; degree 5 dense/derivative not exact | {"dense":1.021405182655144e-14,"endpoint":1.687538997430238e-14,"derivative":1.6564527527407336e-13} | 1e-9 for degrees 1–4; endpoint only for degree 5; pass |
| B8 | Steinebach 2023 §4 Problem 6 / Table 9; Polynomial DAE degree 3: max dense, endpoint and endpoint derivative errors | 0 for degrees 1–4; degree 5 dense/derivative not exact | {"dense":1.554312234475219e-14,"endpoint":0.0,"derivative":1.9539925233402755e-14} | 1e-9 for degrees 1–4; endpoint only for degree 5; pass |
| B8 | Steinebach 2023 §4 Problem 6 / Table 9; Polynomial DAE degree 4: max dense, endpoint and endpoint derivative errors | 0 for degrees 1–4; degree 5 dense/derivative not exact | {"dense":3.108624468950438e-14,"endpoint":3.552713678800501e-15,"derivative":4.263256414560601e-14} | 1e-9 for degrees 1–4; endpoint only for degree 5; pass |
| B8 | Steinebach 2023 §4 Problem 6 / Table 9; Polynomial DAE degree 5: max dense, endpoint and endpoint derivative errors | 0 for degrees 1–4; degree 5 dense/derivative not exact | {"dense":0.41716771835949995,"endpoint":1.9184653865522705e-13,"derivative":0.1338500899133237} | 1e-9 for degrees 1–4; endpoint only for degree 5; diagnostic |
| B9 | Hairer–Wanner IV.7; Julia Rodas5P(); R(-1.0) | 0.36788 | 0.36788 | absolute 1e-13; pass |
| B9 | Hairer–Wanner IV.7; Julia Rodas5P(); R(-10.0) | -0.040373 | -0.040373 | absolute 1e-13; pass |
| B9 | Hairer–Wanner IV.7; Julia Rodas5P(); R(-1000.0) | -0.0120529 | -0.0120529 | absolute 1e-13; pass |
| B9 | Hairer–Wanner IV.7; Julia Rodas5P(); R(-100000000.0) | -1.25209e-07 | -1.25209e-07 | absolute 1e-13; pass |
| B9 | Hairer–Wanner IV.7; Julia Rodas5P(); R(-100000000000000.0) | -1.25209e-13 | -1.25209e-13 | absolute 1e-13; pass |
| B9 | Steinebach 2023; Hairer–Wanner IV.7; L-stable limit at z=-1e14 | 0 | -1.25209e-13 | absolute <1e-10; pass |
| B11 | OrdinaryDiffEqRosenbrock 2.7.1; rodas5p decay: fixed 10 steps h=0.01 (native default central f_t) | [0.9048374180359586] | {"endpoint":[0.9048374180359589],"max_relative_error":2.453972398787416e-16} | relative 1e-12; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rodas5p decay: adaptive rtol=1e-06 | [0.3678794411721777] | {"endpoint":[0.36787944362272557],"max_error_vs_bdf":2.4505478757319565e-09,"error_over_rtol":0.0024505478757319565,"max_error_vs_julia":2.393545472934022e-09,"native_steps":5,"native_rejects":0,"julia_steps":9,"julia_rejects":0} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rodas5p decay: adaptive rtol=1e-09 | [0.3678794411721777] | {"endpoint":[0.3678794411726229],"max_error_vs_bdf":4.4519943287468777e-13,"error_over_rtol":0.0004451994328746877,"max_error_vs_julia":1.0355605262191148e-12,"native_steps":14,"native_rejects":0,"julia_steps":21,"julia_rejects":0} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B11 | OrdinaryDiffEqRosenbrock 2.7.1; rosenbrock23 decay: fixed 10 steps h=0.01 (native default central f_t) | [0.904837051782804] | {"endpoint":[0.904837051782804],"max_relative_error":0.0} | relative 1e-12; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rosenbrock23 decay: adaptive rtol=1e-06 | [0.3678794411721777] | {"endpoint":[0.36786901732486527],"max_error_vs_bdf":1.0423847312424073e-05,"error_over_rtol":10.423847312424073,"max_error_vs_julia":3.0002289458930242e-06,"native_steps":38,"native_rejects":1,"julia_steps":49,"julia_rejects":0} | 10*rtol absolute diagnostic; step counts unasserted; global-error-overrun |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rosenbrock23 decay: adaptive rtol=1e-09 | [0.3678794411721777] | {"endpoint":[0.3678793375718914],"max_error_vs_bdf":1.0360028629419205e-07,"error_over_rtol":103.60028629419205,"max_error_vs_julia":2.4191710412679868e-08,"native_steps":379,"native_rejects":3,"julia_steps":437,"julia_rejects":0} | 10*rtol absolute diagnostic; step counts unasserted; global-error-overrun |
| B11 | OrdinaryDiffEqRosenbrock 2.7.1; rodas5p robertson: fixed 10 steps h=0.001 (native default central f_t) | [0.9996006841629056,3.6450479186134964e-05,0.0003628653579085962] | {"endpoint":[0.9996006841629056,3.645047918613495e-05,0.000362865357908598],"max_relative_error":5.079414865746527e-15} | relative 1e-12; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rodas5p robertson: adaptive rtol=1e-06 | [0.10730042855133996,4.800166973234571e-07,0.8926990914319547] | {"endpoint":[0.10730034673959674,4.800162880723715e-07,0.8926991732441141],"max_error_vs_bdf":8.18121593804122e-08,"error_over_rtol":0.0818121593804122,"max_error_vs_julia":6.576279920622596e-08,"native_steps":66,"native_rejects":5,"julia_steps":97,"julia_rejects":2} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rodas5p robertson: adaptive rtol=1e-09 | [0.10730042855133996,4.800166973234571e-07,0.8926990914319547] | {"endpoint":[0.10730042832695336,4.800166962008385e-07,0.8926990916563485],"max_error_vs_bdf":2.2439383684513814e-10,"error_over_rtol":0.2243938368451381,"max_error_vs_julia":1.592103254788313e-10,"native_steps":220,"native_rejects":13,"julia_steps":304,"julia_rejects":2} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B11 | OrdinaryDiffEqRosenbrock 2.7.1; rosenbrock23 robertson: fixed 10 steps h=0.001 (native default central f_t) | [0.9996006819320089,3.645047866463391e-05,0.0003628675893263426] | {"endpoint":[0.999600681932009,3.64504786646339e-05,0.0003628675893263428],"max_relative_error":4.481809085643224e-16} | relative 1e-12; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rosenbrock23 robertson: adaptive rtol=1e-06 | [0.10730042855133996,4.800166973234571e-07,0.8926990914319547] | {"endpoint":[0.10729693829487816,4.799958987658614e-07,0.8927025817092221],"max_error_vs_bdf":3.490277267426123e-06,"error_over_rtol":3.490277267426123,"max_error_vs_julia":1.0393389183738222e-06,"native_steps":146,"native_rejects":7,"julia_steps":185,"julia_rejects":0} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rosenbrock23 robertson: adaptive rtol=1e-09 | [0.10730042855133996,4.800166973234571e-07,0.8926990914319547] | {"endpoint":[0.10730039643748795,4.800165282211474e-07,0.8926991235459877],"max_error_vs_bdf":3.211403298841731e-08,"error_over_rtol":32.11403298841731,"max_error_vs_julia":7.578513927519737e-09,"native_steps":2095,"native_rejects":8,"julia_steps":2502,"julia_rejects":0} | 10*rtol absolute diagnostic; step counts unasserted; global-error-overrun |
| B11 | OrdinaryDiffEqRosenbrock 2.7.1; rodas5p vdp: fixed 10 steps h=0.00001 (native default central f_t) | [1.9999999909292843,-0.0001727878527461639] | {"endpoint":[1.9999999909292843,-0.0001727878527461634],"max_relative_error":2.8236416499441033e-15} | relative 1e-12; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rodas5p vdp: adaptive rtol=1e-06 | [1.999333370506298,-0.0006670371231732628] | {"endpoint":[1.999333370506309,-0.0006670371231732533],"max_error_vs_bdf":1.1102230246251565e-14,"error_over_rtol":1.1102230246251565e-08,"max_error_vs_julia":4.440892098500626e-16,"native_steps":19,"native_rejects":5,"julia_steps":21,"julia_rejects":3} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rodas5p vdp: adaptive rtol=1e-09 | [1.999333370506298,-0.0006670371231732628] | {"endpoint":[1.9993333705063119,-0.0006670371231732369],"max_error_vs_bdf":1.3988810110276972e-14,"error_over_rtol":1.398881011027697e-05,"max_error_vs_julia":3.3306690738754696e-15,"native_steps":44,"native_rejects":5,"julia_steps":56,"julia_rejects":3} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B11 | OrdinaryDiffEqRosenbrock 2.7.1; rosenbrock23 vdp: fixed 10 steps h=0.00001 (native default central f_t) | [1.9999999909310873,-0.00017279326041364008] | {"endpoint":[1.999999990931087,-0.00017279326041364008],"max_relative_error":1.1102230296594144e-16} | relative 1e-12; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rosenbrock23 vdp: adaptive rtol=1e-06 | [1.999333370506298,-0.0006670371231732628] | {"endpoint":[1.9993333705077168,-0.000667037115401829],"max_error_vs_bdf":7.771433778620829e-12,"error_over_rtol":7.771433778620829e-06,"max_error_vs_julia":4.056754931980322e-13,"native_steps":52,"native_rejects":8,"julia_steps":63,"julia_rejects":0} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rosenbrock23 vdp: adaptive rtol=1e-09 | [1.999333370506298,-0.0006670371231732628] | {"endpoint":[1.9993333705067078,-0.0006670371224692729],"max_error_vs_bdf":7.03989926249815e-13,"error_over_rtol":0.000703989926249815,"max_error_vs_julia":2.398081733190338e-13,"native_steps":421,"native_rejects":10,"julia_steps":490,"julia_rejects":0} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B12 | OrdinaryDiffEqRosenbrock 2.7.1; rodas5p cosine: fixed 10 steps h=0.01 (native default central f_t) | [0.09983341664682904] | {"endpoint":[0.09983341664682915],"max_relative_error":1.1120755573784321e-15} | relative 1e-08; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rodas5p cosine: adaptive rtol=1e-06 | [-0.5440211109717309] | {"endpoint":[-0.5440210064924478],"max_error_vs_bdf":1.0447928311396737e-07,"error_over_rtol":0.10447928311396737,"max_error_vs_julia":6.748459213667957e-08,"native_steps":50,"native_rejects":16,"julia_steps":51,"julia_rejects":7} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rodas5p cosine: adaptive rtol=1e-09 | [-0.5440211109717309] | {"endpoint":[-0.5440211106223769],"max_error_vs_bdf":3.4935399018110047e-10,"error_over_rtol":0.34935399018110047,"max_error_vs_julia":1.2685796857425657e-10,"native_steps":157,"native_rejects":12,"julia_steps":180,"julia_rejects":20} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B12 | OrdinaryDiffEqRosenbrock 2.7.1; rosenbrock23 cosine: fixed 10 steps h=0.01 (native default central f_t) | [0.09983383262061077] | {"endpoint":[0.09983383262061077],"max_relative_error":0.0} | relative 1e-08; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rosenbrock23 cosine: adaptive rtol=1e-06 | [-0.5440211109717309] | {"endpoint":[-0.5440304306855244],"max_error_vs_bdf":9.319713793476403e-06,"error_over_rtol":9.319713793476403,"max_error_vs_julia":9.126343377241497e-06,"native_steps":453,"native_rejects":17,"julia_steps":3687,"julia_rejects":7} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rosenbrock23 cosine: adaptive rtol=1e-09 | [-0.5440211109717309] | {"endpoint":[-0.5440211814643665],"max_error_vs_bdf":7.049263561764718e-08,"error_over_rtol":70.49263561764718,"max_error_vs_julia":7.039139893194601e-08,"native_steps":4431,"native_rejects":9,"julia_steps":114856,"julia_rejects":40} | 10*rtol absolute diagnostic; step counts unasserted; global-error-overrun |
| B12 | OrdinaryDiffEqRosenbrock 2.7.1; rodas5p pr: fixed 10 steps h=0.001 (native default central f_t) | [0.009999833334166675] | {"endpoint":[0.009999833334166939],"max_relative_error":2.6368236303255147e-14} | relative 1e-08; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rodas5p pr: adaptive rtol=1e-06 | [0.8414709848081828] | {"endpoint":[0.8414709716026956],"max_error_vs_bdf":1.3205487259249082e-08,"error_over_rtol":0.013205487259249082,"max_error_vs_julia":1.3079377580993423e-08,"native_steps":15,"native_rejects":0,"julia_steps":16,"julia_rejects":0} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rodas5p pr: adaptive rtol=1e-09 | [0.8414709848081828] | {"endpoint":[0.841470984811659],"max_error_vs_bdf":3.4762193124038276e-12,"error_over_rtol":0.0034762193124038276,"max_error_vs_julia":3.238853629738969e-12,"native_steps":41,"native_rejects":0,"julia_steps":56,"julia_rejects":0} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B12 | OrdinaryDiffEqRosenbrock 2.7.1; rosenbrock23 pr: fixed 10 steps h=0.001 (native default central f_t) | [0.009999834105815892] | {"endpoint":[0.009999834105816908],"max_relative_error":1.0165648211415686e-13} | relative 1e-08; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rosenbrock23 pr: adaptive rtol=1e-06 | [0.8414709848081828] | {"endpoint":[0.8414711717311948],"max_error_vs_bdf":1.869230119577736e-07,"error_over_rtol":0.1869230119577736,"max_error_vs_julia":1.8645794430671714e-07,"native_steps":403,"native_rejects":0,"julia_steps":26647,"julia_rejects":1} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B10/B14 | Exact solution / diffsol BDF at rtol=1e-12; Julia 2.7.1; rosenbrock23 pr: adaptive rtol=1e-09 | [0.8414709848081828] | {"endpoint":[0.8414709876794138],"max_error_vs_bdf":2.8712309196876618e-09,"error_over_rtol":2.8712309196876618,"max_error_vs_julia":2.8710605004533818e-09,"native_steps":4659,"native_rejects":0,"julia_steps":846812,"julia_rejects":1} | 10*rtol absolute diagnostic; step counts unasserted; pass |
| B12/B14 | Julia 2.7.1 with analytic tgrad; Rust test-only analytic kernel; rodas5p, lambda=0, 10 fixed steps h=0.01 | 0.0998334 | {"endpoint":0.0998334166468292,"absolute_error":1.5265566588595902e-16} | 1e-13 absolute; pass |
| B12/B14 | Julia 2.7.1 with analytic tgrad; Rust test-only analytic kernel; rodas5p, lambda=-1000, 10 fixed steps h=0.001 | 0.00999983 | {"endpoint":0.009999833334166678,"absolute_error":3.469446951953614e-18} | 1e-13 absolute; pass |
| B12/B14 | Julia 2.7.1 with analytic tgrad; Rust test-only analytic kernel; rosenbrock23, lambda=0, 10 fixed steps h=0.01 | 0.0998338 | {"endpoint":0.09983383262061077,"absolute_error":0.0} | 1e-13 absolute; pass |
| B12/B14 | Julia 2.7.1 with analytic tgrad; Rust test-only analytic kernel; rosenbrock23, lambda=-1000, 10 fixed steps h=0.001 | 0.00999983 | {"endpoint":0.00999983410581589,"absolute_error":1.734723475976807e-18} | 1e-13 absolute; pass |


## Interpretation and limits

The paper's actual Problem 2 is `y' = -1e5*(y-g(t))+g'(t)`, `g(t)=10-(10+t)*exp(-t)` on `[0,2]`; Table 6 reports order three. The separate sine-forced problem with stiffness 1000 is included in the Julia protocol. The nonlinear, nonzero-boundary parabolic example in Table 7 has mild order reduction: measured native orders are about 3.66 and 3.77, consistent with the paper, while Julia Rodas5 has orders about 1.85 and 1.93. “No order reduction” would overstate this result. Only this parabolic example was reproduced; Lang–Verwer and Rang–Angermann provide context for the order-preserving design conditions.

Stiff accuracy in the transformed representation is `b = A[7,:]+e8`. Literal equality with the last A row would contradict strict lower triangularity. The eighth increment corrects the last stage state. The unit test checks the transformed identity and the L-stable limit. Julia omits the eighth, identically zero column of C; the fixture checks all stored coefficients and that implied column. H is checked through its exact forward transformation to beta, rather than an ill-conditioned inverse of rounded sums.

`NonLinearOpTimePartial` has `impl<T: NonLinearOp> NonLinearOpTimePartial for T {}`. Rust coherence prevents an analytic override on an existing RHS. This is an upstream API limitation, not a new configurable hook. Production retains central differences with `delta=(1+|t|)*sqrt(epsilon)`. A test-only helper supplies analytic `f_t` to the **same** generic stage kernel for fixed nonautonomous comparisons; its checked-in tests isolate the method without changing that API or claiming a supported override. Public native runs use the actual central difference. No one-sided probes or forcing-discontinuity handling were added.

Adaptive step counts are observations, not pass/fail parity checks. Rodas5P stays within the requested `10*rtol` absolute endpoint diagnostic across the five examples. Rosenbrock23 has a second-order main solution and a cubic embedded difference; its local tolerance does not promise global endpoint error `10*rtol`. The explicit overruns for decay, Robertson and cosine at tight tolerance are retained above, rather than changing the norm/controller. Julia also exceeds that global threshold on decay at rtol=1e-9. Its much higher nonautonomous step counts additionally reflect the unscaled `h*f_t` in the third stage. With `gamma*h*f_t`, the Rust estimator retains its cubic local order. Current cosine-at-10 counts at rtol=1e-6 are Rust 453/17 rejected versus Julia 3687/7 rejected; these replace the older, differently configured 466/4013 measurement.

The BDF reference is an independent numerical check, not exact ground truth. Decay, cosine and sine Prothero–Robinson also have exact solutions. Van der Pol here covers the initial layer and endpoint at t=1 with mu=1000, not a full long relaxation oscillation.

Integrated outputs use five Gauss-node evaluations plus the endpoint evaluation needed to update `dg`. The dense output polynomial is the **integral of a degree-four Lagrange interpolant** (thus degree five), not a degree-four integral. The guaranteed output order is limited by the state extension: four for Rodas5P, two for Rosenbrock23. The output estimator compares Gauss-3 with Gauss-2, with local error order five. Forward, adjoint and output sensitivities remain outside this candidate scope.

## Rosenbrock23 derivation

Use `K1=h*k1`, `K2=h*k2`, `K3=h*k3` for the direct derivative stages, and `L=I-gamma*h*J`, `q=6+sqrt(2)`. Reused values satisfy `f0=L*k1-gamma*h*ft` and `fmid=L*k2+gamma*h*J*k1`. Substitution into stage three gives

```text
L*K1 = h*f0 + gamma*h^2*ft
L*K2 = h*fmid - gamma*h*J*K1
L*K3 = h*f1 + gamma*h*J*((q-2)*K1-q*K2) - gamma*h^2*ft
alpha = [0,0,0; 1/2,0,0; 0,1,0]
Gamma = gamma*[1,0,0; -1,1,0; q-2,-q,1]
Gamma^-1 = (1/gamma)*[1,0,0; 1,1,0; 2,q,1]
u = Gamma*K
A = alpha*Gamma^-1 = (1/gamma)*[0,0,0; 1/2,0,0; 1,1,0]
C = I/gamma-Gamma^-1 = -(1/gamma)*[0,0,0; 1,0,0; 2,q,0]
b = [0,1,0]*Gamma^-1 = [1,1,0]/gamma
error = [1,-2,1]*Gamma^-1/6 = [1,q-2,1]/(6*gamma)
time = Gamma*1 = [gamma,0,-gamma]
c = [0,1/2,1]
```

The strictly lower triangular C and common positive diagonal gamma meet the existing kernel's contract. No method-specific branches are needed. Stage three affects the error estimate only. The quadratic `ntrp23s` extension becomes `y(theta)=y0+(theta/gamma)*u1+theta*(theta-2*gamma)/(gamma*(1-2*gamma))*u2`, with a zero beta row for u3. Direct scalar oracles (source `2a255e1`, matched analytic time partial) check both the endpoint and embedded difference to `1e-14`; the autonomous oracle also runs through the public `step` path.


## Reproduce

Use the candidate DiffSol checkout, not PharmFlux's vendored concrete methods. `run-rust.py` takes a caller-supplied checkout and uses the included locked dependency environment; its offline mode requires the dependencies to be cached. From this validation directory:

```sh
python run-rust.py --diffsol /path/to/diffsol --kind native
python run-rust.py --diffsol /path/to/diffsol --kind paper
python run-rust.py --diffsol /path/to/diffsol --kind parabolic
python run-rust.py --diffsol /path/to/diffsol --kind properties
julia --startup-file=no --project=. -e 'using Pkg; Pkg.instantiate()'
julia --startup-file=no --project=. julia-reference.jl
julia --startup-file=no --project=. julia-parabolic.jl
```

The Julia environment pins OrdinaryDiffEqRosenbrock 2.7.1 and records Julia 1.12.6. Fresh default/nalgebra/faer gate logs contain 304 unit tests and four doctests; Cranelift contains 335 and four. Feature flags do not prove exclusive backend isolation. Steinebach Tables 5, 6 and 8 use fixed-step oracles; Table 7 covers the 1000-state parabolic example; Table 9 covers polynomial dense output. Index-2 diagnostics do not extend the supported API beyond index-1 DAEs.

## Provenance and licensing

Ubi-authored evidence code and documentation are Apache-2.0; [LICENSE-APACHE](LICENSE-APACHE) applies. Upstream DiffSol and SciML components retain MIT terms and their own notices. [NOTICE](NOTICE) records the boundary. No publisher PDF, figures, source packet or manuscript prose is included. Reproduced scientific methods and numerical outcomes do not establish permission for unrelated third-party material or clinical use.

References: [Steinebach (2023)](https://doi.org/10.1007/s10543-023-00967-x), [Hairer and Wanner II](https://doi.org/10.1007/978-3-642-05221-7), [Shampine and Reichelt](https://doi.org/10.1137/S1064827594276424), [Lang and Verwer](https://doi.org/10.1023/A:1021900219772), [Rang and Angermann](https://doi.org/10.1007/s10543-005-0035-y).

## Current shared implementation and accuracy improvements

Earlier observations above are historical. The complete uncommitted candidate
has a [fresh final agreement report](accuracy-2026-10-04/FINAL_AGREEMENT.md),
[numerical comparison](accuracy-2026-10-04/final-agreement.json) and
[accuracy details](accuracy-2026-10-04/README.md). The new patch/source hashes
identify the current measurements; historical source/head values identify only
the earlier measurements. Publication and Fable review remain pending.
