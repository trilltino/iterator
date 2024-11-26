// Crust of Rust Iterators

// Iterator is a trait that is implemented by types that can be iterated over.
// the reason an iterator is used is to iterate over a collection of items, one at a time, in a safe and efficient manner. 
// Types that implement the Iterator trait must define an associated Item type

//pub trait Iterator {
//    type Item; 
//    }

// Item Represents the type of the values that the iterator produces. it can be any type. such as a string, integer, etc.
// Item can be specific to the data you need to manipulate for the program



pub trait IteratorExt: Iterator + Sized {
    fn our_flatten(self) -> Flatten<Self>
     where 
        Self::Item: IntoIterator;
}

// Iteratorext allows for the use of different iterator methods.
// our_flatten is a method that is used to flatten nested iterators.
// this means that it will take an iterator of iterators and flatten it into a single iterator.
// this is useful whe you have a collection of collections and you want to iterate over all the elements in all the collections.
// such as a vector of vectors, or a vector of strings.

impl <T> IteratorExt for T
where
T: Iterator,
    {
        fn our_flatten(self) -> Flatten<Self>
    where
    Self::Item: IntoIterator,
    {
    flatten(self)
    } 
}

// Here Impl<T>  means that the Trait ext we defined earlier will be implemented for any type T that implements the Iterator trait.
// the where clause specifies that the Item type of the iterator must implement the IntoIterator trait.
// this is beacase the flatten function requires the type to be an iterator itself
// The method is then defined, our flatten take ownership of self and returns a Flatten iterator.
// the next where constraint ensures that the Item produced by the Iteratots can be turned into an iterator itself.
//the flatten(self) is a method body. this means that the method is implemented by calling the flatten function with the iterator as an argument.
// the iterator is self, which is the iterator that the method is called on.

    pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
        where I: IntoIterator,
        I::Item: IntoIterator,  {
    Flatten::new(iter.into_iter())
        }

// the flatten function takes an iterator as an argument and returns a Flatten iterator. It is generic over the type I, which is the type of the iterator, it being generic means that it can take any type that implements the IntoIterator trait.
// the where clause has two constraints: Type I must implement the IntoIterator trait, and the Item type of I must also implement the IntoIterator trait.
// this is because the flatten function requires the type to be an iterator itself.
//so depending on the iterator you call the item type of the iterator may change
// the functions body callls on Flatten::new, this is a method that is implemented on the Flatten struct.
// The Flatten type takes an iterator of iterators and produces a new iterator that yields all the items from each of the inner iterators in turn.
// new:: means that the method is an associated function of the Flatten struct, and it is called on the type itself, not on an instance of the type. so it is owned by the type itself. not borrowed from an instance of the type.
// type borrowing happens when you call a method on an instance of a type, and the method takes a reference to the instance as an argument.
// this function flattens the iterator by calling the into_iter method on the iterator, which converts the iterator into an iterator that can be consumed.

pub struct Flatten<O>
where 
O: Iterator,
O::Item: IntoIterator,

{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>:: IntoIter>,
}

// this code creates a struct named Flatten that is generic over the type O, which is the type of the outer iterator.
// the outer iterator is the iterator that contains the inner iterators.
// the struct has two fields: outer, which is the outer iterator, and front_iter and back_iter, which are the inner iterators.
// there are three constraints on the type parameters of the struct:
// O must implement the Iterator trait, O::Item must implement the IntoIterator trait, and the Item type of O must also implement the IntoIterator trait.
// back and front iter are options which means that they can be either Some or None, in regard to iterators it means that they can be either an iterator or None.
// the fields in total allow for the struct to keep track of the outer iterator and the inner iterators that are being flattened.

impl<O> Flatten<O>
where

O: Iterator,
O::Item: IntoIterator, {
fn new(iter:O) -> Self {
Flatten {
    outer: iter,
    front_iter: None,
    back_iter: None,
   }
  }
}

// This code creates and implementations for the Flatten struct. This flattens and iterator of iterators into a single iterator 
// This is generic over the type O which differs from the type I in the flatten function.
// type O means that the struct is generic over the type of the outer iterator.
// whilst Type I in the flatten function is the type of the iterator that is being flattened.
// as the flatten function requires all types to implement the IntoIterator trait, the struct also has the same constraints. thus type O must implement the Iterator trait, and O::Item must implement the IntoIterator trait.
// O::Item must also implement the into iterator trait.
// when fn new is called it is used to create a new instance of the Flatten struct.
// the function takes an iterator as an argument and returns a new instance of the struct. in this case it only takes in the outer iterator. which was given type O. in the struct impl.


impl<O> Iterator for Flatten<O>
where
O: Iterator,
O::Item: IntoIterator

{
type Item = <O::Item as IntoIterator>::Item;
fn next(&mut self) -> Option<Self::Item> {
loop {
    if let Some(front_iter) = &mut self.front_iter {
        if let Some(i) = front_iter.next() {
            return Some(i);
        }
        self.front_iter = None;
        }

        if let Some(next_inner) = self.outer.next() {
            self.front_iter = Some(next_inner.into_iter());
        } else {
            return self.back_iter.as_mut()?.next();
        }
        }
    }
}

// This code implements the Iterator trait for the Flatten struct. This allows the struct to be used as an iterator.
// this impl block has two constraints: O must implement the Iterator trait, and O::Item must implement the IntoIterator trait.
// the type associated type Item is defined as the Item type of the inner iterator. which means that the Flatten iterator will yield the items of the inner iterators.
// the next method is implemented for the Flatten struct. this method is called to get the next item from the iterator.
// the method returns an Option<Self::Item> which is the item type of the iterator.
// the method is implemented using a loop. this is because the method needs to keep track of the inner iterators and the outer iterator.
// the loop has two branches: one for the front iterator and one for the back iterator.
// the first branch checks if the front iterator is Some, which means that there is an inner iterator that is being flattened. if none it moves to the next branch.
// the next branch checks if there is a next inner iterator in the outer iterator. if there is it sets the front iterator to the next inner iterator.
// this method checks the front, then the back iterator, and then the outer iterator. if there are no more items in the outer iterator, the method returns None.
// this trait allows for the use of the flatten method on iterators. this method is used to flatten nested iterators into a single iterator.

impl <O> DoubleEndedIterator for Flatten<O>
where
O: DoubleEndedIterator,
O::Item: IntoIterator,
<O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,

{
fn next_back(&mut self) -> Option<Self:: Item> {
    loop {
        if let Some(back_iter) = &mut self.back_iter {
            if let Some(i) = back_iter.next_back() {
                return Some(i);
            }
            self.back_iter = None;
            }

            if let Some(next_back_inner) = self.outer.next_back() {
                self.back_iter = Some(next_back_inner.into_iter());
            }else{
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

#[test]
fn empty () {
    assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
}
// this test checks that the flatten function works with an empty iterator.
// the type of the iterator is std::iter::empty::<Vec<()>>(), which is an empty vector of empty vectors.
// remember flatten works on iterators of iterators, so the type of the iterator is a vector of vectors.

}

#[test]
fn empty_wide(){
assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
}
// this test checks that the flatten function works with a wide iterator.
// a wide iterator is an iterator that has multiple inner iterators.
// a real life example of a wide iterator is a vector of vectors.

#[test]
fn one () {
    assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
}
// this test checks that the flatten function works with an iterator that has one inner iterator.
// this was implemented by calling the std::iter::once function with a vector of one string as an argument.
// a real life example of this is a vector of strings.



#[test]
fn two () {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
}
// this test checks that the flatten function works with an iterator that has two inner iterators.


#[test]
fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }
    // this test checks that the flatten function works with a wide iterator that has two inner iterators.

#[test]
fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
            .rev()
            .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    // this test checks that the flatten function works with an iterator that has two inner iterators. 
    // reverse is called on the iterator to reverse the order of the items.

    
#[test]
fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
            .rev()
            .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }
    // this is similar to above but with a wide iterator.
    // the wided iterator is a vector of vectors. and wide means that there are multiple inner iterators.
    // this is the reversal of the order of the items in the iterator.

#[test]
fn both_ends () {
    let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
    assert_eq!(iter.next(), Some("a1"));
    assert_eq!(iter.next_back(), Some("b3"));
    assert_eq!(iter.next(), Some("a2"));
    assert_eq!(iter.next_back(), Some("b2"));
    assert_eq!(iter.next(), Some("a3"));
    assert_eq!(iter.next_back(), Some("b1"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}

// the both_ends test checks if the flatten function works with an iterator that has multiple inner iterators.
// it calls next and next_back on the iterator to get the items from the front and back of the iterator.
// the item type of the iterator is a string, so the items are strings. 

#[test]
fn inf() {
    let mut iter = flatten((0..).map(|i|0..i));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
}
 // this test checks that the flatten function works with an infinite iterator.
 // an infinite iterator is an iterator that never ends. it goes on forever.
 // a real life example of an infinite iterator is the range function in rust. it creates an iterator that generates a sequence of numbers.
 // assert_eq! is used to check that the items from the iterator are correct. 
 // some(1) means that the iterator has an item of 1. and some(0) means that the iterator has an item of 0.
 // therefore the test checks that the items from the iterator are correct.


#[test]
fn deep() {
    assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
}

// this test checks that the flatten function works with a deep iterator.



#[test]
fn ext() {
    assert_eq!(vec![vec![0, 1]].into_iter().our_flatten().count(), 2);
}
