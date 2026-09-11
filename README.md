# Voxel Orientations

## What is a "Voxel Orientation"?
In blocky voxel games such as Minecraft, Hytale, or Vintage
Story, there are some blocks that can be reoriented such as
stairs, slabs, or logs. These blocks have asymmetrical
appearances, and are often composed of models that are not
cubes. Due to their asymmetrical appearance, they look different
at different angles. If you want asymmetrical blocks in your
game, you're likely going to want to rotate or mirror those
blocks. Usually, voxel engine devs will take the easy route and
and just implement a few orientations, such as axis aligned
orientations, or rotations around the Y axis. Occassionally you
will see someone that has implemented 6 or 24 rotations. But you
are very unlikely to find a voxel engine that has implemented
the full [Octahedral Symmetry Group](https://en.wikipedia.org/wiki/Octahedral_symmetry) (the 48 orientations of a
cube). The full octahedral symmetry group is a seemingly
untenable goal for many voxel engine developers, and so they
don't even make the attempt. But I'm here to tell you that it's
actually far easier than you might think to implement the full
octahedral symmetry group for your voxel engine. In this guide,
I'm going to do my best to explain how to do that. I will be
using Rust code examples. If you would like to see an already
completed solution, you can find one [here](https://github.com/ErisianArchitect/voxel/tree/master/crates/voxel_orient).

## The Octahedral Symmetry Group

I'm not going to get into the nitty gritty abstract algebra of
the Octahedral Symmetry group. Mostly because I don't know, but
also because you don't need to know. I will tell you as much as
I think is relevant for implementing these 48 orientations.

### Base Rotations

There are 24 base rotations. These are all of the ways that you
can reorient a cube without inverting the geometry. For each of
the six sides of a cube, there are 4 rotations. You can
mathematically represent this as a base vector and an angle
around that base vector. For this guide, I will be using the Up
direction as the base vector, and CCW angles. It does not matter
what you pick to be the Up direction of your engine. It could
be `+Y`, `-Y`, `+X`, `-X`, `+Z`, or `-Z`. Throughout this guide,
I will refer to these directions as `PosY`, `NegY`, `PosX`, `NegX`,
`PosZ`, and `NegZ`.

Now we have a base vector (Let's pick PosY), and an angle in 90
degree increments. But we also have to pick a forward direction
as the basis for Angle 0. In this guide, I will be using NegZ as
the forward direction. PosX will be the right direction.

This coordinate system is the right-hand coordinate system. You
can implement voxel orientations for any coordinate system, but
for this guide, I will be using the right-handed system with
PosY as the Upward direction, NegZ as the forward direction, and
PosX as the rightward direction.

So a Rotation is defined as the direction of the Up vector (the
direction that PosY on the cube is pointing), and the angle
around that up direction in 90 degree counter-clockwise
increments.

But there's also one more thing that must be established. The
angle of each of the faces of the cube in relation to the other
faces. I've found that the best choice is to have the Up face
pointing towards the Forward face, the Left/Right/Front/Back
faces pointing towards the Up face, and the Bottom face
pointing towards the Back face. This configuration seems to have
ideal geometric properties. There may be a better configuration
that I had not thought of.

### Reflected Rotations

Now that we have the 24 rotations established, the full
octahedral symmetry group is created by simply adding a single
reflection bit. For each of the 24 rotations, there is a
reflected version of that rotation. Which axis is reflected is
your choice. For my implementation, I allowed the granulity of
all three axes for reflections, which has pros and cons. In
hind-sight, though, I would not recommend this approach, as the
Cayley (lookup) tables become very unwieldy, and can bloat
program memory by kilobytes, and are not very good for cache.
If you want a highly optimized implementation, it is my
recommendation that you stick to a single reflection axis, and
make your implementation work for that axis. It will likely save
you a lot of trouble, and you can achieve the same things as if
there you had all three reflection axes.

This is the full octahedral symmetry group.

## Implementation

Now that we've established what voxel orientations are, I can
make an attempt to explain how they can be implemented. I will
warn you, your implementation will need many lookup tables. I
will guide you on how to generate these lookup tables.

### Types

You'll need a handful of types for the implementation of voxel
orientations. These types are a Direction/Face enum, a Rotation,
and an Orientation.

#### Direction

The first type I'll discuss is the Direction type. This is an
enum with 6 variants, one for each side of the cube. It is
important that you align the discriminants of these enum
variants with the geometric identity of your base orientation.
What this means is that whatever you choose as your base vector
(in our case, PosY) must have a discriminant value of 0. This
will cause your enum variants to not have a lexicographic
ordering, but you can fix that by chooseing either NegX or PosX
as your base vector. But my recommendation is to use whatever
your Up direction is as your base vector, and NegX/PosX are
typically not Up vectors.

```rust
pub enum Direction {
    // IMPORTANT: The value of the discriminants is important! Do not change! (2025-12-28)
    /// Left
    NegX = 4,
    /// Down
    NegY = 3,
    /// Forward
    NegZ = 5,
    /// Right
    PosX = 1,
    /// Up
    PosY = 0,
    /// Back
    PosZ = 2,
}
```

This Direction type is very important to the functionality of
the voxel orientation implementation. It's how you will query
information about oriented faces. It is also how you will
compose your Rotation.

#### Rotation

The Rotation type is a combination of the Direction type and a
2-bit integer that represents the CCW angle around the base
vector.

In my implementation, I used a bitmask. Since there are 6
directions, and 4 angles, you will want to arrange this bitmask
so that the first two bits are the angle, and the next 3 bits
are the Direction. This representation makes it easy to
logically cycle through Rotations, which is a very convenient
property. It also means that your Rotations will have contiguous
bit representations without gaps.

*Work in progress...*
