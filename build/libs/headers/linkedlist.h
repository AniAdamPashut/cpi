#ifndef LINKED_LIST
#define LINKED_LIST

typedef struct Node
{
    void *data;
    struct Node *next;
} Node;

Node *node(void *data, Node *next);

void free_node(Node *head);

#endif
