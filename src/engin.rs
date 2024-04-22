use rand::Rng;

#[derive(Clone)]
pub struct Entity
{
    pub position: (f32, f32),
    pub counter: usize,
    pub network_size: usize,
    pub input_size: usize,
    pub output_size: usize,
    pub neurons: Vec<(f32, Vec<f32>)>,
    pub input_buffer: Vec<f32>,
    pub output_buffer: Vec<f32>,
}

impl Entity
{
    pub fn new(network_size: usize, input_size: usize, output_size: usize) -> Entity
    {
        let mut entity = Entity
        {
            position: (0.5, 0.5),
            counter: 0,
            network_size,
            input_size,
            output_size,
            neurons: vec![(0.0, vec![1.0; network_size]); network_size],
            input_buffer: vec![0.0; input_size],
            output_buffer: vec![0.0; output_size],
        };

        let mut rng = rand::thread_rng();

        for neuron_index in 0..entity.neurons.len()
        {
            for weight_index in 0..entity.neurons[neuron_index].1.len()
            {
                entity.neurons[neuron_index].1[weight_index] = rng.gen_range(-1.0..1.0);
            }
        }

        entity
    }

    pub fn propagate(&mut self)
    {
        let old_neurons = self.neurons.clone();
        for neuron_index in 0..old_neurons.len()
        {
            self.neurons[neuron_index].0 = 0.0;
            for weight_index in 0..old_neurons[neuron_index].1.len()
            {
                self.neurons[neuron_index].0 += old_neurons[weight_index].0 * old_neurons[neuron_index].1[weight_index];
            }
            self.neurons[neuron_index].0 = 1.0/(1.0 + f32::powf(std::f32::consts::E, -self.neurons[neuron_index].0));
        }
    }

    pub fn update_input_buffer(&mut self)
    {
        for buffer_index in 0..self.input_buffer.len()
        {
            self.neurons[buffer_index].0 = self.input_buffer[buffer_index];
        }
    }

    pub fn update_output_buffer(&mut self)
    {
        for buffer_index in 0..self.output_buffer.len()
        {
            self.output_buffer[buffer_index] = self.neurons[self.network_size-self.output_size+buffer_index].0;
        }
    }

    pub fn compute(&mut self)
    {
        self.update_input_buffer();
        self.propagate();
        self.update_output_buffer();
    }

    pub fn mutate(&mut self)
    {
        let mut rng = rand::thread_rng();

        for neuron_index in 0..self.neurons.len()
        {
            for weight_index in 0..self.neurons[neuron_index].1.len()
            {
                self.neurons[neuron_index].1[weight_index] += rng.gen_range(-0.1..0.1);
            }
        }
    }

    pub fn reinit(&mut self, neurons: Vec<(f32, Vec<f32>)>)
    {
        self.position = (0.5, 0.5);

        self.neurons = neurons;
        self.counter = 0;
    }
}