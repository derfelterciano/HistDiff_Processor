pub fn clean_well_names(wells: &Vec<String>) -> Vec<String> {
    let res: Vec<String> = wells
        .iter()
        .map(|x| {
            if x.len() > 2 {
                let letter = &x[0..1];
                let numeric = &x[1..];

                match numeric.parse::<u32>() {
                    Ok(num) => return format!("{}{}", letter, num),
                    Err(_) => return x.clone(),
                }
            } else {
                return x.clone();
            }
        })
        .collect();

    return res;
}
