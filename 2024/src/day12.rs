use std::{collections::HashSet, vec};

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day12, part1)]
pub fn part1(plots: &Vec<Vec<char>>) -> i32 {
    let mut total_price = 0;

    let mut unique_plots: HashSet<char> = HashSet::new();
    plots.into_iter().for_each(|line| {
        line.into_iter().for_each(|&c| {
            unique_plots.insert(c);
        })
    });

    for plot in unique_plots {
        let sections = find_plot_sections(plot, plots);
        total_price += compute_plot_price(&sections)
    }

    return total_price;
}

#[aoc(day12, part2)]
pub fn part2(plots: &Vec<Vec<char>>) -> i32 {
    let mut total_price = 0;

    let mut unique_plots: HashSet<char> = HashSet::new();
    plots.into_iter().for_each(|line| {
        line.into_iter().for_each(|&c| {
            unique_plots.insert(c);
        })
    });

    for plot in unique_plots {
        let sections = find_plot_sections(plot, plots);
        total_price += compute_plot_price_with_discount(&sections)
    }

    return total_price;
}

fn compute_plot_price_with_discount(sections: &Vec<Vec<(usize, usize)>>) -> i32 {
    let mut plot_price = 0;
    sections.into_iter().for_each(|section| {
        let section_clone = section.clone();

        let mut corners = 0;
        section.into_iter().for_each(|&(si, sj)| {
            if !section.contains(&(si - 1, sj)) && !section.contains(&(si, sj - 1)) {
                // External upper left corner
                corners += 1
            }

            if !section.contains(&(si - 1, sj)) && !section.contains(&(si, sj + 1)) {
                // External upper right corner
                corners += 1
            }

            if !section.contains(&(si + 1, sj)) && !section.contains(&(si, sj - 1)) {
                // External lower left corner
                corners += 1
            }

            if !section.contains(&(si + 1, sj)) && !section.contains(&(si, sj + 1)) {
                // External lower right corner
                corners += 1
            }

            if !section.contains(&(si - 1, sj - 1))
                && section.contains(&(si, sj - 1))
                && section.contains(&(si - 1, sj))
            {
                // Internal upper left corner
                corners += 1
            }

            if !section.contains(&(si - 1, sj + 1))
                && section.contains(&(si - 1, sj))
                && section.contains(&(si, sj + 1))
            {
                // Internal upper right corner
                corners += 1
            }

            if !section.contains(&(si + 1, sj - 1))
                && section.contains(&(si + 1, sj))
                && section.contains(&(si, sj - 1))
            {
                // Internal lower left corner
                corners += 1
            }

            if !section.contains(&(si + 1, sj + 1))
                && section.contains(&(si + 1, sj))
                && section.contains(&(si, sj + 1))
            {
                // Internal lower right corner
                corners += 1
            }
        });

        plot_price += corners * section_clone.len() as i32;
    });
    return plot_price;
}

fn compute_plot_price(sections: &Vec<Vec<(usize, usize)>>) -> i32 {
    let mut plot_price = 0;
    sections.into_iter().for_each(|section| {
        let section_clone = section.clone();
        let mut perimeter: i32 = 0;
        section.into_iter().for_each(|&(si, sj)| {
            for neighbor in vec![(si + 1, sj), (si - 1, sj), (si, sj + 1), (si, sj - 1)] {
                if !section_clone.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        });
        plot_price += perimeter * section_clone.len() as i32;
    });
    return plot_price;
}

fn find_plot_sections(plot: char, plots: &Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let mut sections: Vec<Vec<(usize, usize)>> = vec![];

    for (i, row) in plots.into_iter().enumerate() {
        for (j, &value) in row.into_iter().enumerate() {
            if value != plot {
                continue;
            }

            if sections.len() == 0 {
                // No section found for this value yet
                sections.push(vec![(i, j)]);
            } else {
                // Check if the plot belongs to an existing section
                let mut belongs_to_section: i32 = -1;
                let mut sections_to_merge: Vec<usize> = vec![];

                for (s, section) in sections.clone().into_iter().enumerate() {
                    for (si, sj) in section {
                        if (si as i32 - i as i32).abs() + (sj as i32 - j as i32).abs() == 1 {
                            // Right next to current value
                            if belongs_to_section < 0 {
                                belongs_to_section = s as i32;
                            } else if s as i32 != belongs_to_section {
                                sections_to_merge.push(s as usize);
                            }
                        }
                    }
                }
                if belongs_to_section >= 0 {
                    // Append plot to the first section it belongs to
                    sections[belongs_to_section as usize].push((i, j));

                    // Merge plots if needed
                    for &s in &sections_to_merge {
                        let section_to_merge = sections[s].clone();
                        sections[belongs_to_section as usize].extend(section_to_merge);
                    }
                    sections_to_merge.sort_by(|a, b| b.cmp(a)); // Delete merged sections in descending order
                    for s in sections_to_merge {
                        sections.remove(s);
                    }
                } else {
                    sections.push(vec![(i, j)]);
                }
            }
        }
    }

    return sections;
}
