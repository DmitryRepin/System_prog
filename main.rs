use std::fs::{File, OpenOptions, read_to_string};
use std::io::{BufRead, BufReader, Write};

fn main() {
    loop {
        println!(
            "\nВыберите действие:
    1. Добавить студента;
    2. Выбрать студента;
    3. Изменить имя студента;
    4. Изменить группу студента;
    5. Отчислить студента;
    6. Показать список студентов;
    0. Завершить."
        );
        let mut ch: String = String::new();
        std::io::stdin()
            .read_line(&mut ch)
            .ok()
            .expect("Ошибка ввода числа!");
        let choose: u8 = ch.trim().parse().unwrap();
        match choose {
            1 => add_student(),
            2 => choose_student(),
            3 => modify_student_name(),
            4 => modify_student_group(),
            5 => kick_student(),
            6 => show_student(),
            0 => break,
            _ => println!("Неверный выбор действия!"),
        }
    }
}

fn add_student() {
    let _file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("list.txt")
        .expect("Невозможно открыть файл");

    let file = File::open("list.txt").expect("Невозможно открыть файл");
    let reader = BufReader::new(file);
    let mut cnt = 1;

    for _ in reader.lines() {
        cnt = cnt + 1;
    }

    println!("\nВведите имя студента для добавления");
    let mut name = String::new();

    std::io::stdin()
        .read_line(&mut name)
        .ok()
        .expect("Ошибка ввода имени студента!");

    println!("\nВведите группу студента для добавления");
    let mut group = String::new();
    std::io::stdin()
        .read_line(&mut group)
        .ok()
        .expect("Ошибка ввода имени студента!");

    let file_result = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("list.txt");
    //Обратотка исключения (невозможно открыть файл)
    let mut file = match file_result {
        Ok(file) => file,
        Err(_error) => {
            println!("Ошибка открытия файла");
            return;
        }
    };

    let record = format!("{}. {}. {}. Учится", cnt, name.trim(), group.trim());
    println!("\nДобавлена запись: \"{}\"", record);

    writeln!(file, "{}", record).expect("Невозможно записать в файл");
}

fn find_student() -> String {
    let mut nmb = String::new();

    std::io::stdin()
        .read_line(&mut nmb)
        .ok()
        .expect("Ошибка ввода номера студента!");

    let number = nmb.trim().parse().unwrap();

    let file_result = File::open("list.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(_error) => {
            return "Ошибка открытия файла".to_string();
        }
    };
    let reader = BufReader::new(file);

    let mut cnt = 1;
    let mut result = "Ошибка. Студент с таким номером не найден.".to_string();

    for line in reader.lines() {
        if cnt == number {
            result = line.unwrap();
            break;
        } else {
            cnt = cnt + 1;
        }
    }
    result
}

fn choose_student() {
    println!("\nВведите номер студента для получения информации о нем");
    println!("{}", find_student());
}

fn modify_student_name() {
    println!("\nВведите номер студента для изменения ФИО");
    let student = find_student();
    println!("{}", student);
    if student == "Ошибка открытия файла" {
        return;
    }

    if student == "Ошибка. Студент с таким номером не найден.".to_string() {
        return;
    }

    println!("\nВведите новые ФИО студента");
    let mut name = String::new();

    std::io::stdin()
        .read_line(&mut name)
        .ok()
        .expect("Ошибка ввода имени студента!");

    let fio = student.split(". ").nth(1).expect("Ошибка при выделении ФИО");
    let new_student = student.replace(fio, &name.trim());

    let mut data = read_to_string("list.txt")
        .expect("Невозможно считать файл");
    data = data.replace(&student, &new_student);

    println!("\nНовый список студентов:\n{}", data);

    let file_result = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("list.txt");
    let mut file = match file_result {
        Ok(file) => file,
        Err(_error) => {
            println!("Ошибка открытия файла");
            return;
        }
    };

    writeln!(file, "{}", data.trim()).expect("Невозможно записать в файл");
}

fn modify_student_group() {
    println!("\nВведите номер студента для изменения номера группы");
    let student = find_student();
    println!("{}", student);
    if student == "Ошибка открытия файла" {
        return;
    }

    if student == "Ошибка. Студент с таким номером не найден.".to_string() {
        return;
    }

    println!("\nВведите новая группа студента");
    let mut group = String::new();

    std::io::stdin()
        .read_line(&mut group)
        .ok()
        .expect("Ошибка ввода имени студента!");

    let number = student.split(". ").nth(2).expect("Ошибка при выделении номера группы");
    let new_student = student.replace(number, &group.trim());

    let mut data = read_to_string("list.txt")
        .expect("Невозможно считать файл");
    data = data.replace(&student, &new_student);

    println!("\nНовый список студентов:\n{}", data);

    let file_result = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("list.txt");
    let mut file = match file_result {
        Ok(file) => file,
        Err(_error) => {
            println!("Ошибка открытия файла");
            return;
        }
    };

    writeln!(file, "{}", data.trim()).expect("Невозможно записать в файл");
}

fn kick_student() {
    println!("\nВведите номер студента для отчисления");
    let student = find_student();
    println!("{}", student);
    if student == "Ошибка открытия файла" {
        return;
    }

    if student == "Ошибка. Студент с таким номером не найден.".to_string() {
        return;
    }

    let status = student.split(". ").nth(3).expect("Ошибка при выделении ФИО");
    if status == "Отчислен" {
        println!("Студент уже отчислен");
        return;
    }
    let new_student = student.replace(status, "Отчислен".trim());

    let mut data = read_to_string("list.txt")
        .expect("Невозможно считать файл");
    data = data.replace(&student, &new_student);

    println!("\nНовый список студентов:\n{}", data);

    let file_result = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("list.txt");
    let mut file = match file_result {
        Ok(file) => file,
        Err(_error) => {
            println!("Ошибка открытия файла");
            return;
        }
    };

    writeln!(file, "{}", data.trim()).expect("Невозможно записать в файл");
}

fn show_student() {
    let data = read_to_string("list.txt")
        .expect("Невозможно считать файл");

    println!("\nСписок студентов:\n{}", data);
}