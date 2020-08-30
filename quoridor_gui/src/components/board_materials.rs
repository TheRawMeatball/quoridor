use crate::*;

pub struct BoardMaterials {
    pub base_mat_handle: Handle<ColorMaterial>,
    pub wall_slot_mat_handle: Handle<ColorMaterial>,
    pub select: Handle<ColorMaterial>,
    pub highlight: Handle<ColorMaterial>,
    pub wall_mat_handle: Handle<ColorMaterial>,
    pub pawn1_mat_handle: Handle<ColorMaterial>,
    pub pawn2_mat_handle: Handle<ColorMaterial>,
}

impl FromResources for BoardMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

        BoardMaterials {
            base_mat_handle: materials.add(Color::rgb(0.08, 0.01, 0.003).into()),
            wall_slot_mat_handle: materials.add(Color::rgb(0.175, 0.045, 0.004).into()),
            select: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            highlight: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
            wall_mat_handle: materials.add(Color::rgb(0.32, 0.16, 0.04).into()),
            pawn1_mat_handle: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            pawn2_mat_handle: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        }
    }
}
