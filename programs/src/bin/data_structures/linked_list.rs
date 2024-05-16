// Define a struct for the Node of the linked list
struct Node<T> {
    data: T,                    // Data stored in the node
    next: Option<Box<Node<T>>>, // Reference to the next node, wrapped in an Option
}

// Implementation block for the Node struct
impl<T> Node<T> {
    // Constructor function for creating a new Node with the given data
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

// Define the LinkedList struct
pub struct LinkedList<T: std::fmt::Debug> {
    head: Option<Box<Node<T>>>, // Reference to the head of the list, wrapped in an Option
}

// Implementation block for the LinkedList struct
impl<T: std::fmt::Debug> LinkedList<T> {
    // Constructor function for creating a new empty LinkedList
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Method for adding a new node with data to the front of the list
    pub fn push_front(&mut self, data: T) {
        // Create a new node with the given data
        let new_node = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));
        // Update the head of the list to point to the new node
        self.head = new_node;
    }

    // Method for removing and returning the data from the front of the list
    pub fn pop_front(&mut self) -> Option<T> {
        // Take the head of the list and replace it with None, extracting the node
        self.head.take().map(|node| {
            // Extract the data from the node
            let data = node.data;
            // Update the head of the list to point to the next node, if any
            self.head = node.next;
            // Return the extracted data
            data
        })
    }

    // Method for adding a new node with data to the back of the list
    pub fn push_back(&mut self, data: T) {
        // Create a new node with the given data
        let mut current = &mut self.head;
        // Iterate over the list to find the last node
        while let Some(node) = current {
            current = &mut node.next;
        }
        // Update the last node to point to the new node
        *current = Some(Box::new(Node::new(data)));
    }

    // Method for removing and returning the data from the back of the list
    pub fn pop_back(&mut self) -> Option<T> {
        // Check if the list is empty
        if self.head.is_none() {
            return None;
        }
        // Check if the list has only one element
        if self.head.as_ref().unwrap().next.is_none() {
            return self.pop_front();
        }
        // Find the second last node in the list
        let mut current = &mut self.head;
        while current
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .is_some()
        {
            current = &mut current.as_mut().unwrap().next;
        }
        // Take the last node and replace it with None, extracting the node
        current.take().map(|node| {
            // Check if there is a second last node
            if let Some(second_last) = current.as_mut() {
                // Extract the data from the last node
                let data = node.next.unwrap().data;
                // Update the second last node to point to None
                second_last.next = None;
                // Return the extracted data
                data
            } else {
                // Return the data from the current node
                node.data
            }
        })
    }

    // Method for inserting a new node with data at the specified index
    pub fn insert_at_ith(&mut self, index: usize, data: T) {
        // Check if the index is 0
        if index == 0 {
            // Insert at the front of the list
            self.push_front(data);
        } else {
            // Find the node at the index - 1 position
            let mut current = &mut self.head;
            for _ in 0..index - 1 {
                current = &mut current.as_mut().unwrap().next;
            }
            // Create a new node with the given data
            let new_node = Some(Box::new(Node {
                data,
                next: current.as_mut().unwrap().next.take(),
            }));
            // Update the node at the index - 1 position to point to the new node
            current.as_mut().unwrap().next = new_node;
        }
    }

    // Delete at ith
    pub fn delete_at_ith(&mut self, index: usize) {
        // Check if the index is 0
        if index == 0 {
            // Delete at the front of the list
            self.pop_front();
        } else {
            // Find the node at the index - 1 position
            let mut current = &mut self.head;
            for _ in 0..index - 1 {
                current = &mut current.as_mut().unwrap().next;
            }
            // Delete the node at the index position
            current.as_mut().unwrap().next = current.as_mut().unwrap().next.take().unwrap().next;
        }
    }

    // Get at ith
    pub fn get(&self, index: usize) -> Option<&T> {
        // Find the node at the index position
        let mut current = &self.head;
        for _ in 0..index {
            current = &current.as_ref()?.next;
        }
        // Return the data of the node at the index position
        current.as_ref().map(|node| &node.data)
    }

    // Method for displaying the list
    pub fn display(&self) {
        // Start from the head of the list
        let mut current = &self.head;
        // Iterate over each node in the list
        while let Some(node) = current {
            // Print the data of the current node
            print!("{:?} -> ", node.data);
            // Move to the next node
            current = &node.next;
        }
        // Print None at the end of the list
        println!("None");
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_insert_at_ith() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.insert_at_ith(1, 4);
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&4));
        assert_eq!(list.get(2), Some(&2));
        assert_eq!(list.get(3), Some(&3));
    }

    #[test]
    fn test_delete_at_ith() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.delete_at_ith(1);
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&3));
        assert_eq!(list.get(2), None);
    }

    #[test]
    fn test_get() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), None);
    }

    #[test]
    fn test_display() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Redirect stdout to capture the output
        let mut output: Vec<i32> = Vec::new();

        list.display();

    }
}
//
