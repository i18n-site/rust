use std::collections::HashSet;

use rand::Rng;

pub fn random_pos(container_width: u32, container_height: u32, size_li: &[u32]) -> Vec<(u32, u32)> {
  let max_size = *size_li.iter().max().unwrap();
  let grid_width = container_width / max_size;
  let grid_height = container_height / max_size;

  let mut selected_coords = Vec::new();
  let mut rng = rand::thread_rng();
  let mut taken_grids = HashSet::new();

  for _ in 0..size_li.len() {
    let mut valid_grid = false;
    let mut chosen_grid;

    while !valid_grid {
      chosen_grid = rng.gen_range(0..grid_width * grid_height);

      if !taken_grids.contains(&chosen_grid) {
        valid_grid = true;

        // Calculate x, y based on the chosen grid
        let x = (chosen_grid % grid_width) * max_size;
        let y = (chosen_grid / grid_width) * max_size;
        selected_coords.push((x, y));

        // Add the chosen grid and its adjacent grids to the taken_grids HashSet
        taken_grids.insert(chosen_grid);
        taken_grids.insert(chosen_grid.saturating_sub(1));
        taken_grids.insert(chosen_grid.saturating_add(1));

        if chosen_grid >= grid_width {
          taken_grids.insert(chosen_grid - grid_width);
        }

        if chosen_grid < (grid_height - 1) * grid_width {
          taken_grids.insert(chosen_grid + grid_width);
        }
      }
    }
  }

  selected_coords
}
