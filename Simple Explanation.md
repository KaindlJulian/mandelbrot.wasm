# Pseudocode

Following code will determine whether a complex number is or is not inside the Mandelbrot Set.

__Variables__

`c`			...The complex number to be checked

`iterator`	...Result of the iteration, starts with 0	

```pseudocode
for every pixel on the canvas
    while iterator is less than or equal to 2
        iterator = iterator^2 + c
        increase iterationCount
    display pixel color according to the last iterationCount
    reset iterationCount
```





## Simple Explanation

The Mandelbrot set is the set of values of *c* in the complex plane for which the behavior of

$$
z=0
$$
under iteration of
$$
f_c(z)=z^2+c
$$
remains bounded (doesn't grow to infinity).



## Examples

For simplicity we will look at real numbers. Meaning the imaginary part is 0i.



### Example 1

(literally 1)
$$
z = 1
$$


We will plug that into our definition from above and do the first few iterations. (The result of the iteration gets plugged into the next one)
$$
f_1(0)=0²+1=1
$$
$$
f_1(1)=1²+1=2
$$
$$
f_1(2)=2²+1=5
$$
$$
f_1(5)=5²+1=26
$$
$$
f_1(26)=26²+1=677
$$



So for *z =* 1 the values get exponentially bigger. The complex number 
$$
c=1+0i
$$
is not in the Mandelbrot Set.



### Example 2

$$
z = -1
$$



$$
f_{-1}(0)=0²-1=-1
$$

$$
f_{-1}(-1)=(-1)²-1=0
$$

$$
f_{-1}(0)=0²-1=-1
$$

$$
f_{-1}(-1)=(-1)²-1=0
$$

$$
f_{-1}(0)=0²-1=-1
$$



For *z =* -1 the values stay bounded, meaning
$$
c=-1+0i
$$
is  in the Mandelbrot Set.



### Example 3

If the result z of an iteration is ever bigger than 2 the number c is not part of the Set and will get very big a few iterations after. Meaning we can stop after z > 2. 

In Example 1 z was bigger then 2 after 3 iterations. The closer the number is to the Set the more iterations it will take to go beyond 2. By counting the number of iterations points can be color coded accordingly, which is the method used to display the "halos".



When taking a number that is very close to the set, like
$$
z=0.26
$$
(The intersection of the set with the real axis is defined by [-2, 0.25])


$$
f_{0.26}(0)=0.26
$$
$$
f_{0.26}(0.26)=0.3276
$$
$$
f_{0.26}(0.3276)=0.3673
$$
$$
f_{0.26}(0.3673)=0.3949
$$



As you can see the values for z increase very little and it will take a lot of iterations for z to grow bigger than 2.