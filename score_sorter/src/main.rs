use std::io;
use std::env;
use std::time;

fn main()->io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let lines = io::stdin().lines();

    let mut xp_array = vec![Vec::<i32>::new(),Vec::<i32>::new(),Vec::<i32>::new(),Vec::<i32>::new(),Vec::<i32>::new(),Vec::<i32>::new()];
    for line in lines {
        let mut i = 0;
        let mut total_xp = 0;
        for score in line.unwrap().split_whitespace(){
            let val = score.parse::<i32>().unwrap();
            total_xp += val;
            xp_array[i].push(val);
            i += 1;
        }
        xp_array[5].push(total_xp);
    }

    if args[1].eq("standard") {
        for i in 0..6 {
            match i{
                0=>println!("SKILL_BREAKDANCING"),
                1=>println!("SKILL_APICULTURE"),
                2=>println!("SKILL_BASKET"),
                3=>println!("SKILL_XBASKET"),
                4=>println!("SKILL_SWORD"),
                5=>println!("TOTAL_XP"),
                _=>println!("error"),
            }

            let now = time::Instant::now();
            xp_array[i].sort_unstable_by(|a,b| b.cmp(a));
            let microseconds = now.elapsed();
            
            for score in &xp_array[i]{
                println!("{score}");
            }

            println!("time taken: {}\n", microseconds.as_micros());
        }
    }
    else if args[1].eq("custom") {
        for i in 0..6 {
            match i{
                0=>println!("SKILL_BREAKDANCING"),
                1=>println!("SKILL_APICULTURE"),
                2=>println!("SKILL_BASKET"),
                3=>println!("SKILL_XBASKET"),
                4=>println!("SKILL_SWORD"),
                5=>println!("TOTAL_XP"),
                _=>println!("error"),
            }

            let now = time::Instant::now();
            // Implement Custom Sorting
            //
            let microseconds = now.elapsed();
            
            for score in &xp_array[i]{
                println!("{score}");
            }
            
            println!("time taken: {}\n", microseconds.as_micros());
        }
    }

    return Ok(());
}