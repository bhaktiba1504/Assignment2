#include <iostream>

int sumNumbers(int* arr, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += arr[i];
    }
    return sum;
}

int main() {
    int* num = new int[5]{10,20,30,40,50}; // Heap allocation
    std::cout << "Sum: " << sumNumbers(num, 5) << std::endl;
    
    delete[] num; // Manual deallocation (avoids memory leak)
    return 0;
}


