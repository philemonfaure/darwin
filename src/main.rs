mod engin;

use macroquad::prelude::*;
use macroquad::{hash, Window};
use macroquad::ui::{root_ui, widgets};
use crate::engin::Entity;

fn window_conf() -> Conf {
    Conf {
        window_title: "darwin".to_owned(),
        window_width: 700,
        window_height: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let network_size = 10;

    let mut entities: Vec<Entity> = vec![engin::Entity::new(network_size, 2, 2); 500];
    for entity in &mut entities
    {
        for _i in 0..1
        {
            entity.mutate();
        }
    }

    let mut best_neurons = (entities[0].neurons.clone(), 0);

    loop
    {
        for _i in 0..1
        {
            clear_background(BLACK);

            for entity in &mut entities
            {
                entity.compute();

                entity.input_buffer = [entity.position.0, entity.position.1].to_vec();

                entity.position.0 += (entity.output_buffer[0]-0.5)/10.0;
                entity.position.1 += (entity.output_buffer[1]-0.5)/10.0;

                draw_circle(entity.position.0*700.0, entity.position.1*500.0, 5.0, YELLOW);
            }

            next_frame().await
        }

        for entity in &mut entities
        {
            if entity.position.0 > 0.9 || entity.position.0 < 0.1 || entity.position.1 > 0.9 || entity.position.1 < 0.1 || (entity.output_buffer[0] < 0.501 && entity.output_buffer[0] > 0.499) || (entity.output_buffer[1] < 0.501 && entity.output_buffer[1] > 0.499)
            {
                entity.reinit(best_neurons.0.clone());
                entity.mutate();
            }
            entity.counter += 1;
            //if entity.counter > best_neurons.1
            {
                best_neurons.0 = entity.neurons.clone();
                best_neurons.1 = entity.counter;
            }
        }
    }
}