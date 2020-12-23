// --- Day 13: Shuttle Search ---
//
// Your ferry can make it safely to a nearby port, but it won't get much further. When you call to
// book another ship, you discover that no ships embark from that port to your vacation island.
// You'll need to get from the port to the nearest airport.
//
// Fortunately, a shuttle bus service is available to bring you from the sea port to the airport!
// Each bus has an ID number that also indicates how often the bus leaves for the airport.
//
// Bus schedules are defined based on a timestamp that measures the number of minutes since some
// fixed reference point in the past. At timestamp 0, every bus simultaneously departed from the
// sea port. After that, each bus travels to the airport, then various other locations, and finally
// returns to the sea port to repeat its journey forever.
//
// The time this loop takes a particular bus is also its ID number: the bus with ID 5 departs from
// the sea port at timestamps 0, 5, 10, 15, and so on. The bus with ID 11 departs at 0, 11, 22, 33,
// and so on. If you are there when the bus departs, you can ride that bus to the airport!
//
// Your notes (your puzzle input) consist of two lines. The first line is your estimate of the
// earliest timestamp you could depart on a bus. The second line lists the bus IDs that are in
// service according to the shuttle company; entries that show x must be out of service, so you
// decide to ignore them.
//
// To save time once you arrive, your goal is to figure out the earliest bus you can take to the
// airport. (There will be exactly one such bus.)
//
// For example, suppose you have the following notes:
//
// 939
// 7,13,x,x,59,x,31,19
//
// Here, the earliest timestamp you could depart is 939, and the bus IDs in service are 7, 13, 59,
// 31, and 19. Near timestamp 939, these bus IDs depart at the times marked D:
//
// time       bus 7   bus 13  bus 59  bus 31  bus 19
// 929          .       .       .       .       .
// 930          .       .       .       D       .
// 931          D       .       .       .       D
// 932          .       .       .       .       .
// 933          .       .       .       .       .
// 934          .       .       .       .       .
// 935          .       .       .       .       .
// 936          .       D       .       .       .
// 937          .       .       .       .       .
// 938          D       .       .       .       .
// **939**      .       .       .       .       .
// 940          .       .       .       .       .
// 941          .       .       .       .       .
// 942          .       .       .       .       .
// 943          .       .       .       .       .
// **944**      .       .       D       .       .
// 945          D       .       .       .       .
// 946          .       .       .       .       .
// 947          .       .       .       .       .
// 948          .       .       .       .       .
// 949          .       D       .       .       .
//
// The earliest bus you could take is bus ID 59. It doesn't depart until timestamp 944, so you
// would need to wait 944 - 939 = 5 minutes before it departs. Multiplying the bus ID by the number
// of minutes you'd need to wait gives 295.
//
// What is the ID of the earliest bus you can take to the airport multiplied by the number of
// minutes you'll need to wait for that bus?
//
// --- Part Two ---
//
// The shuttle company is running a contest: one gold coin for anyone that can find the earliest
// timestamp such that the first bus ID departs at that time and each subsequent listed bus ID
// departs at that subsequent minute. (The first line in your input is no longer relevant.)
//
// For example, suppose you have the same list of bus IDs as above:
//
// 7,13,x,x,59,x,31,19
//
// An x in the schedule means there are no constraints on what bus IDs must depart at that time.
//
// This means you are looking for the earliest timestamp (called t) such that:
//
//     Bus ID 7 departs at timestamp t.
//     Bus ID 13 departs one minute after timestamp t.
//     There are no requirements or restrictions on departures at two or three minutes after
//      timestamp t.
//     Bus ID 59 departs four minutes after timestamp t.
//     There are no requirements or restrictions on departures at five minutes after timestamp t.
//     Bus ID 31 departs six minutes after timestamp t.
//     Bus ID 19 departs seven minutes after timestamp t.
//
// The only bus departures that matter are the listed bus IDs at their specific offsets from t.
// Those bus IDs can depart at other times, and other bus IDs can depart at those times. For
// example, in the list above, because bus ID 19 must depart seven minutes after the timestamp at
// which bus ID 7 departs, bus ID 7 will always also be departing with bus ID 19 at seven minutes
// after timestamp t.
//
// In this example, the earliest timestamp at which this occurs is 1068781:
//
// time         bus 7   bus 13  bus 59  bus 31  bus 19
// 1068773        .       .       .       .       .
// 1068774        D       .       .       .       .
// 1068775        .       .       .       .       .
// 1068776        .       .       .       .       .
// 1068777        .       .       .       .       .
// 1068778        .       .       .       .       .
// 1068779        .       .       .       .       .
// 1068780        .       .       .       .       .
// **1068781**    D       .       .       .       .
// **1068782**    .       D       .       .       .
// **1068783**    .       .       .       .       .
// **1068784**    .       .       .       .       .
// **1068785**    .       .       D       .       .
// **1068786**    .       .       .       .       .
// **1068787**    .       .       .       D       .
// **1068788**    D       .       .       .       D
// 1068789        .       .       .       .       .
// 1068790        .       .       .       .       .
// 1068791        .       .       .       .       .
// 1068792        .       .       .       .       .
// 1068793        .       .       .       .       .
// 1068794        .       .       .       .       .
// 1068795        D       D       .       .       .
// 1068796        .       .       .       .       .
// 1068797        .       .       .       .       .
//
// In the above example, bus ID 7 departs at timestamp 1068788 (seven minutes after t). This is
// fine; the only requirement on that minute is that bus ID 19 departs then, and it does.
//
// Here are some other examples:
//
//     The earliest timestamp that matches the list 17,x,13,19 is 3417.
//     67,7,59,61 first occurs at timestamp 754018.
//     67,x,7,59,61 first occurs at timestamp 779210.
//     67,7,x,59,61 first occurs at timestamp 1261476.
//     1789,37,47,1889 first occurs at timestamp 1202161486.
//
// However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger
// than 100000000000000!
//
// What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching
// their positions in the list?

fn main() {
    let input = include_str!("../input");

    let part_1 = part_1(&input);
    assert_eq!(part_1, 115);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&input);
    assert_eq!(part_2, 756261495958122);
    println!("Part 2: {}", part_2);
}

fn part_1(input: &str) -> usize {
    let (earliest_departure, services) = parse_input(input);
    let (next_service, next_departure) = next_service(earliest_departure, &services);

    (next_departure - earliest_departure) * next_service
}

fn part_2(input: &str) -> usize {
    let (_earliest_departure, services) = parse_input(input);
    part_2_inner(&services)
}

// https://en.wikipedia.org/wiki/Chinese_remainder_theorem
fn chinese_remainder_theorem(a1: i128, a2: i128, n1: i128, n2: i128) -> i128 {
    let (_, m1, _) = modinverse::egcd(n1, n2);
    (a1 + (a2 - a1) * m1 * n1).rem_euclid(n1 * n2)
}

fn lst_to_dict(services: &[Service]) -> Vec<(i128, i128)> {
    services
        .iter()
        .enumerate()
        .filter_map(|(i, s)| match s {
            Service::WithTime(t) => Some((i, *t)),
            Service::AnyTime => None,
        })
        // c = offset in list
        // n1 = bus number
        .map(|(c, n1)| {
            let c = c as i128;
            let n1 = n1 as i128;
            let a1 = (n1 - c) * if c > 0 { 1 } else { 0 } % n1;
            (n1, a1)
        })
        .collect()
}

fn part_2_inner(services: &[Service]) -> usize {
    let list = lst_to_dict(services);

    // Initialize
    let (mut n1, mut a1) = list[0];
    let (mut n2, mut a2) = list[1];

    let mut rem = chinese_remainder_theorem(a1, a2, n1, n2);

    // Skip 2 (handled above)
    for (new_n1, new_a1) in list.iter().skip(2) {
        // Increment product
        n2 = n1 * n2;

        a2 = rem;

        a1 = *new_a1;
        n1 = *new_n1;

        rem = chinese_remainder_theorem(a1, a2, n1, n2);
    }

    rem as _
}

#[derive(Debug, Clone, Copy)]
enum Service {
    WithTime(usize),
    AnyTime,
}

fn parse_input(input: &str) -> (usize, Vec<Service>) {
    let mut lines = input.lines();

    let earliest_departure = lines
        .next()
        .expect("no timestamp found")
        .parse()
        .expect("not a valid integer");

    let services = lines
        .next()
        .expect("no services found")
        .split(',')
        .map(|s| {
            s.parse()
                .ok()
                .map(Service::WithTime)
                .unwrap_or(Service::AnyTime)
        })
        .collect();

    (earliest_departure, services)
}

fn next_service(earliest_departure: usize, services: &[Service]) -> (usize, usize) {
    services
        .iter()
        .filter_map(|&service| {
            match service {
                Service::WithTime(service) => {
                    // Find the first timestamp after the earliest departure
                    let timestamp = (1..)
                        .find_map(|i| {
                            let timestamp = i * service;
                            if timestamp > earliest_departure {
                                Some(timestamp)
                            } else {
                                None
                            }
                        })
                        .unwrap();

                    Some((service, timestamp))
                }
                Service::AnyTime => None,
            }
        })
        // Find the earliest timestamp
        .min_by_key(|x| x.1)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn services() -> Vec<Service> {
        vec![
            Service::WithTime(7),
            Service::WithTime(13),
            Service::AnyTime,
            Service::AnyTime,
            Service::WithTime(59),
            Service::AnyTime,
            Service::WithTime(31),
            Service::WithTime(19),
        ]
    }

    #[test]
    fn readme_example_part_1() {
        let earliest_departure = 939;
        assert_eq!(next_service(earliest_departure, &services()), (59, 944));
    }

    #[test]
    fn readme_example_part_2() {
        assert_eq!(part_2_inner(&services()), 1068781);
    }

    #[test]
    fn test_chi_rem() {
        assert_eq!(chinese_remainder_theorem(0, 12, 7, 13), 77);
        assert_eq!(chinese_remainder_theorem(55, 77, 59, 91), 350);
    }
}
