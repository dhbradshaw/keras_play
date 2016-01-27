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
