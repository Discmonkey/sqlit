use super::ParserNode;
use std::fmt::{Formatter, Display, Result};
use std::cmp::max;
use std::io::Write;

fn node_print_size(node: &ParserNode) -> usize {
    let mut printable: Vec<u8> = Vec::new();

    write_once(node, &mut printable);


    printable.len()
}

fn find_node_size(node: &ParserNode) -> usize {
    max(node_print_size(node), node.children.iter().map(|child| find_node_size(child)).sum())
}

fn write_once(node: &ParserNode, f: &mut impl Write) -> Result {
    write!(f, "{:?} ", node.node_type);

    node.tokens.iter().for_each(|t| {
        write!(f, "{:?} ", t.get_text());
    });

    Ok(())
}

#[derive(Copy, Clone)]
struct Coord {
    row: usize,
    column: usize,
}

fn fill_offsets(offsets: &mut Vec<Vec<usize>>,
                node: &ParserNode,
                coord: Coord,
                parent_offset: usize) {

    let mut offset = parent_offset;
    let column_base = if offsets.len() > coord.row + 1 {
        offsets[coord.row + 1].len()
    } else {
        0
    };

    node.children.iter().enumerate().for_each(|(num, child)| {
        let child_coord = Coord{row: coord.row + 1, column: column_base + num};

        while child_coord.row + 1 > offsets.len() {
            offsets.push(Vec::new());
        }

        while child_coord.column + 1 > offsets[child_coord.row].len() {
            offsets[child_coord.row].push(0);
        }

        offsets[child_coord.row][child_coord.column] = offset;

        fill_offsets(offsets, child, child_coord, offset);

        offset += find_node_size(child);
    });
}

fn write_to_vec(offsets: &mut Vec<Vec<usize>>,
                writes: &mut Vec<Vec<u8>>,
                column_counters: &mut Vec<usize>,
                node: &ParserNode,
                coord: Coord) {

    while coord.row + 1 > writes.len() {
        writes.push(Vec::new());
    }

    while coord.row + 2 > column_counters.len() {
        column_counters.push(0);
    }

    while writes[coord.row].len() < offsets[coord.row][coord.column] {
        write!(writes[coord.row], "{}", ' ');
    }

    let _ = write_once(node, &mut writes[coord.row]);

    node.children.iter().for_each(|child| {
        let row = coord.row + 1;
        let column = column_counters[row];
        write_to_vec(offsets, writes, column_counters, child.as_ref(), Coord {
            row, column
        });

        column_counters[row] += 1;
    });
}


impl Display for ParserNode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut offsets: Vec<Vec<usize>> = Vec::new();
        offsets.push(vec![0]);

        let mut writes = Vec::new();
        let mut column_counters = Vec::new();

        fill_offsets(&mut offsets, self, Coord{row: 0, column: 0}, 0);
        write_to_vec(&mut offsets, &mut writes, &mut column_counters, self, Coord{row: 0, column: 0});

        for v in writes.iter() {
            writeln!(f, "{}", String::from_utf8(v.clone()).unwrap())?;
        };

        Ok(())
    }
}