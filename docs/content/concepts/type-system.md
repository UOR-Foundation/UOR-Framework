# Type System

## Definition

The UOR type system provides a structured way to classify objects in the ring.
The base class is {@class https://uor.foundation/type/TypeDefinition}.

## Type Hierarchy

| Class | Description |
|-------|-------------|
| {@class https://uor.foundation/type/TypeDefinition} | Base type |
| {@class https://uor.foundation/type/PrimitiveType} | Atomic type (e.g., u8, u16) |
| {@class https://uor.foundation/type/ProductType} | Cartesian product of types |
| {@class https://uor.foundation/type/SumType} | Discriminated union of types |
| {@class https://uor.foundation/type/ConstrainedType} | Type with additional constraints |

## Properties

| Property | Domain | Range | Description |
|----------|--------|-------|-------------|
| {@prop https://uor.foundation/type/bitWidth} | PrimitiveType | xsd:nonNegativeInteger | Bit width |
| {@prop https://uor.foundation/type/component} | ProductType | TypeDefinition | Component types |
| {@prop https://uor.foundation/type/baseType} | SumType/ConstrainedType | TypeDefinition | Base type |
| {@prop https://uor.foundation/type/constraint} | ConstrainedType | xsd:string | Constraint expression |
| {@prop https://uor.foundation/type/contentAddress} | TypeDefinition | u:Address | Content address |

## Example: Primitive Types

```turtle
<https://uor.foundation/instance/type-u8>
    a               type:PrimitiveType ;
    type:bitWidth   "8"^^xsd:nonNegativeInteger .

<https://uor.foundation/instance/type-u64>
    a               type:PrimitiveType ;
    type:bitWidth   "64"^^xsd:nonNegativeInteger .
```

## Integration with State

Types are used in {@class https://uor.foundation/state/Binding} to record the
type of bound values:

```turtle
<https://uor.foundation/instance/binding-x>
    a               state:Binding ;
    state:boundType <https://uor.foundation/instance/type-u8> .
```

## Integration with Partition

Types serve as the source for partition computations via
{@prop https://uor.foundation/partition/sourceType}.
