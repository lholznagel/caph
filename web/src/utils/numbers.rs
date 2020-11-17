use yew::prelude::*;

#[derive(Clone, Debug, Properties)]
pub struct FormatNumberProps {
    pub number: String
}

pub struct FormatNumberComponent {
    output: String
}

impl FormatNumberComponent {
    fn format(&mut self, number: String) {
        let mut result = Vec::new();

        let mut numbers = 0;
        for x in number.chars().rev() {
            if numbers == 3 {
                result.push(46u8);
                numbers = 0;
            }
            result.push(x as u8);
            numbers += 1;
        }

        result.reverse();
        self.output = String::from_utf8(result).unwrap();
    }
}

impl Component for FormatNumberComponent {
    type Properties = FormatNumberProps;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut x = FormatNumberComponent {
            output: String::new()
        };
        x.format(props.number);
        x
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <label>{ &self.output }</label>
        }
    }
}


#[cfg(test)]
mod format_number {
    use super::*;

    #[test]
    fn format_001() {
        dbg!(1.to_string().chars().next().unwrap().to_digit(8).unwrap() as u8);

        let mut instance = FormatNumberComponent { output: String::new() };
        instance.format(1.to_string());
        assert_eq!(instance.output, "1");

        instance.format(10.to_string());
        assert_eq!(instance.output, "10");

        instance.format(100.to_string());
        assert_eq!(instance.output, "100");

        instance.format(1_000.to_string());
        assert_eq!(instance.output, "1.000");

        instance.format(10_000.to_string());
        assert_eq!(instance.output, "10.000");

        instance.format(100_000.to_string());
        assert_eq!(instance.output, "100.000");

        instance.format(1_000_000.to_string());
        assert_eq!(instance.output, "1.000.000");
    }
}