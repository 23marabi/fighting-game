fn player_movement_system(
mut player_query: Query<(&MovementData, &mut Velocity, &mut Transform), With<PlayerTag>>,
keyboard: Res<Input<KeyCode>>,
time: Res<Time>,
) {
let (movement_data, mut rb_vels, mut transform) = player_query.single_mut();

let mut input_vector = Vec2::ZERO;
if keyboard.pressed(KeyCode::W) {
input_vector.y += 1.0;
}

if keyboard.pressed(KeyCode::S) {
input_vector.y -= 1.0;
}
if keyboard.pressed(KeyCode::A) {
input_vector.x -= 1.0;
}

if keyboard.pressed(KeyCode::D) {
input_vector.x += 1.0;
}

input_vector = input_vector.normalize_or_zero();
if input_vector != Vec2::ZERO {
rb_vels.linvel = rb_vels.linvel.lerp(
input_vector * movement_data.max_speed,
movement_data.acceleration * time.delta_seconds()
);
let test = (Vec2::new(transform.translation.x, transform.translation.y) - input_vector).normalize();
let test2 = Quat::from_rotation_arc(Vec3::Y, test.extend(0.));
println!("{}", test2);
transform.rotation = test2;
}
else {
rb_vels.linvel = rb_vels.linvel.lerp(
Vec2::ZERO,
movement_data.friction * time.delta_seconds()
);
}
