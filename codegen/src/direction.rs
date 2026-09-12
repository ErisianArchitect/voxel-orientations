
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Face {
    // The order of the discriminants is arbitrary, but it is
    // helpful to choose an ordering that maintains that your
    // base up vector is 0.
    NegX = 4,
    NegY = 3,
    NegZ = 5,
    PosX = 1,
    PosY = 0,
    PosZ = 2,
}

use Face::*;

/// A padded Cayley table.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
struct FaceCayley<T: Clone + Copy>([T; 6]);

impl<T: Clone + Copy> FaceCayley<T> {
    #[must_use]
    #[inline(always)]
    const fn get(self, item: Face) -> T {
        self.0[item as usize]
    }
}

impl Face {
    // --- CAYLEY TABLES ---

    // ========================
    //            UP
    //        +--------+
    //        |        |
    //   Left |        | Right
    //        |        |
    //        +--------+
    //           DOWN
    // ========================
    // For each face of the cube, the face has an orientation
    // relative to the rest of the cube. These tables determine
    // that orientation.
    //                       Discriminant Order: PosY, PosX, PosZ, NegY, NegX, NegZ
    const UP:     FaceCayley<Face> = FaceCayley([NegZ, PosY, PosY, PosZ, PosY, PosY]);
    const LEFT:   FaceCayley<Face> = FaceCayley([NegX, PosZ, NegX, NegX, NegZ, PosX]);
    const DOWN:   FaceCayley<Face> = FaceCayley([PosZ, NegY, NegY, NegZ, NegY, NegY]);
    const RIGHT:  FaceCayley<Face> = FaceCayley([PosX, NegZ, PosX, PosX, PosZ, NegX]);

    // Within this implementation of voxel orientations, we are
    // going to use counter-clockwise angles. This means that at
    // angle `0`, a face will be have its `UP` direction facing
    // `UP`, and at angle `1`, the `UP` direction will be facing
    // `LEFT`. Modify the order of these tables if you prefer
    // clockwise angles.
    const UP_AT_ANGLE: [FaceCayley<Face>; 4] = [
        Self::UP,
        Self::LEFT,
        Self::DOWN,
        Self::RIGHT,
    ];

    const LEFT_AT_ANGLE: [FaceCayley<Face>; 4] = [
        Self::LEFT,
        Self::DOWN,
        Self::RIGHT,
        Self::UP,
    ];

    const DOWN_AT_ANGLE: [FaceCayley<Face>; 4] = [
        Self::DOWN,
        Self::RIGHT,
        Self::UP,
        Self::LEFT,
    ];

    const RIGHT_AT_ANGLE: [FaceCayley<Face>; 4] = [
        Self::RIGHT,
        Self::UP,
        Self::LEFT,
        Self::DOWN,
    ];

    const INVERT: FaceCayley<Face> = FaceCayley([NegY, NegX, NegZ, PosY, PosX, PosZ]);

    // --- LOOKUP FUNCTIONS ---

    #[must_use]
    #[inline(always)]
    pub const fn up(self) -> Self {
        Self::UP.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn up_at_angle(self, angle: i32) -> Self {
        Self::UP_AT_ANGLE[(angle & 3) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn left(self) -> Self {
        Self::LEFT.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn left_at_angle(self, angle: i32) -> Self {
        Self::LEFT_AT_ANGLE[(angle & 3) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn down(self) -> Self {
        Self::DOWN.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn down_at_angle(self, angle: i32) -> Self {
        Self::DOWN_AT_ANGLE[(angle & 3) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn right(self) -> Self {
        Self::RIGHT.get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn right_at_angle(self, angle: i32) -> Self {
        Self::RIGHT_AT_ANGLE[(angle as i32) as usize].get(self)
    }

    #[must_use]
    #[inline(always)]
    pub const fn invert(self) -> Self {
        Self::INVERT.get(self)
    }
}
