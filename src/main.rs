#![allow(unused)]

use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--------------------------------");
    println!("----    RUSSIAN ROULETTE    ----");
    println!("----  THERE ARE NO RULES.   ----");
    println!("---- PRESS ENTER TO ADVANCE ----");
    println!("---- THE GAME FURTHER. GOOD ----");
    println!("---- LUCK. YOU'LL NEED IT.  ----");

    const CHAMBERS: i32 = 6; //this fucka a six shoota

    let unlucky_chamber: i32 = rand::thread_rng().gen_range(0..=CHAMBERS); //which chamber is loaded
    let mut nextplaya: i32 = rand::thread_rng().gen_range(0..=2); //decice which player goes first
    println!("--------------------------------");
    println!("----  You gaze at me and I  ----");
    println!("----  I gaze back at you.   ----");
    println!("----  We both know what's   ----");
    println!("----  coming. The unloaded  ----");
    println!("----  six shooter lies on   ----");
    println!("----  the dimly lit table   ----");
    println!("----  between you and I.    ----");
    println!("--------------------------------");
    println!("-- Only fate can save you now --");
    io::stdin()
        .read_line(&mut [0u8])
        .expect("Didn't get that.");

    for click in 0..=unlucky_chamber + 1 {
        println!("-----   ROUND NUMBER: {}.   -----\n", click + 1);

        if click == 0 {
            loadGun(nextplaya);
        }

        pullTrigger(nextplaya);

        if click == unlucky_chamber {
            shootplaya(nextplaya);
        } else {
            println!("--------------------------------");
            println!("----   *CLICK!*. Both of    ----");
            println!("----    our brains still    ----");
            println!("----  reside in our heads.  ----");
            io::stdin()
                .read_line(&mut [0u8])
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
        println!("--------------------------------");
        println!("----  I grab the unfeeling  ----");
        println!("----  steel revolver, and I ----");
        println!("----  load one bullet into  ----");
        println!("----  the cold chamber. I   ----");
        println!("----  spin the fucker, and  ----");
        println!("----  throw flick it into   ----");
        println!("----       it's place.      ----");
        thread::sleep(Duration::from_secs(1));
        io::stdin()
            .read_line(&mut [0u8])
            .expect("Didn't get that.");
    } else {
        println!("--------------------------------");
        println!("----  You giggle, and pick  ----");
        println!("----  the revolver up. You  ----");
        println!("----  flick the single 9mm  ----");
        println!("----  bullet into the air,  ----");
        println!("----  and catch it in the   ----");
        println!("----  bloody palm of your   ----");
        println!("----  hand. You place the   ----");
        println!("----    bullet into the     ----");
        println!("---- chamber, and flick the ----");
        println!("----  thing closed. A sick  ----");
        println!("----  grin materializes on  ----");
        println!("---- your emotionless face. ----");
        thread::sleep(Duration::from_secs(1));
        io::stdin()
            .read_line(&mut [0u8])
            .expect("Didn't get that.");
    }
}

fn pullTrigger(player: i32) {
    if player == 0 {
        println!("--------------------------------");
        println!("----  With shaky hands and  ----");
        println!("----  nervous complexion,   ----");
        println!("----   I lift the loaded    ----");
        println!("----   gun to my temple.    ----");
        println!("----   As my finger slips   ----");
        println!("----   around the trigger   ----");
        println!("----   guard and onto the   ----");
        println!("----  cold, steel trigger,  ----");
        println!("----    I consider every    ----");
        println!("----   moment in my life    ----");
        println!("---- that's led up to this. ----");
        println!("----  ...and I squeeze the  ----");
        println!("----        trigger.        ----");
        io::stdin()
            .read_line(&mut [0u8])
            .expect("Didn't get that.");
    } else {
        println!("--------------------------------");
        println!("----  You look across this  ----");
        println!("----   table confidently.   ----");
        println!("---- With a twisted smile,  ----");
        println!("----  you lift the loaded   ----");
        println!("---- gun to your head. Your ----");
        println!("---- hands still, your face ----");
        println!("----        straight...     ----");
        io::stdin()
            .read_line(&mut [0u8])
            .expect("Didn't get that.");
    }
}

fn shootplaya(player: i32) {
    if player == 0 {
        println!("--------------------------------");
        println!("----   I didn't hear the    ----");
        println!("----  shot, but it's left   ----");
        println!("----  me missing a solid    ----");
        println!("----  portion of my skull   ----");
        println!("---- and the vast majority  ----");
        println!("---- of what was inside it. ----");
        println!("----    In other words,     ----");
        println!("----        YOU WON!        ----");
        io::stdin()
           .read_line(&mut [0u8])
            .expect("Didn't get that.");
    } else {
        println!("--------------------------------");
        println!("---- *BANG!* The gray shit  ----");
        println!("---- that used to live in   ----");
        println!("---- between your ears, is  ----");
        println!("---- now nicely decorating  ----");
        println!("---- the wall behind you.   ----");
        println!("---- The lifeless half of   ----");
        println!("---- your head thats still  ----");
        println!("---- attached to your body  ----");
        println!("----  falls to the table,   ----");
        println!("----   with a disturbing    ----");
        println!("---- *thud*. That terrible  ----");
        println!("----  grin, still on your   ----");
        println!("----         face.          ----");
        io::stdin()
            .read_line(&mut [0u8])
            .expect("Didn't get that.");
    }
}
