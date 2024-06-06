use iced::{Application, Command, Settings, Theme};
use rand::distributions::Uniform;
use rand::Rng;

fn main() -> iced::Result {
    AlgorithmsVisualizer::run(Settings::default())
}

struct AlgorithmsVisualizer {}

#[derive(Debug)]
enum Message {}

impl Application for AlgorithmsVisualizer {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("algorithms visualizer")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        "Hello".into()
    }
}

#[allow(dead_code)]
fn bubble_sort(array: &mut [i32]) -> &mut [i32] {
    loop {
        let mut swapped = false;
        for i in 1..array.len() {
            if array[i - 1] > array[i] {
                array.swap(i - 1, i);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
    array
}

fn gen_random_vector(min: i32, max: i32, length: usize) -> Vec<i32> {
    let range = Uniform::new(min, max);
    rand::thread_rng().sample_iter(&range).take(length).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut array = gen_random_vector(-1000, 1000, 100);
        let mut sorted = array.clone();
        sorted.sort();
        assert_eq!(bubble_sort(array), sorted)
    }

    #[test]
    fn bubble_sort_test2() {
        let array = &mut [6789, 675678, 4567454, 4567, 123, 13256731];
        let sorted = &[123, 4567, 6789, 675678, 4567454, 13256731];
        assert_eq!(bubble_sort(array), sorted)
    }
}
