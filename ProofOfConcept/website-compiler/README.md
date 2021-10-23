# Website Compiler
## About this Repo 
This repository provides a proof of concept for a new method of censorship circumvention. The main algorithm prepares a mapping of corresponding sequences based on a _**candidate website**_ and a _**helper website**_. The candidate website is the one that is assumed to be censored, and the helper website is a website which is not assumed to be not censored in a given region.

The main algorithm compares sequences of bytes and prepares a mapping that is provided as an HTML comment on the helper website. This mapping is then used to reconstruct the candidate website without needing to connect to the servers of the candidat website directly (given that it would be censored).

This is the first iteration of this typeof system, and this proof of concept seeks to be a very basic model of what this could look like and is very likely not intended to be a final model of how this type of system should work.

## Requirements
* Make sure you have the [Rust Programming Language](https://www.rust-lang.org/tools/install) installed on your system.
* Also ensure you have [Cargo](https://github.com/rust-lang/cargo) installed on your system.

## How To Guide
* In ```/website-compiler/src/main.rs``` you can adjust the ```candidate_website_url``` and ```helper_website_url variables``` to denote what candidate website you would like to compile based on the helper website provided.
* Once those variables are set, you can type ```cargo build``` in ```/website-compiler/```
* Now you can run ```./target/debug/website-compiler```. Note, this will take a considerable amount of time depending on the websites you've selected. Current design of the algorithm is O(k*n^2), k being the provided sequence length. Work is being done now to increase the speed. This is the first iteration of this type of system.
* After the algorithm is complete, you can open ```/website_store/recompiled_website.com.html``` and have a look at what the main compilation algorithm compiled.

## FAQ
* How come my recompiled website doesn't match the content at the candidate website url provided?
  * The algorithm provides the best reconstruction possible, based on the characters provided on the page. A perfect reconstruction isn't always possible based on the two URLs. Several attempts are made to mitigate this, most importantly reducing the sequence length to 1 if sequences of characters can't be provided of larger sequence length (the starting sequence length can be adjusted in ```src/main.rs``` under the variable ```sequence length```). 
    * Further work can be done on this algorithm to allow for multiple helper websites to be used in constructing a given website. This will increase the probability that all characters can be found and a proper reconstruction can be completed.
  * Some websites also host styling content locally, such as images. Such content is not pulled at this point during the compilation process. 
