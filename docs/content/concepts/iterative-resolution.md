# Iterative Resolution

## Definition

**Iterative resolution** extends the resolution process into a learning loop.
Rather than computing a partition in a single pass, the resolver proceeds
iteratively: each iteration applies a constraint, pins fibers, and checks
whether the budget is closed. The process converges when all fibers are pinned.

## Resolution State

A {@class https://uor.foundation/resolver/ResolutionState} tracks the progress
of an iterative resolution:

| Property | Range | Description |
|----------|-------|-------------|
| {@prop https://uor.foundation/resolver/isComplete} | xsd:boolean | True when all fibers are pinned |
| {@prop https://uor.foundation/resolver/iterationCount} | xsd:nonNegativeInteger | Iterations performed so far |
| {@prop https://uor.foundation/resolver/fiberDeficit} | FiberBudget | Remaining unpinned fibers |
| {@prop https://uor.foundation/resolver/convergenceRate} | xsd:decimal | Fibers pinned per iteration |

A {@class https://uor.foundation/resolver/Resolver} links to its state via
{@prop https://uor.foundation/resolver/resolutionState}.

## Refinement Suggestions

When the resolution is incomplete, the resolver produces
{@class https://uor.foundation/resolver/RefinementSuggestion} objects that
guide the next iteration:

| Property | Range | Description |
|----------|-------|-------------|
| {@prop https://uor.foundation/resolver/suggestedAxis} | MetricAxis | Which metric axis to explore |
| {@prop https://uor.foundation/resolver/suggestedClass} | owl:Class | Which constraint class to apply |
| {@prop https://uor.foundation/resolver/targetFibers} | FiberCoordinate | Which fibers to target |

Suggestions are linked from the state via
{@prop https://uor.foundation/resolver/suggestion}.

## Refinement Steps

Each iteration produces a {@class https://uor.foundation/derivation/RefinementStep}
that records the type-level change:

| Property | Range | Description |
|----------|-------|-------------|
| {@prop https://uor.foundation/derivation/previousType} | TypeDefinition | Type before this step |
| {@prop https://uor.foundation/derivation/appliedConstraint} | Constraint | Constraint applied |
| {@prop https://uor.foundation/derivation/refinedType} | TypeDefinition | Type after this step |
| {@prop https://uor.foundation/derivation/fibersClosed} | xsd:nonNegativeInteger | Fibers pinned in this step |

{@class https://uor.foundation/derivation/RefinementStep} is a subclass of
{@class https://uor.foundation/derivation/DerivationStep}, parallel to
{@class https://uor.foundation/derivation/RewriteStep} for term-level rewriting.

## Convergence

The resolution loop terminates when:
1. {@prop https://uor.foundation/resolver/isComplete} is `true`
2. {@prop https://uor.foundation/partition/isClosed} is `true` on the fiber budget
3. {@prop https://uor.foundation/resolver/fiberDeficit} shows zero free fibers

The {@prop https://uor.foundation/resolver/convergenceRate} tracks the rate of
progress, enabling early detection of stalled resolutions.

## Example: Three-Iteration Resolution

```
Iteration 0: 0/8 fibers pinned  (convergenceRate = 0.0)
Iteration 1: apply ResidueConstraint → 3/8 pinned (rate = 3.0)
Iteration 2: apply CarryConstraint   → 6/8 pinned (rate = 3.0)
Iteration 3: apply DepthConstraint   → 8/8 pinned (rate = 2.67, complete)
```
