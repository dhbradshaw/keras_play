## 2016-05-07

Finished going through 

http://neuralnetworksanddeeplearning.com

Starting to work through

http://www.deeplearningbook.org/

## 2016-04-11
Completed ch 1 of 

http://neuralnetworksanddeeplearning.com


## 2016-04-07

Today was something of a red letter day; I found two resources to let me catch up on the big picture in deep learning.  I'm switching all activities to going through them for now:

http://neuralnetworksanddeeplearning.com
http://www.deeplearningbook.org/

## 2016-03-28

Read the third and fourth portions of the sequence.  It was awesome.

## 2016-03-26
Read the second in the sequence

## 2016-03-25
Read the first in a sequence of three posts for a nice conceptual overview:

http://www.andreykurenkov.com/writing/a-brief-history-of-neural-nets-and-deep-learning/

## 2016-03-24
Implemented my first struct and method today and got lost in the world of abstract types and lifetime borrowing.

## 2016-03-22
Wow, I can't believe it's been 10 days.

I think that I'll pause for awhile on this problem.  I'm using it as a chance to get comfortable with structs and traits.

## 2016-03-12
It took some time to learn how to unpack a table of numbers.  Here's what I ended up doing:

```
fn p11_grid() -> Vec<Vec<u64>> {
    let s="08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48".lines();
s.map(
    |line|
    line.split(' ').map(
        |n|
        n.parse::<u64>().unwrap()
    ).collect::<Vec<u64>>()
).collect::<Vec<Vec<u64>>>()
}
```

## 2016-03-11

The sieve was about 8-fold faster, but it's still slow.  Anyway, it gave me some practice with mutable and immutable referrences during itertion.

## 2016-03-10
Problem 10 

This problem was nice and simple to solve, but my solution was slow--about a second to sum primes under 2,000,000.  Tomorrow I think I'll come back to it to implement it using a sieve.

Hmm.  Or maybe the rustacious thing to do would be to implement a primes iterator.

## 2016-03-10

Problem 9.  Not much to learn here--it was a pretty simple problem.  But it was an excuse to use bare loops and break for the first time.

## 2016-03-09

Problem 8.  The main trick today was converting a string into a sequence of digits.  The solution ended up being this:

First, the &str can be converted into an iterable of chars.  (Direct indexing of the string looks at bytes rather than chars.)

If s is your string literal (&str), 
```
    for (j, ch) in s.chars().enumerate() {
        v[j % size] = d(ch);
        max = cmp::max(v.iter().fold(1, |product, n| product * n), max);
        println!("{}", d(ch))
    }
```
Note the fold for the product is similar to the fold for a sum that I used early since the sum method is in flux.

The actual conversion of a char into a digit was done using the following:
```
fn d(ch: char) -> u64 {
    ch.to_digit(10).unwrap() as u64
}
```

## 2016-03-08

Problem 7 again.  

I added a simple optimization to problem 7 (don't check for factors beyond the square root of the candidate) and found myself unexpectedly fighting the compiler.  

At first I thought the main problem was converting between u64 and f64.  But it turned out to be mostly the fact that when you use the "for prime in &primes" contruction the "prime" is actually a reference.  You can't compare it directly with, for example, a u64.

## 2016-03-07

Problem 7: Vectors!  To the the length of a vector: v.len().  To get its last value: v[v.len()-1] (or v.pop()).
When you iterate through a vector, you iterate through an immutable reference.  So,

```
for prime in &primes {
            if candidate % prime == 0 {
                is_prime = false;
                break
            }
        }
```

## 2016-03-05

Problem 6: to exponentiate, you can't use ^ or **.  Instead, you use 6.pow(2) to get 36.

## 2016-03-04

Not much time this day and problem 5 was so easy that I did it almost by hand and didn't learn much. 

## 2016-03-03

Fourth Euler problem.  Learned about converting to strings.  Starting to like this.

## 2016-03-02

Third Euler problem.  Read up on iterators and their modifiers and consumers.  There's still a lot more to learn there.

## 2016-03-01

Second Euler problem.  Also implemented testing for the first time in rust.

## 2016-02-29

Started working through Euler problems in rust to build up comfort and fluency.  Today I only worked through the first one.  Got stuck because a feature I was trying to use is unstable so I replaced it with a more stable one.

## 2016-02-27

Once again, just read code.

## 2016-02-26

Spent the whole 20 minutes reading leaf docs and code.  I'll probably continue like that for some time since it's a new project and since rust is a new language for me.

## 2016-02-25

I've decided to go about things more directly by focusing on reading the code for leaf.  Today I finished setting up Racer and downloaded Leaf and read through some of the introductory documentation.

## 2016-02-24

Another day (on this project that means 20 minutes) lost to setting up a text editor.  Compiled Racer and the Atom Racer plugin on the new atom setup.

## 2016-02-23

The good: Got a new SSD hard drive
The bad: reinstalling everything today, including rust
The great: rust installation is fast.  So I was able to complete the list of four base combos in the match statement.

## 2016-02-22

The main thing that I learned: don't use single quote marks for string literals.
The other thing: match can use string literals.  If you don't use single quotes.

## 2016-02-20

Got a function to read in a string.  And git can push at least when I'm home.

## 2016-02-19

Got git working.  And modules are working.  Can't push from where I am, but that will come.

## 2016-02-18

Started working within the module structure.  Simple bugs slow me down because I haven't learned to interpret the racer error messages rapidly yet.

## 2016-02-17

Got racer working with Atom.  Had to install the binary using cargo install racer.  Then I had to point the racer Atom plugin to the racer binary and the rust source code in Atom: settings packages racer

## 2016-02-16

Spent the 20 minutes just configuring Atom for rust.  Stuck configuring Racer, but have syntax highlighting, which is something.

## 2016-02-15

I got a working rust module system by finding one rather than working through the tutorial.  Here you go:

https://github.com/steveklabnik/phrases

## 2016-02-13
Added a library that needs to be configured.

## 2016-02-12
Here is the most important concept in rust: linking the functionality of crates and modules together.  So today I read through most of the nice tutorial on that subject:

https://doc.rust-lang.org/book/crates-and-modules.html

## 2016-02-11
Read up on mutability

## 2016-02-10
Read more on lifetimes.  I'm still not sure how to make use of explicit lifetimes.

## 2016-02-09
Spent a tiny bit of time learning about lifetimes syntax.  The syntax is simple enough.  Knowing when you need explicit lifetimes is beyond me right now.

## 2016-02-08
Read section on borrowing

## 2016-02-06
Reviewed ownership and got ready to learn about borrowing.

## 2016-02-05

Just made it through the sections on ownership and on the stack and the heap.

## 2016-02-04
Now here:

https://doc.rust-lang.org/book/ownership.html

## 2016-02-03
Now here:

https://doc.rust-lang.org/book/primitive-types.html


## 2016-02-02
Reviewed dining philosophers, read the section on embedding (which has a nice second reference to the thread spawning idiom below) and moved on to the syntax section.  Just got a first taste of the use of patterns in let statements.

## 2016-02-01
I'll probably study dining philosophers for one more 30m session.  

The key to the code below is closures.  In brief, handles is a vector of elements which all share a type that Rust has to figure out (so it's marked with the underscore).  That vector is created from the philosophers vector.  First the vector is turned into an iterable.  Map is called on the iterable, and map takes a closure.  The closure gets the variable p and in it a thread is spawned.  The thread itself takes a closure, only this one takes ownership of the philosopher (using the move keyword).  The philosopher's method 'eat' is called inside the thread.  Collect grabs the output of the iterable, and needs some kind of collection to put it in.  Here it sees that we want a vector, so that's the collection type that it delivers.



## 2016-01-30
Working through dining philosophers.  I don't understand this bit of code yet:

```
let handles: Vec<_> = philosophers.into_iter().map(|p| {
    thread::spawn(move || {
        p.eat();
    })
}).collect();
```

## 2016-01-29
Completed guessing game tutorial.  Conclusions so far:

1. Wow, this is a very different world.
2. I like it.
3. I really need tab completion.  Every new method and class property comes as a big surprise.

https://github.com/phildawes/racer

## 2016-01-28
Still on the guessing game tutorial.  There is a lot to learn here.

## 2016-01-27
Still on the guessing game tutorial.  But I got rust syntax highlighting and running rust code set up on the atom text editor.
(for the latter, check out

https://atom.io/packages/script
)
## 2016-01-26
To work on leaf, I guess I need to become conversant with rust.  

Here's my current place in the rust book:

https://doc.rust-lang.org/book/guessing-game.html

## 2016-01-25
There's an interesting project centered around machine learning using rust:

https://github.com/autumnai/leaf

## 2016-01-22
I think it's appropriate to be a little flexible on your exact plans in the beginning of an endeavor.  Just like the multiplier to the input of a softmax function can be small in the beginning and larger later, it's worth noting that you are uncertain at the beginning.  Anyway, I just found a free deep learning cource from Udacity at

https://udacity.com/course/deep-learning--ud730

For that I got scipy going on my local computer using

```
$ sudo apt-get install python-numpy python-scipy python-matplotlib ipython ipython-notebook python-pandas python-sympy python-nose
```
as described here:

http://www.scipy.org/install.html

Here's one more interesting link:

https://www.cs.ox.ac.uk/people/nando.defreitas/machinelearning/

Okay, one more.  Apparently TensorFlow has a nice general introduction:

https://www.tensorflow.org/versions/master/tutorials/

## 2016-01-21
It looks like I need to back way up before I can do much that's interesting.  So what I'm going to do is do some reading.  I'll keep a log here just to stay accountable.  The reading list is first the following two links:

http://deeplearning.net/software/theano/tutorial/

http://deeplearning.net/tutorial/

For tonight I'm just starting to get things installed.  I used the commands
```
$ python3 -m venv v3
$ . v3/bin/activate
```
Then I tried pip install theano, but it got hairy from there. I think I'll need to install a bunch of C stuff through apt before a successful install.
