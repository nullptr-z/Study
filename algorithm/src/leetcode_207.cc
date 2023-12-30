#include <stdio.h>
#include <algorithm>

class Solution
{
public:
  int getLength(ListNode *head)
  {
    int length = 0;
    while (head)
    {
      ++length;
      head = head->next;
    }
    return length;
  }

  ListNode *getIntersectionNode(ListNode *headA, ListNode *headB)
  {
    ListNode *dummy1 = headA;
    ListNode *dummy2 = headB;
    long len_a = getLength(headA), len_b = getLength(headB);

    long len = std::min(len_a, len_b);
    long diff = std::abs(len_a - len_b);
    // printf("%d --- %d --- %d --- %d\n", len_a, len_b, len, diff);

    for (size_t i = 0; i < diff; i++)
    {
      if (len_a > len_b)
      {
        dummy1 = dummy1->next;
      }
      if (len_a < len_b)
      {
        dummy2 = dummy2->next;
      }
    }

    ListNode *eq_val = NULL;
    for (size_t i = 0; i < len; i++)
    {
      // printf("eq : %d  %d\n", dummy1->val, dummy2->val);
      if (&dummy1->val == &dummy2->val)
      {
        if (eq_val == NULL)
        {
          eq_val = dummy1;
        }
      }
      else
      {
        eq_val = NULL;
      }
      dummy1 = dummy1->next;
      dummy2 = dummy2->next;
    }

    return eq_val;
  }
};

struct ListNode
{
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};
