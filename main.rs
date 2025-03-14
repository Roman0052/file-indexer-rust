use std::fs;
use chrono::DateTime;
use chrono::offset::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
    name: String,
    last_modified: String,
}

fn get_last_modified_time(file_path: &std::path::Path) -> Result<String, std::io::Error> {
    let metadata = fs::metadata(file_path)?; // Отримуємо метадані файлу
    let modified = metadata.modified()?; // Отримуємо час останнього модифікації
    let duration_since_epoch = modified.duration_since(std::time::UNIX_EPOCH).unwrap();
    let datetime = DateTime::<Utc>::from(std::time::SystemTime::UNIX_EPOCH + duration_since_epoch); // Перетворюємо в DateTime
    Ok(datetime.format("%Y-%m-%d").to_string()) // Форматуємо дату в потрібний формат
}

fn index_files(directory: &str) -> Result<Vec<FileInfo>, std::io::Error> {
    let mut files_info = Vec::new();
    for entry in fs::read_dir(directory)? { // Читаємо вміст директорії
        let entry = entry?; // Обробляємо кожен файл чи директорію
        let file_name = entry.file_name().into_string().unwrap(); // Отримуємо ім'я файлу
        let last_modified = get_last_modified_time(&entry.path())?; // Отримуємо час останньої модифікації
        files_info.push(FileInfo { // Додаємо інформацію про файл в список
            name: file_name,
            last_modified,
        });
    }
    Ok(files_info)
}

fn main() {
    let directory = "."; // Використовуємо поточну директорію
    match index_files(directory) { // Індексуємо файли в директорії
        Ok(files) => {
            for file in files { // Виводимо інформацію про кожен файл
                println!("{:?}, Last Modified: {}", file.name, file.last_modified);
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e), // Виводимо помилку, якщо щось пішло не так
    }
}
