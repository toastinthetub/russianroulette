#![allow(unused)]

use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

const MENU_TEXT: &str = include_str!("./texts/menutext.txt"); //menu text file
const CONTEXT: &str = include_str!("./texts/context.txt"); //freaky context file
const LUCKY: &str = include_str!("./texts/lucky.txt"); //your brains are still in your head!

const I_LOAD: &str = include_str!("./texts/iload.txt"); //i load the gun!
const U_LOAD: &str = include_str!("./texts/uload.txt"); //your creepy ass loads that fucker

const I_PULL: &str = include_str!("./texts/ipull.txt"); // i attempt to kill myself
const U_PULL: &str = include_str!("./texts/upull.txt"); // you attempt to kill yourself

const IM_DEAD: &str = include_str!("./texts/unluckyme.txt"); //i fucking died
const UR_DEAD: &str = include_str!("./texts/unluckyyou.txt"); //you fucking died

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    println!("{}", MENU_TEXT);
    let mut prompt9 = String::new();
    io::stdin()
        .read_line(&mut prompt9)
        .expect("Didn't get that.");
    print!("\x1B[2J\x1B[1;1H");

    const CHAMBERS: i32 = 6; //this fucka a six shoota

    let unlucky_chamber: i32 = rand::thread_rng().gen_range(0..=CHAMBERS); //which chamber is loaded
    let mut nextplaya: i32 = rand::thread_rng().gen_range(0..=1); //decice which player goes first

    println!("{}", CONTEXT);

    let mut prompt7 = String::new();
    io::stdin()
        .read_line(&mut prompt7)
        .expect("Didn't get that.");

    print!("\x1B[2J\x1B[1;1H");

    for click in 0..unlucky_chamber + 1 {
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "|--   ROUND NUMBER: {}.   --------------------------------------|",
            click + 1
        );

        if click == 0 {
            loadGun(nextplaya);
        }

        pullTrigger(nextplaya);

        if click == unlucky_chamber {
            shootplaya(nextplaya);
        } else {
            println!("{}", LUCKY);
            let mut prompt8 = String::new();
            io::stdin()
                .read_line(&mut prompt8)
                .expect("Didn't get that.");
        }

        if nextplaya == 0 {
            nextplaya = 1;
        } else {
            nextplaya = 0;
        }
    }
}

fn loadGun(player: i32) {
    if player == 0 {
        println!("{}", I_LOAD); //i load the gun
        thread::sleep(Duration::from_secs(1));
        let mut prompt2 = String::new();
        io::stdin()
            .read_line(&mut prompt2)
            .expect("Didn't get that.");
    } else {
        println!("{}", U_LOAD); //your creepy ass loads the gun
        thread::sleep(Duration::from_secs(1));
        let mut prompt3 = String::new();
        io::stdin()
            .read_line(&mut prompt3)
            .expect("Didn't get that.");
    }
}

/* this language sucks ong */
fn pullTrigger(player: i32) {
    /* no ternary operator? what. */
    let who_pulls = if player == 0 { I_PULL } else { U_PULL };
    println!("{}", who_pulls);
    /* why give each prompt a new name */
    let mut prompt = String::new();
    io::stdin()
        .read_line(&mut prompt)
        .expect("Didn't get that.");
// credit to @pgdwn for this shi right here
}

fn shootplaya(player: i32) {
    if player == 0 {
        println!("{}", IM_DEAD); //im dead unfortunately
        let mut prompt5 = String::new();
        io::stdin()
            .read_line(&mut prompt5)
            .expect("Didn't get that.");
    } else {
        println!("{}", UR_DEAD); //ur fucking dead thank god
        let mut prompt6 = String::new();
        io::stdin()
            .read_line(&mut prompt6)
            .expect("Didn't get that.");
    }
}

