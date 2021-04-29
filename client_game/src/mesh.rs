use std::collections::HashSet;
use bevy::{prelude::*, render::{mesh::Indices, pipeline::PrimitiveTopology}};

use crate::{dungeons::inst::Dungeon, res::{ARENA_HEIGHT, ARENA_WIDTH, UNIT_SIZE}};

pub fn gen_dun_mesh(dun: &Dungeon) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let mut square_count = 0;

    let paths = dun.paths().collect::<HashSet<&(u32, u32)>>();
    for i in 0..dun.width() {
        for j in 0..dun.height() {
            if !paths.contains(&(i, j)) {
                vertices.push(([(i * UNIT_SIZE) as f32, (j * UNIT_SIZE) as f32, 0.0], [0.0, 0.0]));
                vertices.push(([(i * UNIT_SIZE) as f32, (j * UNIT_SIZE + ARENA_HEIGHT) as f32 , 0.0], [0.0, 1.0]));
                vertices.push(([(i * UNIT_SIZE + ARENA_WIDTH) as f32, (j * UNIT_SIZE) as f32, 0.0], [1.0, 0.0]));
                vertices.push(([(i * UNIT_SIZE + ARENA_WIDTH) as f32, (j * UNIT_SIZE + ARENA_HEIGHT) as f32, 0.0], [1.0, 1.0]));

                indices.extend(vec![square_count, square_count + 1, square_count + 2,
                                    square_count + 2, square_count + 1, square_count + 3]);
                square_count += 4;
            }
        }
    }

    let mut positions = Vec::with_capacity(vertices.len());
    let mut uvs = Vec::with_capacity(vertices.len());

    for (pos, uv) in vertices.iter() {
        positions.push(*pos);
        uvs.push(*uv);
    }

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}