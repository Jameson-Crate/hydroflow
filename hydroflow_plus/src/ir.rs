use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use std::rc::Rc;

use hydroflow_lang::graph::FlatGraphBuilder;
use hydroflow_lang::parse::Pipeline;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::parse_quote;

#[derive(Clone)]
pub struct DebugExpr(pub syn::Expr);

impl From<syn::Expr> for DebugExpr {
    fn from(expr: syn::Expr) -> DebugExpr {
        DebugExpr(expr)
    }
}

impl Deref for DebugExpr {
    type Target = syn::Expr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokens for DebugExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl std::fmt::Debug for DebugExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_token_stream())
    }
}

#[derive(Clone)]
pub struct DebugPipelineFn(pub Rc<dyn Fn(bool) -> Option<(Pipeline, bool)> + 'static>);

impl std::fmt::Debug for DebugPipelineFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function>")
    }
}

/// A source in a Hydroflow+ graph, where data enters the graph.
#[derive(Clone, Debug)]
pub enum HfPlusSource {
    Stream(DebugExpr),
    Iter(DebugExpr),
    Interval(DebugExpr),
    Spin(),
}

/// An leaf in a Hydroflow+ graph, which is an pipeline that doesn't emit
/// any downstream values. Traversals over the dataflow graph and
/// generating Hydroflow IR start from leaves.
#[derive(Clone, Debug)]
pub enum HfPlusLeaf {
    ForEach {
        f: DebugExpr,
        input: Box<HfPlusNode>,
    },
    DestSink {
        sink: DebugExpr,
        input: Box<HfPlusNode>,
    },
    CycleSink {
        ident: syn::Ident,
        location_id: usize,
        input: Box<HfPlusNode>,
    },
}

impl HfPlusLeaf {
    pub fn emit(
        self,
        graph_builders: &mut BTreeMap<usize, FlatGraphBuilder>,
        built_tees: &mut HashMap<*const RefCell<HfPlusNode>, (syn::Ident, usize, bool)>,
        next_stmt_id: &mut usize,
    ) {
        match self {
            HfPlusLeaf::ForEach { f, input } => {
                let (input_ident, input_location_id, input_delta) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                assert!(!input_delta, "for_each on a delta stream");

                graph_builders
                    .entry(input_location_id)
                    .or_default()
                    .add_statement(parse_quote! {
                        #input_ident -> for_each(#f);
                    });
            }

            HfPlusLeaf::DestSink { sink, input } => {
                let (input_ident, input_location_id, input_delta) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                assert!(!input_delta, "sending delta in dest_sink");

                graph_builders
                    .entry(input_location_id)
                    .or_default()
                    .add_statement(parse_quote! {
                        #input_ident -> dest_sink(#sink);
                    });
            }

            HfPlusLeaf::CycleSink {
                ident,
                location_id,
                input,
            } => {
                let (input_ident, input_location_id, input_delta) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                assert!(!input_delta, "sending deltas into a cycle");
                assert_eq!(
                    input_location_id, location_id,
                    "cycle_sink location mismatch"
                );

                graph_builders
                    .entry(location_id)
                    .or_default()
                    .add_statement(parse_quote! {
                        #ident = #input_ident;
                    });
            }
        }
    }
}

/// An intermediate node in a Hydroflow+ graph, which consumes data
/// from upstream nodes and emits data to downstream nodes.
#[derive(Clone, Debug)]
pub enum HfPlusNode {
    Placeholder,
    Persist(Box<HfPlusNode>),
    Delta(Box<HfPlusNode>),
    Source {
        source: HfPlusSource,
        produces_delta: bool,
        location_id: usize,
    },
    CycleSource {
        ident: syn::Ident,
        location_id: usize,
    },
    Tee {
        inner: Rc<RefCell<HfPlusNode>>,
    },
    Union(Box<HfPlusNode>, Box<HfPlusNode>),
    CrossProduct(Box<HfPlusNode>, Box<HfPlusNode>),
    Join(Box<HfPlusNode>, Box<HfPlusNode>),
    Difference(Box<HfPlusNode>, Box<HfPlusNode>),
    AntiJoin(Box<HfPlusNode>, Box<HfPlusNode>),

    Map {
        f: DebugExpr,
        input: Box<HfPlusNode>,
    },

    /// A temporary IR node for a custom pipeline, which eventually will be replaced by a proper node.
    PipelineOp {
        kind: &'static str,
        gen_pipeline: DebugPipelineFn,
        input: Box<HfPlusNode>,
    },

    Network {
        to_location: usize,
        serialize_pipeline: Option<Pipeline>,
        sink_expr: DebugExpr,
        source_expr: DebugExpr,
        deserialize_pipeline: Option<Pipeline>,
        input: Box<HfPlusNode>,
    },
}

impl HfPlusNode {
    pub fn emit(
        self,
        graph_builders: &mut BTreeMap<usize, FlatGraphBuilder>,
        built_tees: &mut HashMap<*const RefCell<HfPlusNode>, (syn::Ident, usize, bool)>,
        next_stmt_id: &mut usize,
    ) -> (syn::Ident, usize, bool) {
        match self {
            HfPlusNode::Placeholder => {
                panic!()
            }

            HfPlusNode::Persist(inner) => {
                let (ident, location, inner_delta) =
                    inner.emit(graph_builders, built_tees, next_stmt_id);
                assert!(!inner_delta, "double persist");
                (ident, location, true)
            }

            HfPlusNode::Delta(inner) => {
                let (inner_ident, location, inner_delta) =
                    inner.emit(graph_builders, built_tees, next_stmt_id);

                if inner_delta {
                    (inner_ident, location, false)
                } else {
                    let delta_id = *next_stmt_id;
                    *next_stmt_id += 1;

                    let delta_ident =
                        syn::Ident::new(&format!("stream_{}", delta_id), Span::call_site());

                    let builder = graph_builders.entry(location).or_default();
                    builder.add_statement(parse_quote! {
                        #delta_ident = #inner_ident -> multiset_delta();
                    });

                    (delta_ident, location, false)
                }
            }

            HfPlusNode::Source {
                source,
                produces_delta,
                location_id,
            } => {
                let source_id = *next_stmt_id;
                *next_stmt_id += 1;

                let source_ident =
                    syn::Ident::new(&format!("stream_{}", source_id), Span::call_site());

                let source_stmt = match source {
                    HfPlusSource::Stream(expr) => {
                        parse_quote! {
                            #source_ident = source_stream(#expr);
                        }
                    }

                    HfPlusSource::Iter(expr) => {
                        parse_quote! {
                            #source_ident = source_iter(#expr);
                        }
                    }

                    HfPlusSource::Interval(expr) => {
                        parse_quote! {
                            #source_ident = source_interval(#expr);
                        }
                    }

                    HfPlusSource::Spin() => {
                        parse_quote! {
                            #source_ident = spin();
                        }
                    }
                };

                graph_builders
                    .entry(location_id)
                    .or_default()
                    .add_statement(source_stmt);

                (source_ident, location_id, produces_delta)
            }

            HfPlusNode::CycleSource { ident, location_id } => (ident.clone(), location_id, false),

            HfPlusNode::Tee { inner } => {
                if let Some(ret) = built_tees.get(&(inner.as_ref() as *const RefCell<HfPlusNode>)) {
                    ret.clone()
                } else {
                    let (inner_ident, inner_location_id, inner_produces_delta) = inner
                        .replace(HfPlusNode::Placeholder)
                        .emit(graph_builders, built_tees, next_stmt_id);

                    let tee_id = *next_stmt_id;
                    *next_stmt_id += 1;

                    let tee_ident =
                        syn::Ident::new(&format!("stream_{}", tee_id), Span::call_site());

                    let builder = graph_builders.entry(inner_location_id).or_default();
                    builder.add_statement(parse_quote! {
                        #tee_ident = #inner_ident -> tee();
                    });

                    built_tees.insert(
                        inner.as_ref() as *const RefCell<HfPlusNode>,
                        (tee_ident.clone(), inner_location_id, inner_produces_delta),
                    );

                    (tee_ident, inner_location_id, inner_produces_delta)
                }
            }

            HfPlusNode::Union(left, right) => {
                let (left_ident, left_location_id, left_delta) =
                    left.emit(graph_builders, built_tees, next_stmt_id);
                let (right_ident, right_location_id, right_delta) =
                    right.emit(graph_builders, built_tees, next_stmt_id);

                assert_eq!(
                    left_location_id, right_location_id,
                    "union inputs must be in the same location"
                );

                assert!(!left_delta, "union on a delta stream");
                assert!(!right_delta, "union on a delta stream");

                let union_id = *next_stmt_id;
                *next_stmt_id += 1;

                let union_ident =
                    syn::Ident::new(&format!("stream_{}", union_id), Span::call_site());

                let builder = graph_builders.entry(left_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #union_ident = union();
                });

                builder.add_statement(parse_quote! {
                    #left_ident -> [0]#union_ident;
                });

                builder.add_statement(parse_quote! {
                    #right_ident -> [1]#union_ident;
                });

                (union_ident, left_location_id, false)
            }

            HfPlusNode::CrossProduct(..) | HfPlusNode::Join(..) => {
                let operator: syn::Ident = if matches!(self, HfPlusNode::CrossProduct(..)) {
                    parse_quote!(cross_join)
                } else {
                    parse_quote!(join)
                };

                if let HfPlusNode::CrossProduct(left, right) | HfPlusNode::Join(left, right) = self
                {
                    let (left_ident, left_location_id, left_delta) =
                        left.emit(graph_builders, built_tees, next_stmt_id);
                    let (right_ident, right_location_id, right_delta) =
                        right.emit(graph_builders, built_tees, next_stmt_id);

                    assert_eq!(
                        left_location_id, right_location_id,
                        "join / cross product inputs must be in the same location"
                    );

                    let stream_id = *next_stmt_id;
                    *next_stmt_id += 1;

                    let stream_ident =
                        syn::Ident::new(&format!("stream_{}", stream_id), Span::call_site());

                    let builder = graph_builders.entry(left_location_id).or_default();

                    let output_delta = match (left_delta, right_delta) {
                        (true, true) => {
                            builder.add_statement(parse_quote! {
                                #stream_ident = #operator::<'static, 'static>();
                            });

                            false // TODO(shadaj): join/cross_join already replays?
                        }
                        (true, false) => {
                            builder.add_statement(parse_quote! {
                                #stream_ident = #operator::<'static, 'tick>();
                            });

                            false
                        }
                        (false, true) => {
                            builder.add_statement(parse_quote! {
                                #stream_ident = #operator::<'tick, 'static>();
                            });

                            false
                        }
                        (false, false) => {
                            builder.add_statement(parse_quote! {
                                #stream_ident = #operator::<'tick, 'tick>();
                            });

                            false
                        }
                    };

                    builder.add_statement(parse_quote! {
                        #left_ident -> [0]#stream_ident;
                    });

                    builder.add_statement(parse_quote! {
                        #right_ident -> [1]#stream_ident;
                    });

                    (stream_ident, left_location_id, output_delta)
                } else {
                    unreachable!()
                }
            }

            HfPlusNode::Difference(..) | HfPlusNode::AntiJoin(..) => {
                let operator: syn::Ident = if matches!(self, HfPlusNode::Difference(..)) {
                    parse_quote!(difference)
                } else {
                    parse_quote!(anti_join)
                };

                if let HfPlusNode::Difference(left, right) | HfPlusNode::AntiJoin(left, right) =
                    self
                {
                    let (left_ident, left_location_id, left_delta) =
                        left.emit(graph_builders, built_tees, next_stmt_id);
                    let (right_ident, right_location_id, right_delta) =
                        right.emit(graph_builders, built_tees, next_stmt_id);

                    assert_eq!(
                        left_location_id, right_location_id,
                        "difference / anti join inputs must be in the same location"
                    );

                    let stream_id = *next_stmt_id;
                    *next_stmt_id += 1;

                    let stream_ident =
                        syn::Ident::new(&format!("stream_{}", stream_id), Span::call_site());

                    let builder = graph_builders.entry(left_location_id).or_default();
                    let output_delta = match (left_delta, right_delta) {
                        (true, true) => {
                            // difference/anti_join<'static, _> does not replay
                            // but we need to re-filter every tick
                            builder.add_statement(parse_quote! {
                                #stream_ident = #operator::<'tick, 'static>();
                            });

                            false
                        }
                        (true, false) => {
                            // difference/anti_join<'static, _> does not replay
                            // but we need to re-filter every tick
                            builder.add_statement(parse_quote! {
                                #stream_ident = #operator::<'tick, 'tick>();
                            });

                            false
                        }
                        (false, true) => {
                            builder.add_statement(parse_quote! {
                                #stream_ident = #operator::<'tick, 'static>();
                            });

                            false
                        }
                        (false, false) => {
                            builder.add_statement(parse_quote! {
                                #stream_ident = #operator::<'tick, 'tick>();
                            });

                            false
                        }
                    };

                    if left_delta {
                        builder.add_statement(parse_quote! {
                            #left_ident -> persist() -> [pos]#stream_ident;
                        });
                    } else {
                        builder.add_statement(parse_quote! {
                            #left_ident -> [pos]#stream_ident;
                        });
                    }

                    builder.add_statement(parse_quote! {
                        #right_ident -> [neg]#stream_ident;
                    });

                    (stream_ident, left_location_id, output_delta)
                } else {
                    unreachable!()
                }
            }

            HfPlusNode::Map { f, input } => {
                let (input_ident, input_location_id, input_delta) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let map_id = *next_stmt_id;
                *next_stmt_id += 1;

                let map_ident = syn::Ident::new(&format!("stream_{}", map_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #map_ident = map(#f);
                });

                builder.add_statement(parse_quote! {
                    #input_ident -> #map_ident;
                });

                (map_ident, input_location_id, input_delta)
            }

            HfPlusNode::Network {
                to_location,
                serialize_pipeline,
                sink_expr,
                source_expr,
                deserialize_pipeline,
                input,
            } => {
                let (input_ident, input_location_id, input_delta) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let sender_builder = graph_builders.entry(input_location_id).or_default();

                if let Some(serialize_pipeline) = serialize_pipeline {
                    sender_builder.add_statement(parse_quote! {
                        #input_ident -> #serialize_pipeline -> dest_sink(#sink_expr);
                    });
                } else {
                    sender_builder.add_statement(parse_quote! {
                        #input_ident -> dest_sink(#sink_expr);
                    });
                }

                let receiver_builder = graph_builders.entry(to_location).or_default();
                let receiver_stream_id = *next_stmt_id;
                *next_stmt_id += 1;

                let receiver_stream_ident =
                    syn::Ident::new(&format!("stream_{}", receiver_stream_id), Span::call_site());

                if let Some(deserialize_pipeline) = deserialize_pipeline {
                    receiver_builder.add_statement(parse_quote! {
                        #receiver_stream_ident = source_stream(#source_expr) -> #deserialize_pipeline;
                    });
                } else {
                    receiver_builder.add_statement(parse_quote! {
                        #receiver_stream_ident = source_stream(#source_expr);
                    });
                }

                (receiver_stream_ident, to_location, input_delta)
            }

            HfPlusNode::PipelineOp {
                kind: _,
                gen_pipeline,
                input,
            } => {
                let (input_ident, input_location_id, input_delta) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let pipeline_id = *next_stmt_id;
                *next_stmt_id += 1;

                let pipeline_ident =
                    syn::Ident::new(&format!("stream_{}", pipeline_id), Span::call_site());

                if let Some((pipeline, is_delta)) = gen_pipeline.0(input_delta) {
                    graph_builders
                        .entry(input_location_id)
                        .or_default()
                        .add_statement(parse_quote! {
                            #pipeline_ident = #input_ident -> #pipeline;
                        });

                    (pipeline_ident, input_location_id, is_delta)
                } else if input_delta {
                    let (pipeline, is_delta) = gen_pipeline.0(false)
                        .expect("pipeline op refused to generate on a non-delta stream");
                    graph_builders
                        .entry(input_location_id)
                        .or_default()
                        .add_statement(parse_quote! {
                            #pipeline_ident = #input_ident -> persist() -> #pipeline;
                        });

                    (pipeline_ident, input_location_id, is_delta)
                } else {
                    panic!("pipeline op refused to generate on a non-delta stream");
                }
            }
        }
    }
}
