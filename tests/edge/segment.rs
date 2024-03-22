use i_float::bit_pack::BitPackVec;
use i_float::point::Point;
use i_float::triangle::Triangle;

#[derive(Debug, PartialEq)]
pub(crate) struct Segment {
    pub(crate) a: Point,
    pub(crate) b: Point,
    pub(crate) is_vertical: bool,
}

impl Segment {
    pub(crate) fn new(a: Point, b: Point) -> Self {
        assert!(a.x <= b.x);
        Segment {
            a,
            b,
            is_vertical: a.x == b.x,
        }
    }

    pub(crate) fn is_under(&self, p: &Point) -> bool {
        assert!(self.a.x <= p.x && p.x <= self.b.x);
        assert!(p != &self.a && p != &self.b);
        Triangle::is_clockwise_point(self.a, p.clone(), self.b)
    }

    pub(crate) fn is_above(&self, p: &Point) -> bool {
        assert!(self.a.x <= p.x && p.x <= self.b.x);
        Triangle::is_clockwise_point(self.a, self.b, p.clone())
    }

    pub(crate) fn is_under_segment(&self, other: &Segment) -> bool {
        if self.a == other.a {
            Triangle.isClockwisePoints(self.a, other.b, self.b)
        } else if self.a.x < other.a.x {
            Triangle.isClockwisePoints(self.a, other.a, self.b)
        } else {
            Triangle.isClockwisePoints(other.a, other.b, self.a)
        }
    }

    pub(crate) fn is_less(&self, other: Segment) -> bool {
        let a0 = self.a.bit_pack();
        let a1 = other.a.bit_pack();
        if a0 != a1 {
            a0 < a1
        } else {
            self.b.bit_pack() < other.b.bit_pack()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct IdSegment {
    pub(crate) index: usize,
    pub(crate) segment: Segment,
}

impl PartialOrd for IdSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.segment.is_under_segment(&other.segment).then(std::cmp::Ordering::Less).unwrap_or(std::cmp::Ordering::Greater))
    }
}

impl Eq for IdSegment {}

impl Ord for IdSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}