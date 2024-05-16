mod data_structures;

fn main() {
    // Create a new empty linked list
    let mut list = data_structures::linked_list::LinkedList::new();

    // Add some elements to the front of the list
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    // Display the list
    list.display();

    // Remove and display the elements from the front of the list
    println!("Popped: {:?}", list.pop_front());
    println!("Popped: {:?}", list.pop_front());
    println!("Popped: {:?}", list.pop_front());

    // Display the list after popping elements
    list.display();

    // Add some elements to the back of the list
    list.push_back(4);

    // Insert an element at index 1
    list.insert_at_ith(1, 5);

    // Insert at head
    list.push_front(6);

    // Insert at tail
    list.push_back(7);

    // Display the list
    list.display();

    // Remove and display the elements from the front of the list
    println!("Popped: {:?}", list.pop_front());

    // Remove at tail
    println!("Popped: {:?}", list.pop_back());

    // Delete at tail
    list.pop_back();

    // Display the list after popping elements
    list.display();

    // Get at index
    println!("Element at index 1: {:?}", list.get(2));

}