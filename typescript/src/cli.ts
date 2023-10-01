#!/usr/bin/env node

class GraphMap<TNode, TEdge> {
  private nodes = new Map<TNode, TNode[]>();
  private edges = new Map<[TNode, TNode], TEdge>();

  addNode(node: TNode) {
    this.nodes.set(node, []);
  }

  addEdge(from: TNode, to: TNode, weight: TEdge) {
    this.edges.set([from, to], weight);
    this.nodes.get(from)?.push(to);
  }
}

function main() {
  console.log("Hello world!");
}

main();
