/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
    排序问题
*/

fn sort<T>(array: &mut [T])
where
    T: Ord, 
{
	//TODO
    bubble_sort(array);
    // insert_sort(array);
    // quick_sort(array);
}

// bubble sort
fn bubble_sort<T>(array: &mut [T])
where
    T: Ord,
{   
    let len = array.len();
    for i in 0..len {    // i表示右侧已经排好序的元素个数,每轮排序都会将0~len-1-i中的最大元素移动到右侧
        for j in 0..len - 1 - i {   
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

// insertion sort
// 从左到右遍历数组，把每个数插到它左边已经排好序的部分的正确位置
fn insert_sort<T>(array: &mut [T])
where
    T: Ord,
{
    let len = array.len();
    for i in 1..len {    // 左边排好序的部分为0..=i-1
       let mut j = i;    // 本轮要排序的元素为arr[i]
       // 扫描排好序的部分，两两交换
       while j > 0 && array[j - 1] > array[j] {
        array.swap(j - 1, j);
        j -= 1;
       }
    }
}

// 快速排序
// 每次都将数组分为两个部分，一部分<base，另一部分>base，这样就能确定base的位置了
// 快速排序作为递归函数，要相信它能够使array有序，它的一步：将中间值（划分pivot）放到对应位置，很多步：将左半部分、右半部分数组进行排序；base：len<=1，就一个元素或者没有
fn quick_sort<T>(array: &mut [T]) 
where 
    T: Ord,
{
    let len = array.len();
    if len <= 1 {
        return;
    }

    // 中间值作为划分pivot，将数组划分为两个部分，并将中间值放到正确的位置上
    let pivot_index = partition(array);
    
    // 递归排序左边和右边的数组
    quick_sort(&mut array[0..pivot_index]);
    quick_sort(&mut array[pivot_index + 1..]);
}

fn partition<T: Ord>(array: &mut [T]) -> usize {
    let len = array.len();
    let pivot_index = len / 2;

    // 先将pivot放到len-1处，方便后续遍历
    array.swap(pivot_index, len-1);

    // 维护一个小于pivot的序列，i指向该序列最后一个元素的下一个元素
    // j遍历整个数组，当发现<pivot的值时，就与arr[i]进行交换，这就从数组中挑出了所有<pivot值的序列 0..i-1
    let mut i = 0;
    for j in 0..len - 1 {
        if array[j] <= array[len - 1] {
            // 将小于pivot的值挑出来，放到array[i]处
            array.swap(i, j);
            i += 1; // i后移
        }
    }

    // 将pivot放到i处，左边是<=pivot的值，右边是>pivot的值
    array.swap(i, len - 1);
    i
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}