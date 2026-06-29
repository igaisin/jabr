/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 * Modified by igaisin 2025
 */
pub mod jab;
pub mod utils;

use std::{path::PathBuf, sync::OnceLock};

static CUSTOM_SEARCH_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
static LIBRARY_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();

#[cfg(all(target_arch = "x86"))]
const JAB_LIB_NAME: &str = "WindowsAccessBridge-32.dll";
#[cfg(all(target_arch = "x86_64"))]
const JAB_LIB_NAME: &str = "WindowsAccessBridge-64.dll";

/// Установить пользовательскую директорию для поиска
pub fn set_custom_search_directory(directory: PathBuf) {
    if CUSTOM_SEARCH_DIRECTORY.set(directory).is_err() {
        eprintln!("Custom search directory is already set and cannot be changed.");
    }
}

/// Получить директорию для поиска
fn get_search_directory() -> PathBuf {
    // Если пользовательская директория установлена, возвращаем её
    if let Some(custom_dir) = CUSTOM_SEARCH_DIRECTORY.get() {
        custom_dir.clone()
    } else {
        // Если пользовательская директория не установлена, возвращаем путь по умолчанию
        PathBuf::from("C:\\Program Files\\Java\\jre1.8.0_*\\bin")
    }
}

/// Найти путь к библиотеке
fn find_library_path(lib_name: &str) -> Option<PathBuf> {
    let dir = get_search_directory();
    let path = dir.join(lib_name);
    if path.exists() { Some(path) } else { None }
}

/// Установить динамическую библиотеку для программы
pub fn setup_for() -> Result<(), String> {
    if let Some(path) = find_library_path(JAB_LIB_NAME) {
        LIBRARY_DIRECTORY
            .set(path.parent().unwrap().to_path_buf())
            .map_err(|_| "Не удалось установить путь к библиотеке".to_string())?;
        println!(
            "Библиотека установлена: {}",
            LIBRARY_DIRECTORY.get().unwrap().display()
        );
        Ok(())
    } else {
        Err(format!(
            "Не удалось найти библиотеку `{}` в указанной директории",
            JAB_LIB_NAME
        ))
    }
}
