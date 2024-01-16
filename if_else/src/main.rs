fn main() {
    let cond = 2 < 3;
    println!("{}", cond);

    //& Compund condition - mutiple condtions chaned together 
            //& and -> &&
            //& or -> ||
            //& not -> !
            //& order -> ! && ||

    let cond2 = true && cond;
    let cond3 = !(true || false);
    println!("{} \n {}", cond2, cond3);

    let food = "cookie";

    if food == "cookie" {
        println!("Yum 🍪");

        if food == "big cookie" {
            println!("YDouble Yumy 🍪");

            if food == "extra big cookie"{
                println!("Double triple Yummy 🍪");
            }

            else {
                println!("Too bad 🍪");
            }
        }

        else {
            println!("Sorry to hear! 🍪");
        }
    }

    else if food == "biscut"{
        println!("Oi Mate! 🇬🇧");
    }

    else {
        println!("Brocili 🥦");
    }

}
