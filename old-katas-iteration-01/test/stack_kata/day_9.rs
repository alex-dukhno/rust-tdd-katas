pub use tdd_kata::stack_kata::day_9::Stack;

describe! stack_tests {

    before_each {
        let mut stack: Stack<i32> = Stack::new(20);
    }

    it "should create a new empty stack" {
        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
    }

    it "should increase the stack size when push into it" {
        let old_size = stack.size();
        stack.push(1);

        assert_eq!(old_size + 1, stack.size());
        assert!(!stack.is_empty());
    }

    it "should decrease stack size when pop from it" {
        stack.push(1);
        let old_size = stack.size();
        stack.pop();

        assert_eq!(old_size - 1, stack.size());
    }

    it "should pop value that was pushed into stack" {
        stack.push(10);

        assert_eq!(stack.pop(), Some(10));

        stack.push(20);

        assert_eq!(stack.pop(), Some(20));
    }

    it "should pop last that was pushed first into stack" {
        stack.push(10);
        stack.push(30);
        stack.push(20);
        stack.push(40);
        stack.push(50);

        assert_eq!(stack.pop(), Some(50));
        assert_eq!(stack.pop(), Some(40));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(10));
    }
}