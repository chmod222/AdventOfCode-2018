type NodeComponent = usize;

#[derive(Debug)]
struct Node<'a>(
    Vec<Node<'a>>, // children
    &'a [NodeComponent] // metadata
);

impl Node<'_> {
    fn sum_metadata(&self) -> usize {
        let mut base_sum = self.1.iter().cloned().sum();

        for c in &self.0 {
            base_sum += c.sum_metadata();
        }

        base_sum
    }

    fn value(&self) -> usize {
        if self.0.len() > 0 {
            let mut base_sum = 0;

            for cid in self.1 {
                if *cid == 0 {
                    continue;
                }

                if let Some(n) = self.0.get(cid - 1) {
                    base_sum += n.value()
                }
            }
            
            base_sum
        } else {
            self.sum_metadata()
        }
    }
}


fn read_node<'a>(ser: &'a [NodeComponent]) -> (Node<'a>, usize) {
    let num_children = ser[0];
    let num_metadata = ser[1] as usize;

    let mut offset = 2;
    let mut children = vec![];

    for _ in 0..num_children {
        let (c, read) = read_node(&ser[offset..]);

        children.push(c);
        offset += read;
    }

    let metadata = &ser[offset..offset+num_metadata];
    offset += num_metadata;

    (Node(children, metadata), offset)
}

fn main() {
    let numbers = shared::input::read_stdin_lines().expect("could not lock stdin");
    let numbers = numbers.first().expect("no input?");
    let numbers: Vec<NodeComponent> = numbers.split(' ').filter_map(|s| s.parse().ok()).collect();

    let (tree, _) = read_node(&numbers);

    println!("Part 1: {}", tree.sum_metadata());
    println!("Part 2: {}", tree.value());
}