//! Core ontology model types.
//!
//! These types represent the UOR Foundation ontology vocabulary as typed Rust
//! data. All instances are built as owned `Vec`s and referenced via borrows.
//! The top-level entry point is [`Ontology::full()`](crate::Ontology::full).

/// Kernel/user/bridge classification for each namespace module.
///
/// - `Kernel`: compiled into ROM; immutable; `u/`, `schema/`, `op/`
/// - `User`: parameterizable at runtime; `type/`, `state/`, `morphism/`
/// - `Bridge`: kernel-computed, user-consumed; all remaining namespaces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    /// Immutable kernel-space: compiled into ROM.
    Kernel,
    /// Parameterizable user-space: runtime declarations.
    User,
    /// Bridge: kernel-computed, user-consumed.
    Bridge,
}

impl Space {
    /// Returns the string value used in the `uor:space` annotation.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Space::Kernel => "kernel",
            Space::User => "user",
            Space::Bridge => "bridge",
        }
    }
}

/// A UOR Foundation namespace (e.g., `u/`, `schema/`, `op/`).
#[derive(Debug, Clone)]
pub struct Namespace {
    /// The prefix used in the `@context` (e.g., `"u"`).
    pub prefix: &'static str,
    /// The full IRI of the namespace (e.g., `"https://uor.foundation/u/"`).
    pub iri: &'static str,
    /// Human-readable label.
    pub label: &'static str,
    /// Description of the namespace.
    pub comment: &'static str,
    /// Kernel/user/bridge classification (Amendment 8).
    pub space: Space,
    /// Full IRIs of imported namespaces (`owl:imports`).
    pub imports: &'static [&'static str],
}

/// An OWL class definition.
#[derive(Debug, Clone)]
pub struct Class {
    /// Full IRI (e.g., `"https://uor.foundation/u/Address"`).
    pub id: &'static str,
    /// Human-readable label.
    pub label: &'static str,
    /// Description.
    pub comment: &'static str,
    /// Full IRIs of parent classes (`rdfs:subClassOf`).
    pub subclass_of: &'static [&'static str],
    /// Full IRIs of mutually exclusive classes (`owl:disjointWith`).
    pub disjoint_with: &'static [&'static str],
}

/// Whether a property is a datatype, object, or annotation property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyKind {
    /// `owl:DatatypeProperty` — relates a resource to an XSD literal.
    Datatype,
    /// `owl:ObjectProperty` — relates two resources.
    Object,
    /// `owl:AnnotationProperty` — used for documentation; not for reasoning.
    Annotation,
}

/// An OWL property definition.
#[derive(Debug, Clone)]
pub struct Property {
    /// Full IRI.
    pub id: &'static str,
    /// Human-readable label.
    pub label: &'static str,
    /// Description.
    pub comment: &'static str,
    /// Datatype, object, or annotation property.
    pub kind: PropertyKind,
    /// Whether this is also an `owl:FunctionalProperty`.
    pub functional: bool,
    /// Full IRI of the domain class, or `None` if unspecified.
    pub domain: Option<&'static str>,
    /// Full IRI of the range class or XSD datatype.
    pub range: &'static str,
}

/// A value in a named individual's property assertion.
#[derive(Debug, Clone)]
pub enum IndividualValue {
    /// A plain string literal.
    Str(&'static str),
    /// An integer literal.
    Int(i64),
    /// A boolean literal.
    Bool(bool),
    /// An IRI reference to another resource.
    IriRef(&'static str),
    /// An ordered `rdf:List` of IRI references (used for `op:composedOf`).
    List(&'static [&'static str]),
}

/// A named individual (OWL `owl:NamedIndividual`).
#[derive(Debug, Clone)]
pub struct Individual {
    /// Full IRI.
    pub id: &'static str,
    /// Full IRI of the class this individual is an instance of.
    pub type_: &'static str,
    /// Human-readable label.
    pub label: &'static str,
    /// Description.
    pub comment: &'static str,
    /// Property assertions: pairs of (property IRI, value).
    pub properties: &'static [(&'static str, IndividualValue)],
}

/// A complete namespace module: namespace metadata + classes + properties + individuals.
#[derive(Debug, Clone)]
pub struct NamespaceModule {
    /// Namespace metadata.
    pub namespace: Namespace,
    /// All OWL classes defined in this namespace.
    pub classes: Vec<Class>,
    /// All OWL properties defined in this namespace.
    pub properties: Vec<Property>,
    /// All named individuals declared in this namespace.
    pub individuals: Vec<Individual>,
}

/// An annotation property defined at the ontology root level.
#[derive(Debug, Clone)]
pub struct AnnotationProperty {
    /// Full IRI.
    pub id: &'static str,
    /// Human-readable label.
    pub label: &'static str,
    /// Description.
    pub comment: &'static str,
    /// Full IRI of the range (typically `xsd:string`).
    pub range: &'static str,
}

/// The complete UOR Foundation ontology.
#[derive(Debug)]
pub struct Ontology {
    /// Ontology version (e.g., `"1.0.0"`).
    pub version: &'static str,
    /// Base IRI of the ontology (e.g., `"https://uor.foundation/"`).
    pub base_iri: &'static str,
    /// All 14 namespace modules in dependency order.
    pub namespaces: Vec<NamespaceModule>,
    /// Root-level annotation properties (Amendment 8: `uor:space`).
    pub annotation_properties: Vec<AnnotationProperty>,
}

impl Ontology {
    /// Looks up a class by its full IRI. Returns `None` if not found.
    #[must_use]
    pub fn find_class(&self, iri: &str) -> Option<&Class> {
        self.namespaces
            .iter()
            .flat_map(|m| m.classes.iter())
            .find(|c| c.id == iri)
    }

    /// Looks up a property by its full IRI. Returns `None` if not found.
    #[must_use]
    pub fn find_property(&self, iri: &str) -> Option<&Property> {
        self.namespaces
            .iter()
            .flat_map(|m| m.properties.iter())
            .find(|p| p.id == iri)
    }

    /// Looks up a named individual by its full IRI. Returns `None` if not found.
    #[must_use]
    pub fn find_individual(&self, iri: &str) -> Option<&Individual> {
        self.namespaces
            .iter()
            .flat_map(|m| m.individuals.iter())
            .find(|i| i.id == iri)
    }

    /// Returns the total number of classes across all namespaces.
    #[must_use]
    pub fn class_count(&self) -> usize {
        self.namespaces.iter().map(|m| m.classes.len()).sum()
    }

    /// Returns the total number of properties across all namespaces.
    ///
    /// Includes the ontology-level `uor:space` annotation property (Amendment 8)
    /// which is defined outside any specific namespace module.
    #[must_use]
    pub fn property_count(&self) -> usize {
        // +1 for the global uor:space annotation property (Amendment 8)
        self.namespaces
            .iter()
            .map(|m| m.properties.len())
            .sum::<usize>()
            + 1
    }

    /// Returns the total number of named individuals across all namespaces.
    #[must_use]
    pub fn individual_count(&self) -> usize {
        self.namespaces.iter().map(|m| m.individuals.len()).sum()
    }
}

/// Returns the `uor:space` annotation property (Amendment 8).
#[must_use]
pub fn annotation_space_property() -> AnnotationProperty {
    AnnotationProperty {
        id: "https://uor.foundation/space",
        label: "space",
        comment: "Whether this module belongs to kernel-space (immutable, compiled), \
                  user-space (parameterizable, runtime), or bridge (kernel-computed, \
                  user-consumed). Values: 'kernel', 'user', 'bridge'.",
        range: "http://www.w3.org/2001/XMLSchema#string",
    }
}

/// Standard IRI constants used across all namespace modules.
pub mod iris {
    /// OWL namespace.
    pub const OWL: &str = "http://www.w3.org/2002/07/owl#";
    /// RDF namespace.
    pub const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
    /// RDFS namespace.
    pub const RDFS: &str = "http://www.w3.org/2000/01/rdf-schema#";
    /// XSD namespace.
    pub const XSD: &str = "http://www.w3.org/2001/XMLSchema#";
    /// SHACL namespace.
    pub const SH: &str = "http://www.w3.org/ns/shacl#";

    /// UOR Foundation base IRI.
    pub const UOR: &str = "https://uor.foundation/";

    // Namespace IRIs
    /// Content addressing namespace.
    pub const NS_U: &str = "https://uor.foundation/u/";
    /// Core schema namespace.
    pub const NS_SCHEMA: &str = "https://uor.foundation/schema/";
    /// Operations namespace.
    pub const NS_OP: &str = "https://uor.foundation/op/";
    /// Query namespace.
    pub const NS_QUERY: &str = "https://uor.foundation/query/";
    /// Resolver namespace.
    pub const NS_RESOLVER: &str = "https://uor.foundation/resolver/";
    /// Type system namespace.
    pub const NS_TYPE: &str = "https://uor.foundation/type/";
    /// Partition namespace.
    pub const NS_PARTITION: &str = "https://uor.foundation/partition/";
    /// Observable namespace.
    pub const NS_OBSERVABLE: &str = "https://uor.foundation/observable/";
    /// Proof namespace.
    pub const NS_PROOF: &str = "https://uor.foundation/proof/";
    /// Derivation namespace.
    pub const NS_DERIVATION: &str = "https://uor.foundation/derivation/";
    /// Trace namespace.
    pub const NS_TRACE: &str = "https://uor.foundation/trace/";
    /// Certificate namespace.
    pub const NS_CERT: &str = "https://uor.foundation/cert/";
    /// Morphism namespace.
    pub const NS_MORPHISM: &str = "https://uor.foundation/morphism/";
    /// State namespace.
    pub const NS_STATE: &str = "https://uor.foundation/state/";

    // XSD datatypes
    /// `xsd:string`.
    pub const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";
    /// `xsd:integer`.
    pub const XSD_INTEGER: &str = "http://www.w3.org/2001/XMLSchema#integer";
    /// `xsd:positiveInteger`.
    pub const XSD_POSITIVE_INTEGER: &str = "http://www.w3.org/2001/XMLSchema#positiveInteger";
    /// `xsd:nonNegativeInteger`.
    pub const XSD_NON_NEGATIVE_INTEGER: &str =
        "http://www.w3.org/2001/XMLSchema#nonNegativeInteger";
    /// `xsd:boolean`.
    pub const XSD_BOOLEAN: &str = "http://www.w3.org/2001/XMLSchema#boolean";
    /// `xsd:decimal`.
    pub const XSD_DECIMAL: &str = "http://www.w3.org/2001/XMLSchema#decimal";
    /// `xsd:dateTime`.
    pub const XSD_DATETIME: &str = "http://www.w3.org/2001/XMLSchema#dateTime";
    /// `owl:Thing`.
    pub const OWL_THING: &str = "http://www.w3.org/2002/07/owl#Thing";
    /// `owl:Class`.
    pub const OWL_CLASS: &str = "http://www.w3.org/2002/07/owl#Class";
    /// `rdf:List`.
    pub const RDF_LIST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#List";
}
