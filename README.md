# Introduction to MPC and TSS

Secure multi-party computation (SMPC/MPC) allows multiple participants to collaboratively compute a pre-agreed function without the need for a trusted third party. This concept originated from Professor Andrew Yao's millionaire problem and its solution using cryptography in 1982. The scheme, which is an interaction between two people, could find out who was richer without revealing their actual wealth. It allows users to collaborate on calculations with each other without revealing any sensitive information. MPC has since evolved into an important branch of modern cryptography.

# Motivating Example

Imagine that a group of 3 researchers is working on a joint project, and each has developed an innovative algorithm with different results. The researchers want to collaborate to find out which of the algorithms is the most efficient, i.e. which has the shortest runtime. However, each researcher considers the details of their own work confidential and does not want to reveal their exact results to the others. To solve this, they need to determine which algorithm is the fastest without exposing each other's runtimes.

<div style="text-align: center;">
    <img src="assets/researchers.jpeg" alt="researchers" width="400" height="250"/>
</div>

The problem can be represented by the function:

$$F(t_1, t_2, t_3) = min(t_1, t_2, t_3)$$

​where $t_1, t_2, t_3$ are the execution times of each of the researchers algorithms.

A simple solution would be for everyone to reveal their execution times to a third party who could calculate the minimum value. However, since they don't trust any external entity to handle this sensitive data, they need a way to find the result without revealing their times to anyone else.

Using a multi-party computation (MPC) protocol, researchers could collaborate to determine which algorithm is the fastest. The protocol would ensure that everyone contributes their execution time to the function, but without anyone being able to infer the others' times beyond knowing which one was the fastest. This way, they can compare their algorithms without compromising the confidentiality of their research.

The two main properties of MPC are correctness and privacy:
- Correctness: The output produced by an algorithm is correct (as expected).
- Privacy: The secret input data that one party possesses will not be leaked to the other parties.

## MPC in Blockchain

To improve the security of digital assets on the blockchain, MPC has been introduced in the multi-signature scenario. Multiple key fragments are used to compute the final signature using MPC protocols during the signing process. This signature can be verified using the corresponding unique public key. This technique, known as MPC multi-signature, provides a highly secure and efficient way to secure digital assets on the blockchain.

# MPC and TSS

We will use MPC to compute a digital signature in a distributed manner. Let us see how the above properties can be applied to signatures. Recall that, for signatures, we have three steps:

- Key generation: The first step is also the most complex. We need to generate a key that will be public and used to verify future signatures. But we also need to generate an individual secret for each party, which we will call the shared secret. In terms of correctness and privacy, we say that the function will generate the same public key for all parties and a different shared secret for each of them, so that: (1) privacy: no data is leaked from the secrets shared between the parties, and (2) correctness: the public key is a function of the shared secrets.

- Signature: This step involves a signature generation function. The input of each party will be its secret part, created as an output of the previous step (distributed key generation). There is also a public input known to all, which is the message to be signed. The output will be a digital signature, and the privacy property ensures that no leaks of secret parts occur during the computation.

- Verification: The verification algorithm remains as in the classical setup. To support single-key signatures, everyone who knows the public key should be able to verify and validate the signatures. This is exactly what blockchain validation nodes do.

Threshold Signature Scheme (TSS) is the name we give to this composition of distributed key generation (DKG) and distributed signature of a threshold signature scheme.

## TSS in Blockchain

We can create a new address by generating a private key and then computing the public key from the private key. Finally, the blockchain address is derived from the public key. Using TSS, we would have a set of n parties jointly computing the public key, each of which owns a secret part of the private key (individual parts are not revealed to the other parties). From the public key, we can derive the address in the same way as in the traditional system, making the blockchain independent of how the address is generated. The advantage is that the private key is no longer a single point of failure because each party owns only a part of it.

Distributed key generation can be done in a way that allows for different types of access structures: the general “$t$ of $n$” configuration will be able to withstand up to $t$ arbitrary failures in operations involving the private key, without compromising security.

- $\{t - n\}$ means that the threshold is $t$ and the number of participants is $n$. At least $t$ participants are required to recover the private key and sign a message.
- $\{n - n\}$ means the threshold is $n$ and the number of participants is $n$.