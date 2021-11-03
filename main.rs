use std::io;
use std::io::{Write};

macro_rules! ask {
    ($output_text:expr) => {{
        let mut buffer = String::new();
        print!("{}", $output_text);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        let strip_buffer = buffer.strip_suffix("\r\n").or(buffer.strip_suffix("\n")).unwrap_or(buffer.as_str());
        match strip_buffer.parse::<u16>() {
            Ok(x) => match x.try_into(){
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                    continue
                }
            },
            Err(_) => {
                println!("Это не число. Попробуйте еще раз.");
                continue
            }
        }
    }}
}

fn main() {
    loop {
        let damage = dps_calculator::calculate(
            ask!("Урон оружия: "),
            ask!("Уровень заточки\n1-Красная\n2-Оранжевая\n3-Желтая\n4-Зеленая\n5-Синяя\n6-Белая\n7-Фиолетовая: "),
            ask!("Шанс крита: "),
            ask!("Урон крита: "),
            ask!("Уязвимость к физики\n1-Одна звезда\n2-Две звезды\n3-Три звезды\nИли введите значение: "),
            ask!("Урон стихии: "),
            ask!("Уязвимость к стихии\n1-Одна звезда\n2-Две звезды\n3-Три звезды\nИли введите значение: "),
        );
        println!("{}", damage);
    };
}
