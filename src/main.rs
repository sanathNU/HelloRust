#![allow(warnings)]

mod level_1;
mod level_2;
mod level_3;
mod level_4;
mod level_5;
mod level_6;
mod level_7;
mod level_8;
mod level_9;
mod level_10;

//default function to prevent any testing
// fn main() {
//     println!("Namaste World!");
// }

fn test_level1(){
    level_1::favapps();
    level_1::stud_read_write();
    level_1::print_ascii_value();
    level_1::far_to_cel();
    level_1::add_three_numbers();
}

fn test_level2(){
    level_2::largest2();
    level_2::min3();
    level_2::calculate_vote_eligibility();
    level_2::vowel_or_consonant();
    level_2::multiplication_table();
}

fn test_level3(){
    level_3::student_grade_calculator();
    level_3::quadratic_eqn();
    level_3::print_natural_numbers(); // handles both forward and reverse
    level_3::check_in_range();
    level_3::average_n();
}
fn test_level4(){
    level_4::counting_characters();
    level_4::prime_or_composite();
    level_4::decimal_to_binary();
    level_4::sum_of_squares();
    level_4::gcd_calculate();
}

fn test_level5(){
    level_5::palindrome_checker();
    level_5::longest_common_substring();
    level_5::anagram_checker();
    level_5::read_and_swap();
    level_5::case_conversion();
    level_5::complex_number_magnitude();

}

fn test_level6(){
    level_6::series_sum();
    level_6::circle_values_calculation();
    level_6::triangle_area_calculation();
}

fn test_level7(){
    level_7::find_factorial();
    level_7::binary_search();
    level_7::linear_search();
}

fn test_level8(){
    level_8::duplicate_elements();
    level_8::string_length();
    level_8::string_concat();
    level_8::array_rotation();
}

fn test_level9(){
    level_9::book_read_write();
    level_9::convert_case();
}

fn test_level10(){
    level_10::multiple_factorial();
    level_10::web_scraping_test();
    level_10::macro_test();
}


#[tokio::main]
async fn main() {

    level_10::macro_test();
}