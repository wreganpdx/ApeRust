use crate::vector::Vector;

#[derive(Default, Debug)]
pub struct OwnerCollision {
    pub mtd: Vector,
    pub vel: Vector,
    pub n: Vector,
    pub d: f64,
    pub o: i32,
    pub ownerRect: i64,
    pub ownerConstraint: i64,
    pub sibling1: i64,
    pub sibling2: i64,
    pub collider: i64,
}

impl OwnerCollision {
    pub fn new(
        mtd: &Vector,
        vel: &Vector,
        n: &Vector,
        d: f64,
        o: i32,
        collider: i64,
        ownerRect: i64,
        ownerConstraint: i64,
        sibling1: i64,
        sibling2: i64,
    ) -> OwnerCollision {
        let mtd: Vector = mtd.clone();
        let vel: Vector = vel.clone();
        let n: Vector = n.clone();
        let d: f64 = d.clone();
        let o: i32 = o.clone();
        let ownerRect: i64 = ownerRect.clone();
        let ownerConstraint: i64 = ownerConstraint.clone();
        let sibling1: i64 = sibling1.clone();
        let sibling2: i64 = sibling2.clone();
        let collider: i64 = collider.clone();
        OwnerCollision {
            mtd: mtd,
            vel: vel,
            n: n,
            d: d,
            o: o,
            collider: collider,
            ownerRect: ownerRect,
            ownerConstraint: ownerConstraint,
            sibling1: sibling1,
            sibling2: sibling2,
        }
    }

    pub fn clone(&self) -> OwnerCollision {
        let mtd: Vector = self.mtd.clone();
        let vel: Vector = self.vel.clone();
        let n: Vector = self.n.clone();
        let d: f64 = self.d.clone();
        let o: i32 = self.o.clone();
        let ownerRect: i64 = self.ownerRect.clone();
        let ownerConstraint: i64 = self.ownerConstraint.clone();
        let sibling1: i64 = self.sibling1.clone();
        let sibling2: i64 = self.sibling2.clone();
        let collider: i64 = self.collider.clone();
        OwnerCollision {
            mtd: mtd,
            vel: vel,
            n: n,
            d: d,
            o: o,
            collider: collider,
            ownerRect: ownerRect,
            ownerConstraint: ownerConstraint,
            sibling1: sibling1,
            sibling2: sibling2,
        }
    }
}
