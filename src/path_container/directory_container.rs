use std::fs::{read_dir, ReadDir};
use std::path::PathBuf;
use std::vec::Vec;

mod directory_item;
pub use directory_item::{DirectoryItem, ItemState, ItemType};

use crate::utils::string::{add_padding_to_center_string, make_repeated_char_string};

#[allow(dead_code)]
enum LowLevelTruncationOptions {
    None,
    ByFileNameLength,
    Constant,
}

#[allow(dead_code)]
enum AutomaticTruncationOptions {
    Statistical,                       // Uses ByFileNameLength
    FitAllDirectoryContainersInOneRow, // Uses Constant
}

pub struct DirectoryContainer {
    pub directory_name: String,
    pub minimum_width: usize,
    pub path_to_directory: PathBuf,
    pub directory_item_vec: Vec<DirectoryItem>,
}

impl DirectoryContainer {
    pub fn new(
        path: PathBuf,
        selected_directory_option: &Option<PathBuf>,
        name_truncation_settings_option: Option<(usize, bool)>,
    ) -> Self {
        let mut directory_item_vec: Vec<DirectoryItem> = Vec::new();
        let read_directory_iterator: ReadDir = read_dir(&path).expect("Oops");
        let mut length_of_longest_file_name: usize = 0;

        for file in read_directory_iterator {
            let mut directory_item: DirectoryItem =
                DirectoryItem::new(file.expect("Oops"), name_truncation_settings_option);

            let length_of_file_name: usize = directory_item.get_file_name_length(true);

            if length_of_file_name > length_of_longest_file_name {
                length_of_longest_file_name = length_of_file_name
            }

            if let Some(selected_directory) = selected_directory_option {
                if selected_directory == &directory_item.directory_entry.path() {
                    directory_item.item_state = ItemState::DirectoryInPath;
                }
            }

            directory_item_vec.push(directory_item);
        }

        directory_item_vec.sort_by(|a, b| {
            a.get_file_name(false)
                .partial_cmp(&b.get_file_name(false))
                .expect("Oops")
        });

        let directory_name: String = match path.file_name() {
            Some(d_name) => d_name.to_string_lossy().to_string(),
            None => path.to_string_lossy().to_string(),
        };

        let length_of_current_directory_name = directory_name.chars().count();

        let minimum_width = if length_of_current_directory_name > length_of_longest_file_name {
            length_of_current_directory_name
        } else {
            length_of_longest_file_name
        };

        DirectoryContainer {
            directory_name,
            minimum_width,
            path_to_directory: path,
            directory_item_vec,
        }
    }

    // fn get_truncation_setting(
    //     low_level_truncation_option: LowLevelTruncationOptions,
    // ) -> Option<(usize, bool)> {
    //     match low_level_truncation_option {
    //         LowLevelTruncationOptions::None => None,
    //         LowLevelTruncationOptions::ByFileNameLength => None,
    //         LowLevelTruncationOptions::Constant => None,
    //     }
    // }

    pub fn print_directory_container_by_row(&self, row_number: usize) {
        if row_number < self.get_total_height_of_directory_container() - 1 {
            match row_number {
                0 => print!(
                    " {} ",
                    make_repeated_char_string('-', self.minimum_width + 2)
                ),
                1 => print!(
                    "|{}|",
                    add_padding_to_center_string(&self.directory_name, self.minimum_width + 2)
                ),
                2 => print!(
                    "|{}|",
                    make_repeated_char_string('=', self.minimum_width + 2)
                ),
                _ => {
                    print!("| ");

                    let directory_item = &self.directory_item_vec[row_number - 3];
                    directory_item.print_styled_file_name(true);

                    let length_of_current_file_name: usize =
                        directory_item.get_file_name_length(true);
                    let difference: usize = self.minimum_width - length_of_current_file_name;

                    print!("{} |", make_repeated_char_string(' ', difference));
                }
            }
        } else {
            print!(
                " {} ",
                make_repeated_char_string('-', self.minimum_width + 2)
            );
        }
    }

    pub fn get_total_width_of_directory_container(&self) -> usize {
        self.minimum_width + 4
    }

    pub fn get_total_height_of_directory_container(&self) -> usize {
        self.get_number_of_directory_items() + 4
    }

    pub fn get_number_of_directory_items(&self) -> usize {
        self.directory_item_vec.len()
    }

    pub fn get_lengths_of_file_names(&self) -> Vec<usize> {
        let mut file_name_length_vec: Vec<usize> = Vec::new();

        for directory_item in &self.directory_item_vec {
            file_name_length_vec.push(directory_item.get_file_name_length(false));
        }

        file_name_length_vec
    }
}
