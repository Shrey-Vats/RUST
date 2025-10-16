use std::collections::{HashMap, LinkedList, VecDeque};

fn main() {
    // VecDeque();
    // linklist();
    fruits();
}

fn iter_throgh_even(){
    let vector: Vec<u32> = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    for x in vector.iter() {
        if x % 2 == 0{
            println!("{}", x);
        }
    }
}

fn square_elements(vec: &mut Vec<u32>) {
    for num in vec.iter_mut() {
        *num *= 2
    };
    println!("{:?}", vec);
}

fn todo_1(number: &mut Vec<u32>){
    // TODO: append 11
    // TODO: remove 5th element
    // TODO: reverse using iterator

    number.push(11);
    println!("after adding 11 {:?}", number);
 
    number.remove(4);
    println!("after removing 5th number, {:?}", number);

    number.reverse();
    println!("after reversing number, {:?}", number);

}

fn VecDeque(){
    let mut deq: VecDeque<u32> = VecDeque::new();

    //TODO: add 1...5 from back
    deq.push_back(1);
    deq.push_back(2);
    deq.push_back(3);
    deq.push_back(4);
    deq.push_back(5);
    println!("After adding 5 num from back, {:?}", deq);

    deq.push_front(0);
    println!("After adding from frontend, {:?}", deq);

    deq.pop_back();
    deq.pop_front();

    println!("After removing from front and back, {:?}", deq)
}

fn linklist(){
    let mut list: LinkedList<String> = LinkedList::new();
    list.push_back("a".to_string());
    list.push_back("b".to_string());
    list.push_back("c".to_string());

    println!("Initial List, {:?}", list);

    list.push_back("d".to_string());
    println!("After appending d, {:?}",list);

    for x in &list {
        println!("{}", x)
    }

    let mut list2: LinkedList<String> = LinkedList::new();

    list.append(&mut list2);
    println!("After appending another list, {:?}", list);
}

fn fruits(){
    let mut fruits: HashMap<String, u32> = HashMap::new();

    // TODO: insert apple and banana
    // TODO: update apple using entry
    // TODO: print all key-value pairs

    fruits.insert("apple".to_string(), 5);
    fruits.insert("banana".to_string(), 8);
    fruits.insert("mango".to_string(), 4);
    println!("after adding values,{:?}", fruits);

    fruits.entry("apple".to_string()).and_modify(|v| *v +=2);
    println!("after increasing apple, {:?}", fruits);

    for (key, pair) in  &fruits {
        println!("{:?} -> {:?}", key, pair)
    };

    fruits.remove("banana");
    println!("After removing banana, {:?}", fruits);

    let is_banana = fruits.get("banana");
    println!("is banana exits, {:?}", is_banana);
}
