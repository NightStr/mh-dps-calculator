use std::io;
use std::io::{Write};

macro_rules! ask {
    ($output_text:expr) => {{
        let mut buffer = String::new();
        print!("{}", $output_text);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer);
        let strip_buffer = buffer.strip_suffix("\r\n").or(buffer.strip_suffix("\n")).unwrap_or(buffer.as_str());
        match strip_buffer.parse::<u16>() {
            Ok(x) => x.into(),
            Err(e) => {
                println!(
                    "Value {} is valid value. Error: {}. Try again", buffer, e
                );
                continue
            }
        }
    }}
}

macro_rules! ask_resistance {
    ($output_text:expr) => {{
        match ask!(format!("{}\n{}", $output_text, "1-Одна звезда\n2-Две звезды\n3-Три звезды\nИли введите значение: ")) {
            1 => dps_calculator::Resistance::OneStar,
            2 => dps_calculator::Resistance::TwoStar,
            3 => dps_calculator::Resistance::ThreeStar,
            v => {
                match format!("0.{}", v).parse::<f32>() {
                    Ok(x) => dps_calculator::Resistance::Custom(x),
                    Err(e) => {
                        println!(
                            "Value {} is valid value. Error: {}. Try again", v, e
                        );
                        continue
                    }
                }
            }
        }.into()
    }}
}

macro_rules! ask_sharpness {
    ($output_text:expr) => {{
        match ask!(format!("{}\n{}", $output_text, "1-Красная\n2-Оранжевая\n3-Желтая\n4-Зеленая\n5-Синяя\n6-Белая\n7-Фиолетовая: ")) {
            1 => dps_calculator::Sharpness::Red,
            2 => dps_calculator::Sharpness::Orange,
            3 => dps_calculator::Sharpness::Yellow,
            4 => dps_calculator::Sharpness::Green,
            5 => dps_calculator::Sharpness::Blue,
            6 => dps_calculator::Sharpness::White,
            7 => dps_calculator::Sharpness::Purple,
            v => {
                    println!(
                        "Неверное значение {} для заточки", v
                    );
                    continue
            }
        }
    }}
}

fn main() {
    loop {
        let damage = dps_calculator::calculate(
            ask!("Урон оружия: "),
            ask_sharpness!("Уровень заточки"),
            ask!("Шанс крита: "),
            ask!("Размер крита: "),
            ask_resistance!("Уязвимость к физики"),
            ask!("Урон стихии: "),
            ask_resistance!("Уязвимость к стихии"),
        );
        println!("{}", damage);
    };
}
