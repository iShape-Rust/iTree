use i_float::point::Point;
use i_tree::tree::Tree;
use crate::edge::segment::{IdSegment, Segment};
use crate::edge::tree_scan::TreeScan;

pub(crate) struct TreePointSolver;

impl TreePointSolver {
    pub(crate) fn run(items: &Vec<IdSegment>, points: &Vec<Point>) -> Vec<usize> {
        let mut scan_list = TreeScan {
            tree: Tree::new(
                IdSegment { index: 0, segment: Segment::new(Point::new(0, 0), Point::new(0, 0)) },
                points.len(),
            )
        };

        let mut result = Vec::with_capacity(items.len());

        let mut i = 0;
        for p in points.iter() {
            while i < items.len() && items[i].segment.a.x <= p.x {
                if !items[i].segment.is_vertical && items[i].segment.b.x > p.x {
                    scan_list.insert(items[i].clone(), p.x);
                }
                i += 1;
            }

            if let Some(seg) = scan_list.find_under(p, p.x) {
                result.push(seg.index);
            } else {
                result.push(usize::MAX);
            }
        }

        result
    }
}