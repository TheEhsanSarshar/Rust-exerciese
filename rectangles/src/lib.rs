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
        // places where disjoint happened
        let mut disjoint_indices: Vec<usize> = vec![];
        let each_line_plus_indices: Vec<String> = each_line
            .chars()
            .enumerate()
            .map(|(index, token)| {
                if token == '+' {
                    format!("{}+", index).to_string()
                } else if token == '|' {
                    format!("{}|", index).to_string()
                } else {
                    if token == ' ' {
                        disjoint_indices.push(index);
                    }
                    "x".to_string()
                }
            })
            .filter(|value| value != "x")
            .collect();

        let mut each_line_limpings_index_pairs: Vec<String> = vec![];
        if each_line_plus_indices.len() > 0 {
            let mut i = 0;
            while i < each_line_plus_indices.len() {
                let i_plus_index = &each_line_plus_indices[i];
                let mut j = i + 1;

                if each_line_plus_indices.len() == 1 {
                    each_line_limpings_index_pairs.push(format!("{}|", i_plus_index));
                    break;
                }

                while j < each_line_plus_indices.len() {
                    let j_plus_index = &each_line_plus_indices[j];
                    // if there is a disjoint like `+  +` between two + then they can't make a like
                    // so skip them
                    if j_plus_index.contains("+") && i_plus_index.contains("+") {
                        let i_token_index_value: i32 = i_plus_index
                            .replace("+", "")
                            .replace("|", "")
                            .parse()
                            .unwrap();
                        let j_token_index_value: i32 = j_plus_index
                            .replace("+", "")
                            .replace("|", "")
                            .parse()
                            .unwrap();

                        let is_there_a_disjoint_between_ivalue_and_jvalue =
                            disjoint_indices.iter().find(|disjoing_index| {
                                disjoing_index > &&(i_token_index_value as usize)
                                    && disjoing_index < &&(j_token_index_value as usize)
                            });
                        if let Some(_) = is_there_a_disjoint_between_ivalue_and_jvalue {
                            break;
                        }
                    }
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
                } else {
                    // if its index|index| or index+index| or index|index+
                    let splitted: Vec<&str> = each_limping.split("+").collect();
                    if splitted.len() == 1 {
                        // it's not a line vesion it's a | vesion
                        break;
                    }
                    let each_limping_joint_version = splitted.join("|");
                    let each_limping_joint_first_vesion =
                        format!("{}|{}+", splitted[0], splitted[1]);
                    let each_limping_joint_second_vesion =
                        format!("{}+{}|", splitted[0], splitted[1]);
                    if !(each_other_row.contains(&each_limping_joint_version)
                        || each_other_row.contains(&each_limping_joint_first_vesion)
                        || each_other_row.contains(&each_limping_joint_second_vesion))
                    {
                        break;
                    }
                }
            }
        }
        i += 1
    }

    number_of_boxes
}
