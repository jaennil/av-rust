use iced::{Application, Command, Settings, Theme};

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
        for i in 0..array.len() - 1 {
            if array[i] > array[i + 1] {
                (array[i], array[i + 1]) = (array[i + 1], array[i]);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
    array
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bubble_sort_test() {
        let array = &mut [3, 2, 1];
        let sorted = &[1, 2, 3];
        assert_eq!(bubble_sort(array), sorted)
    }

    #[test]
    fn bubble_sort_test2() {
        let array = &mut [6789, 675678, 4567454, 4567, 123, 13256731];
        let sorted = &[123, 4567, 6789, 675678, 4567454, 13256731];
        assert_eq!(bubble_sort(array), sorted)
    }
}
