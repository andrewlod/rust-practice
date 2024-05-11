mod it;
mod gcd;
mod max;
mod avg;
mod rng;
mod coin;
mod block;
mod travel;
mod social;
mod shapes;
mod anagram;
mod disease;
mod finance;
mod product;
mod account;
mod is_prime;
mod portfolio;
mod sum_evens;
mod file_manip;
mod battleship;
mod closest_sum;
mod move_zeroes;
mod count_to_10;
mod rotate_array;
mod guessing_game;
mod buy_sell_stock;
mod is_one_distant;
mod reverse_string;
mod math_operations;
mod compress_string;
mod uppercase_count;
mod construct_string;
mod real_state_broker;
mod find_word_in_board;
mod array_combinations;
mod array_intersection;
mod contains_duplicates;
mod k_largest_in_vector;
mod words_follow_pattern;
mod multiplication_table;
mod is_string_palindrome;
mod has_unique_characters;
mod is_integer_palindrome;

fn main() {
    println!("{}", gcd::gcd(16, 40));
    
    count_to_10::count_for(10);
    count_to_10::count_while(10);

    let v = vec![1,2,3,4,5,321,54,2,4564];
    println!("{}", max::max_in_vec(&v).unwrap());

    println!("{}", is_prime::is_prime(1));
    println!("{}", is_prime::is_prime(9));
    println!("{}", is_prime::is_prime(13));
    println!("{}", is_prime::is_prime(49));
    println!("{}", is_prime::is_prime(53));

    multiplication_table::multiplication_table(5);

    let v = vec![1,2,3,4,5,6,7,8,9];
    println!("{}", sum_evens::sum_evens(&v));

    let v: Vec<f64> = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0];
    println!("{}", avg::avg(&v));

    println!("{}", has_unique_characters::has_unique_characters("sldaknmaw"));
    println!("{}", has_unique_characters::has_unique_characters("qwertyuiop"));

    println!("{}", anagram::anagram("abc", "bca"));
    println!("{}", anagram::anagram("abcd", "bca"));

    println!("{}", is_one_distant::is_one_distant("abcde", "ablde"));
    println!("{}", is_one_distant::is_one_distant("abcde", "abcxde"));
    println!("{}", is_one_distant::is_one_distant("abcde", "abde"));
    println!("{}", is_one_distant::is_one_distant("abcde", "abcdxf"));
    println!("{}", is_one_distant::is_one_distant("abcde", "abd"));

    println!("{}", compress_string::compress_string("aabcccdee"));

    println!("{}", is_integer_palindrome::is_integer_palindrome(121));
    println!("{}", is_integer_palindrome::is_integer_palindrome(123));
    println!("{}", is_integer_palindrome::is_integer_palindrome(123454321));
    println!("{}", is_integer_palindrome::is_integer_palindrome(12344321));

    println!("{}", reverse_string::reverse_string(&String::from("abcde")));

    println!("{}", is_string_palindrome::is_palindrome("abcdedcba"));
    println!("{}", is_string_palindrome::is_palindrome("dsacklj"));

    let mut arr = [1,2,3,4,5,6,7,8,9];
    rotate_array::rotate_array(&mut arr, 3);
    println!("{:?}", &arr);

    let mut arr = vec![1,2,3,4,5,6,7,8,8,8,9];
    println!("{}", k_largest_in_vector::k_largest_in_vector(&mut arr, 5));

    println!("{}", buy_sell_stock::buy_sell_stock(&[7, 1, 5, 3, 6, 4]));

    let mut arr = vec![1,0,3,5,1,0,2,3,5,0];
    move_zeroes::move_zeroes(&mut arr);
    println!("{:?}", &arr);

    println!("{}", contains_duplicates::contains_duplicates(&[1,2,3,4,5,6]));
    println!("{}", contains_duplicates::contains_duplicates(&[1,2,3,1,5,6]));

    println!("{}", construct_string::is_constructible("abc", "asjaw"));
    println!("{}", construct_string::is_constructible("abc", "ascab"));

    println!("{}", words_follow_pattern::words_follow_pattern("abba", "dog cat cat dog"));
    println!("{}", words_follow_pattern::words_follow_pattern("abba", "dog cat cat fish"));
    println!("{}", words_follow_pattern::words_follow_pattern("aaaa", "dog cat cat dog"));
    println!("{}", words_follow_pattern::words_follow_pattern("abba", "dog dog dog dog"));

    println!("{:?}", array_intersection::array_intersection(&[1,2,2,1], &[2,2]));
    println!("{:?}", array_intersection::array_intersection(&[1,8,3,4,5], &[4,5,6,7,8]));

    println!("{}", closest_sum::closest_sum(&[-1, 2, 1, -4], 1));

    println!("{:?}", array_combinations::array_combinations(&[1,2,3,4,5,6]));

    println!("{}", uppercase_count::uppercase_count("HeLlo YouTuBe"));

    portfolio::demo();

    coin::demo();

    product::demo();

    block::demo();

    file_manip::file_manip();
    println!("{}", file_manip::count_lines("./assets/data.txt"));

    rng::generate_random_numbers();

    // guessing_game::guessing_game();
    // guessing_game::word_guessing_game();

    shapes::demo();

    disease::demo();

    real_state_broker::demo();

    math_operations::demo();

    finance::demo();

    account::demo();

    social::demo();

    travel::demo();

    println!("{}", find_word_in_board::word_exists_in_board(&vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E']
    ], "SEE"));
    println!("{}", find_word_in_board::word_exists_in_board(&vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E']
    ], "SEA"));
    println!("{}", find_word_in_board::word_exists_in_board(&vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E']
    ], "ABFCEE"));
    println!("{}", find_word_in_board::word_exists_in_board(&vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E']
    ], "ABFCEC"));

    battleship::main();
}
