#include <stdio.h>

#include "foo.h"
#include "linkedlist.h"

void print_linked_list(Node *head) {
    Node *runner = head;
    while (runner != NULL) {
        printf("(data=%p) -> ", runner->data);
        runner = runner->next;
    }
    printf("NULL\n");
}