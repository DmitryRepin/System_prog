use std::fs::{read_to_string, File, OpenOptions};
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
        let choose = input_int();
        match choose {
            1 => {add_student();},
            2 => {choose_student();},
            3 => {modify_student_name();},
            4 => {modify_student_group();},
            5 => {kick_student();},
            6 => {show_student();},
            0 => break,
            _ => println!("Неверный выбор действия!"),
        }
    }
}

fn input_str() -> String {
    let mut str: String = String::new();

    std::io::stdin()
        .read_line(&mut str)
        .ok()
        .expect("Ошибка ввода!");

    str
}

fn input_int() -> i32 {
    let str = input_str();
    let nmb = str.trim().parse().unwrap();
    nmb
}

fn add_student() -> String {
    let file_result = OpenOptions::new().write(true).create(true).open("list.txt");

    let _file = match file_result {
        Ok(file) => file,
        Err(_error) => {
            return "Ошибка открытия файла".to_string();
        }
    };

    let file = File::open("list.txt").expect("Невозможно открыть файл");
    let reader = BufReader::new(file);
    let mut cnt = 1;

    for _ in reader.lines() {
        cnt = cnt + 1;
        println!("qwzsf");
    }

    println!("\nВведите имя студента для добавления");
    let name = input_str();

    println!("\nВведите группу студента для добавления");
    let group = input_str();

    let file_result = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("list.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(_error) => {
            return "Ошибка открытия файла".to_string();
        }
    };

    let record = format!("{}. {}. {}. Учится", cnt, name.trim(), group.trim());
    println!("\nДобавлена запись: \"{}\"", record);

    writeln!(file, "{}", record).expect("Невозможно записать в файл");
    record
}

fn find_student() -> String {
    let number = input_int();

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
    println!("{}", result);
    result
}

fn choose_student() -> String {
    println!("\nВведите номер студента для получения информации о нем");
    let student = find_student();
    student
}

fn modify_student_name() -> String {
    println!("\nВведите номер студента для изменения ФИО");
    let student = find_student();
    if student == "Ошибка открытия файла" {
        return "Ошибка открытия файла".to_string();
    }

    if student == "Ошибка. Студент с таким номером не найден.".to_string()
    {
        return "Ошибка. Студент с таким номером не найден".to_string();
    }

    println!("\nВведите новые ФИО студента");
    let name = input_str();

    let fio = student
        .split(". ")
        .nth(1)
        .expect("Ошибка при выделении ФИО");
    let new_student = student.replace(fio, &name.trim());

    let data_result = read_to_string("list.txt");
    let mut data = match data_result {
        Ok(file) => file,
        Err(_error) => {
            return "Ошибка открытия файла".to_string();
        }
    };
    data = data.replace(&student, &new_student);

    println!("\nНовый список студентов:\n{}", data);

    let file_result = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("list.txt");
    let mut file = match file_result {
        Ok(file) => file,
        Err(_error) => {
            return "Ошибка открытия файла".to_string();
        }
    };

    writeln!(file, "{}", data.trim()).expect("Невозможно записать в файл");
    new_student
}

fn modify_student_group() -> String {
    println!("\nВведите номер студента для изменения номера группы");
    let student = find_student();
    println!("{}", student);
    if student == "Ошибка открытия файла" {
        return "Ошибка открытия файла".to_string();
    }

    if student == "Ошибка. Студент с таким номером не найден.".to_string()
    {
        return "Ошибка. Студент с таким номером не найден.".to_string();
    }

    println!("\nВведите новую группу студента");
    let group = input_str();

    let number = student
        .split(". ")
        .nth(2)
        .expect("Ошибка при выделении номера группы");
    let new_student = student.replace(number, &group.trim());

    let mut data = read_to_string("list.txt").expect("Невозможно считать файл");
    data = data.replace(&student, &new_student);

    println!("\nНовый список студентов:\n{}", data);

    let file_result = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("list.txt");
    let mut file = match file_result {
        Ok(file) => file,
        Err(_error) => {
            return "Ошибка открытия файла".to_string();
        }
    };

    writeln!(file, "{}", data.trim()).expect("Невозможно записать в файл");
    new_student
}

fn kick_student() -> String {
    println!("\nВведите номер студента для отчисления");
    let student = find_student();
    println!("{}", student);
    if student == "Ошибка открытия файла" {
        return "Ошибка открытия файла".to_string();
    }

    if student == "Ошибка. Студент с таким номером не найден.".to_string()
    {
        return "Ошибка. Студент с таким номером не найден.".to_string();
    }

    let status = student
        .split(". ")
        .nth(3)
        .expect("Ошибка при выделении ФИО");
    if status == "Отчислен" {
        return "Студент уже отчислен".to_string();
    }
    let new_student = student.replace(status, "Отчислен".trim());

    let mut data = read_to_string("list.txt").expect("Невозможно считать файл");
    data = data.replace(&student, &new_student);

    println!("\nНовый список студентов:\n{}", data);

    let file_result = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("list.txt");
    let mut file = match file_result {
        Ok(file) => file,
        Err(_error) => {
            return "Ошибка открытия файла".to_string();
        }
    };

    writeln!(file, "{}", data.trim()).expect("Невозможно записать в файл");
    new_student
}

fn show_student() {
    let data = read_to_string("list.txt").expect("Невозможно считать файл");

    println!("\nСписок студентов:\n{}", data);
}