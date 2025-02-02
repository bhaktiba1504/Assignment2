public class MemoryExample {
    public static void main(String[] args) {
        int[] num1 = new int[]{1, 2, 3, 4, 5}; // Heap allocation
        System.out.println("Sum: " + sumNumbers(num1));
    }

    public static int sumNumbers(int[] arr) {
        int sum = 0;
        for (int num2 : arr) {
            sum += num2;
        }
        return sum;
    }
}
