use burn::{
    config::Config,
    module::Module,
    nn::{
        Linear, LinearConfig,
        conv::{Conv2d, Conv2dConfig},
    },
    prelude::Backend,
    tensor::{Tensor, activation::relu},
};

use super::BenchableModel;

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    conv3: Conv2d<B>,
    linear1: Linear<B>,
    linear2: Linear<B>,
}
impl<B: Backend> Model<B> {
    fn forward(&self, input: Tensor<B, 4>) -> Tensor<B, 2> {
        let input = self.conv1.forward(input);
        let input = relu(input);
        let input = self.conv2.forward(input);
        let input = relu(input);
        let input = self.conv3.forward(input);
        let input = relu(input);
        let input = input.flatten(1, 3);
        let input = self.linear1.forward(input);
        let input = relu(input);
        let input = self.linear2.forward(input);
        return input;
    }
}

#[derive(Config, Debug)]
pub struct ModelConfig {
    #[config(default = "8")]
    num_action: usize,
}

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model {
            conv1: Conv2dConfig::new([4, 32], [8, 8])
                .with_stride([4, 4])
                .init(device),
            conv2: Conv2dConfig::new([32, 64], [4, 4])
                .with_stride([2, 2])
                .init(device),

            conv3: Conv2dConfig::new([64, 64], [3, 3])
                .with_stride([1, 1])
                .init(device),
            linear1: LinearConfig::new(3136, 512).init(device),
            linear2: LinearConfig::new(512, self.num_action).init(device),
        }
    }
}

impl<B: Backend> BenchableModel<B> for Model<B> {
    type Input = Tensor<B, 4>;

    fn make_input(batch_size: usize, device: &B::Device) -> Self::Input {
        Tensor::random(
            [batch_size, 4, 84, 84],
            burn::tensor::Distribution::Normal(0., 2.),
            device,
        )
    }

    fn run(&self, input: &Self::Input) {
        let _output = self.forward(input.clone());
    }
}
