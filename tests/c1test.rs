use autoglyph::c1::part1;

const GRID: &str = "\
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

#[test]
fn test_autoglyph_solver() {
    let steps = 6;
    let expected = 16 * 64; // 16 cells visited in 6 steps * 64 per cell
    let answer = part1(GRID, steps);
    assert_eq!(answer, expected);
}
