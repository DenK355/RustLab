use std::io;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Оберіть завдання:");
    println!("1 - Площа прямокутника");
    println!("2 - Конвертація температури (C → F)");
    println!("3 - Парне чи непарне");
    println!("4 - Високосний рік");
    println!("5 - Голосна чи приголосна");
    println!("6 - Калькулятор");
    println!("7 - Максимум із двох чисел");
    println!("8 - Корені квадратного рівняння");
    println!("9 - Знак числа");
    println!("10 - Найбільше з трьох");

    let choice: u32 = read_line().parse().unwrap();

    match choice {
        1 => {
            println!("Введіть сторону a:");
            let a: f64 = read_line().parse().unwrap();
            println!("Введіть сторону b:");
            let b: f64 = read_line().parse().unwrap();
            println!("Площа: {}", a * b);
        }

        2 => {
            println!("Введіть температуру в Цельсіях:");
            let c: f64 = read_line().parse().unwrap();
            let f = c * 1.8 + 32.0;
            println!("У Фаренгейтах: {}", f);
        }

        3 => {
            println!("Введіть ціле число:");
            let n: i32 = read_line().parse().unwrap();
            if n % 2 == 0 {
                println!("парне");
            } else {
                println!("непарне");
            }
        }

        4 => {
            println!("Введіть рік:");
            let year: i32 = read_line().parse().unwrap();
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                println!("Високосний рік");
            } else {
                println!("Не високосний рік");
            }
        }

        5 => {
            println!("Введіть букву:");
            let ch = read_line()
                .chars()
                .next()
                .unwrap()
                .to_ascii_uppercase();

            match ch {
                'A' | 'E' | 'I' | 'O' | 'U' => println!("Голосна"),
                'A'..='Z' => println!("Приголосна"),
                _ => println!("Некоректний символ"),
            }
        }

        6 => {
            println!("Введіть перше число:");
            let a: f64 = read_line().parse().unwrap();
            println!("Введіть друге число:");
            let b: f64 = read_line().parse().unwrap();
            println!("Введіть операцію (+ - * /):");
            let op = read_line().chars().next().unwrap();

            match op {
                '+' => println!("{}", a + b),
                '-' => println!("{}", a - b),
                '*' => println!("{}", a * b),
                '/' => {
                    if b != 0.0 {
                        println!("{}", a / b);
                    } else {
                        println!("Ділення на нуль!");
                    }
                }
                _ => println!("Некоректна операція"),
            }
        }

        7 => {
            println!("Введіть два числа:");
            let a: i32 = read_line().parse().unwrap();
            let b: i32 = read_line().parse().unwrap();

            if a > b {
                println!("Максимум: {}", a);
            } else if b > a {
                println!("Максимум: {}", b);
            } else {
                println!("Числа рівні");
            }
        }

        8 => {
            println!("Введіть a, b, c:");
            let a: f64 = read_line().parse().unwrap();
            let b: f64 = read_line().parse().unwrap();
            let c: f64 = read_line().parse().unwrap();

            let d = b * b - 4.0 * a * c;

            if d < 0.0 {
                println!("Дійсних коренів немає");
            } else if d == 0.0 {
                let x = -b / (2.0 * a);
                println!("Один корінь: {}", x);
            } else {
                let x1 = (-b + d.sqrt()) / (2.0 * a);
                let x2 = (-b - d.sqrt()) / (2.0 * a);
                println!("Два корені: {} і {}", x1, x2);
            }
        }

        9 => {
            println!("Введіть число:");
            let n: f64 = read_line().parse().unwrap();
            if n > 0.0 {
                println!("додатне");
            } else if n < 0.0 {
                println!("від’ємне");
            } else {
                println!("нуль");
            }
        }

        10 => {
            println!("Введіть три числа:");
            let a: i32 = read_line().parse().unwrap();
            let b: i32 = read_line().parse().unwrap();
            let c: i32 = read_line().parse().unwrap();

            let max = if a >= b && a >= c {
                a
            } else if b >= a && b >= c {
                b
            } else {
                c
            };

            println!("Найбільше число: {}", max);
        }

        _ => println!("Невірний вибір"),
    }
}
