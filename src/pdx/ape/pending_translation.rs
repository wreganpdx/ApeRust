use crate::vector::Vector;

#[derive(Default, Debug)]
pub struct PendingTranslation {
    pub loc: Vector,
    pub vel: Vector,
    pub radian: f64,
    pub id: i64,
}

impl PendingTranslation {
    pub fn new(loc: &Vector, vel: &Vector, radian: &f64, id: i64) -> PendingTranslation {
        let loc: Vector = loc.clone();
        let vel: Vector = vel.clone();
        let radian: f64 = radian.clone();
        let id: i64 = id.clone();
        PendingTranslation {
            loc: loc,
            vel: vel,
            radian: radian,
            id: id,
        }
    }

    pub fn clone(&self) -> PendingTranslation {
        let loc: Vector = self.loc.clone();
        let vel: Vector = self.vel.clone();
        let radian: f64 = self.radian.clone();
        let id: i64 = self.id.clone();
        PendingTranslation {
            loc: loc,
            vel: vel,
            radian: radian,
            id: id,
        }
    }
}
