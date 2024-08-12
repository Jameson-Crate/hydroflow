use core::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use variadics::{var_expr, var_type, CloneVariadic, EitherRefVariadic, SplitBySuffix, VariadicExt};

use crate::ght::{GeneralizedHashTrie, GeneralizedHashTrieNode, GhtInner, GhtLeaf, GHT};
use crate::{IsBot, IsTop, LatticeBimorphism, LatticeOrd, Merge};

//////////////////////////
/// Lattice traits for GHT
//////////////////////////

/// Merge
impl<KeyType, ValType, Inner> Merge<GHT<KeyType, ValType, Inner>> for GHT<KeyType, ValType, Inner>
where
    KeyType: VariadicExt,
    ValType: VariadicExt,
    Inner: GeneralizedHashTrieNode + Merge<Inner>,
{
    fn merge(&mut self, other: GHT<KeyType, ValType, Inner>) -> bool {
        self.trie.merge(other.trie)
    }
}

impl<Head, Node> Merge<GhtInner<Head, Node>> for GhtInner<Head, Node>
where
    Node: GeneralizedHashTrieNode + Merge<Node> + Clone,
    Self: GeneralizedHashTrieNode,
    Head: Hash + Eq + Clone,
{
    fn merge(&mut self, other: GhtInner<Head, Node>) -> bool {
        let mut changed = false;

        for (k, v) in other.children.into_iter() {
            match self.children.entry(k) {
                std::collections::hash_map::Entry::Occupied(mut occupied) => {
                    changed |= occupied.get_mut().merge(v)
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(v);
                    changed = true
                }
            }
        }
        changed
    }
}

impl<Schema, SuffixSchema> Merge<GhtLeaf<Schema, SuffixSchema>> for GhtLeaf<Schema, SuffixSchema>
where
    Schema: Eq + Hash,
{
    fn merge(&mut self, other: GhtLeaf<Schema, SuffixSchema>) -> bool {
        let old_len = self.elements.len();
        self.elements.extend(other.elements);
        self.elements.len() > old_len
    }
}

/// PartialEq
impl<KeyType, ValType, Inner> PartialEq<GHT<KeyType, ValType, Inner>>
    for GHT<KeyType, ValType, Inner>
where
    KeyType: VariadicExt,
    ValType: VariadicExt,
    Inner: GeneralizedHashTrieNode + Merge<Inner> + PartialEq,
{
    fn eq(&self, other: &GHT<KeyType, ValType, Inner>) -> bool {
        self.trie.eq(&other.trie)
    }
}

impl<Head, Node> PartialEq<GhtInner<Head, Node>> for GhtInner<Head, Node>
where
    Head: Hash + Eq + 'static + Clone,
    Node: GeneralizedHashTrieNode + 'static + PartialEq,
    Node::Schema: SplitBySuffix<var_type!(Head, ...Node::SuffixSchema)>,
{
    fn eq(&self, other: &GhtInner<Head, Node>) -> bool {
        if self.children.len() != other.children.len() {
            return false;
        }

        for head in self.iter() {
            if other.get(head).is_none() {
                return false;
            }
            if !self.get(head).unwrap().eq(other.get(head).unwrap()) {
                return false;
            }
        }
        true
    }
}

// PartialOrd
impl<KeyType, ValType, Inner> PartialOrd<GHT<KeyType, ValType, Inner>>
    for GHT<KeyType, ValType, Inner>
where
    KeyType: VariadicExt, // + AsRefVariadicPartialEq
    ValType: VariadicExt, // + AsRefVariadicPartialEq
    Inner: GeneralizedHashTrieNode + Merge<Inner> + PartialOrd,
{
    fn partial_cmp(&self, other: &GHT<KeyType, ValType, Inner>) -> Option<Ordering> {
        self.trie.partial_cmp(&other.trie)
    }
}

impl<Head, Node> PartialOrd<GhtInner<Head, Node>> for GhtInner<Head, Node>
where
    Head: Hash + Eq + 'static + Clone,
    Node: 'static + GeneralizedHashTrieNode + PartialEq + PartialOrd,
    Node::Schema: SplitBySuffix<var_type!(Head, ...Node::SuffixSchema)>,
{
    fn partial_cmp(&self, other: &GhtInner<Head, Node>) -> Option<Ordering> {
        if self.children.is_empty() && other.children.is_empty() {
            Some(Equal)
        } else {
            // across all keys, determine if we have stuff in self that's not in other
            // and vice versa
            let (selfonly, mut otheronly) =
                self.children
                    .iter()
                    .fold((false, false), |mut accum, (k, v)| {
                        if !other.children.contains_key(k) {
                            accum.0 = true; // selfonly is true
                            accum
                        } else {
                            match v.partial_cmp(other.children.get(k).unwrap()) {
                                Some(Greater) | None => {
                                    accum.0 = true; // selfonly is true
                                    accum
                                }
                                Some(Less) => {
                                    accum.1 = true; // otheronly is true
                                    accum
                                }
                                Some(Equal) => accum, // no changes
                            }
                        }
                    });
            // now check if other has keys that are missing in self
            otheronly |= !other.children.keys().all(|k| self.children.contains_key(k));

            if selfonly && otheronly {
                // unique stuff on both sides: order is incomparable
                None
            } else if selfonly && !otheronly {
                // unique stuff only in self
                Some(Greater)
            } else if !selfonly && otheronly {
                // unique stuff only in other
                Some(Less)
            } else {
                // nothing unique on either side
                Some(Equal)
            }
        }
    }
}

impl<Schema, SuffixSchema> PartialOrd<GhtLeaf<Schema, SuffixSchema>>
    for GhtLeaf<Schema, SuffixSchema>
where
    Schema: Eq + Hash,
    SuffixSchema: Eq + Hash,
{
    fn partial_cmp(&self, other: &GhtLeaf<Schema, SuffixSchema>) -> Option<Ordering> {
        match self.elements.len().cmp(&other.elements.len()) {
            Greater => {
                if other
                    .elements
                    .iter()
                    .all(|head| self.elements.contains(head))
                {
                    Some(Greater)
                } else {
                    None
                }
            }
            Equal => {
                if self
                    .elements
                    .iter()
                    .all(|head| other.elements.contains(head))
                {
                    Some(Equal)
                } else {
                    None
                }
            }
            Less => {
                if self
                    .elements
                    .iter()
                    .all(|head| other.elements.contains(head))
                {
                    Some(Less)
                } else {
                    None
                }
            }
        }
    }
}

// LatticeOrd
impl<KeyType, ValType, Inner> LatticeOrd<GHT<KeyType, ValType, Inner>>
    for GHT<KeyType, ValType, Inner>
where
    KeyType: VariadicExt, // + AsRefVariadicPartialEq
    ValType: VariadicExt, // + AsRefVariadicPartialEq
    Inner: GeneralizedHashTrieNode,
    Self: PartialOrd<GHT<KeyType, ValType, Inner>>,
{
}

impl<Head, Node> LatticeOrd<GhtInner<Head, Node>> for GhtInner<Head, Node>
where
    Self: PartialOrd<GhtInner<Head, Node>>,
    Head: Clone,
    Node: GeneralizedHashTrieNode,
{
}
impl<Schema, SuffixSchema> LatticeOrd<GhtLeaf<Schema, SuffixSchema>>
    for GhtLeaf<Schema, SuffixSchema>
where
    Schema: Eq + Hash,
    SuffixSchema: Eq + Hash,
{
}

// IsBot
impl<KeyType, ValType, Inner> IsBot for GHT<KeyType, ValType, Inner>
where
    KeyType: VariadicExt, // + AsRefVariadicPartialEq
    ValType: VariadicExt, // + AsRefVariadicPartialEq
    Inner: GeneralizedHashTrieNode + IsBot,
{
    fn is_bot(&self) -> bool {
        self.trie.is_bot()
    }
}

impl<Head, Node> IsBot for GhtInner<Head, Node>
where
    Head: Clone,
    Node: GeneralizedHashTrieNode + IsBot,
{
    fn is_bot(&self) -> bool {
        self.children.iter().all(|(_, v)| v.is_bot())
    }
}

impl<Schema, SuffixSchema> IsBot for GhtLeaf<Schema, SuffixSchema>
where
    Schema: Eq + Hash,
    SuffixSchema: Eq + Hash,
{
    fn is_bot(&self) -> bool {
        self.elements.is_empty()
    }
}

// IsTop
impl<KeyType, ValType, Inner> IsTop for GHT<KeyType, ValType, Inner>
where
    KeyType: VariadicExt, // + AsRefVariadicPartialEq
    ValType: VariadicExt, // + AsRefVariadicPartialEq
    Inner: GeneralizedHashTrieNode + IsTop,
{
    fn is_top(&self) -> bool {
        self.trie.is_top()
    }
}

impl<Head, Node> IsTop for GhtInner<Head, Node>
where
    Head: Clone,
    Node: GeneralizedHashTrieNode,
{
    fn is_top(&self) -> bool {
        false
    }
}

impl<Schema, SuffixSchema> IsTop for GhtLeaf<Schema, SuffixSchema>
where
    Schema: Eq + Hash,
    SuffixSchema: Eq + Hash,
{
    fn is_top(&self) -> bool {
        false
    }
}

//////////////////////////
/// BiMorphisms for GHT
//////////////////////////
/// Bimorphism for the cartesian product of two GHTs. Output is a set of all possible pairs of
/// items from the two input sets.
pub struct GhtCartesianProductBimorphism<GhtOut> {
    _phantom: std::marker::PhantomData<fn() -> GhtOut>,
}
impl<GhtOut> Default for GhtCartesianProductBimorphism<GhtOut> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
impl<'a, 'b, GhtA, GhtB, GhtOut> LatticeBimorphism<&'a GhtA, &'b GhtB>
    for GhtCartesianProductBimorphism<GhtOut>
where
    GhtA: GeneralizedHashTrieNode,
    GhtB: GeneralizedHashTrieNode,
    GhtOut: FromIterator<var_type!(...GhtA::Schema, ...GhtB::Schema)>,
    GhtA::Schema: CloneVariadic,
    GhtB::Schema: CloneVariadic,
{
    type Output = GhtOut;

    fn call(&mut self, ght_a: &'a GhtA, ght_b: &'b GhtB) -> Self::Output {
        ght_a.recursive_iter().flat_map(|a| {
            ght_b
                .recursive_iter()
                .map(move |b| var_expr!(...<GhtA::Schema as CloneVariadic>::clone_var_ref(a), ...<GhtB::Schema as CloneVariadic>::clone_var_ref(b)))
        }).collect()
    }
}

/// Forms the cartesian product of the SchemaSuffix only
/// Used on GhtLeaf nodes to implement DeepJoinLatticeBimorphism
pub struct GhtSuffixProductBimorphism<GhtOut> {
    _phantom: std::marker::PhantomData<fn() -> GhtOut>,
}
impl<GhtOut> Default for GhtSuffixProductBimorphism<GhtOut> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
impl<'a, 'b, GhtA, GhtB, GhtOut> LatticeBimorphism<&'a GhtA, &'b GhtB>
    for GhtSuffixProductBimorphism<GhtOut>
where
    GhtA: GeneralizedHashTrieNode,
    GhtB: GeneralizedHashTrieNode,
    GhtOut: FromIterator<var_type!(...GhtA::Schema, ...GhtB::SuffixSchema)>,
    GhtA::Schema: Eq + Hash + CloneVariadic, // + SplitBySuffix<GhtA::SuffixSchema>,
    // GhtA::SuffixSchema: CloneVariadic,
    GhtB::Schema: Eq + Hash + SplitBySuffix<GhtB::SuffixSchema>,
    GhtB::SuffixSchema: CloneVariadic,
    // <GhtB::SuffixSchema as VariadicExt>::AsRefVar<'b>: Eq + Hash,
    // + CloneRefVariadic,
    //
    // + FromIterator<<<<GhtA as GeneralizedHashTrieNode>::Schema as EitherRefVariadic>::UnRefVar as VariadicExt>::Extend<<<GhtB as GeneralizedHashTrieNode>::SuffixSchema as EitherRefVariadic>::UnRefVar>>,
    //    + FromIterator<<<GhtA as GeneralizedHashTrieNode>::Schema as VariadicExt>::Extend<<GhtB as GeneralizedHashTrieNode>::SuffixSchema>>,
    // FromIterator<
    //     <<GhtA as GeneralizedHashTrieNode>::Schema as VariadicExt>::Extend<<GhtB as GeneralizedHashTrieNode>::SuffixSchema>>,
    // <GhtA::Schema as VariadicExt>::AsRefVar<'a>: CloneRefVariadic, // + Eq + Hash,
    // <GhtB::Schema as VariadicExt>::AsRefVar<'b>: Eq + Hash + SplitBySuffix<GhtB::SuffixSchema>,
    // <GhtB::SuffixSchema as VariadicExt>::AsRefVar<'b>: Eq + Hash + CloneRefVariadic,
    // <GhtA::SuffixSchema as VariadicExt>::AsRefVar<'a>: CloneRefVariadic,
    //  GhtB::SuffixSchema: CloneRefVariadic,
    // <GhtA as GeneralizedHashTrieNode>::SuffixSchema: EitherRefVariadic + CloneRefVariadic,
    // <GhtA as GeneralizedHashTrieNode>::Schema: EitherRefVariadic,
{
    type Output = GhtOut;

    fn call(&mut self, ght_a: &'a GhtA, ght_b: &'b GhtB) -> Self::Output {
        ght_a.recursive_iter().flat_map(|a| {
            // let (_prefix_a, suffix_a)
            //     = <GhtA::Schema as SplitBySuffix<GhtA::SuffixSchema>>::split_by_suffix_ref(a);
            ght_b
                .recursive_iter()
                .map(move |b| {
                    let (_prefix_b, suffix_b)
                        = <GhtB::Schema as SplitBySuffix<GhtB::SuffixSchema>>::split_by_suffix_ref(b);
                    var_expr!(...<GhtA::Schema as CloneVariadic>::clone_var_ref(a), ...<GhtB::SuffixSchema as CloneVariadic>::clone_var_ref(suffix_b))
                }
            )
        }).collect()
    }
}

/// Composable bimorphism, wraps an existing morphism by partitioning it per key.
///
/// For example, `GhtKeyedBimorphism<..., GhtCartesianProduct<...>>` is a join.
#[derive(Default)]
pub struct GhtBimorphism<Bimorphism> {
    bimorphism: Bimorphism,
    // _phantom: std::marker::PhantomData<fn() -> MapOut>,
}
impl<Bimorphism> GhtBimorphism<Bimorphism> {
    /// Create a `KeyedBimorphism` using `bimorphism` for handling values.
    pub fn new(bimorphism: Bimorphism) -> Self {
        Self {
            bimorphism,
            // _phantom: std::marker::PhantomData,
        }
    }
}

impl<GhtA, GhtB, ValFunc, Output> LatticeBimorphism<GhtA, GhtB> for GhtBimorphism<ValFunc>
where
    GhtA: GeneralizedHashTrie,
    GhtB: GeneralizedHashTrie,
    for<'a, 'b> ValFunc: LatticeBimorphism<&'a GhtA::Trie, &'b GhtB::Trie, Output = Output>,
    Output: GeneralizedHashTrieNode,
    // <<GhtA as GeneralizedHashTrie>::ValType as VariadicExt>::Extend<
    //     <GhtB as GeneralizedHashTrie>::ValType,
    // >: + AsRefVariadicPartialEq
{
    type Output = GHT<GhtA::KeyType, var_type!(...GhtA::ValType, ...GhtB::ValType), Output>; // HashMap<Head, ValFunc::Output>; // GhtOut;

    fn call(&mut self, ght_a: GhtA, ght_b: GhtB) -> Self::Output {
        let node_bim = &mut self.bimorphism; // GhtNodeKeyedBimorphism::<ValFunc>::new(self.bimorphism);
        let trie = node_bim.call(ght_a.get_trie(), ght_b.get_trie());
        GHT {
            trie,
            _key: std::marker::PhantomData,
            _val: std::marker::PhantomData,
        }
    }
}

#[derive(Default)]
/// bimorphism trait for equijoining Ght Nodes
pub struct GhtNodeKeyedBimorphism<Bimorphism> {
    bimorphism: Bimorphism,
}
/// bimorphism implementation for equijoining Ght Nodes
impl<Bimorphism> GhtNodeKeyedBimorphism<Bimorphism> {
    /// initialize bimorphism
    pub fn new(bimorphism: Bimorphism) -> Self {
        Self { bimorphism }
    }
}
/// bimorphism implementation for equijoining Ght Nodes
impl<'a, 'b, Head, GhtA, GhtB, ValFunc> LatticeBimorphism<&'a GhtA, &'b GhtB>
    for GhtNodeKeyedBimorphism<ValFunc>
where
    Head: Clone + Hash + Eq,
    ValFunc: LatticeBimorphism<&'a GhtA::Get, &'b GhtB::Get>,
    ValFunc::Output: GeneralizedHashTrieNode,
    GhtA: GeneralizedHashTrieNode<Head = Head>,
    GhtB: GeneralizedHashTrieNode<Head = Head>,
    <GhtA::SuffixSchema as VariadicExt>::AsRefVar<'a>: CloneVariadic,
    <GhtB::SuffixSchema as VariadicExt>::AsRefVar<'b>: CloneVariadic,
    // <GhtA as GeneralizedHashTrieNode>::Leaf: VariadicExt,
    // <GhtB as GeneralizedHashTrieNode>::Leaf: VariadicExt,
    // <<GhtA as GeneralizedHashTrieNode>::Leaf as VariadicExt>::Extend<
    //     <GhtB as GeneralizedHashTrieNode>::Leaf,
    // >: Eq + Hash,
{
    type Output = GhtInner<Head, ValFunc::Output>; // HashMap<Head, ValFunc::Output>; // GhtOut;

    fn call(&mut self, ght_a: &'a GhtA, ght_b: &'b GhtB) -> Self::Output {
        // let ght_out = Self::Output::default();

        let mut children = HashMap::<Head, ValFunc::Output>::new();
        for head in ght_b.iter() {
            if let Some(get_a) = ght_a.get(head) {
                let get_b = ght_b.get(head).unwrap();
                let val = self.bimorphism.call(get_a, get_b);
                children.insert(head.clone(), val);
            }
        }
        GhtInner::<Head, ValFunc::Output> {
            children,
            // _leaf: Default::default(),
        }
    }
}

/// bimorphism trait for equijoin on full tuple (keys in all GhtInner nodes)
pub trait DeepJoinLatticeBimorphism {
    /// bimorphism type for equijoin on full tuple (keys in all GhtInner nodes)
    type DeepJoinLatticeBimorphism;
}
/// bimorphism implementation for equijoin on full tuple (keys in all GhtInner nodes)
impl<Head, NodeA, NodeB> DeepJoinLatticeBimorphism
    for (GhtInner<Head, NodeA>, GhtInner<Head, NodeB>)
where
    Head: 'static + Hash + Eq + Clone,
    NodeA: 'static + GeneralizedHashTrieNode,
    NodeB: 'static + GeneralizedHashTrieNode,
    (NodeA, NodeB): DeepJoinLatticeBimorphism,
{
    type DeepJoinLatticeBimorphism = GhtNodeKeyedBimorphism<
        <(NodeA, NodeB) as DeepJoinLatticeBimorphism>::DeepJoinLatticeBimorphism,
    >;
}
impl<SchemaA, SuffixSchemaA, SchemaB, SuffixSchemaB> DeepJoinLatticeBimorphism
    for (
        GhtLeaf<SchemaA, SuffixSchemaA>,
        GhtLeaf<SchemaB, SuffixSchemaB>,
    )
where
    SchemaA: 'static + VariadicExt + Eq + Hash + SplitBySuffix<SuffixSchemaA>, /* + AsRefVariadicPartialEq */
    SuffixSchemaA: 'static + VariadicExt + Eq + Hash, // + AsRefVariadicPartialEq
    SchemaB: 'static + VariadicExt + Eq + Hash + SplitBySuffix<SuffixSchemaB>, /* + AsRefVariadicPartialEq */
    SuffixSchemaB: 'static + VariadicExt + Eq + Hash, // + AsRefVariadicPartialEq
    for<'x> SchemaA::AsRefVar<'x>: CloneVariadic,
    for<'x> SchemaB::AsRefVar<'x>: CloneVariadic,
    // for<'a, 'b> A::AsRefVar<'a>: PartialEq<A::AsRefVar<'b>>,
    // for<'a, 'b> B::AsRefVar<'a>: PartialEq<B::AsRefVar<'b>>,
    var_type!(...SchemaA, ...SuffixSchemaB): Eq + Hash,
    // <<SchemaA as SplitBySuffix<SuffixSchemaA>>::Prefix as VariadicExt>::Extend<
    //     // <<SchemaB as SplitBySuffix<SuffixSchemaB>>::Prefix as VariadicExt>::Extend<
    //     <SuffixSchemaA as VariadicExt>::Extend<SuffixSchemaB>,
    //     // >,
    // >: Eq + Hash,
    // for<'a, 'b> <var_type!(...A, ...B) as VariadicExt>::AsRefVar<'a>:
    //     PartialEq<<var_type!(...A, ...B) as VariadicExt>::AsRefVar<'b>>,
{
    type DeepJoinLatticeBimorphism = GhtSuffixProductBimorphism<
        GhtLeaf<
            var_type!(...SchemaA, ...SuffixSchemaB),
            var_type!(...SuffixSchemaA, ...SuffixSchemaB),
        >,
    >;
}
