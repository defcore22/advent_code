#include <iostream>
#include <fstream>
#include <string>
#include <print>

// /* 
// * Read the list.
// * The first value is the the first piece of paper
// * and the second on is the second.
// * read all of them and sort them in order.
// * find the distance of every number
// * and find the total of it.
// *\

// const
const int kMaxSize {10};


// prototype
int FindSmall(int *arr);


int main() {
    std::ifstream infile {"list.adv"};
    
    int *arrR {nullptr};
    int *arrL {nullptr};

    arrR = new int[kMaxSize];
    arrL = new int[kMaxSize];

    do {
        int R {};
        int L {};

        infile >> R;
        ++arrR[R];

        infile >> L;
        ++arrL[L];
        
    } while(infile);

    int totalD {};

    for(int i {}; i < kMaxSize; ++i) {
        
        totalD += FindSmall(arrL) - FindSmall(arrR); // left to right
    }
    
    std::println("{}", totalD);

    delete[] arrR, arrL;

    infile.close();
    return 0;
}

int FindSmall(int *arr) {
    for(int i {}; i < kMaxSize; ++i) {
        if(arr[i] > 0) {
            --arr[i];
            return i;
        }
    }
    return 0;
}

