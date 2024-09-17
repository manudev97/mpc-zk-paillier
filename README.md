# Introduction to MPC and TSS

Secure multi-party computation (SMPC/MPC) allows multiple participants to collaboratively compute a pre-agreed function without the need for a trusted third party. This concept originated from Professor Andrew Yao's millionaire problem and its solution using cryptography in 1982. The scheme, which is an interaction between two people, could find out who was richer without revealing their actual wealth. It allows users to collaborate on calculations with each other without revealing any sensitive information. MPC has since evolved into an important branch of modern cryptography.

# Ejemplo Motivador

Imagine that a group of 3 researchers is working on a joint project, and each has developed an innovative algorithm with different results. The researchers want to collaborate to find out which of the algorithms is the most efficient, i.e. which has the shortest runtime. However, each researcher considers the details of their own work confidential and does not want to reveal their exact results to the others. To solve this, they need to determine which algorithm is the fastest without exposing each other's runtimes.

<div style="text-align: center;">
    <img src="assets/researchers.jpeg" alt="researchers" width="400" height="250"/>
</div>

The problem can be represented by the function:

$$F(t_1, t_2, t_3) = min(t_1, t_2, t_3)$$

​where $t_1, t_2, t_3$ are the execution times of each of the researchers algorithms.

A simple solution would be for everyone to reveal their execution times to a third party who could calculate the minimum value. However, since they don't trust any external entity to handle this sensitive data, they need a way to find the result without revealing their times to anyone else.

Using a multi-party computation (MPC) protocol, researchers could collaborate to determine which algorithm is the fastest. The protocol would ensure that everyone contributes their execution time to the function, but without anyone being able to infer the others' times beyond knowing which one was the fastest. This way, they can compare their algorithms without compromising the confidentiality of their research.