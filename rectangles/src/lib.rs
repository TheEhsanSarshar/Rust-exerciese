pub fn count(lines: &[&str]) -> u32 {
    if lines.len() == 0 {
        return 0;
    }

    let mut num_of_plus = 0;

    let mut lines_of_limpings: Vec<Vec<String>> = vec![];
    for each_line in lines {
        if each_line.contains('+') {
            num_of_plus += 1;
        }
        let each_line_plus_indices: Vec<usize> = each_line
            .match_indices('+')
            .map(|(index, _)| index)
            .collect();

        let mut each_line_limpings_index_pairs: Vec<String> = vec![];
        if each_line_plus_indices.len() > 0 {
            let mut i = 0;
            while i < each_line_plus_indices.len() {
                let i_plus_index = each_line_plus_indices[i];
                let mut j = i + 1;

                while j < each_line_plus_indices.len() {
                    let j_plus_index = each_line_plus_indices[j];
                    each_line_limpings_index_pairs
                        .push(format!("{}{}", i_plus_index, j_plus_index));
                    j += 1;
                }
                i += 1;
            }
        }
        if each_line_limpings_index_pairs.len() > 0 {
            lines_of_limpings.push(each_line_limpings_index_pairs)
        }
    }

    if num_of_plus == 0 {
        return 0;
    }

    let mut number_of_boxes = 0;
    let mut i = 0;
    while i < lines_of_limpings.len() {
        let each_line_of_limpings = &lines_of_limpings[i];
        let slice_of_other_remained_rows = &lines_of_limpings[i + 1..];
        for each_limping in each_line_of_limpings {
            for each_other_row in slice_of_other_remained_rows {
                if each_other_row.contains(each_limping) {
                    number_of_boxes += 1;
                }
            }
        }
        i += 1
    }

    println!("number of boxex {number_of_boxes}");

    number_of_boxes
}
