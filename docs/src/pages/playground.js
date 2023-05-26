import React, { useEffect, useState } from 'react';
import Layout from '@theme/Layout';

import Editor from "@monaco-editor/react";

import ExecutionEnvironment from '@docusaurus/ExecutionEnvironment';

import * as wasm from "website_playground/website_playground_bg.wasm";
import { __wbg_set_wasm, init, compile_hydroflow, compile_datalog } from "website_playground/website_playground_bg.js";

if (ExecutionEnvironment.canUseDOM) {
  __wbg_set_wasm(wasm);
} else {
  const wasmUri = require("website_playground/website_playground_bg.wasm");
  const wasmBuffer = Buffer.from(wasmUri.split(",")[1], 'base64');
  const wasm = new WebAssembly.Module(wasmBuffer);
  const instance = new WebAssembly.Instance(wasm, {
    "./website_playground_bg.js": require("website_playground/website_playground_bg.js")
  });
  __wbg_set_wasm(instance.exports);
}

import mermaid from "mermaid";

import styles from "./playground.module.css";

init();

function MermaidGraph({ id, source }) {
  const [svg, setSvg] = useState({ __html: 'Loading Mermaid graph...' });
  useEffect(() => {
    mermaid.render(id, source, svg => {
      setSvg({
        __html: svg,
      });
    });
  }, [source]);

  return <div className={styles["mermaidContainer"]} style={{
    marginTop: "-7px"
  }} dangerouslySetInnerHTML={svg}></div>;
}

const hydroflowExamples = {
  "Simplest": `\
// https://hydro.run/docs/hydroflow/quickstart/example_1_simplest
source_iter(0..10) -> for_each(|n| println!("Hello {}", n));`,

  "Simple": `\
// https://hydro.run/docs/hydroflow/quickstart/example_2_simple
source_iter(0..10)
  -> map(|n| n * n)
  -> filter(|&n| n > 10)
  -> map(|n| (n..=n+1))
  -> flatten()
  -> for_each(|n| println!("Howdy {}", n));`,

  "Chat Server": `\
// https://hydro.run/docs/hydroflow/quickstart/example_8_chat_server
// Define shared inbound and outbound channels
outbound_chan = union() -> dest_sink_serde(outbound);
inbound_chan = source_stream_serde(inbound)
    ->  demux(|(msg, addr), var_args!(clients, msgs, errs)|
            match msg {
                Message::ConnectRequest => clients.give(addr),
                Message::ChatMsg {..} => msgs.give(msg),
                _ => errs.give(msg),
            }
        );
clients = inbound_chan[clients] -> tee();
inbound_chan[errs] -> for_each(|m| println!("Received unexpected message type: {:?}", m));
// Pipeline 1: Acknowledge client connections
clients[0] -> map(|addr| (Message::ConnectResponse, addr)) -> [0]outbound_chan;
// Pipeline 2: Broadcast messages to all clients
broadcast = cross_join() -> [1]outbound_chan;
inbound_chan[msgs] -> [0]broadcast;
      clients[1] -> [1]broadcast;`,

  "Chat Client": `\
// https://hydro.run/docs/hydroflow/quickstart/example_8_chat_server
// set up channels
outbound_chan = union() -> dest_sink_serde(outbound);
inbound_chan = source_stream_serde(inbound) -> map(|(m, _)| m)
    ->  demux(|m, var_args!(acks, msgs, errs)|
            match m {
                Message::ConnectResponse => acks.give(m),
                Message::ChatMsg {..} => msgs.give(m),
                _ => errs.give(m),
            }
        );
inbound_chan[errs] -> for_each(|m| println!("Received unexpected message type: {:?}", m));
// send a single connection request on startup
source_iter([()]) -> map(|_m| (Message::ConnectRequest, server_addr)) -> [0]outbound_chan;
// take stdin and send to server as a msg
// the cross_join serves to buffer msgs until the connection request is acked
msg_send = cross_join() -> map(|(msg, _)| (msg, server_addr)) -> [1]outbound_chan;
lines = source_stdin()
  -> map(|l| Message::ChatMsg {
            nickname: opts.name.clone(),
            message: l.unwrap(),
            ts: Utc::now()})
  -> [0]msg_send;
inbound_chan[acks] -> [1]msg_send;
// receive and print messages
inbound_chan[msgs] -> for_each(pretty_print_msg);`,

  "Graph Neighbors": `\
// https://hydro.run/docs/hydroflow/quickstart/example_4_neighbors
// inputs: the origin vertex (vertex 0) and stream of input edges
origin = source_iter(vec![0]);
stream_of_edges = source_stream(edges_recv);
// the join
my_join = join() -> flat_map(|(src, (_, dst))| [src, dst]);
origin -> map(|v| (v, ())) -> [0]my_join;
stream_of_edges -> [1]my_join;
// the output
my_join -> unique() -> for_each(|n| println!("Reached: {}", n));`,

  "Graph Reachability": `\
// https://hydro.run/docs/hydroflow/quickstart/example_5_reachability
// inputs: the origin vertex (vertex 0) and stream of input edges
origin = source_iter(vec![0]);
stream_of_edges = source_stream(edges_recv);
reached_vertices = union();
origin -> [0]reached_vertices;
// the join
my_join_tee = join() -> flat_map(|(src, ((), dst))| [src, dst]) -> tee();
reached_vertices -> map(|v| (v, ())) -> [0]my_join_tee;
stream_of_edges -> [1]my_join_tee;
// the loop and the output
my_join_tee[0] -> [1]reached_vertices;
my_join_tee[1] -> unique() -> for_each(|x| println!("Reached: {}", x));`,

  "Graph Un-Reachability": `\
// https://hydro.run/docs/hydroflow/quickstart/example_6_unreachability
origin = source_iter(vec![0]);
stream_of_edges = source_stream(pairs_recv) -> tee();
reached_vertices = union()->tee();
origin -> [0]reached_vertices;
// the join for reachable vertices
my_join = join() -> flat_map(|(src, ((), dst))| [src, dst]);
reached_vertices[0] -> map(|v| (v, ())) -> [0]my_join;
stream_of_edges[1] -> [1]my_join;
// the loop
my_join -> [1]reached_vertices;
// the difference all_vertices - reached_vertices
all_vertices = stream_of_edges[0]
  -> flat_map(|(src, dst)| [src, dst]) -> tee();
unreached_vertices = difference();
all_vertices[0] -> [pos]unreached_vertices;
reached_vertices[1] -> [neg]unreached_vertices;
// the output
all_vertices[1] -> unique() -> for_each(|v| println!("Received vertex: {}", v));
unreached_vertices -> for_each(|v| println!("unreached_vertices vertex: {}", v));`
};

const datalogExamples = {
  "Simplest": `\
.input foo \`null()\`
.output bar \`null()\`
bar(x) :- foo(x)`,
  "Graph Reachability": `\
.input edges \`null()\`
.input seed_reachable \`null()\`
.output reachable \`null()\`
reachable(x) :- seed_reachable(x)
reachable(y) :- reachable(x), edges(x, y)`
};

function HydroflowSurfaceDemo() {
  return <EditorDemo compileFn={compile_hydroflow} examples={hydroflowExamples} mermaidId="mermaid-hydroflow"></EditorDemo>
}
function DatalogDemo() {
  return <EditorDemo compileFn={compile_datalog} examples={datalogExamples} mermaidId="mermaid-datalog"></EditorDemo>
}

function EditorDemo({ compileFn, examples, mermaidId }) {
  const [program, setProgram] = useState(Object.values(examples)[0]);
  const [showingMermaid, setShowingMermaid] = useState(true);
  const [editorAndMonaco, setEditorAndMonaco] = useState(null);

  const { output, diagnostics } = (compileFn)(program);
  const numberOfLines = program.split("\n").length;

  useEffect(() => {
    if (editorAndMonaco) {
      const { editor, monaco } = editorAndMonaco;
      monaco.editor.setModelMarkers(editor.getModel(), "hydroflow", diagnostics.map(d => {
        return {
          startLineNumber: d.span.start.line,
          startColumn: d.span.start.column + 1,
          endLineNumber: d.span.end ? d.span.end.line : numberOfLines + 1,
          endColumn: d.span.end ? d.span.end.column + 1 : 0,
          message: d.message,
          severity: d.is_error ? monaco.MarkerSeverity.Error : monaco.MarkerSeverity.Warning
        };
      }));
    }
  }, [editorAndMonaco, numberOfLines, diagnostics]);

  return <div style={{ display: "flex", flexWrap: "wrap" }}>
    <div className={styles["panel"]}>
      <span>Template: </span><select style={{
        fontFamily: "inherit",
        fontSize: "inherit",
        marginBottom: "5px"
      }} onChange={(e) => {
        setProgram(examples[e.target.value]);
      }}>{Object.keys(examples).map((name) => {
        return <option key={name} value={name}>{name}</option>;
      })}</select>
      <Editor
        height="70vh"
        theme="vs-dark"
        defaultLanguage="rust"
        value={program}
        onChange={(value, _event) => {
          setProgram(value);
        }}
        onMount={(editor, monaco) => {
          setEditorAndMonaco({ editor, monaco });
        }}
      />
    </div>
    <div className={styles["panel"]} style={{ marginRight: "0" }}>
      <div style={{ textAlign: "center", fontWeight: "700", marginBottom: "9px" }}>
        <a className={showingMermaid ? styles["selected-tab"] : styles["unselected-tab"]} style={{
          cursor: "pointer"
        }} onClick={() => {
          setShowingMermaid(true);
        }}>Graph</a> / <a className={!showingMermaid ? styles["selected-tab"] : styles["unselected-tab"]} style={{
          cursor: "pointer"
        }} onClick={() => {
          setShowingMermaid(false);
        }}>Compiled Rust</a>
      </div>
      {(() => {
        if (null == output) {
          return <div>
            <p>Failed to compile:</p>
            <ul>{diagnostics.map(diag => <li key={Math.random()}>{diag.is_error ? "Error" : "Warning"}: {diag.message} ({diag.span.start.line}:{diag.span.start.column})</li>)}</ul>
          </div>;
        }
        if (showingMermaid) {
          return <MermaidGraph id={mermaidId} source={output.mermaid} />;
        } else {
          return <Editor
            height="70vh"
            theme="vs-dark"
            defaultLanguage="rust"
            value={output.compiled}
            options={{
              readOnly: true
            }}
          />;
        }
      })()}
    </div>
  </div>
}

export default function Playground() {
  return (
    <Layout
      description="Playground for the Hydroflow compiler">
      <main>
        <div style={{
          maxWidth: "calc(min(1600px, 100vw - 60px))",
          marginLeft: "auto",
          marginRight: "auto",
          marginTop: "30px",
          marginBottom: "30px"
        }}>
          <h1 style={{
            fontSize: "3.5rem"
          }}>Playground</h1>
          <p>In these interactive editors, you can experiment with the Hydroflow compiler by running it in your browser (through WebAssembly)! Try selecting one of the templates or edit the code yourself to see how Hydroflow logic is compiled into a dataflow graph and executable Rust.</p>
          <h1 style={{
            fontSize: "2.5rem"
          }}>Hydroflow</h1>
          <HydroflowSurfaceDemo />
          <h1 style={{
            fontSize: "2.5rem"
          }}>Datalog</h1>
          <DatalogDemo />
        </div>
      </main>
    </Layout>
  );
}
