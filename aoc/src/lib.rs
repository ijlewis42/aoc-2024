pub fn permutations<T: Copy>(items: Vec<T>, length: usize) -> impl Iterator<Item = Vec<T>> {
    let mut indexes = vec![0; length];
    let mut finished = false;

    std::iter::from_fn(move | | {
        if finished {
            return None;
        }

        let mut temp: Vec<T> = Vec::new();
        for index in indexes.iter() {
            temp.push(items[*index]);
        }

        let mut i = (indexes.len() - 1) as i32;
        while i >= 0 {
            indexes[i as usize] += 1;
            if indexes[i as usize] < items.len() {
                break;
            } else {
                indexes[i as usize] = 0;
                i -= 1;
            }
        }

        if i < 0 { finished = true; }

        Some(temp)
    })
}

pub fn permutations_from_iter<T: Copy>(items: impl Iterator<Item = T> + Clone, length: usize) -> impl Iterator<Item = Vec<T>> {
    let mut indexes = vec![0; length];
    let mut finished = false;
    let items = items.collect::<Vec<T>>();

    std::iter::from_fn(move | | {
        if finished {
            return None;
        }

        let mut temp: Vec<T> = Vec::new();
        for index in indexes.iter() {
            temp.push(items[*index]);
        }

        let mut i = (indexes.len() - 1) as i32;
        while i >= 0 {
            indexes[i as usize] += 1;
            if indexes[i as usize] < items.len() {
                break;
            } else {
                indexes[i as usize] = 0;
                i -= 1;
            }
        }

        if i < 0 { finished = true; }

        Some(temp)
    })
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/