use super::ParserNode;
use std::fmt::{Formatter, Display, Result};
use std::cmp::max;


fn node_print_size(node: &ParserNode) -> usize {
    let mut printable = vec!();

    write!(printable, "{:?}", node.node_type);

    node.tokens.iter().for_each(|t| {
        write!(printable, "{:?}", t);
    });


    printable.len()
}
fn find_node_size(node: &ParserNode) -> usize {
    return max(
        node_print_size(node),
        node.children.iter().map(|child| node_print_size(child)).sum()
    );
}


impl Display for ParserNode {
        fn fmt(&self, f: &mut Formatter) -> Result {
            struct Info {
                pub my_size: usize,

            }
            let mut working_layers = vec!();
            let mut offsets = vec!();

            let mut children = vec!(self);
            let mut max_size = 1;


            while children.len() > 0 {
                offsets.push(vec!());

                working_layers.push(children);

                children = vec!();
                working_layers.last().unwrap().iter().for_each(|parent| {
                    parent.children.iter().for_each(|child| {
                        children.push(child.as_ref());
                    })
                });

                if children.len() > max_size {
                    max_size = children.len();
                }
            }

            offsets.iter().for_each(|mut sizes| {
                sizes.resize(max_size, 0);
            });


            working_layers.iter().for_each(|slice| {
                writeln!(f);
            });


            Ok(())
        }

}