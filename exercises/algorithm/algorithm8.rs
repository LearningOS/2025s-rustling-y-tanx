/*
	queue
	This question requires you to use queues to implement the functionality of the stac
    实现队列
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,   // 用栈模拟队列
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    // 入队，加入到队列（栈）的末尾
    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    // 出队，移除队列（栈）的头部的元素
    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))    
        } else {
            Err("Queue is empty")
        }
    }

    // 队首元素
    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

// 用两个队列模拟栈，一个模拟栈
pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            // q1是主队列，q2用于暂存新数据
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        // 首先q2将新元素入队，这使得新元素在队首
        self.q2.enqueue(elem);

        // 然后q1将原有的元素依次加入到q2，这仍然保证了新元素在队首
        while let Ok(val) = self.q1.dequeue() {
            self.q2.enqueue(val);
        }

        // 交换q1和q2，q1总是主队列，q2被清空了
        std::mem::swap(&mut self.q1, &mut self.q2);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        // q1直接出队就可以了
        self.q1.dequeue().map_err(|e| "Stack is empty")
		
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        if self.q1.is_empty() {
            true
        }else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}