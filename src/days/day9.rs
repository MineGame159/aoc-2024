use crate::Day;

pub struct Day9 {}

impl Day for Day9 {
    fn part1(&self, lines: &[&str]) -> i64 {
        let mut blocks = parse(lines);
        compact_blocks(&mut blocks);

        checksum(&blocks)
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        let mut blocks = parse(lines);
        compact_files(&mut blocks);

        checksum(&blocks)
    }
}

fn compact_blocks(blocks: &mut Vec<Block>) {
    while has_empty_between_files(blocks) {
        let last_file = blocks.iter().rposition(|block| matches!(block, Block::File(_))).unwrap();
        let first_empty = blocks.iter().position(|block| matches!(block, Block::Empty)).unwrap();

        blocks.swap(last_file, first_empty);
    }
}

fn compact_files(blocks: &mut Vec<Block>) {
    let max_id = *match blocks.iter().rfind(|block| matches!(block, Block::File(_))).unwrap() {
        Block::File(id) => id,
        _ => panic!("Shouldn't happen"),
    };

    for id in (0..max_id + 1).rev() {
        // Find file

        let end_file_i = blocks
            .iter()
            .rposition(|block| matches!(block, Block::File(id2) if *id2 == id))
            .unwrap();

        let mut begin_file_i = end_file_i;

        while (begin_file_i as isize) - 1 >= 0 && matches!(blocks[begin_file_i-1], Block::File(id2) if id2 == id) {
            begin_file_i -= 1;
        }

        let file_size = end_file_i - begin_file_i + 1;

        // Find empty

        let mut begin_empty_i = usize::MAX;

        for i in 0..begin_file_i {
            let empty_size = blocks
                .iter()
                .skip(i)
                .take(file_size)
                .filter(|block| matches!(block, Block::Empty))
                .count();

            if empty_size == file_size {
                begin_empty_i = i;
                break;
            }
        }

        // Swap

        if begin_empty_i != usize::MAX {
            for i in 0..file_size {
                blocks.swap(begin_file_i + i, begin_empty_i + i);
            }
        }
    }
}

fn has_empty_between_files(blocks: &[Block]) -> bool {
    let mut seen_file = false;

    for block in blocks.iter().rev() {
        match block {
            Block::Empty => {
                if seen_file {
                    return true;
                }
            }
            Block::File(_) => seen_file = true,
        }
    }

    false
}

fn checksum(blocks: &[Block]) -> i64 {
    let mut sum = 0;

    for i in 0..blocks.len() {
        if let Block::File(id) = blocks[i] {
            sum += (id as i64) * (i as i64);
        }
    }

    sum
}

#[derive(Copy, Clone)]
enum Block {
    Empty,
    File(u32),
}

fn parse(lines: &[&str]) -> Vec<Block> {
    let mut blocks = Vec::new();

    let mut i = 0;
    let mut id: u32 = 0;

    for ch in lines[0].chars() {
        let block = if i % 2 == 0 {
            id += 1;
            Block::File(id - 1)
        } else {
            Block::Empty
        };

        let size = ch.to_digit(10).unwrap();

        for _ in 0..size {
            blocks.push(block);
        }

        i += 1;
    }

    blocks
}
