CSE5402 Fall 2025 - Lab 2
Name: Daniel Palmer
Email: d.m.palmer@wustl.edu


Fixing Lab1:

The first thing that I did was address all points of failure from my original lab1 implementation. This involved removing the appended path and making it such that the program would succeed when the config file was badly formed.

The best demonstration of this testing was running "diff <(../lab2 hamlet_ii_2_rough_config.txt) <(../lab2 hamlet_ii_2_config.txt)" which resulted in nothing being printed, indicating that output is the same for both programs which is exactly desired behavior. 


Structs:

The approach that I took to refactor my code into structs and their implementations is very straightforward following the instructions for the assignment. I first created the Player struct and its implementation which was very straightforward from the script_gen functions in Lab1. I then created the Play struct and its implementation which was mostly also straightforward. The recite function took more effort to design, particularly identifying efficient ways to skip (but whinge about) missing lines and to get all of the characters which had the expected next line. I decided to use a nested while loop to skip over missing numbers so that if consecutive numbers are missing they would be skipped without recalculating the same minimum line number which runs until the expected next line number has caught up with the minimum line number. I also ran into difficulty actually retrieving the minimum line number over all of the players, particularly running into nested Option types. After research on rustdoclang and using ChatGPT I found filter_map which works very well with min to return Some(min) if it exists and None otherwise. I learned that if you use just map, None is always less than Some so the min doesn't work as intended once one of the Players has finished speaking. Then, I refactored the main function to create a Play struct and use it's methods to generate the script. Finally, I got rid of the script_gen.rs file and its module, putting the only remaining util function grab_trimmed_file_lines (which both the Play and Player struct implementations used) and put it into declarations.

I then tested my program using Lab1 testing files to confirm that output is the exact same and that I just had an implementation for Lab1 which uses structs and implementations.


Return Wrapper:

My implementation of the Return Wrapper is incredibly simple. I started by just following the instructions which declared the struct and the termination implementation for it. The only design choice I made in was in the new function declaring the struct which I had take a single parameter of Result type and I match on the Ok and Err cases to create a struct with 0 as success for an Ok and a struct with the error value in the case of error.

I then tested my implementation by running it in ways that would generate different return values. This included a successful run, a run with an incorrect command line, and a run with files that would not open. Each case succeeded. On success no error code was printed and echo $? gave 0. On command line fail the error code 1 was printed and echo $? also gave 1. Finally, on file open fail the error code 2 was printed and echo $? gave 2. These are all intended behavior. 
