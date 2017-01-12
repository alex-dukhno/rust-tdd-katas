use tdd_kata::linked_list_stack_kata::day_2::Stack;

#[test]
fn creates_an_empty_stack() {
    let mut stack: Stack<i32> = Stack::new();
    assert_eq!(stack.pop(), None);
}

#[test]
fn adds_an_element_to_a_stack() {
    let mut stack = Stack::new();

    stack.push(1);

    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

#[test]
fn adds_elements_to_a_stack() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    assert_eq!(stack.pop(), Some(30));
    assert_eq!(stack.pop(), Some(20));
    assert_eq!(stack.pop(), Some(10));
    assert_eq!(stack.pop(), None);
}
