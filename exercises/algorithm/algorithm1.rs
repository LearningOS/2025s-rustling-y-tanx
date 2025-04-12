/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
    链表合并
*/
// I AM DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>, // 安全地表示"非空的裸指针"，这相当于直接使用指针来操作了
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: i32,
    start: Option<NonNull<Node<T>>>,    // 节点指针
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    // 添加一个节点
    pub fn add(&mut self, obj: T) {
        let node = Box::new(Node::new(obj));    // 将node分配到堆上，这相当于为node创建了地址
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });    // 获得了指向node在堆上地址的裸指针，将给它传递NonNull::new_unchecked()，这获得了对node的指针
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },    // end_ptr.as_ptr()指向了在堆上的节点node，可以解引用获得node节点本身
        }
        self.end = node_ptr;
        self.length += 1;
    }

    /// 找到索引为index的节点，获得它的值
    pub fn get_val(&mut self, index: i32) -> Option<&T> {
        // self.get_ith_node_val(self.start, index)
        let node_ptr = self.get_node(index);
        match node_ptr {
            None => None,
            Some(node) => Some(unsafe { &(*node.as_ptr()).val}),
        }
    }

    /// 递归寻找，base条件：要么没有找到，到达了None；要么递归到目标节点，index = 0
    // fn get_ith_node_val(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        // match node {
        //     None => None,
        //     Some(next_ptr) => match index {
        //         0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
        //         _ => self.get_ith_node_val(unsafe { (*next_ptr.as_ptr()).next }, index - 1),    // 递归查找
        //     },
        // }

    // }
    // 如果index超出了长度或小于0，则返回None；否则返回索引为index的节点的指针
    fn get_node(&mut self, index: i32) -> Option<NonNull<Node<T>>> {
        if index < 0 {
            return None;
        }
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<NonNull<Node<T>>> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => node,
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            } 
        }
    } 

    /// 插入到指定位置
    fn insert(&mut self, obj: T, index: i32) -> Result<(), String> {
        // 合法性检查
        if index < 0 || index > self.length {
            return Err("index is invalid!".to_string());
        }

        // 为obj创建node节点
        let mut node = Box::new(Node::new(obj));
        let node_ptr: NonNull<Node<T>> = unsafe { NonNull::new_unchecked(Box::into_raw(node)) };

        // 特殊情况：index=0，插入到头部
        if index == 0 {
            // 先插入
            unsafe { (*node_ptr.as_ptr()).next = self.start };

            // 再更新self.start
            self.start = Some(node_ptr);
            self.length += 1;
            return Ok(());
        }

        // 其他情况：先链接到后面的节点，然后链接前面的节点
        // 获得index前后的指针
        let back_ptr: Option<NonNull<Node<T>>> = self.get_node(index); 
        let front_ptr: NonNull<Node<T>> = self.get_node(index - 1).expect("front node is None!");

        // 先设置node的next指向后节点（可以为None，例如插入到end_node之后）
        unsafe {(*node_ptr.as_ptr()).next = back_ptr };
        // 再设置前节点的next指向node
        unsafe { (*front_ptr.as_ptr()).next = Some(node_ptr) };

        // 注意更新self记录
        if index == self.length {
            self.end = Some(node_ptr);
        }
        self.length += 1;

        Ok(())
    }

	pub fn merge(mut list_a:LinkedList<T>, mut list_b:LinkedList<T>) -> Self
    where
        T: Ord + ToOwned<Owned = T>,
	{
		//TODO
        // 新创建list_c，用于装载二者的merge结果
        let mut list_c = LinkedList::<T>::new();

        // 双指针排序
        let mut a_node_index = 0;
        let mut b_node_index = 0;

        while (a_node_index < list_a.length) && (b_node_index < list_b.length) {
            let a_val = list_a.get_val(a_node_index).unwrap();
            let b_val = list_b.get_val(b_node_index).unwrap();
            
            // 比较两个节点的值
            if a_val < b_val {
                // a的值更小，将a的节点加入到list_c中
                list_c.add(a_val.to_owned());
                a_node_index += 1;
            }else {
                list_c.add(b_val.to_owned());
                b_node_index += 1;
            }
        }

        // 插入剩余的节点
        while a_node_index < list_a.length {
            let a_val = list_a.get_val(a_node_index).unwrap();
            list_c.add(a_val.to_owned());
            a_node_index += 1;
        }

        while b_node_index < list_b.length {
            let b_val = list_b.get_val(b_node_index).unwrap();
            list_c.add(b_val.to_owned());
            b_node_index += 1;
        }

        list_c
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get_val(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get_val(i as i32).unwrap());
		}
	}
}