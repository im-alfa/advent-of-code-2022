



fn main() {
    let input = include_str!("../input.txt");

    let lines = input.split("\n");

    for line in lines {
        let len = line.len();

        println!("{} -- {}", &line[0..len/2], &line[len/2..len])

        // convert the

    }

}
