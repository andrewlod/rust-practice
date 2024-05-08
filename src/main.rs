mod gcd;
mod max;
mod avg;
mod anagram;
mod is_prime;
mod sum_evens;
mod count_to_10;
mod is_one_distant;
mod compress_string;
mod multiplication_table;
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
}
