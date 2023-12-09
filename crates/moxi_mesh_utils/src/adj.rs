use crate::*;
use cube::*;

/// This function will iron out any problems caused by generating two chunks that are adjacent to
/// each other seperatly. For example, it will cull the unneeded vertices between them.
/// reg: the [`MeshRegistry`]
/// main_mesh: the [`Mesh`] to change
/// main_md: the [`metadata`](`CubeMD`) of the mesh to change (must be cube mesh)
/// connection_side: from the POV of the main mesh, where is the adjacent mesh?
/// adjacent_chunk_grid: the grid of the chunk to introduce
pub fn introduce_adjacent_chunks<B: BlockInGrid, const N: usize>(
    reg: &impl MeshRegistry<B>,
    main_md: &mut CubeMD<B>,
    connection_side: Face,
    adjacent_chunk_grid: &ChunkGrid<B, N>,
) {
    assert_eq!(
        adjacent_chunk_grid.len(),
        main_md.vivi.vivi.len(),
        "Cannot introduce chunks with different sizes to each other"
    );
    let dims = main_md.dims;
    for block_pos in iter_blocks_on_edge(connection_side, dims) {
        if !main_md.quad_exists(block_pos, connection_side) {
            continue;
        }
        let adj_block_pos = neighbor_across_chunk(block_pos, connection_side, dims).unwrap();

        let adj_block = adjacent_chunk_grid.get_block(adj_block_pos).unwrap();
        if reg.is_cube(&adj_block) {
            let mut tmp = [None; 6];
            tmp[connection_side as usize] = Some(adj_block);
            main_md.log(BlockMeshChange::CullFaces, block_pos, adj_block, tmp)
        }
    }
}
