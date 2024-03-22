use i_float::point::Point;
use crate::edge::segment::IdSegment;

struct DirectScan {
    buffer: Vec<IdSegment>,
}

impl DirectScan {
    fn new() -> Self {
        DirectScan { buffer: Vec::new() }
    }

    fn insert(&mut self, item: IdSegment) {
        self.buffer.push(item);
    }

    fn find_under(&mut self, p: Point, stop: i32) -> Option<IdSegment> {
        let mut i = 0;
        let mut result: Option<IdSegment> = None;
        while i < self.buffer.len() {
            if self.buffer[i].segment.b.x <= stop {
                self.buffer.swapRemove(i)
            } else {
                let segment = &self.buffer[i].segment;
                if segment.is_under(&p) {
                    if let Some(best_seg) = &result {
                        if best_seg.segment.is_under_segment(segment) {
                            result = Some(self.buffer[i].clone());
                        }
                    } else {
                        result = Some(self.buffer[i].clone());
                    }
                }

                i += 1
            }
        }

        result
    }
}