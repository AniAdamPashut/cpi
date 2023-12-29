#include <stdio.h>

#include "linkedlist.h"

int main()
{
    Node *lst = node(0, NULL);
    printf("%p\n", lst);
    free_node(lst);
    printf("done!\n");
    return 0; 
}