use std::io;
use std::collections::HashMap;

// not my code (but modified)
fn insert_at_predicate<T, P>(vec: &mut Vec<T>, predicate: P, value: T)
where
    P: Fn(&T, &T) -> bool,
{
    if let Some(index) = vec.iter().position(|element| predicate(element, &value)) {
        vec.insert(index, value);
    } else {
        // Where predicate doesn't match any element, insert at the end:
        vec.push(value); 
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let mut before_lists: HashMap<i32, Vec<i32>> = HashMap::new();

    // non-idiomatic approach to reading all the input
    // process the top section of the input until we reach an empty line
    while let Some(Ok(line)) = lines.next() {
        //println!("{:?}", line);

        // empty line found, bail out
        if line.len() == 0 { break };

        let pages: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
        //println!("{} {}", pages[0], pages[1]);
        
        before_lists
            .entry(pages[0]) // find the vector of pages after this page
            .or_insert(Vec::new())  // or make a new vector 
            .push(pages[1]);                              // add the new page
    }

    // classic rust moment, I can't declare this closure _before_ I've built the before_list as that runs afoul of the borrow checker
    let page_appears_in_before_list = |page_that_is_before: &i32, page_that_is_after: &i32|  {
        return before_lists.get(page_that_is_before)
            .and_then(|list| Some(list.contains(page_that_is_after)))
            .unwrap_or(false);

        // I think I like this style better, so leaving it here for reference
        /*return if let Some(list) = before.get(element) {
            list.contains(new_value)
        } else {
            false
        };*/
    };

    let mut total = 0;

    // process the bottom section of the input, and calculate the total as we go
    while let Some(Ok(line)) = lines.next() {
        let pages: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();      

        // loop through all pages in the list (except the last as it is correctly ordered if we get that far)
        let is_correctly_ordered = pages.iter().take(pages.len() -1).enumerate().all(|(i, page_that_is_before)| {
            // check that all pages following the current page appear in the before list
            pages.iter().skip(i + 1).all(|page_that_is_after| page_appears_in_before_list(page_that_is_before, page_that_is_after))
        });

        // if not correctly ordered, add the pages one at a time in their correct ordered positions
        if !is_correctly_ordered {
            let mut ordered:Vec<i32> = Vec::new();
            for page in pages {
                insert_at_predicate(&mut ordered, |x, y| { !page_appears_in_before_list(x, y)}, page);
            }
            //println!("{ordered:?}");

            total += ordered[ordered.len() / 2];
        }
    }

    println!("{}", total);
}
