#include <stdio.h>
#include <unordered_map>
#include <cstdint>

class Solution
{
public:
  ListNode *detectCycle(ListNode *head)
  {
    ListNode *fast = head;
    ListNode *slow = head;

    while (fast != NULL && fast->next != NULL)
    {
      fast = fast->next->next;
      slow = slow->next;

      if (fast == slow)
      {
        auto start = head;
        while (start != slow)
        {
          start = start->next;
          slow = slow->next;
        }

        return start;
      }
    }

    return NULL;
  }

  ListNode *detectCycle_hash(ListNode *head)
  {
    std::unordered_map<ListNode *, int> map;

    while (head != NULL)
    {
      if (map.count(head) != NULL)
      {
        return head;
      }
      else
      {
        map[head] = 1;
      }

      head = head->next;
    }

    return NULL;
  }
};

struct ListNode
{
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};
