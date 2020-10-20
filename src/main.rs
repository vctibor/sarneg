use clap::{Arg, App, SubCommand};

type Key = [char; 10];

type Message = Vec<usize>;

/// Receives SARNEG key as a string.
/// Checks constraints:
/// - key must be ASCII (because of potentially weird definition of single "char" of higher unicode values)
/// - key length is exactly 10 characters
/// - characters can be only letters
/// - no character is repeated in key
/// If key string conforms to all constraints,
/// it is turned into array of chars of length ten.
fn create_key(key: &str) -> Result<Key, String>
{
    let key = key.trim();

    if !key.is_ascii() {
        return Err("Key must be only ASCII values.".to_owned())
    }

    if key.len() != 10 {
        return Err("Key length must be exactly 10.".to_owned());
    }

    let key = key.to_ascii_uppercase();

    let mut key_chars = Vec::new();

    for character in key.chars() {

        if !character.is_ascii_alphabetic() {
            return Err("Key can contain only letters.".to_owned())
        }
        
        if key_chars.contains(&character) {
            return Err(format!("Characters in key must not repeat. Character {} is repeated.", character));
        }

        key_chars.push(character);
    }

    key_chars.reverse();

    let mut array: [char; 10] = ['*'; 10];

    for ix in 0..10 {
        array[ix] = key_chars.pop().expect("Unexpected error.");
    }

    Ok(array) 
}

/// Pretty prints key in a table corellating letter to digit.
fn print_key(key: Key)
{
    println!(" {} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
        key[0], key[1], key[2], key[3], key[4], key[5], key[6], key[7], key[8], key[9]);
    println!("---|---|---|---|---|---|---|---|---|---");
    println!(" 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9");
}

/// Accepts encryption input as a string, checks that it contains only digits,
/// and returns vector of these digits.
fn create_message(msg: &str) -> Result<Message, String>
{
    let mut vector: Vec<usize> = Vec::new();

    for character in msg.chars() {
        if !character.is_digit(10) {
            return Err("Message can contain only digits 0 - 9.".to_owned());
        }

        vector.push(character.to_digit(10).expect("Unexpected error.") as usize);
    }

    return Ok(vector);
}

fn encrypt(key: Key, message: Message) -> String
{
    let mut encrypted: Vec<char> = Vec::new();

    for digit in message {
        encrypted.push(key[digit]);
    }

    encrypted.into_iter().collect()
}

fn decrypt(encrypted: &str, key: Key) -> Result<Message, String>
{
    let mut decrypted: Message = Vec::new();

    for character in encrypted.chars() {

        if !key.contains(&character) {
            return Err(format!("Character {} not found in key.", character));
        }

        decrypted.push(key.iter().position(|ch| ch == &character).expect("Unexpected error."));
    }

    Ok(decrypted)
}

fn main() -> Result<(), String>
{
    let matches = App::new("SARNEG")
        .version("1.0")
        .author("Vladimír Ctibor <vladimir.ctibor@gmail.com>")
        .about("SAR Numerical Encryption Grid tools.")
        .subcommand(SubCommand::with_name("print")
            .arg(Arg::with_name("key"))
        )
        .subcommand(SubCommand::with_name("encrypt")
            .arg(Arg::with_name("key"))
            .arg(Arg::with_name("msg"))
        )
        .subcommand(SubCommand::with_name("decrypt")
            .arg(Arg::with_name("key"))
            .arg(Arg::with_name("msg"))
        )
        .get_matches();

    return match matches.subcommand() {
        
        ("print", Some(print_matches)) => {
            let key = print_matches.value_of("key").unwrap();
            let key = create_key(key)?;
            print_key(key);
            Ok(())
        },
        
        ("encrypt", Some(encrypt_matches)) => {
            let key = encrypt_matches.value_of("key").unwrap();
            let msg = encrypt_matches.value_of("msg").unwrap();
            let key = create_key(key)?;
            let msg = create_message(msg)?;
            let encrypted = encrypt(key, msg);
            println!("{}", encrypted);
            Ok(())
        },

        ("decrypt", Some(decrypt_matches)) => {
            let key = decrypt_matches.value_of("key").unwrap();
            let encrypted = decrypt_matches.value_of("msg").unwrap();
            let key = create_key(key)?;
            let msg = decrypt(encrypted, key)?;
            println!("{:?}", msg);
            Ok(())
        },

        ("", None) => {
            println!("No subcommand was used");
            Ok(())
        },
        _ => unreachable!(),
    }
}
