use linked_list::singly_linked_list::SinglyLinkedList;

fn main() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    println!("List is empty? >> {}", sll.is_empty());
    println!("List:");
    sll.print();
    sll.push(31);
    sll.push(-268);
    sll.push(4);
    sll.push(20);
    sll.push(2);
    println!("Added a few elements to the list.");
    println!("List is empty? >> {}", sll.is_empty());
    println!("List:");
    sll.print();
    sll.pop();
    sll.pop();
    println!("Popped of 2 elements from the list.");
    println!("Top: {}", sll.peek().unwrap());
    println!("Pushed \"54\" to the list.");
    sll.push(54);
    println!("List:");
    sll.print();
}
