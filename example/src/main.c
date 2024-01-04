#include <stdlib.h>

#include "foo.h"
#include "linkedlist.h"

int main() {
    int *data = malloc(sizeof(int));
    *data = 3;
    Node *list = node(data, NULL);
    print_linked_list(list);
    return 0;
}