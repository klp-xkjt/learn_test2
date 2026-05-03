#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quality {
    Bad,
    Good,
    VeryGood
}
#[derive(Debug)]
pub struct Dishes {
    pub quality: Quality
}
pub fn choose_good_dishes(dish: Vec<Dishes>) -> Vec<Dishes> {
    let dish_iter = dish.into_iter();
    dish_iter
        .filter(|x| x.quality == Quality::Good || x.quality == Quality::VeryGood)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn next_test1() {
        let vec_test = vec![0, 1, 2, 3];
        let mut vec_iter = vec_test.iter();
        assert_eq!(vec_iter.next(), Some(&0));
        println!("{vec_test:?}")
    }
    #[test]
    fn next_test2() {
        let vec_test = vec![0, 1, 2, 3];
        let mut vec_into = vec_test.into_iter();
        assert_eq!(vec_into.next(), Some(0));
        // println!("{vec_test:?}");
        // vec_test.push(4);
    }
    #[test]
    fn next_test3() {
        let mut vec_test = vec![0, 1, 2, 3];
        let mut vec_into = vec_test.iter_mut();
        assert_eq!(vec_into.next(), Some(&mut 0));
        println!("{vec_test:?}");
        vec_test.push(4);
        println!("{vec_test:?}");
    }
    #[test]
    fn consuming() {
        let vec_test = vec![0, 1, 2, 3];
        let vec_iter = vec_test.iter();
        println!("Sum: {vec_iter:?}");
        let vec_sum: i32 = vec_iter.sum();
        println!("Sum: {}", vec_sum);
        // println!("Sum: {vec_iter:?}");
    }
    #[test]
    fn adaptors() {
        let vec_test = vec![-12, -1, 0, 16, 25, 33];
        let vec_iter = vec_test.iter();
        let results: Vec<i32> = vec_iter
            .filter(|&&x| x >= 0)
            .map(|&x| x * 2)
            .collect();
        println!("{:?}", results);
    }
    #[test]
    fn dishes_choose() {
        let dishes_before: Vec<Dishes> = vec![
            Dishes {
                quality: Quality::Good
            },
            Dishes {
                quality: Quality::VeryGood
            },
            Dishes {
                quality: Quality::Bad
            },
            Dishes {
                quality: Quality::Good
            }
        ];
        let after_choosing = choose_good_dishes(dishes_before);
        println!("{after_choosing:?}")
    }
}