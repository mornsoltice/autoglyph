use autoglyph::c2;
const CONTENT: &'static str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

const CONTENT2: &'static str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

#[test]
fn part1_1() {
    let answer = c2::part1(CONTENT);
    assert_eq!(answer, 4);
}

#[test]
fn part1_2() {
    let answer = c2::part1(CONTENT2);
    assert_eq!(answer, 8);
}

const CONTENT3: &'static str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

const CONTENT4: &'static str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

const CONTENT5: &'static str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

#[test]
fn part2_1() {
    let answer = c2::part2(CONTENT3, false);
    assert_eq!(answer, 4);
}

#[test]
fn part2_2() {
    let answer = c2::part2(CONTENT4, false);
    assert_eq!(answer, 8);
}

#[test]
fn part2_3() {
    let answer = c2::part2(CONTENT5, true);
    assert_eq!(answer, 10);
}
