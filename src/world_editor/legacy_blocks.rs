//! Legacy block format support for Minecraft 1.8.9
//!
//! Converts modern block names to 1.8.9 numeric block IDs and metadata values.

use crate::block_definitions::Block;

/// Maps a block name to (block_id, metadata) for Minecraft 1.8.9
pub fn get_legacy_block_id(block_name: &str) -> (u8, u8) {
    match block_name {
        "air" => (0, 0),
        "stone" => (1, 0),
        "granite" => (1, 1),
        "polished_granite" => (1, 2),
        "diorite" => (1, 3),
        "polished_diorite" => (1, 4),
        "andesite" => (1, 5),
        "polished_andesite" => (1, 6),
        "grass_block" => (2, 0),
        "dirt" => (3, 0),
        "coarse_dirt" => (3, 1),
        "podzol" => (3, 2),
        "cobblestone" => (4, 0),
        "oak_planks" => (5, 0),
        "spruce_planks" => (5, 1),
        "birch_planks" => (5, 2),
        "jungle_planks" => (5, 3),
        "acacia_planks" => (5, 4),
        "dark_oak_planks" => (5, 5),
        "sand" => (12, 0),
        "red_sand" => (12, 1),
        "gravel" => (13, 0),
        "gold_ore" => (14, 0),
        "iron_ore" => (15, 0),
        "coal_ore" => (16, 0),
        "oak_log" => (17, 0),
        "spruce_log" => (17, 1),
        "birch_log" => (17, 2),
        "jungle_log" => (17, 3),
        "oak_leaves" => (18, 0),
        "spruce_leaves" => (18, 1),
        "birch_leaves" => (18, 2),
        "jungle_leaves" => (18, 3),
        "sponge" => (19, 0),
        "glass" => (20, 0),
        "sandstone" => (24, 0),
        "chiseled_sandstone" => (24, 1),
        "smooth_sandstone" => (24, 2),
        "short_grass" | "grass" => (31, 1),
        "fern" => (31, 2),
        "white_wool" => (35, 0),
        "orange_wool" => (35, 1),
        "magenta_wool" => (35, 2),
        "light_blue_wool" => (35, 3),
        "yellow_wool" => (35, 4),
        "lime_wool" => (35, 5),
        "pink_wool" => (35, 6),
        "gray_wool" => (35, 7),
        "light_gray_wool" => (35, 8),
        "cyan_wool" => (35, 9),
        "purple_wool" => (35, 10),
        "blue_wool" => (35, 11),
        "brown_wool" => (35, 12),
        "green_wool" => (35, 13),
        "red_wool" => (35, 14),
        "black_wool" => (35, 15),
        "gold_block" => (41, 0),
        "iron_block" => (42, 0),
        "stone_slab" => (44, 0),
        "bricks" => (45, 0),
        "mossy_cobblestone" => (48, 0),
        "obsidian" => (49, 0),
        "torch" => (50, 0),
        "diamond_ore" => (56, 0),
        "diamond_block" => (57, 0),
        "farmland" => (60, 0),
        "ladder" => (65, 0),
        "rail" => (66, 0),
        "cobblestone_wall" => (139, 0),
        "ice" => (79, 0),
        "snow_block" => (80, 0),
        "cactus" => (81, 0),
        "clay" => (82, 0),
        "pumpkin" => (86, 0),
        "netherrack" => (87, 0),
        "soul_sand" => (88, 0),
        "glowstone" => (89, 0),
        "stone_bricks" => (98, 0),
        "mossy_stone_bricks" => (98, 1),
        "cracked_stone_bricks" => (98, 2),
        "chiseled_stone_bricks" => (98, 3),
        "oak_fence" => (85, 0),
        "glass_pane" => (102, 0),
        "iron_bars" => (101, 0),
        "nether_bricks" | "nether_brick" => (112, 0),
        "red_nether_bricks" | "red_nether_brick" => (215, 0),
        "quartz_block" => (155, 0),
        "smooth_quartz" => (155, 0),
        "terracotta" => (172, 0),
        "white_terracotta" => (159, 0),
        "orange_terracotta" => (159, 1),
        "red_terracotta" => (159, 14),
        "gray_terracotta" => (159, 7),
        "light_blue_terracotta" => (159, 3),
        "green_terracotta" => (159, 13),
        "blue_terracotta" => (159, 11),
        "cyan_terracotta" => (159, 9),
        "hay_block" => (170, 0),
        "white_concrete" => (251, 0),  // Using concrete powder as fallback
        "gray_concrete" => (251, 7),
        "light_gray_concrete" => (251, 8),
        "black_concrete" => (251, 15),
        "blue_concrete" => (251, 11),
        "cyan_concrete" => (251, 9),
        "lime_concrete" => (251, 5),
        "yellow_concrete" => (251, 4),
        "light_blue_concrete" => (251, 3),
        "purple_concrete" => (251, 10),
        "prismarine" => (168, 0),
        "smooth_stone" => (1, 0),  // Use stone as fallback
        "water" => (9, 0),  // Stationary water
        "lava" => (11, 0),
        "packed_ice" => (174, 0),
        "coal_block" => (173, 0),
        "purpur_block" => (201, 0),
        "purpur_pillar" => (202, 0),
        "end_stone_bricks" => (206, 0),
        "poppy" => (38, 0),
        "dandelion" => (37, 0),
        "blue_orchid" => (38, 1),
        "azure_bluet" => (38, 3),
        "cauldron" => (118, 0),
        "moss_block" => (48, 0),  // Use mossy cobblestone as fallback
        // Modern blocks that don't exist in 1.8.9 - map to similar blocks
        "blackstone" => (112, 0),  // Map to nether brick
        "polished_blackstone" => (112, 0),
        "polished_blackstone_bricks" => (112, 0),
        "deepslate_bricks" => (98, 0),  // Map to stone bricks
        "polished_deepslate" => (98, 0),
        "mud_bricks" => (45, 0),  // Map to bricks
        "netherite_block" => (215, 0),  // Map to red nether bricks
        "warped_planks" => (5, 5),  // Map to dark oak planks
        "crimson_planks" => (5, 4),  // Map to acacia planks
        "polished_basalt" => (1, 0),  // Map to stone
        "scaffolding" => (85, 0),  // Map to fence
        "smooth_red_sandstone" => (179, 0),
        "cut_sandstone" => (24, 0),
        "quartz_bricks" => (155, 0),
        "stone_brick_slab" => (44, 5),
        "oak_slab" => (126, 0),
        "white_stained_glass" => (95, 0),
        _ => (1, 0),  // Default to stone for unknown blocks
    }
}

/// Converts a section's blocks to legacy Blocks and Data byte arrays
/// Returns (Blocks: Vec<u8>, Data: Vec<u8>)
pub fn section_to_legacy_format(blocks: &[Block; 4096]) -> (Vec<u8>, Vec<u8>) {
    let mut block_ids = Vec::with_capacity(4096);
    let mut data_values = vec![0u8; 2048]; // 4096 blocks, 4 bits each = 2048 bytes
    
    for (index, block) in blocks.iter().enumerate() {
        let block_name = block.name();
        let (block_id, metadata) = get_legacy_block_id(block_name);
        
        block_ids.push(block_id);
        
        // Pack metadata into 4 bits (2 blocks per byte)
        let data_index = index / 2;
        if index % 2 == 0 {
            // Lower 4 bits
            data_values[data_index] = (data_values[data_index] & 0xF0) | (metadata & 0x0F);
        } else {
            // Upper 4 bits
            data_values[data_index] = (data_values[data_index] & 0x0F) | ((metadata & 0x0F) << 4);
        }
    }
    
    (block_ids, data_values)
}
