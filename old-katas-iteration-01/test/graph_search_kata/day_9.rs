pub use tdd_kata::graph_search_kata::day_9::{UnidirectedGraph, DepthFirstSearch, BreadthFirstSearch};

pub use expectest::prelude::{be_equal_to, be_some, be_none, be_true, be_false};

describe! graph_search {

    before_each {
        let mut graph = UnidirectedGraph::default();
    }

    describe! graph {

        it "should create a new emoty graph" {
            expect!(graph.vertices()).to(be_equal_to(0));
            expect!(graph.edges()).to(be_equal_to(0));
        }

        it "should add an edge to a graph" {
            graph.add_edge(1, 2);

            expect!(graph.vertices()).to(be_equal_to(2));
            expect!(graph.edges()).to(be_equal_to(1));
        }

        it "should add edges to a graph" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(1, 4);

            expect!(graph.vertices()).to(be_equal_to(4));
            expect!(graph.edges()).to(be_equal_to(3));
        }

        it "should have adjacent to each other" {
            graph.add_edge(1, 2);

            expect!(graph.adjacent_to(1)).to(be_some().value(&vec![2]));
            expect!(graph.adjacent_to(2)).to(be_some().value(&vec![1]));
        }
    }

    describe! dfs {

        it "should create a depth first search" {
            graph.add_edge(1, 1);

            expect!(DepthFirstSearch::new(&graph, 1)).to(be_some());
        }

        it "should not create a depth first search from an empty graph" {
            expect!(DepthFirstSearch::new(&graph, 1)).to(be_none());
        }

        it "should have path to transient vertices" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);

            let search = DepthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(3)).to(be_true());
        }

        it "should not have path to not connected vertices" {
            graph.add_edge(1, 2);
            graph.add_edge(3, 4);

            let search = DepthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(4)).to(be_false());
        }
    }

    describe! bfs {

        it "should create a breadth first search" {
            graph.add_edge(1, 1);

            expect!(BreadthFirstSearch::new(&graph, 1)).to(be_some());
        }

        it "should not create a breadth first search from an empty graph" {
            expect!(BreadthFirstSearch::new(&graph, 1)).to(be_none());
        }

        it "should have path to transient vertices" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);

            let search =BreadthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(3)).to(be_true());
        }

        it "should not have path to not connected vertices" {
            graph.add_edge(1, 2);
            graph.add_edge(3, 4);

            let search = BreadthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(4)).to(be_false());
        }
    }
}
