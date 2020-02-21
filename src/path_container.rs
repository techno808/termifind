use std::collections::VecDeque;
use std::path::{Path, PathBuf};

mod directory_container;
use directory_container::DirectoryContainer;
use directory_container::ItemState;

use crate::utils::string::make_repeated_char_string;

#[allow(dead_code)]
pub struct PathContainer {
    current_path: PathBuf,
    directory_container_vec_deque: VecDeque<DirectoryContainer>,
    spacing_between_directory_containers: usize,
    spacing_between_directory_containers_char: char,
    spacing_between_directory_container_rows: usize,
    spacing_between_directory_container_rows_char: char,
    terminal_dimensions: (usize, usize),
}

impl PathContainer {
    pub fn new(path: PathBuf) -> Self {
        let mut directory_container_vec_deque: VecDeque<DirectoryContainer> = VecDeque::new();
        let mut parent_path: &Path = &path;

        loop {
            directory_container_vec_deque.push_front(DirectoryContainer::new(
                parent_path.to_path_buf(),
                &PathContainer::get_selected_directory_option(&directory_container_vec_deque),
            ));

            match parent_path.parent() {
                Some(p_path) => parent_path = p_path,
                None => break,
            }
        }

        PathContainer::select_first_directory_in_current_directory_container(
            &mut directory_container_vec_deque,
        );

        PathContainer {
            current_path: path,
            directory_container_vec_deque,
            spacing_between_directory_containers: 1,
            spacing_between_directory_containers_char: ' ',
            spacing_between_directory_container_rows: 1,
            spacing_between_directory_container_rows_char: ' ',
            terminal_dimensions: term_size::dimensions().expect("Oops"),
        }
    }

    fn get_selected_directory_option(
        directory_container_vec_deque: &VecDeque<DirectoryContainer>,
    ) -> Option<PathBuf> {
        if directory_container_vec_deque.is_empty() {
            return None;
        }
        match directory_container_vec_deque.front() {
            Some(first_directory_container) => {
                Some(first_directory_container.path_to_directory.clone())
            }
            None => None,
        }
    }

    fn select_first_directory_in_current_directory_container(
        directory_container_vec_deque: &mut VecDeque<DirectoryContainer>,
    ) {
        if let Some(directory_container) = directory_container_vec_deque.back_mut() {
            if let Some(directory_item) = directory_container.directory_item_vec.first_mut() {
                directory_item.item_state = ItemState::Selected
            }
        }
    }

    pub fn print_path(&self) {
        let mut start_and_end_iteration_tuple: (usize, usize) =
            self.update_start_and_end_iteration_tuple((0, 0));

        while start_and_end_iteration_tuple.0 < self.directory_container_vec_deque.len() {
            self.print_one_line_of_directory_containers(start_and_end_iteration_tuple);

            start_and_end_iteration_tuple =
                self.update_start_and_end_iteration_tuple(start_and_end_iteration_tuple);
        }
    }

    // REFACTOR
    fn update_start_and_end_iteration_tuple(
        &self,
        mut start_and_end_iteration_tuple: (usize, usize),
    ) -> (usize, usize) {
        let mut previous_directory_containers_space_requirement = 0;

        start_and_end_iteration_tuple.0 = start_and_end_iteration_tuple.1;

        loop {
            let current_directory_container_space_requirement =
                if start_and_end_iteration_tuple.1 < self.directory_container_vec_deque.len() {
                    self.directory_container_vec_deque[start_and_end_iteration_tuple.1]
                        .get_total_width_of_directory_container()
                        + self.spacing_between_directory_containers
                } else {
                    0
                };

            let all_directory_containers_space_requirement =
                previous_directory_containers_space_requirement
                    + current_directory_container_space_requirement;

            // This boolean represents there being enough room to print the current directory
            // containers and assumes there will be possibly more that can fit in this same row.
            let a = all_directory_containers_space_requirement < self.terminal_dimensions.0;

            // This boolean represents there being enough room to print the current directory
            // containers and assumes this row is complete and no more directory containers will
            // fit.
            let b = all_directory_containers_space_requirement
                - self.spacing_between_directory_containers
                < self.terminal_dimensions.0;

            let at_end_of_directory_container_deque =
                start_and_end_iteration_tuple.1 >= self.directory_container_vec_deque.len();

            let can_fit_current_directory_containers_in_row =
                (a || b) && !at_end_of_directory_container_deque;

            if can_fit_current_directory_containers_in_row {
                previous_directory_containers_space_requirement += self
                    .directory_container_vec_deque[start_and_end_iteration_tuple.1]
                    .get_total_width_of_directory_container()
                    + self.spacing_between_directory_containers;
                start_and_end_iteration_tuple.1 += 1;
            } else {
                break start_and_end_iteration_tuple;
            }
        }
    }

    fn print_one_line_of_directory_containers(
        &self,
        start_and_end_iteration_tuple: (usize, usize),
    ) {
        let height_of_tallest_container =
            self.get_height_of_tallest_directory_container_in_range(start_and_end_iteration_tuple);

        for i in 0..height_of_tallest_container + self.spacing_between_directory_container_rows {
            for j in start_and_end_iteration_tuple.0..start_and_end_iteration_tuple.1 {
                self.print_one_row_of_each_directory_container(
                    j,
                    i,
                    j < start_and_end_iteration_tuple.1 - 1,
                );
            }

            println!();
        }
    }

    fn get_height_of_tallest_directory_container_in_range(
        &self,
        start_and_end_iteration_tuple: (usize, usize),
    ) -> usize {
        let mut height_of_tallest_container = 0;

        for i in start_and_end_iteration_tuple.0..start_and_end_iteration_tuple.1 {
            let directory_container = &self.directory_container_vec_deque[i];
            let container_height = directory_container.get_total_height_of_directory_container();

            if height_of_tallest_container < container_height {
                height_of_tallest_container = container_height;
            }
        }

        height_of_tallest_container
    }

    fn print_one_row_of_each_directory_container(
        &self,
        x: usize,
        row_number: usize,
        should_print_spacing_between_directory_containers: bool,
    ) {
        if row_number
            < self.directory_container_vec_deque[x].get_total_height_of_directory_container()
        {
            self.directory_container_vec_deque[x].print_directory_container_by_row(row_number);
        } else {
            print!(
                "{}",
                make_repeated_char_string(
                    self.spacing_between_directory_container_rows_char,
                    self.directory_container_vec_deque[x].get_total_width_of_directory_container()
                )
            );
        }
        if should_print_spacing_between_directory_containers {
            print!(
                "{}",
                make_repeated_char_string(
                    self.spacing_between_directory_containers_char,
                    self.spacing_between_directory_containers
                )
            );
        }
    }
}
