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
