CSE5402 Fall 2025 - Lab 2
Name: Daniel Palmer
Email: d.m.palmer@wustl.edu


Fixing Lab1:

The first thing that I did was address all points of failure from my original lab1 implementation. This involved removing the appended path and making it such that the program would succeed when the config file was badly formed.

The best demonstration of this testing was running "diff <(../lab2 hamlet_ii_2_rough_config.txt) <(../lab2 hamlet_ii_2_config.txt)" which resulted in nothing being printed, indicating that output is the same for both programs which is exactly desired behavior. 


Structs:

The approach that I took to refactor my code into structs and their implementations is very straightforward following the instructions for the assignment. I first created the Player struct and its implementation which was very straightforward from the script_gen functions in Lab1. I then created the Play struct and its implementation which was mostly also straightforward. The recite function took more effort to design, particularly identifying efficient ways to skip (but whinge about) missing lines and to get all of the characters which had the expected next line. I decided to use a nested while loop to skip over missing numbers so that if consecutive numbers are missing they would be skipped without recalculating the same minimum line number which runs until the expected next line number has caught up with the minimum line number. I also ran into difficulty actually retrieving the minimum line number over all of the players, particularly running into nested Option types. After research on rustdoclang and using ChatGPT I found filter_map which works very well with min to return Some(min) if it exists and None otherwise. I learned that if you use just map, None is always less than Some so the min doesn't work as intended once one of the Players has finished speaking. Then, I refactored the main function to create a Play struct and use it's methods to generate the script. Finally, I got rid of the script_gen.rs file and its module, putting the only remaining util function grab_trimmed_file_lines (which both the Play and Player struct implementations used) and put it into declarations.

I then tested my program using Lab1 testing files to confirm that output is the exact same and that I just had an implementation for Lab1 which uses structs and implementations.


