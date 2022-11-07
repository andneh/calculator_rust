#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char* argv[]) {
  printf("Enter q to quit.\n");
  float valueOne, valueTwo, answer;
  char operator= ' ';
  while (1 == 1) {
  reset:
    printf("Enter problem: ");
    if (scanf("%f %c %f", &valueOne, &operator, & valueTwo) ==
        3)  // Check if exactly one parameter was read.
    {
      switch (operator) {
        case '/':
          answer = valueOne / valueTwo;
          break;
        case '*':
          answer = valueOne * valueTwo;
          break;
        case '+':
          answer = valueOne + valueTwo;
          break;
        case '-':
          answer = valueOne - valueTwo;
          break;
        case '^':
          answer = pow(valueOne, valueTwo);
          break;
        default:
          printf("Try again.\n");
          goto reset;
          break;
      }
      printf("%.9g %c %.9g =  %.6g\n", valueOne, operator, valueTwo, answer);
    } else if (scanf("%c", &operator) == 1) {
      if (operator== 'q') {
        printf("Bye.\n");
        return 0;
      } else {
        printf("Try again.\n");
        goto reset;
      }
    }
  }
}