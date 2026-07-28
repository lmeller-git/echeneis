Premise: we test for OBSTRUCTION freedom of spawned threads.

To show that a thread is obstruction free, we have to check that it terminates in finite time if run in isolation from any state reachable through any possible schedule of all active threads.
To do so in acceptable time, we use PCT to show that the thread is obstruction free within some probability bound.

Since an algorithm/ds may have a number of obstruction free AND non-obsturction free methods/functions, we allow spawning "checked" threads and blocking threads.
The only thing we check is that all "checked" threads are obstruction free, we do not care about termination of the complete algorithm.
While this CAN be used to also check for correctness of the "checked" threads across a number of schedules, this is NOT the primary use case for this and is better handled by shuttle.

Given this the algorithm follows:

Run the model (of N (potentially) blocking and M obstruction free threads) until some state S under PCT scheduling.
At some priority change point T, we freeze the model and run all "checked" threads to completion.
If any of these threaads does nto finish in bounded time, the test fails.

Running the "checked" threads to completion from S at T using there priorities at T should keep the statistical properties of PCT intact. (TODO validate this. Since we do not do any priority changes after T, this is not strictly PCT anymore. Might be worth going through the maths here.).

PCT reasoning:

The freeze point T is on a priority change point. Thus it is part of the schedule.
So if a schedule S ends in a freeze at T and we run threads M..M+k in order of priorities as determined by this schedule S, there exists another schedule S', where T lies at T', with the priorities of threads M..M+k orderd differently, but the state being identical between T and T'.
Further if a bug requires switching from M to M + l AFTER T, this is part of an entyirely different schedule and we must check both schedules to guarantee obstruction freedom, which we do probabilistically.

This implies that the statisitical properties of PCT survive. However we need to consider what happens after T:
We do NOT run all threads to completion given their current priority, instead discarding all blocking threads ans only driving the "checked" threads.
This also depends on wether we choose the LAST priority change point in S as freeze point, or any arbitrary one. In the latter case, pcts guarantees likely do not survive, because we only perform D priority chanegs with probability P instead of 1.
In the first case, since the definition of a bug is here ONLY violation of obsturction freedom (or failing asserts, ...) on the "checked" threads, this is equivalent to running all relevane threads to conclusion.


Implementation

Execution flow:

model F -> Runner -> drives Execution -> drives Scheduler.
thread::spawn in F -> spawn coro in Execution and add task to scheduler.
on yield: (atoimic/explcit, ...) -> currrent task handle yield -> behaviour differs by state/env. -> yields Execution to next scheduled coro
