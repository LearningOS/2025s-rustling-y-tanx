// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {   // 创建迭代器into_iter()（消耗本身），iter()（不可变的引用），iter_mut（可变引用）
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2;  // 迭代器中每个元素的类型都是&i32
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| { 
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2
    }).collect()    // collect转换为数组Vec<i32>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        // Range类型，迭代器的元素类型为i32
        // 闭包参数不会发生自动解引用，而是在%号处发生了自动解引用
        // 如果不使用Range类型，而是一般的元素类型为&i32的迭代器，应该写为filter(|&&x| x %2 == 0)
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
