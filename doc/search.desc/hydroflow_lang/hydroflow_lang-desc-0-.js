searchState.loadedDescShard("hydroflow_lang", 0, "Hydroflow surface syntax\nCompatibility for <code>proc_macro</code> diagnostics, which are …\nGraph representation stages for Hydroflow graphs.\nAST for surface syntax, modelled on <code>syn</code>’s ASTs.\nPretty, human-readable printing of <code>proc_macro2::Span</code>s.\nUtility methods for processing singleton references: …\nUnion-find data structure, see <code>UnionFind</code>.\nDiagnostic. A warning or error (or lower <code>Level</code>) with a …\nAn error.\nA help message.\nDiagnostic reporting level.\nA note.\nA serializable and deserializable version of <code>Span</code>. Cannot …\nA warning.\nColumn number, one-indexed.\nEmit the diagnostic. Only works from the <code>proc_macro</code> …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nIf this level is <code>Level::Error</code>.\nIf this diagnostic’s level is <code>Level::Error</code>.\nSeverity level.\nLine number, one-indexed.\nHuman-readable message.\nThe source file path.\nSpan (source code location).\nCreate a new diagnostic from the given span, level, and …\nConverts this into a serializable and deserializable …\nUsed to emulate <code>Diagnostic::emit</code> by turning this …\nPush, Pull, Comp, or Hoff polarity.\nComputation (yellow)\nA directed multigraph where an vertex’s inbound and …\nDot (Graphviz) graphs.\nElided, unspecified port. We have this variant, rather …\nWraper around <code>HydroflowGraph</code> to build a flat graph from …\nID to identify an edge.\nA node, corresponding to an operator or a handoff.\nID to identify a node (operator or handoff) in …\nID to identify a subgraph in <code>HydroflowGraph</code>.\nA handoff point, used between subgraphs (or within a …\nHandoff (grey) – not a color for operators, inserted …\nAn abstract “meta graph” representation of a Hydroflow …\nAn integer value: <code>[0]</code>, <code>[1]</code>, etc. Can be negative although …\nMermaid graphs.\nModule Boundary, used for importing modules. Only exists …\nOperator generic arguments, split into specific categories.\nAn operator.\nMeta-data relating to operators which may be useful …\nA name or path. <code>[pos]</code>, <code>[neg]</code>, etc. Can use <code>::</code> separators …\nHelper struct for <code>PortIndex</code> which keeps span information …\nPull (green)\nPush (blue)\nConfiguration for writing graphs.\nEnum for choosing between mermaid and dot graph writing.\nAdd a single <code>HfStatement</code> line to this <code>HydroflowGraph</code>.\nArguments provided by the user into the operator as …\nUnparsed arguments, for singleton parsing.\nEmit this <code>HydroflowGraph</code> as runnable Rust source code …\nFormats self as a human-readable string for error messages.\nAssert that <code>self</code> is in a consistent state, for debugging. …\nBuild into an unpartitioned <code>HydroflowGraph</code>, returning a …\nThe main function of this module. Compiles a <code>HfCode</code> AST …\nReturn <code>Err(self)</code> if there is a conflict.\nThe degree (number of edges/vertices) going into <code>v</code>, i.e. …\nThe degree (number of edges/vertices) coming out of <code>v</code>, …\nGet the source and destination vertex IDs for the given …\nGet the <code>src</code> and <code>dst</code> for an edge: …\nReturn an iterator over all edge IDs <code>E</code>.\nIterator of all edge IDs <code>GraphEdgeId</code>.\nGet the source and destination ports for an edge: …\nReturn an iterator over all edges in form <code>(E, (V, V))</code>.\nIterator over all edges: …\nRemoves missing unions and tees. Must be applied BEFORE …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert the Hydroflow code AST into a graph builder.\nConvert the Hydroflow code AST into a graph builder.\nFor a <code>Ported</code> value like <code>[port_in]name[port_out]</code>, get the …\nOperator generic (type or lifetime) arguments.\nGeneric arguments.\nGets the generic arguments for the operator.\nGeneral graph algorithm utility functions\nPort values used as this operator’s input.\nCreates an edge going from <code>src</code> to <code>dst</code> and returns the edge …\nInsert an edge between nodes thru the given ports.\nInserts a node between two existing nodes connected by the …\nFor an <code>edge</code> from <code>A --&gt; B</code>, insert a new vertex <code>V</code> along that …\nInsert a node, assigning the given varname.\nInsert an operator instance for the given node. Panics if …\nAssign all operator instances if not set. Write diagnostic …\nCreate a subgraph consisting of <code>node_ids</code>. Returns an error …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns <code>true</code> if <code>self</code> is not <code>PortIndexValue::Elided</code>.\nReturns the the stratum number of the largest (latest) …\nWhen modules are imported into a flat graph, they come …\nConvert into a mermaid graph. Ignores subgraphs.\nCreates an empty <code>DiMulGraph</code>.\nCreate a new empty graph builder.\nCreate a new empty <code>HydroflowGraph</code>.\nWill not render handoffs if set.\nWill not render pull/push shapes if set.\nWill not render singleton references if set.\nSubgraphs will not be rendered if set.\nVariable names will not be rendered if set.\nGet a node with its operator instance (if applicable).\nColor mode (pull vs. push, handoff vs. comp) for nodes. …\nDegree into a node, i.e. the number of predecessors.\nDegree out of a node, i.e. the number of successors.\nIterator of node IDs <code>GraphNodeId</code>.\nGet the <code>OperatorInstance</code> for a given node. Node must be an …\nPredecessor edges, iterator of <code>GraphEdgeId</code> of incoming …\nPredecessor nodes, iterator of <code>GraphNodeId</code>.\nPredecessors, iterator of <code>(GraphEdgeId, GraphNodeId)</code> of …\nGets the singletons referenced by a node. Returns an empty …\nGet subgraph for node.\nSuccessor edges, iterator of <code>GraphEdgeId</code> of outgoing edges.\nSuccessor nodes, iterator of <code>GraphNodeId</code>.\nSuccessors, iterator of <code>(GraphEdgeId, GraphNodeId)</code> of …\nGet the debug variable name attached to a graph node.\nIterator over <code>(GraphNodeId, &amp;Node)</code> pairs.\nName of the operator (will match <code>OperatorConstraints::name</code>…\nOp text will only be their name instead of the whole …\nOp text will exclude any line that starts with “use”.\nOpens this as dot/graphviz graph in the Graphviz Online …\nOpens the graph based on <code>graph_type</code>, which can be parsed …\nOpens this as a mermaid graph in the mermaid.live browser …\nHydroflow’s operators\nPort values used as this operator’s output.\nMain method for this module. Partions a flat <code>HydroflowGraph</code>…\nLifetime persistence arguments. Corresponds to a prefix of …\nReturn an iterator of all edge IDs going into <code>v</code>.\nReturn an iterator of all predecessor vertex IDs of <code>v</code>.\nReturn an iterator of all predecessor edge IDs <em>and</em> vertex …\nRemove an edge from the graph. If the edgeId is found then …\nRemoves an edge and its corresponding ports and edge type …\nRemoves a node from its subgraph. Returns true if the node …\nRemove the node <code>node_id</code> but preserves and connects the …\nFor a vertex with one incoming edge and one outgoing edge, …\nRemove a vertex from the graph, it must have no edges to …\nSet the singletons referenced for the <code>node_id</code> operator. …\nSet subgraph’s laziness, returning the old value.\nSet subgraph’s stratum number, returning the old value …\nSingleton references within the operator arguments.\nReturn the source code span of the node (for operators) or …\nReturns the span of this port value.\nNodes belonging to the given subgraph.\nIterator over all subgraph IDs.\nGets the stratum number of the subgraph.\nIterator over all subgraphs, ID and members: …\nReturn an iterator of all edge IDs coming out of <code>v</code>.\nReturn an iterator of all successor vertex IDs of <code>v</code>.\nReturn an iterator of all successor edge IDs <em>and</em> vertex …\nConvert back into surface syntax.\nWrites this graph as DOT (graphviz) into a string.\nWrites this graph as mermaid into a string.\nReturn the name of the node as a string, excluding …\nReturn the node as a human-readable string.\nType persistence arguments. Corersponds to a (suffix) of …\nCreates a <code>DiMulGraph</code> with pre-allocated memory for <code>capacity</code>…\nWrites this graph as DOT (graphviz) into the given <code>Write</code>.\nWrites this graph as mermaid into the given <code>Write</code>.\nConvert into a mermaid graph. Ignores subgraphs.\nConvert back into surface syntax.\nThe span of the output out of the handoff.\nThe span of the import!() expression that imported this …\nIf this module is an input or output boundary.\nThe span of the input into the handoff.\nFinds the strongly connected components in the graph. A …\nTopologically sorts a set of nodes. Returns a list where …\nComputers the topological sort of the nodes of a possibly …\nThe delay (soft barrier) type, for each input to an …\nA specific number of named ports.\n<code>OperatorConstraints::write_fn</code> for unary identity operators.\nMonotone accumulation: can delay to reduce flow rate, but …\nMutability.\n<code>OperatorConstraints::write_fn</code> for the null operator - an …\nAll Hydroflow operators.\nOperator categories, for docs.\nAn instance of this struct represents a single hydroflow …\nThe code generated and returned by a …\nPersistence lifetimes: <code>&#39;tick</code>, <code>&#39;static</code>, or <code>&#39;mutable</code>.\nSpecification of the named (or unnamed) ports for an …\nConvenience range: exactly zero.\nConvenience range: exactly one.\nConvenience range: zero or more (any number).\nAn object-safe version of <code>RangeBounds</code>.\nPersistene across all ticks.\nInput must be collected over the preceeding stratum.\nInput must be collected over the previous tick.\nPersistence for one tick at-a-time only.\nInput must be collected over the previous tick but also …\nAny number of unnamed (or optionally named) ports.\nContext arguments provided to <code>OperatorConstraints::write_fn</code>…\nType alias for <code>OperatorConstraints::write_fn</code>’s type.\nArguments provided by the user into the operator as …\nSame as <code>Self::arguments</code> but with only <code>StateHandle</code>s, no …\nOperator categories, for docs.\nReturns if <code>item</code> is contained in this range.\n<code>context</code> ident, the name of the provided …\nHuman description, for docs.\nEnd (upper) bound.\nFind an operator by <code>GraphNode</code>.\nFind an operator by an AST <code>Operator</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nInput argument range required to not show an error.\nOutput argument range required to not show an error.\nIf this operator has a singleton reference output. For …\nTurn this range into a human-readable string.\n<code>df</code> ident, the name of the …\nIdent the iterator or pullerator should be assigned to.\nHelper to write the <code>write_iterator</code> portion of …\nDetermines if this input must be preceeded by a stratum …\nInput operator idents (or ref idents; used for pull).\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nIf this operator receives external inputs and therefore …\nIf a pull iterator (true) or pusherator (false) should be …\nGenerate a (almost certainly) unique identifier with the …\nHuman-readible heading name, for docs.\nOperator’s name.\nNode ID identifying this operator in the flat or …\nHelper to write the <code>write_iterator</code> portion of …\nNumber of arguments i.e. <code>operator(a, b, c)</code> has <code>num_args = 3</code>…\nOperator instance arguments object.\nOperator name.\nThe source span of this operator.\nGet the operator lookup table, generating it if needed.\nOutput operator idents (or ref idents; used for push).\nHow many persistence lifetime arguments can be provided.\nWhat named or numbered input ports to expect?\nWhat named or numbered output ports to expect?\n<code>hydroflow</code> crate name for <code>use #root::something</code>.\nIdent for the singleton output of this operator, if any.\nInput argument range required to not show a warning.\nOutput argument range required to not show an warning.\nStart (lower) bound.\nSubgraph ID in which this operator is contained.\nHow many generic type arguments can be provided.\nThe operator’s codegen. Returns code that is emited is …\nIterator (or pusherator) code inside the subgraphs. The …\nCode which runs after iterators have been run. Mainly for …\nCode which runs once outside the subgraph to set up any …\nPort can either be an int or a name (path).\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nOutput the operator as a formatted string using …\nHelper struct which displays the span as <code>row:col</code> for human …\nHelper struct which displays the span as <code>path:row:col</code> for …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReplaces singleton references <code>#my_var</code> with the code needed …\nSame as <code>postprocess_singletons</code> but generates just the …\nFinds all the singleton references <code>#my_var</code> and appends …\nUnion-find data structure.\nFinds the “representative” item for <code>k</code>. Each set of …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreates a new <code>UnionFind</code>, same as <code>Default::default()</code>.\nReturns if <code>a</code> and <code>b</code> are equivalent, i.e. in the same set.\nCombines two items <code>a</code> and <code>b</code> as equivalent, in the same set.\nCreates a new <code>UnionFind</code> with the given key capacity …")