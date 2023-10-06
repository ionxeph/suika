use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::resources::MassSetting;
use crate::setup::Score;
use crate::{resources::ScoreTracker, Fruit};

use super::create_fruit_bundle;

#[derive(Component)]
pub struct MarkForDelete;

#[derive(Component, Clone)]
pub struct MarkForMerge {
    id: Entity,
    merge_target: Entity,
}

pub fn collision(
    mut collisions: EventReader<CollisionEvent>,
    fruits: Query<&Fruit>,
    mut commands: Commands,
) {
    for collision in collisions.iter() {
        if let CollisionEvent::Started(collider_a, collider_b, _) = collision {
            if let Ok([fruit_a, fruit_b]) = fruits.get_many([*collider_a, *collider_b]) {
                if fruit_a.size == fruit_b.size {
                    let entity_a = commands.entity(*collider_a).id();
                    let entity_b = commands.entity(*collider_b).id();

                    commands.entity(*collider_a).insert(MarkForMerge {
                        id: entity_a,
                        merge_target: entity_b,
                    });
                    commands.entity(*collider_b).insert(MarkForMerge {
                        id: entity_b,
                        merge_target: entity_a,
                    });
                }
            }
        }
    }
}

pub fn remove_used_fruits(
    fruits_marked_for_delete: Query<Entity, With<MarkForDelete>>,
    mut commands: Commands,
) {
    for fruit in fruits_marked_for_delete.iter() {
        commands.entity(fruit).despawn();
    }
}

// wrote all this extra code to try to handle when a single fruit collides with two fruits of the same size at the same time
// which would remove all three, and then spawn two fruits of the next size up
// ideal result should be to remove only two of the three, and then spawn one fruit of the next size up
pub fn merge_fruits(
    mut fruits_marked_for_merge: Query<(Entity, &mut MarkForMerge, &Fruit, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score_tracker: ResMut<ScoreTracker>,
    mut score_query: Query<&mut Text, With<Score>>,
    mass_setting: Res<MassSetting>,
) {
    struct ShouldMerge {
        entities: (Entity, Entity),
        merge_result: Option<Fruit>,
        translation: ((f32, f32), (f32, f32)),
    }
    let mut possible_orphans: Vec<(MarkForMerge, Entity, (f32, f32))> = Vec::new();
    let mut should_merge_list: Vec<ShouldMerge> = Vec::new();
    for (entity, marked, fruit, transform) in fruits_marked_for_merge.iter_mut() {
        if let Some((possible_target, _, translation)) = possible_orphans
            .iter()
            .find(|(prcssed, _, _)| prcssed.id == marked.merge_target)
        {
            if possible_target.merge_target == marked.id {
                should_merge_list.push(ShouldMerge {
                    entities: (marked.id, possible_target.id),
                    merge_result: fruit.merge(),
                    translation: (
                        (transform.translation.x, transform.translation.y),
                        *translation,
                    ),
                });
                continue;
            }
        }

        if !possible_orphans
            .iter()
            .any(|(prcssed, _, _)| prcssed.id == marked.id)
        {
            possible_orphans.push((
                marked.clone(),
                entity,
                (transform.translation.x, transform.translation.y),
            ));
        }
    }

    for (_, orphan, _) in possible_orphans.iter() {
        commands.entity(*orphan).remove::<MarkForMerge>();
    }

    for should_merge in should_merge_list.iter() {
        let new_x = (should_merge.translation.0 .0 + should_merge.translation.1 .0) / 2.0;
        let new_y = (should_merge.translation.0 .1 + should_merge.translation.1 .1) / 2.0;
        // Fruit.merged_size returns None if two largest fruits collide
        // in this case, both are despawned, and no new fruits created
        match &should_merge.merge_result {
            Some(fruit) => {
                let texture_handle = asset_server.load(&fruit.image_file_name);
                score_tracker.add_score(fruit.score);
                let mut score = score_query.single_mut();
                score.sections[0].value = score_tracker.score.to_string();
                commands
                    .spawn(create_fruit_bundle(
                        texture_handle,
                        new_x,
                        new_y,
                        fruit.clone(),
                    ))
                    .insert(AdditionalMassProperties::Mass(mass_setting.get_mass()));
            }
            None => (),
        }

        commands
            .entity(should_merge.entities.0)
            .remove::<(RigidBody, SpriteBundle, Collider)>()
            .insert(MarkForDelete);
        commands
            .entity(should_merge.entities.1)
            .remove::<(RigidBody, SpriteBundle, Collider)>()
            .insert(MarkForDelete);
    }
}
