use autoglyph::c1::part1;

const CONTENT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

#[test]
fn test_part1() {
    let answer = part1(CONTENT, 6);
    assert_eq!(answer, 16); // Adjust this based on the expected output
}

#[test]
fn test_part2a() {
    let answer = part1(CONTENT, 10);
    assert_eq!(answer, 50);
    let answer = part1(CONTENT, 50);
    assert_eq!(answer, 1594);
    let answer = part1(CONTENT, 100);
    assert_eq!(answer, 6536);
    // Uncomment these tests as needed
    // let answer = part1(CONTENT, 500);
    // assert_eq!(answer, 167004);
    // let answer = part1(CONTENT, 1000);
    // assert_eq!(answer, 668697);
    // let answer = part1(CONTENT, 5000);
    // assert_eq!(answer, 16733044);
}
