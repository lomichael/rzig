#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "%s: invalid number of arguments\n", argv[0]);
    return 1;
  }

  printf(".intel_syntax noprefix\n"); // asm to use intel instead of att syntax
  printf(".global main\n"); // defines a global label main
  printf("main:\n"); 
  printf("  mov rax, %d\n", atoi(argv[1])); // setting the arg in the return value register (rax)
  printf("  ret\n");
  return 0;
}
