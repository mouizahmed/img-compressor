
struct NodeChildren {
    top_left_idx: usize,
    top_right_idx: usize,
    bottom_left_idx: usize,
    bottom_right_idx: usize,
}

struct Node {
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    children: Optional<NodeChildren>,
}

impl Node {
    pub fn leaf(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
            children: None,
        }
    }
    
    fn height(&self) -> u64 {
        (self.bottom_right.0 - self.top_left.0) as u64
    }

    fn width(&self) -> u64 {
        (self.bottom_right.1 - self.top_left.1) as u64
    }

    fn can_split(&self) -> bool {
        self.height() > 1 && self.width() > 1
    }

    fn split(&self) -> Option<(Node, Node, Node, Node)> {
        if !self.can_split() {
            return None;
        }

        let height_mid = (self.top_left.0 + self.bottom_right.0) / 2;
        let width_mid = (self.top_left.1 + self.bottom_right.1) / 2;

        let top_left_idx = Node::leaf(self.top_left, (height_mid, width_mid));
        let top_right_idx = Node::leaf((self.top_left.0, width_mid + 1), self.bottom_right);
        let bottom_left_idx = Node::leaf((height_mid + 1, self.top_left.1), self.bottom_right);
        let bottom_right_idx = Node::leaf((height_mid + 1, width_mid + 1), self.bottom_right);

        Some((top_left_idx, top_right_idx, bottom_left_idx, bottom_right_idx))
    }
}

struct OrdNode {
    node_idx: usize,
    variance: u64,
}


