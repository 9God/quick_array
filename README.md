# quick_array

A memory continuous array. The operation Time complexities are:
insert_before: O(1)
insert_after: O(1)
push_back: O(1)
push_front: O(1)
remove_at: O(1)
expand_to: O(N)
shrink_to: not supported
sort: not supported

The quick array is more like a LIST that is suitable for high frequncy of insertion and removal, but avoid allocating or copy memory at runtime.
I utilize this array to implement our new matching-engine's infrastructure.
It's also quite suitable for containers in frame synchronization game.
