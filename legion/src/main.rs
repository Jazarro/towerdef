#![allow(unused_imports)]

#[macro_use]
extern crate legion;

use amethyst::core::ecs::{CommandBuffer, World, Resources, Schedule};

// components are normal Rust structs
struct Person {
    name: &'static str
}

struct Age(u8);

// systems are normal Rust fns, annotated with #[system]
#[system(for_each)]
fn say_hello(person: &Person) {
    println!("hello, {}!", person.name);
}

#[system]
fn introduce_people(commands: &mut CommandBuffer, #[resource] names: &Vec<&'static str>) {
// entities are inserted as tuples of components
    for name in names {
        commands.push((Person { name }, Age(35)));
    }
}

fn main() {
    // the world is a container of entities
    let mut world = World::default();

    // resources can be shared between systems
    let mut resources = Resources::default();
    resources.insert(vec!["Jane Doe", "John Smith"]);

    // schedule will automatically execute systems in parallel
    let mut schedule = Schedule::builder()
        .add_system(introduce_people_system())
        .add_system(say_hello_system())
        .build();

    // run our schedule
    schedule.execute(&mut world, &mut resources);
}