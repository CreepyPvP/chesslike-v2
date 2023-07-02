use bevy::prelude::Vec2;

#[derive(Clone)]
pub struct Triangle {
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
}

impl Triangle {
    pub fn new(p1: Vec2, p2: Vec2, p3: Vec2) -> Self {
        Triangle { p1, p2, p3 }
    }
}

impl Triangle {
    fn sign(p1: &Vec2, p2: &Vec2, p3: &Vec2) -> f32 {
        (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
    }

    // see: https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
    pub fn contains(&self, pt: &Vec2) -> bool {
        let (d1, d2, d3) = (
            Self::sign(pt, &self.p1, &self.p2),
            Self::sign(pt, &self.p2, &self.p3),
            Self::sign(pt, &self.p3, &self.p1),
        );

        let has_neg = (d1 < 0.) || (d2 < 0.) || (d3 < 0.);
        let has_pos = (d1 > 0.) || (d2 > 0.) || (d3 > 0.);

        !(has_neg && has_pos)
    }
}
