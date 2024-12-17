use bevy::prelude::*;

fn apply_value_updated(mut events: EventReader<ValueUpdated>, mut query: Query<(&mut Value, &Id)>) {
    for event in events.read() {
        for (mut value, id) in &query {
            if id == event.target_id {
                value = event.new_value;
                break;
            }
        }
    }
}

fn apply_value_increased(
    mut events: EventReader<ValueIncreased>,
    mut query: Query<(&mut Value, &Id)>,
    mut event_writer: EventWriter<ValueUpdated>,
) {
    for event in events.read() {
        for (mut value, id) in &query {
            if id == event.target_id {
                event_writer.send(ValueUpdated {
                    target_id: id,
                    new_value: value + event.increase_value,
                });
                break;
            }
        }
    }
}

fn apply_value_decreased(
    mut events: EventReader<ValueDecreased>,
    mut query: Query<(&mut Value, &Id)>,
    mut event_writer: EventWriter<ValueUpdated>,
) {
    for event in events.read() {
        for (mut value, id) in &query {
            if id == event.target_id {
                event_writer.send(ValueUpdated {
                    target_id: id,
                    new_value: value - event.decrease_value,
                });
                break;
            }
        }
    }
}
