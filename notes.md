# Algorithm Ideas

## 1

Go through requested kanji list. Call function A on each kanji

Function A: Copy key+value to result list, then go through radicals list. If radical is in studied list, delete it.
otherwise, call function A on it.

## 2 BETTER YET

Leave "hasBeenStudied" field in master list default to false

Iterate through Studied list and set master[kanji]["hasBeenStudied"] to true.

Go through requested kanji list. Call function A on each kanji

Function A: If master[kanji][hasBeenStudied] is True or result[kanji] exists, return false. Else, copy kanji and
radicals key+value pair to result list, then call function A on each radical. If function A returns false, delete it
from the list. *wont work right* Return true.

## 3 Deletion version

Leave "hasBeenChecked" field in master list default to false

Iterate through Studied list and delete master[kanji].

Go through requested kanji list. Call function A on each kanji

Function A: If master[kanji] doesn't exist, return false. If master[kanji][hasBeenChecked] is True, return True. Else,
set it to true. Call function A on each element in master[kanji]. If it returns false, delete the radical. {{{flag if
pure radical? or if pure kanji?}}} Return true.

## 4 Deletion  two way

Leave "hasBeenChecked" field in master list default to false

Function P: Iterate through master[kanji]["parents"] and delete self from ["children"], then delete master[kanji]

Iterate through Studied list. For each kanji, call Function P.

Go through requested kanji list. Call function A on each kanji

Function A: If master[kanji]["hasBeenChecked"] is True, return. Else, set it to true. Call function A on each element in
master[kanji]["children"]. {{{store if pure radical? or if pure kanji?}}}

### 4.1. Deletion 2 way resulting printing algoritm

In previous algorithm, store furthest depths and track by variable in recursive function.

Build heads in 2D array with index being depth. Have a "lowest depth" variable. Each time a head is created, it is added
to heads[depth]
heads[depth] should be set or dict to avoid duplication. (duplication should only happen in the same depth)

Function createHead (kanji): heads[master[kanji]["furthestDepth"]].append (kanji);

while lowest_depth >= 0
For each head in heads[lowest_depth], / *if master[kanji] exists,*/ print the kanji then for each
master[kanji]["parents"] call createHead.
// Delete master[kanji] {{{optionally delete self from children's parent list}}}.
lowest_depth--;

// commented parts: need to implement radical clumping which can cause duplication from adjacent children if parent is
printed without deleting.

## 5.

..... would it be possible for all kanji to be loaded into the depth thing after the first check and skip having to have
the parent sublist?
if doing that, could move parents of pure radicals into their first pure radical child (and not add them to the depth
list, maybe delete them from the master list after)
......also, could use stringbuilders? pass the master list once, then pass again and add each character into the string
for their depth. then merge all the depths together
That seems like the winner!!!!!!! this algo and then Deletion (one way) (or maybe two way idk) (oh but if depth
tracking, need to follow all routes so can't skip bc already hasBeenChecked and maybe need to watch deletion idk)

## changes to actual implemented algorithm

radical clumping will be difficult.  
need to move lowest parent of each pure radical into ONE of their pure radical children, and ensure any other radicals
of that parent are printed before printing the merged parent.  
OR use a different method.  
maybe find the parent that needs to be moved, then concat all of its children, then move it at the end.  
also, this could cause depths to be skipped

### idea

in parse_kanji_and_children, if a kanji has no children and it's depth is updated  
(meaning it is the new lowest point found for that kanji), return some value that tells the caller (operating on the
parent) to store that kanji as the pure radical's current lowest parent.    
then, after everything has been processed, iterate through that list and merge parents into their children.  
IDEALLY, put the child in question last, just before the parent.  
ALSO parents who are the lowest parent of two pure radical children will get flagged twice, so handle that. maybe merge
those two children on the spot

### new idea

above is inaccurate. instead, detect radicals by whether or not they are in the joyo list. however, that will be
inefficient if it has to search every single kanji. if it only searches kanji with no children, and only runs if the
user requests it, that's not bad. but maybe somehow I could do something else. maybe tag radicals in the dataset with a
character

## other thing

not sure yet but maybe completely separate trees should actually ideally align themselves with the lowest tree?  
trees with multiple tops should def align all of their tops together but I think completely disconnected trees should
prob not have their tops at the same level. For example, a kanji with no new radicals or sub kanji should not
necessarily be at the end of the list, since it has no dependencies to learn. But that seems super complicated to
implement