use crate::span::{CollisionType, Span};

///SpanTree structure. Invariant:
/// range of children will always be inside the range of the parent.
///
/// a single member may appear in multiple nodes in the tree, if it's been split up.
///
///
struct SpanTree<T, I> {
    span: Span<T>,
    members: Vec<I>,
    left: Option<Box<SpanTree<T, I>>>,
    right: Option<Box<SpanTree<T, I>>>,
}

impl<T, I> SpanTree<T, I>
where
    T: PartialOrd + Copy + Ord,
    I: Clone,
{
    pub fn new_empty(span: Span<T>) -> Self {
        Self {
            span,
            members: vec![],
            left: None,
            right: None,
        }
    }
    pub fn new_from_single(span: Span<T>, item: I) -> Self {
        Self {
            span,
            members: vec![item],
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, span: Span<T>, item: I) {
        let big_span = self.span.union(&span);
        match self.span.collide_with(&span) {
            CollisionType::Equal => {
                self.members.push(item);
            }
            CollisionType::Before(_) => {
                let mut new = Self::new_empty(big_span);
                std::mem::swap(&mut new, self);
                let new_right = Self::new_from_single(span, item);
                self.left = Some(Box::new(new));
                self.right = Some(Box::new(new_right));
            }
            CollisionType::After(_) => {
                let mut new = Self::new_empty(big_span);
                std::mem::swap(&mut new, self);
                let new_left = Self::new_from_single(span, item);
                self.left = Some(Box::new(new_left));
                self.right = Some(Box::new(new));
            }
            CollisionType::OverlapsStart(pre, shared, post) => {
                //just me in pre, both in shared, just other in post.
                //equiv to self.insert(shared, i).insert(post,i);
                self.insert(shared, item);
            }
            CollisionType::StrictlyBigger(_, _, _) => todo!(),
            CollisionType::StrictlySmaller(_, _, _) => todo!(),
            CollisionType::OverlapsEnd(_, _, _) => todo!(),
        }
        // if a.end < b.start {
        //     //non overlap. need a new empty self, self goes left, newbie goes right.
        //     let mut new = Self::new_empty(*a.start()..=*b.end());
        //     std::mem::swap(&mut new, self);
        //     let new_right = Self::new_from_single(range, item);
        //     *self.left = Some(new);
        //     *self.right = Some(new_right);
        // } else if b.end() < a.start() {
        //     //non overlap the other way.
        //     let mut new = Self::new_empty(*b.start()..=*a.end());
        //     std::mem::swap(&mut new, self);
        //     let new_left = Self::new_from_single(range, item);
        //     *self.left = Some(new_left);
        //     *self.right = Some(new);
        // }
    }
}
