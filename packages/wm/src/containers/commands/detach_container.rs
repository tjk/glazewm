use std::collections::VecDeque;

use anyhow::Context;

use crate::containers::{
  traits::{CommonGetters, TilingSizeGetters},
  Container,
};

use super::flatten_split_container;

/// Removes a container from the tree.
pub fn detach_container(child_to_remove: Container) -> anyhow::Result<()> {
  let mut parent = child_to_remove.parent().context("No parent.")?;

  // Flatten the parent split container if it'll be empty after removing
  // the child.
  if let Some(split_parent) = parent.as_split().cloned() {
    if split_parent.child_count() == 1 {
      parent = parent.parent().context("No parent.")?;
      flatten_split_container(split_parent)?;
    }
  }

  parent
    .borrow_children_mut()
    .retain(|c| c.id() != child_to_remove.id());

  parent
    .borrow_child_focus_order_mut()
    .retain(|id| *id != child_to_remove.id());

  *child_to_remove.borrow_parent_mut() = None;
  *child_to_remove.borrow_children_mut() = VecDeque::new();

  // Resize the siblings if it is a tiling container.
  if let Ok(child_to_remove) = child_to_remove.as_tiling_container() {
    resize_sibling_containers(parent, child_to_remove.tiling_size())?;
  }

  Ok(())
}

fn resize_sibling_containers(
  parent: Container,
  tiling_size: f32,
) -> anyhow::Result<()> {
  let tiling_siblings = parent.tiling_children().collect::<Vec<_>>();
  let tiling_size_increment = tiling_size / tiling_siblings.len() as f32;

  // Adjust size of the siblings based on the freed up space.
  for sibling in &tiling_siblings {
    sibling.set_tiling_size(sibling.tiling_size() + tiling_size_increment);
  }

  // If there is exactly *one* sibling to the detached container, then
  // flatten that sibling if it's a split container. This is to handle
  // layouts like H[1 V[2 H[3]]], where container 2 gets detached.
  if tiling_siblings.len() == 1 {
    if let Some(split_sibling) = tiling_siblings[0].as_split().cloned() {
      let split_sibling_parent =
        split_sibling.parent().context("No parent.")?;

      flatten_split_container(split_sibling)?;

      // Additionally flatten parent to handle deeply nested layouts.
      if let Some(split_sibling_parent) =
        split_sibling_parent.as_split().cloned()
      {
        flatten_split_container(split_sibling_parent)?;
      }
    }
  }

  Ok(())
}
