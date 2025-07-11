use crate::image_processor::{ImageData, RGB};
use image::{ImageBuffer, Pixel, Rgb, RgbImage, Rgba, RgbaImage};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

struct NodeChildren {
    top_left_idx: usize,
    top_right_idx: usize,
    bottom_left_idx: usize,
    bottom_right_idx: usize,
}

struct Node {
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    children: Option<NodeChildren>,
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
        (self.bottom_right.0 as u64) - (self.top_left.0 as u64)
    }

    fn width(&self) -> u64 {
        (self.bottom_right.1 as u64) - (self.top_left.1 as u64)
    }

    fn can_split(&self) -> bool {
        self.width() > 1 && self.height() > 1
    }

    fn split(&self) -> Option<(Node, Node, Node, Node)> {
        if !self.can_split() {
            return None;
        }

        let split_h = (self.top_left.0 + self.bottom_right.0) / 2;
        let split_w = (self.top_left.1 + self.bottom_right.1) / 2;

        let top_left_node = Node::leaf(self.top_left, (split_h, split_w));
        let top_right_node = Node::leaf(
            (self.top_left.0, split_w + 1),
            (split_h, self.bottom_right.1),
        );
        let bottom_left_node = Node::leaf(
            (split_h + 1, self.top_left.1),
            (self.bottom_right.0, split_w),
        );
        let bottom_right_node = Node::leaf((split_h + 1, split_w + 1), self.bottom_right);

        Some((
            top_left_node,
            top_right_node,
            bottom_left_node,
            bottom_right_node,
        ))
    }
}

struct OrdNode {
    node_idx: usize,
    variance: u64,
}

impl OrdNode {
    pub fn new(nodes: &Vec<Node>, idx: usize, image_data: &ImageData) -> Self {
        let top_left = nodes[idx].top_left;
        let bottom_right = nodes[idx].bottom_right;
        Self {
            node_idx: idx,
            variance: image_data.variance(top_left, bottom_right),
        }
    }
}

impl PartialEq for OrdNode {
    fn eq(&self, other: &Self) -> bool {
        self.variance == other.variance
    }
}

impl Eq for OrdNode {}

impl PartialOrd for OrdNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.variance.cmp(&other.variance))
    }
}

impl Ord for OrdNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.variance.cmp(&other.variance)
    }
}

pub struct QuadTree {
    image_data: ImageData,
    nodes: Vec<Node>,
    priority_queue: BinaryHeap<OrdNode>,
    dimensions: (usize, usize),
}

const MAX_ALPHA: u8 = 100;

impl QuadTree {
    pub fn new(image_data: ImageData) -> Self {
        let dimensions = (image_data.height(), image_data.width());
        let root = Node::leaf((0, 0), (dimensions.0 - 1, dimensions.1 - 1));
        let nodes = vec![root];
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(OrdNode::new(&nodes, 0, &image_data));

        Self {
            image_data,
            nodes,
            priority_queue,
            dimensions,
        }
    }

    fn push_node(&mut self, node: Node) -> usize {
        let new_node_idx = self.nodes.len();
        self.nodes.push(node);
        new_node_idx
    }

    pub fn split_next(&mut self) -> Result<(), String> {
        loop {
            let Some(top) = self.priority_queue.pop() else {
                return Err("No more nodes to split".to_string());
            };

            if let Some((top_left, top_right, bottom_left, bottom_right)) =
                self.nodes[top.node_idx].split()
            {
                let top_left_idx = self.push_node(top_left);
                let top_right_idx = self.push_node(top_right);
                let bottom_left_idx = self.push_node(bottom_left);
                let bottom_right_idx = self.push_node(bottom_right);

                let parent_node = &mut self.nodes[top.node_idx];
                parent_node.children = Some(NodeChildren {
                    top_left_idx,
                    top_right_idx,
                    bottom_left_idx,
                    bottom_right_idx,
                });

                for child in [
                    top_left_idx,
                    top_right_idx,
                    bottom_left_idx,
                    bottom_right_idx,
                ]
                .into_iter()
                {
                    self.priority_queue
                        .push(OrdNode::new(&self.nodes, child, &self.image_data));
                }

                return Ok(());
            }
        }
    }

    pub fn render<T>(
        &self,
        color_to_pixel: fn(RGB<u64>) -> T,
        outline: Option<RGB<u8>>,
    ) -> ImageBuffer<T, Vec<u8>>
    where
        T: Pixel<Subpixel = u8>,
    {
        let (height, width) = self.dimensions;
        let mut image = ImageBuffer::new(width as u32, height as u32);

        let outline_pixel = outline.map(|c| color_to_pixel(c.into()));

        let mut queue = VecDeque::new();
        queue.push_back(0);

        while let Some(current_node_idx) = queue.pop_front() {
            let node = &self.nodes[current_node_idx];

            if let Some(children) = &node.children {
                queue.push_back(children.top_left_idx);
                queue.push_back(children.top_right_idx);
                queue.push_back(children.bottom_left_idx);
                queue.push_back(children.bottom_right_idx);
            } else {
                let (start_y, start_x) = node.top_left;
                let (end_y, end_x) = node.bottom_right;
                let color = self.image_data.average(node.top_left, node.bottom_right);
                let pixel = color_to_pixel(color);

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        image.put_pixel(x as u32, y as u32, pixel);
                    }
                }

                if let Some(outline_pixel) = outline_pixel {
                    for y in [start_y, end_y].into_iter() {
                        for x in start_x..=end_x {
                            image.put_pixel(x as u32, y as u32, outline_pixel);
                        }
                    }

                    for x in [start_x, end_x].into_iter() {
                        for y in start_y..=end_y {
                            image.put_pixel(x as u32, y as u32, outline_pixel);
                        }
                    }
                }
            }
        }

        image
    }

    pub fn render_rgb(&self, outline: Option<RGB<u8>>) -> RgbImage {
        self.render(
            |color| Rgb([color.r as u8, color.g as u8, color.b as u8]),
            outline,
        )
    }

    pub fn render_rgba(&self, outline: Option<RGB<u8>>) -> RgbaImage {
        self.render(
            |color| Rgba([color.r as u8, color.g as u8, color.b as u8, MAX_ALPHA]),
            outline,
        )
    }
}
