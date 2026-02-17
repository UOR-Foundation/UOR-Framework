# Resolution

## Definition

**Resolution** is the process of computing the canonical representation of an object
in the ring substrate. The {@class https://uor.foundation/resolver/Resolver} hierarchy
implements this process.

## Resolver Hierarchy

| Class | Role |
|-------|------|
| {@class https://uor.foundation/resolver/Resolver} | Abstract resolver base |
| {@class https://uor.foundation/resolver/DihedralFactorizationResolver} | Factorizes in D_{2^n} |
| {@class https://uor.foundation/resolver/CanonicalFormResolver} | Computes canonical form |
| {@class https://uor.foundation/resolver/EvaluationResolver} | Evaluates the canonical form |

## Resolution Process

1. A {@class https://uor.foundation/query/Query} specifies what to resolve
2. The resolver uses its strategy ({@prop https://uor.foundation/resolver/strategy})
3. The input type ({@prop https://uor.foundation/resolver/inputType}) is the source
4. Resolution produces a {@class https://uor.foundation/partition/Partition}
   (output type is `partition:Partition` via {@prop https://uor.foundation/resolver/outputType})

## Query Types

Three specialized queries correspond to the three resolver strategies:

| Query | Description |
|-------|-------------|
| {@class https://uor.foundation/query/CoordinateQuery} | Resolves spatial coordinates |
| {@class https://uor.foundation/query/MetricQuery} | Resolves metric properties |
| {@class https://uor.foundation/query/RepresentationQuery} | Resolves canonical representation |

## Complexity

The property {@prop https://uor.foundation/resolver/complexity} declares the
computational complexity of the resolver (e.g., `"O(n)"` or `"O(log n)"`).

## Output

The final output of resolution is:
- A {@class https://uor.foundation/partition/Partition} of R_n
- A {@class https://uor.foundation/trace/ComputationTrace} recording the steps
- Optionally, a {@class https://uor.foundation/cert/Certificate} attesting the result
