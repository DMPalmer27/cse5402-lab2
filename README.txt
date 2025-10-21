CSE5402 Lab 2


The first thing that I did was address all points of failure from my original lab1 implementation. This involved removing the appended path and making it such that the program would succeed when the config file was badly formed.

The best demonstration of this testing was running "diff <(../lab2 hamlet_ii_2_rough_config.txt) <(../lab2 hamlet_ii_2_config.txt)" which resulted in nothing being printed, indicating that output is the same for both programs which is exactly desired behavior. 
