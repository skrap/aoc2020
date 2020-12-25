fn xform(subject: usize, loops: usize) -> usize {
    let mut value = 1;
    for _ in 0..loops {
        value *= subject;
        value %= 20201227;
    }
    value
}

fn find_loops(subject: usize, result: usize) -> usize {
    let mut value = 1;
    let mut loops = 0;
    while value != result {
        value *= subject;
        value %= 20201227;
        loops += 1;
    }    
    loops
}

fn part1(card_pub: usize, door_pub: usize) -> usize {
    // The door transforms the subject number of 7 according to the door's secret loop size. The result is called the door's public key.
    let door_loops = find_loops(7, door_pub);
    // The door transforms the subject number of the card's public key according to the door's loop size. The result is the same encryption key as the card calculated.
    let encr_key = xform(card_pub, door_loops);

    encr_key
}

fn main() {
    let input = [335121,363891];
    dbg!(part1(input[0], input[1]));
}
